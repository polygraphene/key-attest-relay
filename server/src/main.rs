/*
 * Copyright 2026 polygraphene
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */
mod keymint_client;
use std::ptr;
use std::sync::OnceLock;

use base64::Engine;
use binder::Parcelable;
use km_aidl::android_hardware_security_keymint_V4::aidl::android::hardware::security::keymint::{
    KeyParameter::KeyParameter,
    SecurityLevel::SecurityLevel,
};
use km_aidl::android_system_keystore2_V5::aidl::android::system::keystore2::{
    KeyDescriptor::KeyDescriptor,
    KeyMetadata::KeyMetadata,
    KeyParameters::KeyParameters,
};
use tonic::{
    Request, Response, Status,
    metadata::MetadataValue,
    transport::{CertificateDer, Identity, Server, ServerTlsConfig},
};
pub mod keyattest {
    tonic::include_proto!("keyattest");
}

use ka_common::calc_pubkey_hash_from_certificate_der;
use keyattest::key_attest_server::{KeyAttest, KeyAttestServer};
use rustls::pki_types::pem::PemObject;

use crate::keymint_client::{GenerationResult, has_keymint, keymint_generate_key};

#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("ka-server/include/km.h");
        fn keymaster_test(save_cert: bool, cert_prefix: &String);
        fn generate_key(
            params: &Vec<u8>,
            app_id: &Vec<u8>,
            key_blob: &mut Vec<u8>,
            metadataParceled: &mut Vec<u8>,
            hasError: &mut bool,
            error_response: &mut Vec<u8>,
        );
    }
    extern "Rust" {
        unsafe fn vec_u8_set_len(v: &mut Vec<u8>, new_len: usize);
    }
}
unsafe fn vec_u8_set_len(v: &mut Vec<u8>, new_len: usize) {
    v.reserve(new_len);
    unsafe { v.set_len(new_len) }
}

static KEYMINT_EXISTS: OnceLock<bool> = OnceLock::new();

#[derive(Debug, Default)]
pub struct KeyAttestService {}

#[tonic::async_trait]
impl KeyAttest for KeyAttestService {
    async fn generate_key(
        &self,
        request: Request<keyattest::GenerateKeyRequest>,
    ) -> Result<Response<keyattest::GenerateKeyResponse>, Status> {
        eprintln!("GenerateKey request: {:?}", request);

        let parcel = binder::binder_impl::Parcel::unmarshal(request.get_ref().generate_key_parcel.as_slice());
        let borrow = parcel.borrowed_ref();

        let token_len = unsafe { ptr::read_unaligned(request.get_ref().generate_key_parcel.as_ptr().add(12) as *const u32) };
        let header_size = (16 + token_len * 2 + 3) & !3;

        unsafe {
            if let Err(e) = borrow.set_data_position(header_size as i32) {
                eprintln!("set_data_position failed: {:?}", e);
                return Err(Status::internal("Failed to set data position"));
            }
        }

        // Read arguments based on IKeystoreSecurityLevel::generateKey AIDL definition.
        let key: KeyDescriptor = match borrow.read() {
            Ok(k) => k,
            Err(e) => {
                eprintln!("Failed to read key: {:?}", e);
                return Err(Status::internal("Failed to read key"));
            }
        };
        eprintln!("Key: {:?}", key);

        let attestation_key: Option<KeyDescriptor> = borrow.read().unwrap_or(None);
        eprintln!("Attestation Key: {:?}", attestation_key);

        let params: Vec<KeyParameter> = borrow.read().unwrap_or_else(|_| Vec::new());
        eprintln!("Params: {} items", params.len());
        for p in &params {
            eprintln!("  Param: {:?}", p);
        }
        let kps = KeyParameters { keyParameter: params };

        let flags: i32 = borrow.read().unwrap_or(0);
        eprintln!("Flags: {}", flags);

        let entropy: Vec<u8> = borrow.read().unwrap_or_else(|_| Vec::new());
        eprintln!("Entropy: {} bytes", entropy.len());

        let mut has_error = false;
        let mut error_response = Vec::new();
        let metadata_and_key_blob = if !KEYMINT_EXISTS.get().unwrap_or(&false) {
            // Regenerate parcel buffer from params to feed C++
            let mut params_parcel = binder::binder_impl::Parcel::new();
            kps.write_to_parcel(&mut params_parcel.borrowed())
                .map_err(|e| Status::internal(format!("Failed to write params: {:?}", e)))?;
            let ve = params_parcel.marshal();
            eprintln!("Params parcel: {} {:?}", ve.len(), ve);
            let mut key_blob = Vec::new();
            let mut metadata_parceled = Vec::new();
            ffi::generate_key(
                &ve,
                &request.get_ref().app_id,
                &mut key_blob,
                &mut metadata_parceled,
                &mut has_error,
                &mut error_response,
            );
            if !has_error {
                // unmarshal metadata_parceled
                let metadata_parcel = binder::binder_impl::Parcel::unmarshal(metadata_parceled.as_slice());

                let metadata_borrow = metadata_parcel.borrowed_ref();
                unsafe {
                    metadata_borrow
                        .set_data_position(0)
                        .map_err(|e| Status::internal(format!("Failed to set data position: {:?}", e)))?;
                }
                //let mut metadata: mangled::_7_android_6_system_9_keystore2_11_KeyMetadata = metadata_borrow.read().unwrap();
                let mut metadata: KeyMetadata = KeyMetadata::default();
                metadata
                    .read_from_parcel(metadata_borrow)
                    .map_err(|e| Status::internal(format!("Failed to read metadata: {:?}", e)))?;
                eprintln!("Metadata: {:?}", metadata);
                GenerationResult::Generated((metadata, key_blob))
            } else {
                if error_response.is_empty() {
                    GenerationResult::NotIntercept
                } else {
                    GenerationResult::ErrorResponse(error_response)
                }
            }
        } else {
            keymint_generate_key(kps, &request.get_ref().app_id).map_err(|e| {
                eprintln!("Failed to generate key: {:?}", e);
                Status::internal(format!("Failed to generate key: {:?}", e))
            })?
        };

        let reply = match metadata_and_key_blob {
            GenerationResult::Generated((mut metadata, key_blob)) => {
                metadata.key = key;
                metadata.keySecurityLevel = SecurityLevel::TRUSTED_ENVIRONMENT;

                // re-marshal metadata
                let mut metadata_parcel = binder::binder_impl::Parcel::new();
                metadata
                    .write_to_parcel(&mut metadata_parcel.borrowed())
                    .map_err(|e| Status::internal(format!("Failed to write metadata: {:?}", e)))?;
                let metadata_ve = metadata_parcel.marshal();

                keyattest::GenerateKeyResponse {
                    generate_key_result_parcel: metadata_ve,
                    intercept: true,
                    key_blob: key_blob,
                    has_error: false,
                    error_response: Vec::new(),
                }
            }
            GenerationResult::ErrorResponse(error_response) => keyattest::GenerateKeyResponse {
                generate_key_result_parcel: Vec::new(),
                intercept: true,
                key_blob: Vec::new(),
                has_error: true,
                error_response: error_response,
            },
            GenerationResult::NotIntercept => keyattest::GenerateKeyResponse {
                generate_key_result_parcel: Vec::new(),
                intercept: false,
                key_blob: Vec::new(),
                has_error: true,
                error_response: Vec::new(),
            },
        };
        Ok(Response::new(reply))
    }
}

async fn server(addr_str: String, server_key: &[u8], server_crt: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
    let _ = KEYMINT_EXISTS.set(has_keymint());

    let addr = addr_str.parse()?;
    eprintln!("Listening on {}", addr);
    let key_attest_service = KeyAttestService::default();
    let key_attest_server = KeyAttestServer::with_interceptor(key_attest_service, check_auth);
    let identity = Identity::from_pem(server_crt, server_key);
    Server::builder()
        .tls_config(ServerTlsConfig::new().identity(identity))?
        .add_service(key_attest_server)
        .serve(addr)
        .await?;
    Ok(())
}

static AUTH_HEADER_VALUE: OnceLock<MetadataValue<tonic::metadata::Ascii>> = OnceLock::new();

fn check_auth(req: Request<()>) -> Result<Request<()>, Status> {
    let expected = AUTH_HEADER_VALUE.get().expect("AUTH_HEADER_VALUE not initialized");

    match req.metadata().get("authorization") {
        Some(t) if expected == t => Ok(req),
        _ => Err(Status::unauthenticated("No valid auth token")),
    }
}

// Auto generated token length in bytes. Token string representation will be hex of 64 characters.
const TOKEN_LEN: usize = 32;

// Run keymaster/keymint attestation.
// Save certificates to files if cert_prefix is provided.
// $ ./ka-server test-keymaster [cert_prefix]
// $ ./ka-server test-keymint [cert_prefix]
//
// Run attestation server for relay client.
// $ ./ka-server server [bind_addr] [server.key] [server.crt] [token]
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cmd = std::env::args().nth(1).unwrap_or_else(|| "test-keymaster".to_string());
    match cmd.as_str() {
        "test-keymaster" => {
            if let Some(cert_prefix) = std::env::args().nth(2) {
                ffi::keymaster_test(true, &cert_prefix);
            } else {
                ffi::keymaster_test(false, &"".to_string());
            }
            Ok(())
        }
        "test-keymint" => {
            if let Some(cert_prefix) = std::env::args().nth(2) {
                keymint_client::keymint_test(true, &cert_prefix)?;
            } else {
                keymint_client::keymint_test(false, &"".to_string())?;
            }
            Ok(())
        }
        "server" => {
            let addr = std::env::args().nth(2).expect("Bind address not provided");
            let server_key = std::env::args().nth(3).expect("Server key not provided");
            let server_crt = std::env::args().nth(4).expect("Server certificate not provided");
            let token = std::env::args().nth(5).unwrap_or_else(|| {
                eprintln!("Generating random token");
                let mut bytes = [0u8; TOKEN_LEN];
                rand::fill(&mut bytes);
                bytes.iter().map(|b| format!("{:02x}", b)).collect::<String>()
            });
            eprintln!("Auth token: {}", token);

            let server_crt2 = std::fs::read(server_crt.clone()).map_err(|e| format!("Failed to read server certificate: {}", e))?;
            let server_key = std::fs::read(server_key).map_err(|e| format!("Failed to read server key: {}", e))?;

            let cert = CertificateDer::pem_file_iter(server_crt)?.next().ok_or("No certificate found")??;
            let key_fp = calc_pubkey_hash_from_certificate_der(&cert)?;
            let key_fp_base64 = base64::prelude::BASE64_URL_SAFE_NO_PAD.encode(key_fp);
            eprintln!("kar://{}/key_fp/{}/token/{}", addr, key_fp_base64, token);

            let auth_header = MetadataValue::try_from(format!("Bearer {}", token)).map_err(|e| format!("Invalid token: {}", e))?;
            AUTH_HEADER_VALUE.set(auth_header).unwrap();

            server(addr, &server_key, &server_crt2).await?;
            Ok(())
        }
        _ => Err(format!("Unknown command: {}", cmd).into()),
    }
}

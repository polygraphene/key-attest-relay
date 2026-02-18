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
use std::str::FromStr;
use std::sync::Arc;

use hyper::Uri;
use hyper_util::rt::TokioExecutor;
use rustls::client::danger::{ServerCertVerified, ServerCertVerifier};
use rustls::crypto::aws_lc_rs::default_provider;
use rustls::pki_types::{CertificateDer, ServerName, UnixTime};
use rustls::server::ParsedCertificate;
use rustls::{CertificateError, ClientConfig};
use tonic::Status;
use rustls::crypto::verify_tls12_signature;
use rustls::crypto::verify_tls13_signature;

use crate::GenericError;
use crate::keyattest::key_attest_client::KeyAttestClient;
use crate::keyattest::{GenerateKeyRequest, GenerateKeyResponse};
use crate::{__android_log_print, ANDROID_LOG_INFO, LOG_TAG, log_info, log_print};
use hyper_util::client::legacy::connect::HttpConnector;
use ka_common::calc_pubkey_hash;
use std::ffi::CString;

#[derive(Debug)]
struct FingerprintVerifier {
    expected_fp: Vec<u8>,
}

impl ServerCertVerifier for FingerprintVerifier {
    fn verify_server_cert(
        &self,
        end_entity: &CertificateDer<'_>,
        _intermediates: &[CertificateDer<'_>],
        _server_name: &ServerName<'_>,
        _ocsp_response: &[u8],
        _now: UnixTime,
    ) -> Result<ServerCertVerified, rustls::Error> {
        let parsed_cert = ParsedCertificate::try_from(end_entity)?;

        let actual_fp = calc_pubkey_hash(&parsed_cert)?;

        log_info!("Actual fingerprint: {:x?}", actual_fp);
        log_info!("Expected fingerprint: {:x?}", self.expected_fp);
        if actual_fp.as_slice() == self.expected_fp.as_slice() {
            Ok(ServerCertVerified::assertion())
        } else {
            log_info!("Certificate verification failed");
            Err(rustls::Error::InvalidCertificate(
                CertificateError::BadSignature.into(),
            ))
        }
    }

    fn verify_tls12_signature(
        &self,
        message: &[u8],
        cert: &CertificateDer<'_>,
        dss: &rustls::DigitallySignedStruct,
    ) -> Result<rustls::client::danger::HandshakeSignatureValid, rustls::Error> {
        verify_tls12_signature(message, cert, dss, &default_provider().signature_verification_algorithms)
    }

    fn verify_tls13_signature(
        &self,
        message: &[u8],
        cert: &CertificateDer<'_>,
        dss: &rustls::DigitallySignedStruct,
    ) -> Result<rustls::client::danger::HandshakeSignatureValid, rustls::Error> {
        verify_tls13_signature(message, cert, dss, &default_provider().signature_verification_algorithms)
    }

    fn supported_verify_schemes(&self) -> Vec<rustls::SignatureScheme> {
        default_provider().signature_verification_algorithms.supported_schemes()
    }
}

pub struct KaClient {
    addr: String,
    token: String,
    key_fingerprint: Vec<u8>,
}

impl KaClient {
    pub fn new(addr: String, token: String, key_fingerprint: Vec<u8>) -> Self {
        Self {
            addr,
            token,
            key_fingerprint,
        }
    }

    pub async fn generate_key(
        &self,
        key_desc: GenerateKeyRequest,
    ) -> Result<tonic::Response<GenerateKeyResponse>, GenericError> {
        let mut rustls_config = ClientConfig::builder()
            .with_root_certificates(rustls::RootCertStore::empty())
            .with_no_client_auth();

        rustls_config
            .dangerous()
            .set_certificate_verifier(Arc::new(FingerprintVerifier {
                expected_fp: self.key_fingerprint.clone(),
            }));

        let mut http = HttpConnector::new();
        http.enforce_http(false);

        let uri =
            Uri::from_str(&self.addr).map_err(|_| Status::invalid_argument("Invalid address"))?;

        let connector = tower::ServiceBuilder::new()
            .layer_fn(move |s| {
                let tls = rustls_config.clone();

                hyper_rustls::HttpsConnectorBuilder::new()
                    .with_tls_config(tls)
                    .https_only()
                    .enable_http2()
                    .wrap_connector(s)
            })
            .map_request(move |_| uri.clone())
            .service(http);

        let client = hyper_util::client::legacy::Client::builder(TokioExecutor::new()).build(connector);

        // Server name is not verified because we already replaced verification code completely with fingerprint verification.
        let uri = Uri::from_static("https://keyattest");
        let mut client = KeyAttestClient::with_origin(client, uri);

        log_info!("Connected {} with token {}", self.addr, self.token);
        let mut request = tonic::Request::new(key_desc);
        request.metadata_mut().insert(
            "authorization",
            format!("Bearer {}", self.token).parse().unwrap(),
        );
        let response = client.generate_key(request).await?;
        Ok(response)
    }
}

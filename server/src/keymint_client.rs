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

use km_aidl::android_hardware_security_keymint::aidl::android::hardware::security::keymint::{
    self,
    ErrorCode::ErrorCode,
    IKeyMintDevice::{BpKeyMintDevice, IKeyMintDevice},
    KeyParameter::KeyParameter,
    KeyParameterValue::KeyParameterValue,
    SecurityLevel::SecurityLevel,
    Tag::Tag,
};
use km_aidl::android_system_keystore2_V5::aidl::android::system::keystore2::{
    Authorization::Authorization,
    KeyMetadata::KeyMetadata,
    KeyParameters::KeyParameters,
};

pub const UNDEFINED_NOT_AFTER: i64 = 253402300799000i64;

fn get_current_time_ms() -> i64 {
    std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis() as i64
}

pub fn has_keymint() -> bool {
    let descriptor = <BpKeyMintDevice as IKeyMintDevice>::get_descriptor();
    let instances = binder::get_declared_instances(descriptor).unwrap_or_default();
    !instances.is_empty()
}

pub fn keymint_test(save_cert: bool, cert_prefix: &str) -> Result<(), Box<dyn std::error::Error>> {
    let descriptor = <BpKeyMintDevice as IKeyMintDevice>::get_descriptor();
    println!("{}", descriptor);
    let instances = binder::get_declared_instances(descriptor)?;
    println!("{:?}", instances);
    if instances.is_empty() {
        return Err("No KeyMint instances found".into());
    }

    let service_name = format!("{}/{}", descriptor, instances[1]);
    println!("{:?}", service_name);

    let km_device = binder::wait_for_interface::<dyn IKeyMintDevice>(&service_name)?;
    println!("{:?}", km_device.getHardwareInfo()?);

    let mut key_params = vec![];
    let rsa = false;
    if rsa {
        key_params.push(KeyParameter { tag: Tag::ALGORITHM, value: KeyParameterValue::Algorithm(keymint::Algorithm::Algorithm::RSA) });
        key_params.push(KeyParameter { tag: Tag::KEY_SIZE, value: KeyParameterValue::Integer(2048) });
        key_params.push(KeyParameter { tag: Tag::PADDING, value: KeyParameterValue::PaddingMode(keymint::PaddingMode::PaddingMode::RSA_PKCS1_1_5_SIGN) });
        key_params.push(KeyParameter { tag: Tag::RSA_PUBLIC_EXPONENT, value: KeyParameterValue::LongInteger(65537) });
    } else {
        key_params.push(KeyParameter { tag: Tag::ALGORITHM, value: KeyParameterValue::Algorithm(keymint::Algorithm::Algorithm::EC) });
        key_params.push(KeyParameter { tag: Tag::KEY_SIZE, value: KeyParameterValue::Integer(256) });
        key_params.push(KeyParameter { tag: Tag::EC_CURVE, value: KeyParameterValue::EcCurve(keymint::EcCurve::EcCurve::P_256) });
    }
    key_params.push(KeyParameter { tag: Tag::DIGEST, value: KeyParameterValue::Digest(keymint::Digest::Digest::SHA_2_256) });
    key_params.push(KeyParameter { tag: Tag::PURPOSE, value: KeyParameterValue::KeyPurpose(keymint::KeyPurpose::KeyPurpose::SIGN) });
    key_params.push(KeyParameter { tag: Tag::NO_AUTH_REQUIRED, value: KeyParameterValue::BoolValue(true) });
    if true {
        key_params.push(KeyParameter { tag: Tag::CERTIFICATE_NOT_BEFORE, value: KeyParameterValue::DateTime(0) });
        key_params.push(KeyParameter { tag: Tag::CERTIFICATE_NOT_AFTER, value: KeyParameterValue::DateTime(UNDEFINED_NOT_AFTER) });
    } else {
        key_params.push(KeyParameter { tag: Tag::CERTIFICATE_NOT_BEFORE, value: KeyParameterValue::DateTime(get_current_time_ms() - 1000 * 1000) });
        key_params.push(KeyParameter { tag: Tag::CERTIFICATE_NOT_AFTER, value: KeyParameterValue::DateTime(get_current_time_ms() + 1000 * 1000) });
    }
    key_params.push(KeyParameter { tag: Tag::ATTESTATION_CHALLENGE, value: KeyParameterValue::Blob(("challenge123".repeat(1)).as_bytes().to_vec()) });

    let app_id = vec![0x30, 0x36, 0x31, 0x10, 0x30, 0x0e, 0x04, 0x09, 0x74, 0x65, 0x73, 0x74, 0x61, 0x70, 0x70, 0x69, 0x64, 0x02, 0x01, 0x0b, 0x31, 0x22, 0x04, 0x20, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f];
    key_params.push(KeyParameter { tag: Tag::ATTESTATION_APPLICATION_ID, value: KeyParameterValue::Blob(app_id) });
    key_params.push(KeyParameter { tag: Tag::CERTIFICATE_SERIAL, value: KeyParameterValue::Blob(vec![1]) });
    let cert_subject = vec![0x30, 0x1f, 0x31, 0x1d, 0x30, 0x1b, 0x06, 0x03, 0x55, 0x04, 0x03, 0x0c, 0x14, 0x41, 0x6e, 0x64, 0x72, 0x6f, 0x69, 0x64, 0x20, 0x4b, 0x65, 0x79, 0x73, 0x74, 0x6f, 0x72, 0x65, 0x20, 0x4b, 0x65, 0x79];
    key_params.push(KeyParameter { tag: Tag::CERTIFICATE_SUBJECT, value: KeyParameterValue::Blob(cert_subject) });
    key_params.push(KeyParameter { tag: Tag::CREATION_DATETIME, value: KeyParameterValue::DateTime(std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs() as i64 * 1000) });

    let result = km_device.generateKey(&key_params, None)?;

    for kc in &result.keyCharacteristics {
        println!("{:?}", kc);
    }
    for (i, cert) in result.certificateChain.iter().enumerate() {
        println!("{:?}", cert);
        if save_cert {
            let cert_bytes = cert.encodedCertificate.as_slice();
            let cert_path = format!("{}{}.der", cert_prefix, i);
            std::fs::write(&cert_path, cert_bytes)?;
            println!("Certificate saved to {}", cert_path);
        }
    }

    Ok(())
}

fn is_device_id_attestation_kp(tag: Tag) -> bool {
    match tag {
        Tag::ATTESTATION_ID_BRAND => true,
        Tag::ATTESTATION_ID_DEVICE => true,
        Tag::ATTESTATION_ID_PRODUCT => true,
        Tag::ATTESTATION_ID_SERIAL => true,
        Tag::ATTESTATION_ID_IMEI => true,
        Tag::ATTESTATION_ID_MEID => true,
        Tag::ATTESTATION_ID_MANUFACTURER => true,
        Tag::ATTESTATION_ID_MODEL => true,
        _ => false,
    }
}
pub enum GenerationResult {
    Generated((KeyMetadata, Vec<u8>)),
    ErrorResponse(Vec<u8>),
    NotIntercept,
}

pub fn keymint_generate_key(kps: KeyParameters, app_id: &[u8]) -> Result<GenerationResult, Box<dyn std::error::Error>> {
    let mut kp = kps.keyParameter;
    kp.push(KeyParameter { tag: Tag::CREATION_DATETIME, value: KeyParameterValue::DateTime(get_current_time_ms()) });
    kp.push(KeyParameter { tag: Tag::ATTESTATION_APPLICATION_ID, value: KeyParameterValue::Blob(app_id.to_vec()) });
    if !kp.iter().any(|kp| kp.tag == Tag::ATTESTATION_CHALLENGE) {
        return Err("Not attestation".into());
    }
    if kp.iter().any(|kp| is_device_id_attestation_kp(kp.tag)) {
        let mut parcel = binder::binder_impl::Parcel::new();
        const EX_SERVICE_SPECIFIC: i32 = -8;
        parcel.write(&EX_SERVICE_SPECIFIC)?;

        // Dummy Empty String16
        parcel.write(&0)?;
        parcel.write(&0)?;

        // Stack trace header size
        parcel.write(&0)?;

        // Keymint error code
        parcel.write(&ErrorCode::CANNOT_ATTEST_IDS)?;

        return Ok(GenerationResult::ErrorResponse(parcel.marshal()));
    }

    let descriptor = <BpKeyMintDevice as IKeyMintDevice>::get_descriptor();
    let instances = binder::get_declared_instances(descriptor)?;
    if instances.is_empty() {
        return Err("No KeyMint instances found".into());
    }

    let use_instance = instances.iter().position(|x| x == "default").ok_or("No TEE KeyMint instance found".to_string())?;
    // let use_instance = if instances.contains(&"strongbox".to_string()) {
    //     instances.iter().position(|x| x == "strongbox").unwrap()
    // } else if instances.contains(&"default".to_string()) {
    //     instances.iter().position(|x| x == "default").unwrap()
    // } else {
    //     return Err("No known valid instance.".into());
    // };
    let service_name = format!("{}/{}", descriptor, instances[use_instance]);
    println!("Instance selection: instances={:?} index={}", instances, use_instance);
    let km_device = binder::wait_for_interface::<dyn IKeyMintDevice>(&service_name)?;
    println!("hardware_info={:?}", km_device.getHardwareInfo()?);

    let result = km_device.generateKey(&kp, None)?;

    let mut metadata = KeyMetadata::default();
    metadata.keySecurityLevel = if instances[use_instance] == "strongbox" { SecurityLevel::STRONGBOX } else { SecurityLevel::TRUSTED_ENVIRONMENT };

    for kc in result.keyCharacteristics {
        println!("{:?}", kc);
        for auth in kc.authorizations {
            metadata.authorizations.push(Authorization { securityLevel: kc.securityLevel, keyParameter: auth });
        }
    }
    metadata.authorizations.push(Authorization { securityLevel: SecurityLevel::SOFTWARE, keyParameter: KeyParameter { tag: Tag::USER_ID, value: KeyParameterValue::Integer(0) } });
    metadata.certificate = Some(result.certificateChain[0].encodedCertificate.clone());
    metadata.certificateChain = Some(vec![]);
    for (i, cert) in result.certificateChain.iter().enumerate() {
        println!("{:?}", cert);
        if i == 0 {
            continue;
        }
        metadata.certificateChain.as_mut().unwrap().extend(cert.encodedCertificate.as_slice());
    }

    Ok(GenerationResult::Generated((metadata, result.keyBlob)))
}

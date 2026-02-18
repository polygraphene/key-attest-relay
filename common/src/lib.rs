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

use rustls::CertificateError;
use rustls::pki_types::CertificateDer;
use rustls::server::ParsedCertificate;
use sha2::{Digest, Sha256};

pub fn calc_pubkey_hash_from_certificate_der(certificate_der: &CertificateDer) -> Result<Vec<u8>, rustls::Error> {
    let parsed_cert = ParsedCertificate::try_from(certificate_der)?;
    calc_pubkey_hash(&parsed_cert)
}

pub fn calc_pubkey_hash(parsed_cert: &ParsedCertificate) -> Result<Vec<u8>, rustls::Error> {
    let mut hasher = Sha256::new();
    let spki = parsed_cert.subject_public_key_info();
    let spki_bytes = spki.as_ref();

    // Extract the bit string (the actual public key) from SPKI
    // SPKI is a SEQUENCE of (AlgorithmIdentifier, subjectPublicKey BIT STRING)
    let mut offset = 0;
    let pk = (|| {
        // SEQUENCE (SPKI)
        if *spki_bytes.get(offset)? != 0x30 {
            return None;
        }
        offset += 1;
        let (_, len_bytes) = parse_asn1_len(&spki_bytes.get(offset..)?)?;
        offset += len_bytes;

        // AlgorithmIdentifier (SEQUENCE)
        if *spki_bytes.get(offset)? != 0x30 {
            return None;
        }
        offset += 1;
        let (alg_len, alg_len_bytes) = parse_asn1_len(&spki_bytes.get(offset..)?)?;
        offset += alg_len_bytes + alg_len;

        // subjectPublicKey (BIT STRING)
        if *spki_bytes.get(offset)? != 0x03 {
            return None;
        }
        offset += 1;
        let (bit_str_len, bit_str_len_bytes) = parse_asn1_len(&spki_bytes.get(offset..)?)?;
        offset += bit_str_len_bytes;

        // BIT STRING has one byte of "unused bits" padding at the start.
        if bit_str_len < 1 {
            return None;
        }
        if spki_bytes.get(offset + 1..offset + bit_str_len).is_none() {
            return None;
        }
        Some(&spki_bytes[offset + 1..offset + bit_str_len])
    })()
    .ok_or(rustls::Error::InvalidCertificate(CertificateError::BadEncoding.into()))?;

    hasher.update(pk);
    let actual_fp = hasher.finalize();
    Ok(actual_fp.to_vec())
}

fn parse_asn1_len(data: &[u8]) -> Option<(usize, usize)> {
    let first = *data.get(0)?;
    if first < 0x80 {
        Some((first as usize, 1))
    } else {
        let n = (first & 0x7f) as usize;
        if n == 0 || n > 4 || data.len() < 1 + n {
            return None;
        }
        let mut len = 0usize;
        for i in 0..n {
            len = (len << 8) | (data[1 + i] as usize);
        }
        Some((len, 1 + n))
    }
}

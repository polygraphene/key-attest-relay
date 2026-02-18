use km_aidl::android_system_keystore2_V5::aidl::android::system::keystore2::{IKeystoreSecurityLevel, IKeystoreService};
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
use km_aidl::android_system_keystore2_V5::aidl::android::system::keystore2::KeyDescriptor::KeyDescriptor;
use km_aidl::android_system_keystore2_V5::aidl::android::system::keystore2::KeyMetadata::KeyMetadata;
use km_aidl::android_system_keystore2_V5::aidl::android::system::keystore2::KeyEntryResponse::KeyEntryResponse;
use km_aidl::android_system_keystore2_V5::aidl::android::system::keystore2::Domain::Domain;
use km_aidl::android_hardware_security_keymint_V4::aidl::android::hardware::security::keymint::Tag::Tag;
use km_aidl::android_hardware_security_keymint_V4::aidl::android::hardware::security::keymint::KeyParameter::KeyParameter;
use libc;
use std::cell::Cell;
use std::ffi::{CStr, CString};
use std::mem;
use std::ptr;
use std::sync::Mutex;
use tonic::Status;
pub mod keyattest {
    tonic::include_proto!("keyattest");
}
use binder::Parcelable;
use std::collections::BTreeMap;
use std::sync::OnceLock;
mod kaclient;
use kaclient::KaClient;

#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("libkarelay/include/appid.h");
        fn get_app_id(uid: i32, result: &mut u32) -> Vec<u8>;
    }
}

const ANDROID_LOG_INFO: libc::c_int = 4;
const LOG_TAG: *const libc::c_char = "LIBKARELAY\0".as_ptr() as *const _;

unsafe extern "C" {
    fn __android_log_print(
        prio: libc::c_int,
        tag: *const libc::c_char,
        fmt: *const libc::c_char,
        ...
    ) -> libc::c_int;
}

#[macro_export]
macro_rules! log_print {
    ($prio:expr, $($arg:tt)*) => {
        {
            let s = format!($($arg)*);
            if let Ok(c_str) = CString::new(s) {
                #[allow(unused_unsafe)]
                unsafe {
                    __android_log_print($prio, LOG_TAG, "%s\0".as_ptr() as *const libc::c_char, c_str.as_ptr());
                }
            }
        }
    };
}

#[macro_export]
macro_rules! log_info {
    ($($arg:tt)*) => {
        log_print!(ANDROID_LOG_INFO, $($arg)*)
    };
}

#[derive(Debug)]
#[allow(dead_code)]
enum GenericError {
    TonicStatus(Status),
    TransportError(tonic::transport::Error),
    Message(String),
}

impl From<Status> for GenericError {
    fn from(s: Status) -> Self {
        GenericError::TonicStatus(s)
    }
}

impl From<tonic::transport::Error> for GenericError {
    fn from(e: tonic::transport::Error) -> Self {
        GenericError::TransportError(e)
    }
}

impl From<&str> for GenericError {
    fn from(s: &str) -> Self {
        GenericError::Message(s.to_string())
    }
}

impl std::error::Error for GenericError {}

impl std::fmt::Display for GenericError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GenericError::TonicStatus(s) => write!(f, "TonicStatus: {}", s),
            GenericError::TransportError(e) => write!(f, "TransportError: {}", e),
            GenericError::Message(s) => write!(f, "Message: {}", s),
        }
    }
}

#[repr(C)]
struct Elf64_Dyn {
    d_tag: i64,
    d_un: u64,
}

#[repr(C)]
struct Elf64_Rela {
    r_offset: u64,
    r_info: u64,
    r_addend: i64,
}

#[repr(C)]
struct Elf64_Sym {
    st_name: u32,
    st_info: u8,
    st_other: u8,
    st_shndx: u16,
    st_value: u64,
    st_size: u64,
}

const DT_NULL: i64 = 0;
const DT_PLTRELSZ: i64 = 2;
const DT_STRTAB: i64 = 5;
const DT_SYMTAB: i64 = 6;
const DT_JMPREL: i64 = 23;

const BINDER_WRITE_READ: libc::c_ulong = 0xc0306201;
const BC_REPLY: u32 = 0x40406301;
const BC_TRANSACTION: u32 = 0x40406300;
const BC_TRANSACTION_SEC_CTX: u32 = 0x40486300;
const BC_TRANSACTION_SG: u32 = 0x40486311;

const BR_TRANSACTION: u32 = 0x80407202;
const BR_TRANSACTION_SEC_CTX: u32 = 0x80487202;
const BR_NOOP: u32 = 0x720c;
const BR_TRANSACTION_COMPLETE: u32 = 0x7206;
const BR_REPLY: u32 = 0x80407203;

fn command_to_string(cmd: u32) -> String {
    match cmd {
        BC_REPLY => "BC_REPLY".to_string(),
        BC_TRANSACTION => "BC_TRANSACTION".to_string(),
        BC_TRANSACTION_SEC_CTX => "BC_TRANSACTION_SEC_CTX".to_string(),
        BC_TRANSACTION_SG => "BC_TRANSACTION_SG".to_string(),
        BR_TRANSACTION => "BR_TRANSACTION".to_string(),
        BR_TRANSACTION_SEC_CTX => "BR_TRANSACTION_SEC_CTX".to_string(),
        BR_NOOP => "BR_NOOP".to_string(),
        BR_TRANSACTION_COMPLETE => "BR_TRANSACTION_COMPLETE".to_string(),
        BR_REPLY => "BR_REPLY".to_string(),
        _ => format!("{:x}", cmd),
    }
}

// Do hexdump of the buffer
fn hexdump(buf: *const u8, len: usize) {
    for i in (0..len).step_by(16) {
        let mut line = String::new();
        line.push_str(&format!("{:04x}: ", i));

        let mut hex_part = String::new();
        let mut ascii_part = String::new();

        for j in 0..16 {
            if i + j < len {
                let b = unsafe { *buf.add(i + j) };
                hex_part.push_str(&format!("{:02x} ", b));
                if b >= 0x20 && b <= 0x7e {
                    ascii_part.push(b as char);
                } else {
                    ascii_part.push('.');
                }
            } else {
                hex_part.push_str("   ");
                ascii_part.push(' ');
            }
        }

        log_info!("{} {} |{}|", line, hex_part, ascii_part);
    }
}

#[repr(C)]
#[derive(Clone, Debug)]
struct binder_write_read {
    write_size: u64,
    write_consumed: u64,
    write_buffer: u64,
    read_size: u64,
    read_consumed: u64,
    read_buffer: u64,
}

#[repr(C)]
#[derive(Default)]
struct binder_transaction_data {
    target: binder_transaction_data_target,
    cookie: u64,
    code: u32,
    flags: u32,
    sender_pid: i32,
    sender_euid: u32,
    data_size: u64,
    offsets_size: u64,
    data: binder_transaction_data_data,
}

#[repr(C)]
union binder_transaction_data_target {
    handle: u32,
    ptr: u64,
}
impl Default for binder_transaction_data_target {
    fn default() -> Self {
        Self { handle: 0 }
    }
}
#[repr(C)]
union binder_transaction_data_data {
    ptr: binder_transaction_data_data_ptr,
    buf: [u8; 8],
}
impl Default for binder_transaction_data_data {
    fn default() -> Self {
        Self { buf: [0; 8] }
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
struct binder_transaction_data_data_ptr {
    buffer: u64,
    offsets: u64,
}

static mut ORIGINAL_IOCTL: Option<
    unsafe extern "C" fn(libc::c_int, libc::c_ulong, ...) -> libc::c_int,
> = None;

static GENERATED_KEYS: OnceLock<Mutex<BTreeMap<(u32, KeyDescriptor), Vec<u8>>>> = OnceLock::new();
static KA_CLIENT: OnceLock<KaClient> = OnceLock::new();

fn get_generated_keys() -> &'static Mutex<BTreeMap<(u32, KeyDescriptor), Vec<u8>>> {
    GENERATED_KEYS.get_or_init(|| Mutex::new(BTreeMap::new()))
}

fn parse_generate_key(
    slice: &[u8],
) -> Result<(bool, bool, KeyDescriptor), Box<dyn std::error::Error>> {
    let parcel = binder::binder_impl::Parcel::unmarshal(slice);
    let borrow = parcel.borrowed_ref();

    let token_len =
        unsafe { ptr::read_unaligned((slice.as_ptr() as *const u8).add(12) as *const u32) };
    let header_size = (16 + token_len * 2 + 3) & !3;

    unsafe {
        if let Err(e) = borrow.set_data_position(header_size as i32) {
            log_info!("set_data_position failed: {:?}\n", e);
            return Err(e.into());
        }
    }

    // Read arguments based on IKeystoreSecurityLevel::generateKey AIDL definition.
    let key: KeyDescriptor = match borrow.read() {
        Ok(k) => k,
        Err(e) => {
            log_info!("Failed to read key: {:?}\n", e);
            return Err(e.into());
        }
    };
    log_info!("Key: {:?}\n", key);

    let attestation_key: Option<KeyDescriptor> = borrow.read().unwrap_or(None);
    log_info!("Attestation Key: {:?}\n", attestation_key);
    let has_attestation_key = attestation_key.is_some();
    if has_attestation_key {
        // Currently unsupported. Sorry!
        log_info!(
            "Attestation Key is present, but currently unsupported. Fallback to keystore2.\n"
        );
    }

    let params: Vec<KeyParameter> =
        borrow.read().unwrap_or_else(|_| Vec::new());
    log_info!("Params: {} items\n", params.len());
    let mut has_attestation_challenge = false;
    for p in &params {
        if p.tag == Tag::ATTESTATION_CHALLENGE {
            has_attestation_challenge = true;
        }

        log_info!("  Param: {:?}\n", p);
    }

    let flags: i32 = borrow.read().unwrap_or(0);
    log_info!("Flags: {}\n", flags);

    let entropy: Vec<u8> = borrow.read().unwrap_or_else(|_| Vec::new());
    log_info!("Entropy: {} bytes\n", entropy.len());

    Ok((has_attestation_key, has_attestation_challenge, key))
}

fn on_generate_key(buf: u64, len: u64, uid: u32) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    log_info!(
        "on_generate_key: buf=0x{:x}, len={}, uid={}\n",
        buf,
        len,
        uid
    );

    let slice = unsafe { std::slice::from_raw_parts(buf as *const u8, len as usize) };
    let (has_attestation_key, has_attestation_challenge, key) = parse_generate_key(slice)?;
    if has_attestation_key {
        return Err(
            "Attestation Key is present, but currently unsupported. Fallback to keystore2.".into(),
        );
    }
    if !has_attestation_challenge {
        return Err("Attestation Challenge is not present. Fallback to keystore2.".into());
    }

    let mut result: u32 = 0;
    // Create another thread to run get_app_id
    let app_id = std::thread::spawn(move || {
        let app_id = ffi::get_app_id(uid as i32, &mut result);
        app_id
    })
    .join()
    .unwrap();
    log_info!("Result: {:?}\n", result);
    log_info!("App ID:\n");
    hexdump(app_id.as_ptr(), app_id.len());

    // Connect grpc tcp server
    let result: Result<tonic::Response<keyattest::GenerateKeyResponse>, GenericError> =
        tokio::runtime::Runtime::new().unwrap().block_on(async {
            KA_CLIENT.get().unwrap().generate_key(keyattest::GenerateKeyRequest {
                generate_key_parcel: slice.to_vec(),
                app_id: app_id,
            }).await
        });
    match result {
        Ok(res) => {
            log_info!("gRPC call successful: {:?}\n", res);

            let res = res.into_inner();
            if !res.intercept {
                log_info!("Not intercept\n");
                return Err("Not intercept".into());
            }
            if !res.has_error {
                log_info!("Attestation succeeded.\n");
                let key_id = rand::random::<i64>();
                let key_generated = KeyDescriptor {
                    domain: Domain::KEY_ID,
                    nspace: key_id,
                    ..Default::default()
                };

                let metadata_parcel = binder::binder_impl::Parcel::unmarshal(&res.generate_key_result_parcel.as_slice());

                let metadata_borrow = metadata_parcel.borrowed_ref();
                unsafe {
                    metadata_borrow.set_data_position(0)?;
                }
                let mut metadata: KeyMetadata = KeyMetadata::default();
                metadata.read_from_parcel(metadata_borrow)?;
                log_info!("Generated key id: {:?}", key_generated);

                metadata.key = key_generated.clone();
                format!("{:?}", metadata)
                    .as_bytes()
                    .chunks(120)
                    .for_each(|chunk| {
                        log_info!("M:{}", String::from_utf8_lossy(chunk));
                    });

                // re-marshal metadata
                let mut metadata_parcel = binder::binder_impl::Parcel::new();
                metadata
                    .write_to_parcel(&mut metadata_parcel.borrowed())
                    .map_err(|e| Status::internal(format!("Failed to write metadata: {:?}", e)))?;
                let metadata_ve = metadata_parcel.marshal();
                get_generated_keys()
                    .lock()
                    .unwrap()
                    .insert((uid, key), metadata_ve.clone());
                let mut d2 = vec![];

                d2.extend_from_slice(&0u32.to_le_bytes());
                d2.extend_from_slice(&1u32.to_le_bytes());
                d2.extend_from_slice(&metadata_ve);
                Ok(d2)
            } else {
                log_info!("Attestation failed: {:?}\n", res.error_response);
                Ok(res.error_response)
            }
        }
        Err(e) => {
            log_info!("gRPC call failed: {:?}\n", e);
            Err(e.into())
        }
    }
}

fn on_get_key_entry(buf: u64, len: u64, uid: u32) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    log_info!(
        "on_get_key_entry: buf=0x{:x}, len={}, uid={}\n",
        buf,
        len,
        uid
    );
    let slice = unsafe { std::slice::from_raw_parts(buf as *const u8, len as usize) };
    let parcel = binder::binder_impl::Parcel::unmarshal(slice);
    let borrow = parcel.borrowed_ref();

    let token_len = unsafe { ptr::read_unaligned((buf + 12) as *const u32) };
    let header_size = (16 + token_len * 2 + 3) & !3;

    unsafe {
        if let Err(e) = borrow.set_data_position(header_size as i32) {
            log_info!("set_data_position failed: {:?}\n", e);
            return Err(e.into());
        }
    }
    let key_descriptor: KeyDescriptor = borrow.read()?;
    log_info!("key descriptor: {:?}", key_descriptor);
    let d = match get_generated_keys()
        .lock()
        .unwrap()
        .get(&(uid, key_descriptor))
    {
        Some(d) => {
            log_info!("Key found: Length:{}\n", d.len());
            d.clone()
        }
        None => {
            log_info!("Key not found\n");
            return Err("Key not found".into());
        }
    };
    let slice = unsafe { std::slice::from_raw_parts(d.as_ptr() as *const u8, d.len()) };
    let parcel = binder::binder_impl::Parcel::unmarshal(slice);
    let borrow = parcel.borrowed_ref();

    let re = unsafe { borrow.set_data_position(0) };
    log_info!(
        "set_data_position: {:?} data_size: {}",
        re,
        borrow.get_data_size()
    );

    let mut meta2 = KeyMetadata::default();
    let result = meta2.read_from_parcel(borrow);
    log_info!("meta2: {:?} result:{:?}", meta2, result);

    let key_result = KeyEntryResponse {
        iSecurityLevel: None,
        metadata: meta2,
    };
    let mut result_parcel = binder::binder_impl::Parcel::new();
    result_parcel.write(&0u32)?;
    result_parcel.write(&key_result)?;
    let result_ve = result_parcel.marshal();
    Ok(result_ve)
}

fn on_delete_key(buf: u64, len: u64, uid: u32) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    log_info!("on_delete_key: buf=0x{:x}, len={}, uid={}", buf, len, uid);
    let slice = unsafe { std::slice::from_raw_parts(buf as *const u8, len as usize) };
    let parcel = binder::binder_impl::Parcel::unmarshal(slice);
    let borrow = parcel.borrowed_ref();

    let token_len = unsafe { ptr::read_unaligned((buf + 12) as *const u32) };
    let header_size = (16 + token_len * 2 + 3) & !3;

    unsafe {
        if let Err(e) = borrow.set_data_position(header_size as i32) {
            log_info!("set_data_position failed: {:?}\n", e);
            return Err(e.into());
        }
    }
    let key_descriptor: KeyDescriptor = borrow.read()?;
    let mut keys = get_generated_keys().lock().unwrap();
    if keys.contains_key(&(uid, key_descriptor.clone())) {
        log_info!("Deleting key descriptor: {:?}", key_descriptor);
        keys.remove(&(uid, key_descriptor));
        Ok(vec![0, 0, 0, 0])
    } else {
        // Fallback to keystore2 for keys which are not managed by us.
        Err("Key not found".into())
        // log_info!("Key not found: {:?}", key_descriptor);
        // const EX_SERVICE_SPECIFIC: i32 = -8;
        // let mut v = vec![];
        // v.extend_from_slice(&EX_SERVICE_SPECIFIC.to_le_bytes());
        // v.extend_from_slice(&0u32.to_le_bytes());
        // v.extend_from_slice(&0u32.to_le_bytes());
        // v.extend_from_slice(&0u32.to_le_bytes());
        // v.extend_from_slice(
        //     &km_aidl::mangled::_7_android_6_system_9_keystore2_12_ResponseCode::KEY_NOT_FOUND
        //         .0
        //         .to_le_bytes(),
        // );
        // Ok(v)
    }
}

fn intercept_ioctl(
    _fd: libc::c_int,
    _request: libc::c_ulong,
    _arg: *mut libc::c_void,
) -> (bool, libc::c_int) {
    return (false, 0);
}

#[derive(Debug, Copy, Clone)]
enum Method {
    None,
    GenerateKey,
    DeleteKey,
    GetKey,
}
thread_local! {
    static FLAG : Cell<u32> = const { Cell::new(0) };
    static CURRENT_METHOD : Cell<Method> = const { Cell::new(Method::None) };
}
fn process_ioctl_response(
    _fd: libc::c_int,
    request: libc::c_ulong,
    arg: *mut libc::c_void,
) -> Option<Vec<u8>> {
    let mut ret = None;
    if request == BINDER_WRITE_READ {
        let bwr = unsafe { &*(arg as *const binder_write_read) };

        if FLAG.get() > 0 {
            //FLAG.set(FLAG.get() - 1);
            log_info!("write: {} {}\n", bwr.write_size, bwr.write_consumed);
            if bwr.write_size > 0 {
                let mut pos = 0;
                while pos + 4 <= bwr.write_size {
                    let cmd_ptr = (bwr.write_buffer + pos) as *const u32;
                    let cmd = unsafe { *cmd_ptr };
                    let size = (cmd >> 16) & 0x3fff;
                    log_info!("write cmd:{} pos: {}", command_to_string(cmd), pos);

                    if cmd == BC_REPLY
                        || cmd == BC_TRANSACTION
                        || cmd == BC_TRANSACTION_SEC_CTX
                        || cmd == BC_TRANSACTION_SG
                    {
                        if pos + 4 + mem::size_of::<binder_transaction_data>() as u64
                            <= bwr.write_size
                        {
                            let data_ptr =
                                (bwr.write_buffer + pos + 4) as *const binder_transaction_data;
                            let data = unsafe { ptr::read_unaligned(data_ptr) };
                            log_info!(
                                "write: 0x{:08x}, {}, code={}, data_size={}",
                                cmd,
                                command_to_string(cmd),
                                data.code,
                                data.data_size,
                            );
                            if cmd == BC_REPLY {
                                if bwr.read_size >= 8 {
                                    unsafe {
                                        log_info!(
                                            "BC_REPLY: {:?}, {:x} {:x}",
                                            bwr,
                                            *(bwr.read_buffer as *const u32),
                                            *((bwr.read_buffer + 4) as *const u32)
                                        );
                                    }
                                }
                                FLAG.set(0);
                                let buf = unsafe { data.data.ptr }.buffer;
                                hexdump(buf as *const u8, data.data_size as usize);
                                let slice = unsafe {
                                    std::slice::from_raw_parts(
                                        buf as *const u8,
                                        data.data_size as usize,
                                    )
                                };
                                let parcel = binder::binder_impl::Parcel::unmarshal(slice);
                                let borrow = parcel.borrowed_ref();

                                let _ = unsafe { borrow.set_data_position(0) };

                                let error_code: Result<u32, _> = borrow.read();

                                let v: Result<u32, _> = borrow.read();
                                //let k: Result<u32, _> = borrow.read();
                                match CURRENT_METHOD.get() {
                                    Method::None => {}
                                    Method::GenerateKey => {
                                        let mut meta2: KeyMetadata = KeyMetadata::default();
                                        let result = meta2.read_from_parcel(borrow);
                                        //let meta:Result<km_aidl::mangled::_7_android_6_system_9_keystore2_11_KeyMetadata, _> = borrow.read();
                                        let meta_str = format!("{:?}", meta2);
                                        // Split meta_str into 120 chars per line
                                        let meta_lines: Vec<&str> = meta_str
                                            .as_bytes()
                                            .chunks(120)
                                            .map(|s| std::str::from_utf8(s).unwrap())
                                            .collect();
                                        for line in meta_lines {
                                            log_info!("GenKey: {}", line);
                                        }
                                        log_info!(
                                            "Generate key result: {:?} {:?} result:{:?} {:?}",
                                            error_code,
                                            v,
                                            result,
                                            meta2
                                        );
                                    }
                                    Method::DeleteKey => {}
                                    Method::GetKey => {}
                                }
                            }
                        }
                    }
                    pos += 4 + size as u64;
                }
            }
        }
        if bwr.read_size > 0 {
            let mut pos = 0;
            while pos + 4 <= bwr.read_consumed {
                let cmd_ptr = (bwr.read_buffer + pos) as *const u32;
                let cmd = unsafe { *cmd_ptr };
                let size = (cmd >> 16) & 0x3fff;
                log_info!("cmd:{} pos: {}\n", command_to_string(cmd), pos);

                if cmd == BR_TRANSACTION || cmd == BR_TRANSACTION_SEC_CTX {
                    if pos + 4 + mem::size_of::<binder_transaction_data>() as u64
                        <= bwr.read_consumed
                    {
                        let data_ptr =
                            (bwr.read_buffer + pos + 4) as *const binder_transaction_data;
                        let data = unsafe { ptr::read_unaligned(data_ptr) };
                        log_info!(
                            "read: 0x{:08x}, {}, code={}, data_size={}\n",
                            cmd,
                            command_to_string(cmd),
                            data.code,
                            data.data_size,
                        );
                        if data.data_size >= 16 {
                            let buf = unsafe { data.data.ptr }.buffer;
                            let data_ptr = buf + 12;
                            let len = unsafe { ptr::read_unaligned(data_ptr as *const u32) };
                            let data_ptr = buf + 16;
                            // Read len characters (code points) of utf16le from data_ptr
                            let interface_token = String::from_utf16_lossy(unsafe {
                                std::slice::from_raw_parts(data_ptr as *const u16, len as usize)
                            });
                            log_info!("interface_token: {:?}\n", interface_token);
                            //hexdump(buf as *const u8, data.data_size as usize);

                            let enable = true;
                            if interface_token == "android.system.keystore2.IKeystoreService" {
                                if data.code == IKeystoreService::transactions::r#getKeyEntry {
                                    if enable {
                                        let res = on_get_key_entry(buf, data.data_size, data.sender_euid);
                                        if res.is_ok() {
                                            ret = Some(res.unwrap());
                                        }
                                    } else {
                                        FLAG.set(1);
                                        CURRENT_METHOD.set(Method::GetKey);
                                    }
                                } else if data.code == IKeystoreService::transactions::r#deleteKey {
                                    if enable {
                                        let res = on_delete_key(buf, data.data_size, data.sender_euid);
                                        if res.is_ok() {
                                            ret = Some(res.unwrap());
                                        }
                                    } else {
                                        FLAG.set(1);
                                        CURRENT_METHOD.set(Method::DeleteKey);
                                    }
                                }
                            } else if interface_token == "android.system.keystore2.IKeystoreSecurityLevel" {
                                if data.code == IKeystoreSecurityLevel::transactions::r#generateKey {
                                    if enable {
                                        let res = on_generate_key(buf, data.data_size, data.sender_euid);
                                        if res.is_ok() {
                                            ret = Some(res.unwrap());
                                        }
                                    } else {
                                        FLAG.set(1);
                                        CURRENT_METHOD.set(Method::GenerateKey);
                                        log_info!(
                                            "on_generate_key (disabled): buf=0x{:x}, len={}\n",
                                            buf,
                                            len,
                                        );

                                        let slice = unsafe { std::slice::from_raw_parts(buf as *const u8, data.data_size as usize) };
                                        let _ = parse_generate_key(slice);
                                    }
                                }
                            }
                        }
                    }
                }

                pos += 4 + size as u64;
            }
        }
    }
    ret
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn hooked_ioctl(
    fd: libc::c_int,
    request: libc::c_ulong,
    arg: *mut libc::c_void,
) -> libc::c_int {
    unsafe {
        log_info!("ioctl(fd={}, request=0x{:x}) hooked!\n", fd, request);

        match intercept_ioctl(fd, request, arg) {
            (true, ret) => return ret,
            _ => {}
        }

        if let Some(original) = ORIGINAL_IOCTL {
            if request == BINDER_WRITE_READ {
                let bwr2 = &mut *(arg as *mut binder_write_read);
                let copied_bwr = bwr2.clone();
                // log_info!("bwr2 before ioctl: {:?}\n", bwr2);
                let ret = original(fd, request, arg);
                // log_info!("bwr2 after ioctl: {:?}\n", bwr2);
                // log_info!("copied_bwr after ioctl: {:?}\n", copied_bwr);
                let ret2 = process_ioctl_response(fd, request, arg);
                if let Some(ret2) = ret2 {
                    let mut bwr = std::mem::zeroed::<binder_write_read>();
                    let mut write_buf = vec![];
                    // write_buf.extend_from_slice(&BR_NOOP.to_le_bytes());
                    // write_buf.extend_from_slice(&BR_TRANSACTION_COMPLETE.to_le_bytes());
                    write_buf.extend_from_slice(&BC_REPLY.to_le_bytes());

                    let mut tr = binder_transaction_data::default();
                    tr.data_size = ret2.len() as u64;
                    tr.data.ptr.buffer = ret2.as_ptr() as u64;
                    tr.target.ptr = 0;
                    tr.target.handle = 0xFFFFFFFF;
                    tr.code = 0;
                    tr.cookie = 0;
                    tr.sender_euid = 0;
                    tr.sender_pid = 0;
                    write_buf.extend_from_slice(std::slice::from_raw_parts(
                        &tr as *const _ as *const u8,
                        mem::size_of::<binder_transaction_data>(),
                    ));
                    hexdump(ret2.as_ptr(), ret2.len().min(32));

                    bwr.write_size = write_buf.len() as u64;
                    bwr.write_buffer = write_buf.as_ptr() as u64;
                    bwr.write_consumed = 0;
                    bwr.read_size = copied_bwr.read_size;
                    bwr.read_consumed = copied_bwr.read_consumed;
                    bwr.read_buffer = copied_bwr.read_buffer;

                    let ret = original(fd, BINDER_WRITE_READ, &bwr as *const _ as *mut libc::c_void);
                    log_info!("ioctl(fd={}, request=0x{:x}) injected ret={} {:?} {} {}", fd, request, ret, bwr,
                    *(bwr.read_buffer as *const u32), *((bwr.read_buffer + 4) as *const u32));
                    //libc::abort();
                    bwr2.read_size = bwr.read_size;
                    bwr2.read_consumed = 0;
                    bwr2.read_buffer = bwr.read_buffer;
                    ret
                } else {
                    ret
                }
            } else {
                original(fd, request, arg)
            }
        } else {
            -1
        }
    }
}

unsafe fn patch_module(info: &libc::dl_phdr_info) {
    unsafe {
        let base = info.dlpi_addr;
        let mut dynamic: *const Elf64_Dyn = ptr::null();

        for i in 0..info.dlpi_phnum {
            let phdr = &*info.dlpi_phdr.add(i as usize);
            if phdr.p_type == libc::PT_DYNAMIC {
                dynamic = (base + phdr.p_vaddr) as *const Elf64_Dyn;
                break;
            }
        }

        if dynamic.is_null() {
            return;
        }

        let mut jmprel = 0;
        let mut pltrelsz = 0;
        let mut symtab = 0;
        let mut strtab = 0;

        let mut d = dynamic;
        while (*d).d_tag != DT_NULL {
            match (*d).d_tag {
                DT_JMPREL => jmprel = base + (*d).d_un,
                DT_PLTRELSZ => pltrelsz = (*d).d_un,
                DT_SYMTAB => symtab = base + (*d).d_un,
                DT_STRTAB => strtab = base + (*d).d_un,
                _ => {}
            }
            d = d.add(1);
        }

        if jmprel == 0 || symtab == 0 || strtab == 0 {
            return;
        }

        let rela = jmprel as *const Elf64_Rela;
        let nrela = pltrelsz / mem::size_of::<Elf64_Rela>() as u64;

        for i in 0..nrela {
            let r = &*rela.add(i as usize);
            let sym_idx = r.r_info >> 32;
            let sym = &*(symtab as *const Elf64_Sym).add(sym_idx as usize);
            let sym_name = CStr::from_ptr((strtab + sym.st_name as u64) as *const libc::c_char);

            if sym_name.to_bytes() == b"ioctl" {
                let got_entry = (base + r.r_offset) as *mut *const libc::c_void;

                let page_size = libc::sysconf(libc::_SC_PAGESIZE) as usize;
                let page_start = (got_entry as usize) & !(page_size - 1);

                libc::mprotect(
                    page_start as *mut _,
                    page_size,
                    libc::PROT_READ | libc::PROT_WRITE,
                );

                *got_entry = hooked_ioctl as *const libc::c_void;

                libc::mprotect(page_start as *mut _, page_size, libc::PROT_READ);
            }
        }
    }
}

unsafe extern "C" fn dl_callback(
    info: *mut libc::dl_phdr_info,
    _size: libc::size_t,
    _data: *mut libc::c_void,
) -> libc::c_int {
    unsafe {
        let info = &*info;
        let name_ptr = info.dlpi_name;
        let name = if name_ptr.is_null() || *name_ptr == 0 {
            "main"
        } else {
            CStr::from_ptr(name_ptr).to_str().unwrap_or("unknown")
        };

        if name.contains("libc.so") || name.contains("libkarelay.so") {
            return 0;
        }

        patch_module(info);
        0
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn karelay_init(
    host: *const libc::c_char,
    port: u32,
    token: *const libc::c_char,
    key_fingerprint: *const libc::c_char,
    key_fingerprint_len: u32,
) -> i64 {
    unsafe {
        let addr = if !host.is_null() {
            let host_str = match CStr::from_ptr(host).to_str() {
                Ok(s) => s,
                Err(_) => {
                    log_info!("libkarelay: Invalid relay host.\n");
                    return 1;
                }
            };
            let addr = format!("https://{}:{}", host_str, port);
            log_info!("libkarelay: Relay address set to {}\n", addr);
            addr
        } else {
            log_info!("libkarelay: No relay host set.\n");
            return 1;
        };

        let token = if !token.is_null() {
            match CStr::from_ptr(token).to_str() {
                Ok(token_str) => token_str.to_string(),
                Err(_) => {
                    log_info!("libkarelay: Invalid relay token.\n");
                    return 1;
                }
            }
        } else {
            log_info!("libkarelay: No relay token set.\n");
            return 1;
        };

        let _ = KA_CLIENT.set(KaClient::new(addr, token, std::slice::from_raw_parts(key_fingerprint, key_fingerprint_len as usize).to_vec()));

        log_info!("libkarelay: Initializing PLT hook for ioctl...\n");

        let original = libc::dlsym(libc::RTLD_DEFAULT, "ioctl\0".as_ptr() as *const _);
        if !original.is_null() {
            ORIGINAL_IOCTL = Some(mem::transmute(original));
            libc::dl_iterate_phdr(Some(dl_callback), ptr::null_mut());
            log_info!("libkarelay: PLT hook initialized.\n");
        } else {
            log_info!("libkarelay: Failed to find original ioctl.\n");
            return 1;
        }
        return 0;
    }
}

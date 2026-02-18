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
use base64::Engine;
use libc;
use std::ffi::CStr;
use std::fs;
use std::io::{BufRead, BufReader};

fn get_module_base(pid: i32, module_name: &str) -> Option<u64> {
    let path = format!("/proc/{}/maps", pid);
    let file = fs::File::open(path).ok()?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.ok()?;
        if line.contains(module_name) {
            let addr_str = line.split('-').next()?;
            return u64::from_str_radix(addr_str, 16).ok();
        }
    }
    None
}

fn write_remote_mem(pid: i32, addr: u64, data: &[u8]) -> Result<(), String> {
    let mut i = 0;
    while i < data.len() {
        let mut word = [0u8; 8];
        let remaining = data.len() - i;
        let orig = unsafe {
            let res = libc::ptrace(
                libc::PTRACE_PEEKDATA,
                pid,
                (addr + i as u64) as *mut libc::c_void,
                0,
            );
            res.to_ne_bytes()
        };
        word.copy_from_slice(&orig);
        let len = remaining.min(8);
        word[..len].copy_from_slice(&data[i..i + len]);
        let val = i64::from_ne_bytes(word);
        unsafe {
            libc::ptrace(
                libc::PTRACE_POKEDATA,
                pid,
                (addr + i as u64) as *mut libc::c_void,
                val as *mut libc::c_void,
            );
        }
        i += 8;
    }
    Ok(())
}

fn read_remote_mem(pid: i32, addr: u64, len: usize) -> Result<Vec<u8>, String> {
    let mut res = Vec::with_capacity(len);
    let mut i = 0;
    while i < len {
        unsafe {
            let word = libc::ptrace(
                libc::PTRACE_PEEKDATA,
                pid,
                (addr + i as u64) as *mut libc::c_void,
                0,
            );
            let bytes = word.to_ne_bytes();
            let remaining = len - i;
            let to_copy = remaining.min(8);
            res.extend_from_slice(&bytes[..to_copy]);
            i += 8;
        }
    }
    Ok(res)
}

fn read_remote_string(pid: i32, addr: u64) -> Result<String, String> {
    let mut res = Vec::new();
    let mut i = 0;
    loop {
        let chunk = read_remote_mem(pid, addr + i, 8)?;
        for &b in &chunk {
            if b == 0 {
                return Ok(String::from_utf8_lossy(&res).to_string());
            }
            res.push(b);
        }
        i += 8;
        if i > 1024 {
            break;
        }
    }
    Ok(String::from_utf8_lossy(&res).to_string())
}

const NT_PRSTATUS: i32 = 1;

fn get_regs(pid: i32) -> Result<libc::user_regs_struct, String> {
    let mut regs: libc::user_regs_struct = unsafe { std::mem::zeroed() };
    let mut iov = libc::iovec {
        iov_base: &mut regs as *mut _ as *mut libc::c_void,
        iov_len: std::mem::size_of::<libc::user_regs_struct>(),
    };
    let res = unsafe {
        libc::ptrace(
            libc::PTRACE_GETREGSET,
            pid,
            NT_PRSTATUS as usize as *mut libc::c_void,
            &mut iov as *mut _ as *mut libc::c_void,
        )
    };
    if res == -1 {
        return Err(format!(
            "PTRACE_GETREGSET failed: {}",
            std::io::Error::last_os_error()
        ));
    }
    Ok(regs)
}

fn set_regs(pid: i32, regs: &libc::user_regs_struct) -> Result<(), String> {
    let iov = libc::iovec {
        iov_base: regs as *const _ as *const libc::c_void as *mut libc::c_void,
        iov_len: std::mem::size_of::<libc::user_regs_struct>(),
    };
    let res = unsafe {
        libc::ptrace(
            libc::PTRACE_SETREGSET,
            pid,
            NT_PRSTATUS as usize as *mut libc::c_void,
            &iov as *const _ as *mut libc::c_void,
        )
    };
    if res == -1 {
        return Err(format!(
            "PTRACE_SETREGSET failed: {}",
            std::io::Error::last_os_error()
        ));
    }
    Ok(())
}

fn run_until(pid: i32, return_pc: u64, regs: &mut libc::user_regs_struct) -> Result<(), String> {
    for _ in 0..10 {
        // TODO: Check return value.
        unsafe {
            libc::ptrace(
                libc::PTRACE_CONT,
                pid,
                0 as *mut libc::c_void,
                0 as *mut libc::c_void,
            );
        }
        let mut status = 0;
        // TODO: Check return value.
        unsafe {
            libc::waitpid(pid, &mut status, 0);
        }
        *regs = get_regs(pid)?;
        println!("Wait stopped. pc:{:x}", regs.pc);
        if regs.pc == return_pc {
            return Ok(());
        }
    }
    Err("Could not run until return_pc. There may be a SEGV on the injected library.".into())
}

fn get_pid_from_prop() -> Result<i32, Box<dyn std::error::Error>> {
    let mut s = [0; 1024];
    let re = unsafe {
        libc::__system_property_get(
            "init.svc_debug_pid.keystore2\0".as_ptr() as *const _,
            s.as_mut_ptr() as *mut _,
        )
    };
    if re <= 0 {
        return Err("Failed to get keystore2 PID".into());
    }
    let pid_str = std::str::from_utf8(&s[..re as usize])?.trim_matches(char::from(0));
    let pid = pid_str.parse::<i32>()?;
    Ok(pid)
}

// 1. ptrace keystore2.
// 2. Calculate the offset of dlopen and dlerror in the keystore2 process.
// 3. Execute dlopen on "/data/misc/keystore/libkarelay.so".
fn client(
    arg_pid: Option<i32>,
    host: String,
    port: u32,
    token: String,
    key_fingerprint: Vec<u8>,
) -> Result<(), Box<dyn std::error::Error>> {
    let pid = arg_pid.unwrap_or(get_pid_from_prop()?);

    println!("Target PID: {}", pid);
    if unsafe { libc::ptrace(libc::PTRACE_ATTACH, pid, 0, 0) } == -1 {
        return Err(format!("Failed to attach: {}", std::io::Error::last_os_error()).into());
    }
    // Wait for the process to stop.
    let mut status = 0;
    unsafe {
        libc::waitpid(pid, &mut status, 0);
    }

    let local_dlopen = unsafe { libc::dlsym(libc::RTLD_DEFAULT, "dlopen\0".as_ptr() as *const _) };
    let local_dlsym = unsafe { libc::dlsym(libc::RTLD_DEFAULT, "dlsym\0".as_ptr() as *const _) };
    let local_dlerror =
        unsafe { libc::dlsym(libc::RTLD_DEFAULT, "dlerror\0".as_ptr() as *const _) };

    let mut info: libc::Dl_info = unsafe { std::mem::zeroed() };
    unsafe {
        libc::dladdr(local_dlopen, &mut info);
    }

    let module_path = unsafe { CStr::from_ptr(info.dli_fname).to_str()? };
    println!("Module path: {} {:x}", module_path, local_dlopen as u64);

    let local_base = info.dli_fbase as u64;
    let remote_base = get_module_base(pid, module_path).ok_or("Failed to find module in remote")?;
    println!("Local base: {:x}", local_base);
    println!("Remote base: {:x}", remote_base);

    let remote_dlopen = (local_dlopen as u64 - local_base) + remote_base;
    let remote_dlsym = (local_dlsym as u64 - local_base) + remote_base;
    let remote_dlerror = (local_dlerror as u64 - local_base) + remote_base;

    let mut regs = get_regs(pid)?;
    println!("PC: {:x}", regs.pc);

    // Copy the struct
    let saved_regs = unsafe { std::ptr::read(&regs) };

    // TODO: What's the reasonable offset?
    let string_addr = (regs.sp - 0x10000) & !0xf;
    let lib_path = "/data/misc/keystore/libkarelay.so\0";
    write_remote_mem(pid, string_addr, lib_path.as_bytes())?;
    // let lib_path = read_remote_mem(pid, string_addr, lib_path.len())?;
    // println!("Lib path: {}", String::from_utf8_lossy(&lib_path));

    regs.regs[0] = string_addr;
    regs.regs[1] = 2; // RTLD_NOW
    regs.pc = remote_dlopen;
    // Return address after execution of dlopen.
    regs.regs[30] = 0;

    // Prepare registers for execution of dlopen, then PTRACE_CONT until it execute return address (0).
    // It may need multiple PTRACE_CONT under some conditions like signal delivery.
    set_regs(pid, &regs)?;
    run_until(pid, 0, &mut regs)?;

    let handle = regs.regs[0];
    println!("dlopen return value: {:x}", handle);

    // Call dlerror if handle is null

    if handle == 0 {
        println!("FATAL: dlopen failed.");
        regs.pc = remote_dlerror;
        regs.regs[0] = handle;
        regs.regs[30] = 0;
        set_regs(pid, &regs)?;

        run_until(pid, 0, &mut regs)?;
        let error = read_remote_string(pid, regs.regs[0])?;
        println!("dlerror: {}", error);
    } else {
        // Call karelay_init(host, port)
        println!("Calling karelay_init({}, {})...", host, port);
        let init_name_addr = (regs.sp - 0x11000) & !0xf;
        write_remote_mem(pid, init_name_addr, "karelay_init\0".as_bytes())?;

        regs.regs[0] = handle;
        regs.regs[1] = init_name_addr;
        regs.pc = remote_dlsym;
        regs.regs[30] = 0;
        set_regs(pid, &regs)?;
        run_until(pid, 0, &mut regs)?;

        let init_ptr = regs.regs[0];
        println!("karelay_init pointer: {:x}", init_ptr);

        if init_ptr != 0 {
            let host_addr = (regs.sp - 0x12000) & !0xf;
            let host_cstr = host + "\0";
            write_remote_mem(pid, host_addr, host_cstr.as_bytes())?;

            let token_addr = (regs.sp - 0x13000) & !0xf;
            let token_cstr = token + "\0";
            write_remote_mem(pid, token_addr, token_cstr.as_bytes())?;

            let key_fingerprint_addr = (regs.sp - 0x14000) & !0xf;
            write_remote_mem(pid, key_fingerprint_addr, &key_fingerprint)?;

            regs.regs[0] = host_addr;
            regs.regs[1] = port as u64;
            regs.regs[2] = token_addr;
            regs.regs[3] = key_fingerprint_addr;
            regs.regs[4] = key_fingerprint.len() as u64;
            regs.pc = init_ptr;
            regs.regs[30] = 0;
            set_regs(pid, &regs)?;
            run_until(pid, 0, &mut regs)?;
            if regs.regs[0] == 0 {
                println!("karelay_init succeeded.");
            } else {
                println!("karelay_init failed. {}", regs.regs[0]);
            }
        } else {
            println!("FATAL: karelay_init not found.");
        }
    }

    unsafe {
        set_regs(pid, &saved_regs)?;
        libc::ptrace(
            libc::PTRACE_DETACH,
            pid,
            0 as *mut libc::c_void,
            0 as *mut libc::c_void,
        );
    }
    println!("Restored and detached.");
    Ok(())
}

// Inject libkarelay.so into keystore2 process.
// Manually place libkarelay.so on /data/misc/keystore/libkarelay.so and run:
// adb shell "chcon u:object_r:system_lib_file:s0 /data/misc/keystore/libkarelay.so"
// adb shell "chown keystore:keystore /data/misc/keystore/libkarelay.so"
// adb shell "chmod 0600 /data/misc/keystore/libkarelay.so"
//
// Usage: "./ka-injector client [url] [pid]" or "./ka-injector client [url]".
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cmd = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "client".to_string());

    match cmd.as_str() {
        "client" => {
            let url = std::env::args()
                .nth(2)
                .expect("URL is required: ka-injector client [url] [pid]");
            // Sample url: kar://127.0.0.1:11127/key_fp/.../token/...
            let url = url.parse::<url::Url>()?;
            let host = url.host_str().expect("URL must have a host");
            let port = url.port().expect("URL must have a port");
            println!("Host: {}", host);
            println!("Port: {}", port);
            let arg_pid = std::env::args()
                .nth(3)
                .map(|s| s.parse::<i32>().expect("PID is not valid integer"));
            let key_fingerprint = url.path_segments().expect("Invalid url").nth(1).expect("Invalid url");
            let token = url.path_segments().expect("Invalid url").nth(3).expect("Invalid url");
            let key_fingerprint = base64::prelude::BASE64_URL_SAFE_NO_PAD
                .decode(key_fingerprint.as_bytes())
                .expect("key_fp is not valid base64");
            client(
                arg_pid,
                host.to_string(),
                port as u32,
                token.to_string(),
                key_fingerprint,
            )
        }
        _ => Err(format!("Unknown command: {}", cmd).into()),
    }
}

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
fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_prost_build::compile_protos("../proto/keyattest.proto")?;
    let source = std::env::var("AOSP_SOURCE").expect("AOSP_SOURCE env is not specified");
    cxx_build::bridge("src/main.rs")
        .file("src/km.cc")
        .flag("-nostdinc++")
        .flag("-fno-rtti")
        .include(source.to_string() + "/out/soong/.intermediates/system/libhidl/transport/manager/1.2/android.hidl.manager@1.2_genc++_headers/gen")
        .include(source.to_string() + "/out/soong/.intermediates/system/libhidl/transport/base/1.0/android.hidl.base@1.0_genc++_headers/gen")
        .include(source.to_string() + "/out/soong/.intermediates/system/libhidl/transport/manager/1.1/android.hidl.manager@1.1_genc++_headers/gen")
        .include(source.to_string() + "/out/soong/.intermediates/system/libhidl/transport/manager/1.0/android.hidl.manager@1.0_genc++_headers/gen")
        .include(source.to_string() + "/frameworks/native/libs/binder/include")
        .include(source.to_string() + "/frameworks/native/libs/binder/ndk/include_ndk")
        .include(source.to_string() + "/system/libhidl/base/include")
        .include(source.to_string() + "/system/core/include")
        .include(source.to_string() + "/system/libfmq/base")
        .include(source.to_string() + "/system/libbase/include")
        .include(source.to_string() + "/system/logging/liblog/include")
        .include(source.to_string() + "/system/security/keystore2/src/km_compat/")
        .include(source.to_string() + "/hardware/interfaces/security/keymint/support/include")
        .include(source.to_string() + "/hardware/interfaces/keymaster/4.1/support/include")
        .include(source.to_string() + "/out/soong/.intermediates/hardware/interfaces/keymaster/4.1/android.hardware.keymaster@4.1_genc++_headers/gen")
        .include(source.to_string() + "/out/soong/.intermediates/hardware/interfaces/keymaster/4.0/android.hardware.keymaster@4.0_genc++_headers/gen")
        .include(source.to_string() + "/out/soong/.intermediates/system/hardware/interfaces/keystore2/aidl/android.system.keystore2-V5-ndk-source/gen/include")
        .include(source.to_string() + "/out/soong/.intermediates/hardware/interfaces/security/keymint/aidl/android.hardware.security.keymint-V4-ndk-source/gen/include")
        .include(source.to_string() + "/hardware/interfaces/keymaster/4.0/support/include")
        .include(source.to_string() + "/prebuilts/clang/host/linux-x86/clang-r563880c/android_libc++/platform/aarch64/include/c++/v1")
        .include(source.to_string() + "/prebuilts/clang/host/linux-x86/clang-r563880c/include/c++/v1")
        .compile("server");

    println!("cargo::rustc-link-lib=hidlbase");
    println!("cargo::rustc-link-lib=log");
    println!("cargo::rustc-link-lib=base");
    println!("cargo::rustc-link-lib=c++");
    println!("cargo::rustc-link-lib=utils");
    println!("cargo::rustc-link-arg={source}/out/target/product/generic_arm64/system/lib64/libkeymaster4support.so");
    println!("cargo::rustc-link-arg={source}/out/target/product/generic_arm64/system/lib64/libkeymaster4_1support.so");
    println!("cargo::rustc-link-arg={source}/out/target/product/generic_arm64/system/lib64/android.system.keystore2-V5-ndk.so");
    println!("cargo::rustc-link-arg={source}/out/target/product/generic_arm64/system/lib64/android.hardware.security.keymint-V4-ndk.so");
    println!("cargo::rustc-link-arg={source}/out/soong/.intermediates/hardware/interfaces/keymaster/4.1/android.hardware.keymaster@4.1/android_arm64_armv8-a_shared_cfi/unstripped/android.hardware.keymaster@4.1.so");
    println!("cargo::rustc-link-arg={source}/out/soong/.intermediates/hardware/interfaces/keymaster/4.0/android.hardware.keymaster@4.0/android_arm64_armv8-a_shared_cfi/unstripped/android.hardware.keymaster@4.0.so");
    println!("cargo::rustc-link-search={source}/out/target/product/generic_arm64/system/lib64");
    println!("cargo:rerun-if-changed=src/km.cc");
    println!("cargo:rerun-if-changed=include/km.h");
    Ok(())
}

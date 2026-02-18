# Key Attest Relay

Key Attest Relay relays key attestation request between Android devices.

> [!NOTE]
> This code is provided for research purposes and is not intended for production use.

> [!NOTE]
> This is a highly experimental prototype with minimal testing and should be treated as such.

## Architecture

1. ka-injector: ptrace & hooks keystore2 process to inject libkarelay
2. libkarelay: Hooks key attestation and transfers requests to ka-server which is running on a different device
3. ka-server: Accepts requests and forwards them to keymaster/keymint HAL

## Environment

1. Client device: Android 15-16 rooted device
2. Server device: Android 16 rooted device with keymaster HAL 4.0

Currently generated binaries are dynamically linked to shared libraries from AOSP build of Android 16 QPR2. It is very likely that it won't run on other version of Android.

## Build

### Prerequisites

- [protoc](https://protobuf.dev/installation/)
- [rustc and cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)
- [Android NDK](https://developer.android.com/ndk)

### Instructions

```bash
$ git clone repository
$ cd (cloned directory)
$ export ANDROID_NDK_HOME=(Ndk directory)
$ export CXX=$ANDROID_NDK_HOME/toolchains/llvm/prebuilt/linux-x86_64/bin/aarch64-linux-android34-clang++
$ export CC=$ANDROID_NDK_HOME/toolchains/llvm/prebuilt/linux-x86_64/bin/aarch64-linux-android34-clang
$ export AR=$ANDROID_NDK_HOME/toolchains/llvm/prebuilt/linux-x86_64/bin/llvm-ar
$ export LD=$ANDROID_NDK_HOME/toolchains/llvm/prebuilt/linux-x86_64/bin/aarch64-linux-android34-clang
$ export LIBCLANG_PATH=$ANDROID_NDK_HOME/toolchains/llvm/prebuilt/linux-x86_64/musl/lib
$ export AOSP_SOURCE=$PWD/aosp
$ cargo build --release --target aarch64-linux-android --config target.aarch64-linux-android.linker=\"$LD\"
```

## Usage

1. Server:
```bash
$ openssl genpkey -algorithm ED25519 -out server.key
$ openssl req -new -x509 -days 10000 -key server.key -out server.crt -subj "/CN=keyattest"
$ adb push server.key server.crt /data/local/tmp/
$ adb push target/aarch64-linux-android/release/ka-server /data/local/tmp/
$ adb shell
(In adb shell)
$ su
# killall ka-server
# /data/local/tmp/ka-server server (Bind IP Address):11129 /data/local/tmp/server.key /data/local/tmp/server.crt
```

ka-server prints url like kar://... in the console. Copy that url.

2. Client:
```bash
$ wget https://github.com/topjohnwu/Magisk/releases/download/v30.6/Magisk-v30.6.apk 
$ unzip Magisk-v30.6.apk lib/arm64-v8a/libmagiskpolicy.so
$ adby7 push lib/arm64-v8a/libmagiskpolicy.so /data/local/tmp/magiskpolicy
$ adb push target/aarch64-linux-android/release/ka-injector /data/local/tmp/
$ adb push target/aarch64-linux-android/release/libkarelay.so /data/local/tmp/
$ adb shell
(In adb shell)
$ su
# chmod 700 /data/local/tmp/magiskpolicy
# /data/local/tmp/magiskpolicy --live 'allow keystore keystore tcp_socket { create getopt connect setopt write read shutdown getattr }'
# /data/local/tmp/magiskpolicy --live 'allow keystore port tcp_socket { name_connect }'
// It is optional to print stack trace of keystore process.
# /data/local/tmp/magiskpolicy --live 'allow crash_dump keystore process { ptrace }'
# cp /data/local/tmp/libkarelay.so /data/misc/keystore
# chown keystore:keystore /data/misc/keystore/libkarelay.so
# chmod 600 /data/misc/keystore/libkarelay.so
# chcon u:object_r:system_lib_file:s0 /data/misc/keystore/libkarelay.so
# killall keystore2
# /data/local/tmp/ka-injector client <Copied URL>
Host: ***
Port: 11129
Target PID: 15387
Module path: /apex/com.android.runtime/lib64/bionic/libdl.so 7cfb655014
Local base: 7cfb651000
Remote base: 7f3646f000
PC: 7f32927a88
Wait stopped. pc:0
dlopen return value: cd73c6797313d7
Calling karelay_init(***, 11129)...
Wait stopped. pc:0
karelay_init pointer: 7c9a0b20d0
Wait stopped. pc:0
karelay_init succeeded.
Restored and detached.
# 
```

Now attestation requests are hooked. Test them with [Android Key Attestation Test App](https://github.com/vvb2060/KeyAttestation).

## Note

1. Both client and server devices have to be rooted. Exploiting a kernel vulnerability or similar on server devices is required to spoof the unlock status.

## Acknowledgement

- [TrickyStore](https://github.com/5ec1cff/TrickyStore)
- [Tricky Store OSS](https://github.com/beakthoven/TrickyStoreOSS)
- [TEESimulator](https://github.com/JingMatrix/TEESimulator)
- [Android Open Source Project](https://source.android.com/)

## License

Apache 2.0
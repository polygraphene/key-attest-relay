#![allow(non_snake_case)]
#![allow(missing_docs)]
#[deprecated(note = "Please access via libbinder_rs binder::")]
pub use binder;
pub mod aidl {
  pub mod android {
    pub mod hardware {
      pub mod security {
        pub mod secureclock {
          pub mod ISecureClock {
            /*
             * This file is auto-generated.  DO NOT MODIFY.
             * Using: out/host/linux-x86/bin/aidl --lang=rust --structured --version 1 --hash cd55ca9963c6a57fa5f2f120a45c6e0c4fafb423 -t --stability vintf --min_sdk_version 35 --ninja -d out/soong/.intermediates/hardware/interfaces/security/secureclock/aidl/android.hardware.security.secureclock-V1-rust-source/gen/android/hardware/security/secureclock/ISecureClock.rs.d -o out/soong/.intermediates/hardware/interfaces/security/secureclock/aidl/android.hardware.security.secureclock-V1-rust-source/gen -Nhardware/interfaces/security/secureclock/aidl/aidl_api/android.hardware.security.secureclock/1 hardware/interfaces/security/secureclock/aidl/aidl_api/android.hardware.security.secureclock/1/android/hardware/security/secureclock/ISecureClock.aidl
             *
             * DO NOT CHECK THIS FILE INTO A CODE TREE (e.g. git, etc..).
             * ALWAYS GENERATE THIS FILE FROM UPDATED AIDL COMPILER
             * AS A BUILD INTERMEDIATE ONLY. THIS IS NOT SOURCE CODE.
             */
            #![forbid(unsafe_code)]
            #![cfg_attr(rustfmt, rustfmt_skip)]
            #![allow(non_upper_case_globals)]
            #![allow(non_snake_case)]
            #[allow(unused_imports)] use binder::binder_impl::IBinderInternal;
            #[cfg(any(android_vndk, not(android_ndk)))]
            const FLAG_PRIVATE_LOCAL: binder::binder_impl::TransactionFlags = binder::binder_impl::FLAG_PRIVATE_LOCAL;
            #[cfg(not(any(android_vndk, not(android_ndk))))]
            const FLAG_PRIVATE_LOCAL: binder::binder_impl::TransactionFlags = 0;
            use binder::declare_binder_interface;
            declare_binder_interface! {
              ISecureClock["android.hardware.security.secureclock.ISecureClock"] {
                native: BnSecureClock(on_transact),
                proxy: BpSecureClock {
                  cached_version: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(-1),
                  cached_hash: std::sync::Mutex<Option<String>> = std::sync::Mutex::new(None)
                },
                async: ISecureClockAsync(try_into_local_async),
                stability: binder::binder_impl::Stability::Vintf,
              }
            }
            pub trait ISecureClock: binder::Interface + Send {
              fn get_descriptor() -> &'static str where Self: Sized { "android.hardware.security.secureclock.ISecureClock" }
              fn r#generateTimeStamp<'a, >(&'a self, _arg_challenge: i64) -> binder::Result<crate::mangled::_7_android_8_hardware_8_security_11_secureclock_14_TimeStampToken>;
              fn r#getInterfaceVersion<'a, >(&'a self) -> binder::Result<i32> {
                Ok(VERSION)
              }
              fn r#getInterfaceHash<'a, >(&'a self) -> binder::Result<String> {
                Ok(HASH.into())
              }
              fn getDefaultImpl() -> ISecureClockDefaultRef where Self: Sized {
                DEFAULT_IMPL.lock().unwrap().clone()
              }
              fn setDefaultImpl(d: ISecureClockDefaultRef) -> ISecureClockDefaultRef where Self: Sized {
                std::mem::replace(&mut *DEFAULT_IMPL.lock().unwrap(), d)
              }
              fn try_as_async_server<'a>(&'a self) -> Option<&'a (dyn ISecureClockAsyncServer + Send + Sync)> {
                None
              }
            }
            pub trait ISecureClockAsync<P>: binder::Interface + Send {
              fn get_descriptor() -> &'static str where Self: Sized { "android.hardware.security.secureclock.ISecureClock" }
              fn r#generateTimeStamp<'a, >(&'a self, _arg_challenge: i64) -> binder::BoxFuture<'a, binder::Result<crate::mangled::_7_android_8_hardware_8_security_11_secureclock_14_TimeStampToken>>;
              fn r#getInterfaceVersion<'a, >(&'a self) -> binder::BoxFuture<'a, binder::Result<i32>> {
                Box::pin(async move { Ok(VERSION) })
              }
              fn r#getInterfaceHash<'a, >(&'a self) -> binder::BoxFuture<'a, binder::Result<String>> {
                Box::pin(async move { Ok(HASH.into()) })
              }
            }
            #[::async_trait::async_trait]
            pub trait ISecureClockAsyncServer: binder::Interface + Send {
              fn get_descriptor() -> &'static str where Self: Sized { "android.hardware.security.secureclock.ISecureClock" }
              async fn r#generateTimeStamp<'a, >(&'a self, _arg_challenge: i64) -> binder::Result<crate::mangled::_7_android_8_hardware_8_security_11_secureclock_14_TimeStampToken>;
            }
            impl BnSecureClock {
              /// Create a new async binder service.
              pub fn new_async_binder<T, R>(inner: T, rt: R, features: binder::BinderFeatures) -> binder::Strong<dyn ISecureClock>
              where
                T: ISecureClockAsyncServer + binder::Interface + Send + Sync + 'static,
                R: binder::binder_impl::BinderAsyncRuntime + Send + Sync + 'static,
              {
                struct Wrapper<T, R> {
                  _inner: T,
                  _rt: R,
                }
                impl<T, R> binder::Interface for Wrapper<T, R> where T: binder::Interface, R: Send + Sync + 'static {
                  fn as_binder(&self) -> binder::SpIBinder { self._inner.as_binder() }
                  fn dump(&self, _writer: &mut dyn std::io::Write, _args: &[&std::ffi::CStr]) -> std::result::Result<(), binder::StatusCode> { self._inner.dump(_writer, _args) }
                }
                impl<T, R> ISecureClock for Wrapper<T, R>
                where
                  T: ISecureClockAsyncServer + Send + Sync + 'static,
                  R: binder::binder_impl::BinderAsyncRuntime + Send + Sync + 'static,
                {
                  fn r#generateTimeStamp<'a, >(&'a self, _arg_challenge: i64) -> binder::Result<crate::mangled::_7_android_8_hardware_8_security_11_secureclock_14_TimeStampToken> {
                    self._rt.block_on(self._inner.r#generateTimeStamp(_arg_challenge))
                  }
                  fn try_as_async_server(&self) -> Option<&(dyn ISecureClockAsyncServer + Send + Sync)> {
                    Some(&self._inner)
                  }
                }
                let wrapped = Wrapper { _inner: inner, _rt: rt };
                Self::new_binder(wrapped, features)
              }
              pub fn try_into_local_async<P: binder::BinderAsyncPool + 'static>(_native: binder::binder_impl::Binder<Self>) -> Option<binder::Strong<dyn ISecureClockAsync<P>>> {
                struct Wrapper {
                  _native: binder::binder_impl::Binder<BnSecureClock>
                }
                impl binder::Interface for Wrapper {}
                impl<P: binder::BinderAsyncPool> ISecureClockAsync<P> for Wrapper {
                  fn r#generateTimeStamp<'a, >(&'a self, _arg_challenge: i64) -> binder::BoxFuture<'a, binder::Result<crate::mangled::_7_android_8_hardware_8_security_11_secureclock_14_TimeStampToken>> {
                    Box::pin(self._native.try_as_async_server().unwrap().r#generateTimeStamp(_arg_challenge))
                  }
                }
                if _native.try_as_async_server().is_some() {
                  Some(binder::Strong::new(Box::new(Wrapper { _native }) as Box<dyn ISecureClockAsync<P>>))
                } else {
                  None
                }
              }
            }
            pub trait ISecureClockDefault: Send + Sync {
              fn r#generateTimeStamp<'a, >(&'a self, _arg_challenge: i64) -> binder::Result<crate::mangled::_7_android_8_hardware_8_security_11_secureclock_14_TimeStampToken> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
            }
            pub mod transactions {
              pub const r#generateTimeStamp: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 0;
              pub const r#getInterfaceVersion: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 16777214;
              pub const r#getInterfaceHash: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 16777213;
            }
            pub type ISecureClockDefaultRef = Option<std::sync::Arc<dyn ISecureClockDefault>>;
            static DEFAULT_IMPL: std::sync::Mutex<ISecureClockDefaultRef> = std::sync::Mutex::new(None);
            pub const r#TIME_STAMP_MAC_LABEL: &str = "Auth Verification";
            pub const VERSION: i32 = 1;
            pub const HASH: &str = "cd55ca9963c6a57fa5f2f120a45c6e0c4fafb423";
            impl BpSecureClock {
              fn build_parcel_generateTimeStamp(&self, _arg_challenge: i64) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.write(&_arg_challenge)?;
                Ok(aidl_data)
              }
              fn read_response_generateTimeStamp(&self, _arg_challenge: i64, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<crate::mangled::_7_android_8_hardware_8_security_11_secureclock_14_TimeStampToken> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as ISecureClock>::getDefaultImpl() {
                    return _aidl_default_impl.r#generateTimeStamp(_arg_challenge);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                let _aidl_status: binder::Status = _aidl_reply.read()?;
                if !_aidl_status.is_ok() { return Err(_aidl_status); }
                let _aidl_return: crate::mangled::_7_android_8_hardware_8_security_11_secureclock_14_TimeStampToken = _aidl_reply.read()?;
                Ok(_aidl_return)
              }
              fn build_parcel_getInterfaceVersion(&self) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                Ok(aidl_data)
              }
              fn read_response_getInterfaceVersion(&self, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<i32> {
                let _aidl_reply = _aidl_reply?;
                let _aidl_status: binder::Status = _aidl_reply.read()?;
                if !_aidl_status.is_ok() { return Err(_aidl_status); }
                let _aidl_return: i32 = _aidl_reply.read()?;
                self.cached_version.store(_aidl_return, std::sync::atomic::Ordering::Relaxed);
                Ok(_aidl_return)
              }
              fn build_parcel_getInterfaceHash(&self) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                Ok(aidl_data)
              }
              fn read_response_getInterfaceHash(&self, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<String> {
                let _aidl_reply = _aidl_reply?;
                let _aidl_status: binder::Status = _aidl_reply.read()?;
                if !_aidl_status.is_ok() { return Err(_aidl_status); }
                let _aidl_return: String = _aidl_reply.read()?;
                *self.cached_hash.lock().unwrap() = Some(_aidl_return.clone());
                Ok(_aidl_return)
              }
            }
            impl ISecureClock for BpSecureClock {
              fn r#generateTimeStamp<'a, >(&'a self, _arg_challenge: i64) -> binder::Result<crate::mangled::_7_android_8_hardware_8_security_11_secureclock_14_TimeStampToken> {
                let _aidl_data = self.build_parcel_generateTimeStamp(_arg_challenge)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#generateTimeStamp, _aidl_data, FLAG_PRIVATE_LOCAL);
                self.read_response_generateTimeStamp(_arg_challenge, _aidl_reply)
              }
              fn r#getInterfaceVersion<'a, >(&'a self) -> binder::Result<i32> {
                let _aidl_version = self.cached_version.load(std::sync::atomic::Ordering::Relaxed);
                if _aidl_version != -1 { return Ok(_aidl_version); }
                let _aidl_data = self.build_parcel_getInterfaceVersion()?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#getInterfaceVersion, _aidl_data, FLAG_PRIVATE_LOCAL);
                self.read_response_getInterfaceVersion(_aidl_reply)
              }
              fn r#getInterfaceHash<'a, >(&'a self) -> binder::Result<String> {
                {
                  let _aidl_hash_lock = self.cached_hash.lock().unwrap();
                  if let Some(ref _aidl_hash) = *_aidl_hash_lock {
                    return Ok(_aidl_hash.clone());
                  }
                }
                let _aidl_data = self.build_parcel_getInterfaceHash()?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#getInterfaceHash, _aidl_data, FLAG_PRIVATE_LOCAL);
                self.read_response_getInterfaceHash(_aidl_reply)
              }
            }
            impl<P: binder::BinderAsyncPool> ISecureClockAsync<P> for BpSecureClock {
              fn r#generateTimeStamp<'a, >(&'a self, _arg_challenge: i64) -> binder::BoxFuture<'a, binder::Result<crate::mangled::_7_android_8_hardware_8_security_11_secureclock_14_TimeStampToken>> {
                let _aidl_data = match self.build_parcel_generateTimeStamp(_arg_challenge) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return Box::pin(std::future::ready(Err(err))),
                };
                let binder = self.binder.clone();
                P::spawn(
                  move || binder.submit_transact(transactions::r#generateTimeStamp, _aidl_data, FLAG_PRIVATE_LOCAL),
                  move |_aidl_reply| async move {
                    self.read_response_generateTimeStamp(_arg_challenge, _aidl_reply)
                  }
                )
              }
              fn r#getInterfaceVersion<'a, >(&'a self) -> binder::BoxFuture<'a, binder::Result<i32>> {
                let _aidl_version = self.cached_version.load(std::sync::atomic::Ordering::Relaxed);
                if _aidl_version != -1 { return Box::pin(std::future::ready(Ok(_aidl_version))); }
                let _aidl_data = match self.build_parcel_getInterfaceVersion() {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return Box::pin(std::future::ready(Err(err))),
                };
                let binder = self.binder.clone();
                P::spawn(
                  move || binder.submit_transact(transactions::r#getInterfaceVersion, _aidl_data, FLAG_PRIVATE_LOCAL),
                  move |_aidl_reply| async move {
                    self.read_response_getInterfaceVersion(_aidl_reply)
                  }
                )
              }
              fn r#getInterfaceHash<'a, >(&'a self) -> binder::BoxFuture<'a, binder::Result<String>> {
                {
                  let _aidl_hash_lock = self.cached_hash.lock().unwrap();
                  if let Some(ref _aidl_hash) = *_aidl_hash_lock {
                    return Box::pin(std::future::ready(Ok(_aidl_hash.clone())));
                  }
                }
                let _aidl_data = match self.build_parcel_getInterfaceHash() {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return Box::pin(std::future::ready(Err(err))),
                };
                let binder = self.binder.clone();
                P::spawn(
                  move || binder.submit_transact(transactions::r#getInterfaceHash, _aidl_data, FLAG_PRIVATE_LOCAL),
                  move |_aidl_reply| async move {
                    self.read_response_getInterfaceHash(_aidl_reply)
                  }
                )
              }
            }
            impl ISecureClock for binder::binder_impl::Binder<BnSecureClock> {
              fn r#generateTimeStamp<'a, >(&'a self, _arg_challenge: i64) -> binder::Result<crate::mangled::_7_android_8_hardware_8_security_11_secureclock_14_TimeStampToken> { self.0.r#generateTimeStamp(_arg_challenge) }
              fn r#getInterfaceVersion<'a, >(&'a self) -> binder::Result<i32> { self.0.r#getInterfaceVersion() }
              fn r#getInterfaceHash<'a, >(&'a self) -> binder::Result<String> { self.0.r#getInterfaceHash() }
            }
            fn on_transact(_aidl_service: &dyn ISecureClock, _aidl_code: binder::binder_impl::TransactionCode, _aidl_data: &binder::binder_impl::BorrowedParcel<'_>, _aidl_reply: &mut binder::binder_impl::BorrowedParcel<'_>) -> std::result::Result<(), binder::StatusCode> {
              match _aidl_code {
                transactions::r#generateTimeStamp => {
                  let _arg_challenge: i64 = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#generateTimeStamp(_arg_challenge);
                  match &_aidl_return {
                    Ok(_aidl_return) => {
                      _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
                      _aidl_reply.write(_aidl_return)?;
                    }
                    Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
                  }
                  Ok(())
                }
                transactions::r#getInterfaceVersion => {
                  let _aidl_return = _aidl_service.r#getInterfaceVersion();
                  match &_aidl_return {
                    Ok(_aidl_return) => {
                      _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
                      _aidl_reply.write(_aidl_return)?;
                    }
                    Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
                  }
                  Ok(())
                }
                transactions::r#getInterfaceHash => {
                  let _aidl_return = _aidl_service.r#getInterfaceHash();
                  match &_aidl_return {
                    Ok(_aidl_return) => {
                      _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
                      _aidl_reply.write(_aidl_return)?;
                    }
                    Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
                  }
                  Ok(())
                }
                _ => Err(binder::StatusCode::UNKNOWN_TRANSACTION)
              }
            }
            pub(crate) mod mangled {
             pub use super::r#ISecureClock as _7_android_8_hardware_8_security_11_secureclock_12_ISecureClock;
            }
          }
          pub mod TimeStampToken {
            /*
             * This file is auto-generated.  DO NOT MODIFY.
             * Using: out/host/linux-x86/bin/aidl --lang=rust --structured --version 1 --hash cd55ca9963c6a57fa5f2f120a45c6e0c4fafb423 -t --stability vintf --min_sdk_version 35 --ninja -d out/soong/.intermediates/hardware/interfaces/security/secureclock/aidl/android.hardware.security.secureclock-V1-rust-source/gen/android/hardware/security/secureclock/TimeStampToken.rs.d -o out/soong/.intermediates/hardware/interfaces/security/secureclock/aidl/android.hardware.security.secureclock-V1-rust-source/gen -Nhardware/interfaces/security/secureclock/aidl/aidl_api/android.hardware.security.secureclock/1 hardware/interfaces/security/secureclock/aidl/aidl_api/android.hardware.security.secureclock/1/android/hardware/security/secureclock/TimeStampToken.aidl
             *
             * DO NOT CHECK THIS FILE INTO A CODE TREE (e.g. git, etc..).
             * ALWAYS GENERATE THIS FILE FROM UPDATED AIDL COMPILER
             * AS A BUILD INTERMEDIATE ONLY. THIS IS NOT SOURCE CODE.
             */
            #![forbid(unsafe_code)]
            #![cfg_attr(rustfmt, rustfmt_skip)]
            #[derive(Debug, Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
            pub struct r#TimeStampToken {
              pub r#challenge: i64,
              pub r#timestamp: crate::mangled::_7_android_8_hardware_8_security_11_secureclock_9_Timestamp,
              pub r#mac: Vec<u8>,
            }
            impl Default for r#TimeStampToken {
              fn default() -> Self {
                Self {
                  r#challenge: 0,
                  r#timestamp: Default::default(),
                  r#mac: Default::default(),
                }
              }
            }
            impl binder::Parcelable for r#TimeStampToken {
              fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_write(|subparcel| {
                  subparcel.write(&self.r#challenge)?;
                  subparcel.write(&self.r#timestamp)?;
                  subparcel.write(&self.r#mac)?;
                  Ok(())
                })
              }
              fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_read(|subparcel| {
                  if subparcel.has_more_data() {
                    self.r#challenge = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#timestamp = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#mac = subparcel.read()?;
                  }
                  Ok(())
                })
              }
            }
            binder::impl_serialize_for_parcelable!(r#TimeStampToken);
            binder::impl_deserialize_for_parcelable!(r#TimeStampToken);
            impl binder::binder_impl::ParcelableMetadata for r#TimeStampToken {
              fn get_descriptor() -> &'static str { "android.hardware.security.secureclock.TimeStampToken" }
              fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
            }
            pub(crate) mod mangled {
             pub use super::r#TimeStampToken as _7_android_8_hardware_8_security_11_secureclock_14_TimeStampToken;
            }
          }
          pub mod Timestamp {
            /*
             * This file is auto-generated.  DO NOT MODIFY.
             * Using: out/host/linux-x86/bin/aidl --lang=rust --structured --version 1 --hash cd55ca9963c6a57fa5f2f120a45c6e0c4fafb423 -t --stability vintf --min_sdk_version 35 --ninja -d out/soong/.intermediates/hardware/interfaces/security/secureclock/aidl/android.hardware.security.secureclock-V1-rust-source/gen/android/hardware/security/secureclock/Timestamp.rs.d -o out/soong/.intermediates/hardware/interfaces/security/secureclock/aidl/android.hardware.security.secureclock-V1-rust-source/gen -Nhardware/interfaces/security/secureclock/aidl/aidl_api/android.hardware.security.secureclock/1 hardware/interfaces/security/secureclock/aidl/aidl_api/android.hardware.security.secureclock/1/android/hardware/security/secureclock/Timestamp.aidl
             *
             * DO NOT CHECK THIS FILE INTO A CODE TREE (e.g. git, etc..).
             * ALWAYS GENERATE THIS FILE FROM UPDATED AIDL COMPILER
             * AS A BUILD INTERMEDIATE ONLY. THIS IS NOT SOURCE CODE.
             */
            #![forbid(unsafe_code)]
            #![cfg_attr(rustfmt, rustfmt_skip)]
            #[derive(Debug, Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
            pub struct r#Timestamp {
              pub r#milliSeconds: i64,
            }
            impl Default for r#Timestamp {
              fn default() -> Self {
                Self {
                  r#milliSeconds: 0,
                }
              }
            }
            impl binder::Parcelable for r#Timestamp {
              fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_write(|subparcel| {
                  subparcel.write(&self.r#milliSeconds)?;
                  Ok(())
                })
              }
              fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_read(|subparcel| {
                  if subparcel.has_more_data() {
                    self.r#milliSeconds = subparcel.read()?;
                  }
                  Ok(())
                })
              }
            }
            binder::impl_serialize_for_parcelable!(r#Timestamp);
            binder::impl_deserialize_for_parcelable!(r#Timestamp);
            impl binder::binder_impl::ParcelableMetadata for r#Timestamp {
              fn get_descriptor() -> &'static str { "android.hardware.security.secureclock.Timestamp" }
              fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
            }
            pub(crate) mod mangled {
             pub use super::r#Timestamp as _7_android_8_hardware_8_security_11_secureclock_9_Timestamp;
            }
          }
        }
      }
    }
  }
}
pub mod mangled {
  pub use super::aidl::android::hardware::security::secureclock::ISecureClock::mangled::*;
  pub use super::aidl::android::hardware::security::secureclock::TimeStampToken::mangled::*;
  pub use super::aidl::android::hardware::security::secureclock::Timestamp::mangled::*;
}

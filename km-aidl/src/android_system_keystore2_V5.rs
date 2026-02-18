#![allow(non_snake_case)]
#![allow(missing_docs)]
#[deprecated(note = "Please access via libbinder_rs binder::")]
pub use binder;
pub mod aidl {
  pub mod android {
    pub mod system {
      pub mod keystore2 {
        pub mod AuthenticatorSpec {
          /*
           * This file is auto-generated.  DO NOT MODIFY.
           * Using: out/host/linux-x86/bin/aidl --lang=rust --structured --version 5 --hash 98d815116c190250e9e5a1d9182cea8126fd0e97 -t --stability vintf --min_sdk_version current -pout/soong/.intermediates/hardware/interfaces/security/keymint/aidl/android.hardware.security.keymint_interface/4/preprocessed.aidl --ninja -d out/soong/.intermediates/system/hardware/interfaces/keystore2/aidl/android.system.keystore2-V5-rust-source/gen/android/system/keystore2/AuthenticatorSpec.rs.d -o out/soong/.intermediates/system/hardware/interfaces/keystore2/aidl/android.system.keystore2-V5-rust-source/gen -Nsystem/hardware/interfaces/keystore2/aidl/aidl_api/android.system.keystore2/5 system/hardware/interfaces/keystore2/aidl/aidl_api/android.system.keystore2/5/android/system/keystore2/AuthenticatorSpec.aidl
           *
           * DO NOT CHECK THIS FILE INTO A CODE TREE (e.g. git, etc..).
           * ALWAYS GENERATE THIS FILE FROM UPDATED AIDL COMPILER
           * AS A BUILD INTERMEDIATE ONLY. THIS IS NOT SOURCE CODE.
           */
          #![forbid(unsafe_code)]
          #![cfg_attr(rustfmt, rustfmt_skip)]
          #[derive(Debug)]
          pub struct r#AuthenticatorSpec {
            pub r#authenticatorType: crate::mangled::_7_android_8_hardware_8_security_7_keymint_25_HardwareAuthenticatorType,
            pub r#authenticatorId: i64,
          }
          impl Default for r#AuthenticatorSpec {
            fn default() -> Self {
              Self {
                r#authenticatorType: crate::mangled::_7_android_8_hardware_8_security_7_keymint_25_HardwareAuthenticatorType::NONE,
                r#authenticatorId: 0,
              }
            }
          }
          impl binder::Parcelable for r#AuthenticatorSpec {
            fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
              parcel.sized_write(|subparcel| {
                subparcel.write(&self.r#authenticatorType)?;
                subparcel.write(&self.r#authenticatorId)?;
                Ok(())
              })
            }
            fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
              parcel.sized_read(|subparcel| {
                if subparcel.has_more_data() {
                  self.r#authenticatorType = subparcel.read()?;
                }
                if subparcel.has_more_data() {
                  self.r#authenticatorId = subparcel.read()?;
                }
                Ok(())
              })
            }
          }
          binder::impl_serialize_for_parcelable!(r#AuthenticatorSpec);
          binder::impl_deserialize_for_parcelable!(r#AuthenticatorSpec);
          impl binder::binder_impl::ParcelableMetadata for r#AuthenticatorSpec {
            fn get_descriptor() -> &'static str { "android.system.keystore2.AuthenticatorSpec" }
            fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
          }
          pub(crate) mod mangled {
           pub use super::r#AuthenticatorSpec as _7_android_6_system_9_keystore2_17_AuthenticatorSpec;
          }
        }
        pub mod Authorization {
          /*
           * This file is auto-generated.  DO NOT MODIFY.
           * Using: out/host/linux-x86/bin/aidl --lang=rust --structured --version 5 --hash 98d815116c190250e9e5a1d9182cea8126fd0e97 -t --stability vintf --min_sdk_version current -pout/soong/.intermediates/hardware/interfaces/security/keymint/aidl/android.hardware.security.keymint_interface/4/preprocessed.aidl --ninja -d out/soong/.intermediates/system/hardware/interfaces/keystore2/aidl/android.system.keystore2-V5-rust-source/gen/android/system/keystore2/Authorization.rs.d -o out/soong/.intermediates/system/hardware/interfaces/keystore2/aidl/android.system.keystore2-V5-rust-source/gen -Nsystem/hardware/interfaces/keystore2/aidl/aidl_api/android.system.keystore2/5 system/hardware/interfaces/keystore2/aidl/aidl_api/android.system.keystore2/5/android/system/keystore2/Authorization.aidl
           *
           * DO NOT CHECK THIS FILE INTO A CODE TREE (e.g. git, etc..).
           * ALWAYS GENERATE THIS FILE FROM UPDATED AIDL COMPILER
           * AS A BUILD INTERMEDIATE ONLY. THIS IS NOT SOURCE CODE.
           */
          #![forbid(unsafe_code)]
          #![cfg_attr(rustfmt, rustfmt_skip)]
          #[derive(Debug)]
          pub struct r#Authorization {
            pub r#securityLevel: crate::mangled::_7_android_8_hardware_8_security_7_keymint_13_SecurityLevel,
            pub r#keyParameter: crate::mangled::_7_android_8_hardware_8_security_7_keymint_12_KeyParameter,
          }
          impl Default for r#Authorization {
            fn default() -> Self {
              Self {
                r#securityLevel: crate::mangled::_7_android_8_hardware_8_security_7_keymint_13_SecurityLevel::SOFTWARE,
                r#keyParameter: Default::default(),
              }
            }
          }
          impl binder::Parcelable for r#Authorization {
            fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
              parcel.sized_write(|subparcel| {
                subparcel.write(&self.r#securityLevel)?;
                subparcel.write(&self.r#keyParameter)?;
                Ok(())
              })
            }
            fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
              parcel.sized_read(|subparcel| {
                if subparcel.has_more_data() {
                  self.r#securityLevel = subparcel.read()?;
                }
                if subparcel.has_more_data() {
                  self.r#keyParameter = subparcel.read()?;
                }
                Ok(())
              })
            }
          }
          binder::impl_serialize_for_parcelable!(r#Authorization);
          binder::impl_deserialize_for_parcelable!(r#Authorization);
          impl binder::binder_impl::ParcelableMetadata for r#Authorization {
            fn get_descriptor() -> &'static str { "android.system.keystore2.Authorization" }
            fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
          }
          pub(crate) mod mangled {
           pub use super::r#Authorization as _7_android_6_system_9_keystore2_13_Authorization;
          }
        }
        pub mod CreateOperationResponse {
          /*
           * This file is auto-generated.  DO NOT MODIFY.
           * Using: out/host/linux-x86/bin/aidl --lang=rust --structured --version 5 --hash 98d815116c190250e9e5a1d9182cea8126fd0e97 -t --stability vintf --min_sdk_version current -pout/soong/.intermediates/hardware/interfaces/security/keymint/aidl/android.hardware.security.keymint_interface/4/preprocessed.aidl --ninja -d out/soong/.intermediates/system/hardware/interfaces/keystore2/aidl/android.system.keystore2-V5-rust-source/gen/android/system/keystore2/CreateOperationResponse.rs.d -o out/soong/.intermediates/system/hardware/interfaces/keystore2/aidl/android.system.keystore2-V5-rust-source/gen -Nsystem/hardware/interfaces/keystore2/aidl/aidl_api/android.system.keystore2/5 system/hardware/interfaces/keystore2/aidl/aidl_api/android.system.keystore2/5/android/system/keystore2/CreateOperationResponse.aidl
           *
           * DO NOT CHECK THIS FILE INTO A CODE TREE (e.g. git, etc..).
           * ALWAYS GENERATE THIS FILE FROM UPDATED AIDL COMPILER
           * AS A BUILD INTERMEDIATE ONLY. THIS IS NOT SOURCE CODE.
           */
          #![forbid(unsafe_code)]
          #![cfg_attr(rustfmt, rustfmt_skip)]
          #[derive(Debug)]
          pub struct r#CreateOperationResponse {
            pub r#iOperation: Option<binder::Strong<dyn crate::mangled::_7_android_6_system_9_keystore2_18_IKeystoreOperation>>,
            pub r#operationChallenge: Option<crate::mangled::_7_android_6_system_9_keystore2_18_OperationChallenge>,
            pub r#parameters: Option<crate::mangled::_7_android_6_system_9_keystore2_13_KeyParameters>,
            pub r#upgradedBlob: Option<Vec<u8>>,
          }
          impl Default for r#CreateOperationResponse {
            fn default() -> Self {
              Self {
                r#iOperation: Default::default(),
                r#operationChallenge: Default::default(),
                r#parameters: Default::default(),
                r#upgradedBlob: Default::default(),
              }
            }
          }
          impl binder::Parcelable for r#CreateOperationResponse {
            fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
              parcel.sized_write(|subparcel| {
                let __field_ref = self.r#iOperation.as_ref().ok_or(binder::StatusCode::UNEXPECTED_NULL)?;
                subparcel.write(__field_ref)?;
                subparcel.write(&self.r#operationChallenge)?;
                subparcel.write(&self.r#parameters)?;
                subparcel.write(&self.r#upgradedBlob)?;
                Ok(())
              })
            }
            fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
              parcel.sized_read(|subparcel| {
                if subparcel.has_more_data() {
                  self.r#iOperation = Some(subparcel.read()?);
                }
                if subparcel.has_more_data() {
                  self.r#operationChallenge = subparcel.read()?;
                }
                if subparcel.has_more_data() {
                  self.r#parameters = subparcel.read()?;
                }
                if subparcel.has_more_data() {
                  self.r#upgradedBlob = subparcel.read()?;
                }
                Ok(())
              })
            }
          }
          binder::impl_serialize_for_parcelable!(r#CreateOperationResponse);
          binder::impl_deserialize_for_parcelable!(r#CreateOperationResponse);
          impl binder::binder_impl::ParcelableMetadata for r#CreateOperationResponse {
            fn get_descriptor() -> &'static str { "android.system.keystore2.CreateOperationResponse" }
            fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
          }
          pub(crate) mod mangled {
           pub use super::r#CreateOperationResponse as _7_android_6_system_9_keystore2_23_CreateOperationResponse;
          }
        }
        pub mod Domain {
          /*
           * This file is auto-generated.  DO NOT MODIFY.
           * Using: out/host/linux-x86/bin/aidl --lang=rust --structured --version 5 --hash 98d815116c190250e9e5a1d9182cea8126fd0e97 -t --stability vintf --min_sdk_version current -pout/soong/.intermediates/hardware/interfaces/security/keymint/aidl/android.hardware.security.keymint_interface/4/preprocessed.aidl --ninja -d out/soong/.intermediates/system/hardware/interfaces/keystore2/aidl/android.system.keystore2-V5-rust-source/gen/android/system/keystore2/Domain.rs.d -o out/soong/.intermediates/system/hardware/interfaces/keystore2/aidl/android.system.keystore2-V5-rust-source/gen -Nsystem/hardware/interfaces/keystore2/aidl/aidl_api/android.system.keystore2/5 system/hardware/interfaces/keystore2/aidl/aidl_api/android.system.keystore2/5/android/system/keystore2/Domain.aidl
           *
           * DO NOT CHECK THIS FILE INTO A CODE TREE (e.g. git, etc..).
           * ALWAYS GENERATE THIS FILE FROM UPDATED AIDL COMPILER
           * AS A BUILD INTERMEDIATE ONLY. THIS IS NOT SOURCE CODE.
           */
          #![forbid(unsafe_code)]
          #![cfg_attr(rustfmt, rustfmt_skip)]
          #![allow(non_upper_case_globals)]
          use binder::declare_binder_enum;
          declare_binder_enum! {
            #[repr(C, align(4))]
            r#Domain : [i32; 5] {
              r#APP = 0,
              r#GRANT = 1,
              r#SELINUX = 2,
              r#BLOB = 3,
              r#KEY_ID = 4,
            }
          }
          pub(crate) mod mangled {
           pub use super::r#Domain as _7_android_6_system_9_keystore2_6_Domain;
          }
        }
        pub mod EphemeralStorageKeyResponse {
          /*
           * This file is auto-generated.  DO NOT MODIFY.
           * Using: out/host/linux-x86/bin/aidl --lang=rust --structured --version 5 --hash 98d815116c190250e9e5a1d9182cea8126fd0e97 -t --stability vintf --min_sdk_version current -pout/soong/.intermediates/hardware/interfaces/security/keymint/aidl/android.hardware.security.keymint_interface/4/preprocessed.aidl --ninja -d out/soong/.intermediates/system/hardware/interfaces/keystore2/aidl/android.system.keystore2-V5-rust-source/gen/android/system/keystore2/EphemeralStorageKeyResponse.rs.d -o out/soong/.intermediates/system/hardware/interfaces/keystore2/aidl/android.system.keystore2-V5-rust-source/gen -Nsystem/hardware/interfaces/keystore2/aidl/aidl_api/android.system.keystore2/5 system/hardware/interfaces/keystore2/aidl/aidl_api/android.system.keystore2/5/android/system/keystore2/EphemeralStorageKeyResponse.aidl
           *
           * DO NOT CHECK THIS FILE INTO A CODE TREE (e.g. git, etc..).
           * ALWAYS GENERATE THIS FILE FROM UPDATED AIDL COMPILER
           * AS A BUILD INTERMEDIATE ONLY. THIS IS NOT SOURCE CODE.
           */
          #![forbid(unsafe_code)]
          #![cfg_attr(rustfmt, rustfmt_skip)]
          #[derive(Debug)]
          pub struct r#EphemeralStorageKeyResponse {
            pub r#ephemeralKey: Vec<u8>,
            pub r#upgradedBlob: Option<Vec<u8>>,
          }
          impl Default for r#EphemeralStorageKeyResponse {
            fn default() -> Self {
              Self {
                r#ephemeralKey: Default::default(),
                r#upgradedBlob: Default::default(),
              }
            }
          }
          impl binder::Parcelable for r#EphemeralStorageKeyResponse {
            fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
              parcel.sized_write(|subparcel| {
                subparcel.write(&self.r#ephemeralKey)?;
                subparcel.write(&self.r#upgradedBlob)?;
                Ok(())
              })
            }
            fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
              parcel.sized_read(|subparcel| {
                if subparcel.has_more_data() {
                  self.r#ephemeralKey = subparcel.read()?;
                }
                if subparcel.has_more_data() {
                  self.r#upgradedBlob = subparcel.read()?;
                }
                Ok(())
              })
            }
          }
          binder::impl_serialize_for_parcelable!(r#EphemeralStorageKeyResponse);
          binder::impl_deserialize_for_parcelable!(r#EphemeralStorageKeyResponse);
          impl binder::binder_impl::ParcelableMetadata for r#EphemeralStorageKeyResponse {
            fn get_descriptor() -> &'static str { "android.system.keystore2.EphemeralStorageKeyResponse" }
            fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
          }
          pub(crate) mod mangled {
           pub use super::r#EphemeralStorageKeyResponse as _7_android_6_system_9_keystore2_27_EphemeralStorageKeyResponse;
          }
        }
        pub mod IKeystoreOperation {
          /*
           * This file is auto-generated.  DO NOT MODIFY.
           * Using: out/host/linux-x86/bin/aidl --lang=rust --structured --version 5 --hash 98d815116c190250e9e5a1d9182cea8126fd0e97 -t --stability vintf --min_sdk_version current -pout/soong/.intermediates/hardware/interfaces/security/keymint/aidl/android.hardware.security.keymint_interface/4/preprocessed.aidl --ninja -d out/soong/.intermediates/system/hardware/interfaces/keystore2/aidl/android.system.keystore2-V5-rust-source/gen/android/system/keystore2/IKeystoreOperation.rs.d -o out/soong/.intermediates/system/hardware/interfaces/keystore2/aidl/android.system.keystore2-V5-rust-source/gen -Nsystem/hardware/interfaces/keystore2/aidl/aidl_api/android.system.keystore2/5 system/hardware/interfaces/keystore2/aidl/aidl_api/android.system.keystore2/5/android/system/keystore2/IKeystoreOperation.aidl
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
            IKeystoreOperation["android.system.keystore2.IKeystoreOperation"] {
              native: BnKeystoreOperation(on_transact),
              proxy: BpKeystoreOperation {
                cached_version: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(-1),
                cached_hash: std::sync::Mutex<Option<String>> = std::sync::Mutex::new(None)
              },
              async: IKeystoreOperationAsync(try_into_local_async),
              stability: binder::binder_impl::Stability::Vintf,
            }
          }
          pub trait IKeystoreOperation: binder::Interface + Send {
            fn get_descriptor() -> &'static str where Self: Sized { "android.system.keystore2.IKeystoreOperation" }
            fn r#updateAad<'a, 'l1, >(&'a self, _arg_aadInput: &'l1 [u8]) -> binder::Result<()>;
            fn r#update<'a, 'l1, >(&'a self, _arg_input: &'l1 [u8]) -> binder::Result<Option<Vec<u8>>>;
            fn r#finish<'a, 'l1, 'l2, >(&'a self, _arg_input: Option<&'l1 [u8]>, _arg_signature: Option<&'l2 [u8]>) -> binder::Result<Option<Vec<u8>>>;
            fn r#abort<'a, >(&'a self) -> binder::Result<()>;
            fn r#getInterfaceVersion<'a, >(&'a self) -> binder::Result<i32> {
              Ok(VERSION)
            }
            fn r#getInterfaceHash<'a, >(&'a self) -> binder::Result<String> {
              Ok(HASH.into())
            }
            fn getDefaultImpl() -> IKeystoreOperationDefaultRef where Self: Sized {
              DEFAULT_IMPL.lock().unwrap().clone()
            }
            fn setDefaultImpl(d: IKeystoreOperationDefaultRef) -> IKeystoreOperationDefaultRef where Self: Sized {
              std::mem::replace(&mut *DEFAULT_IMPL.lock().unwrap(), d)
            }
            fn try_as_async_server<'a>(&'a self) -> Option<&'a (dyn IKeystoreOperationAsyncServer + Send + Sync)> {
              None
            }
          }
          pub trait IKeystoreOperationAsync<P>: binder::Interface + Send {
            fn get_descriptor() -> &'static str where Self: Sized { "android.system.keystore2.IKeystoreOperation" }
            fn r#updateAad<'a, >(&'a self, _arg_aadInput: &'a [u8]) -> binder::BoxFuture<'a, binder::Result<()>>;
            fn r#update<'a, >(&'a self, _arg_input: &'a [u8]) -> binder::BoxFuture<'a, binder::Result<Option<Vec<u8>>>>;
            fn r#finish<'a, >(&'a self, _arg_input: Option<&'a [u8]>, _arg_signature: Option<&'a [u8]>) -> binder::BoxFuture<'a, binder::Result<Option<Vec<u8>>>>;
            fn r#abort<'a, >(&'a self) -> binder::BoxFuture<'a, binder::Result<()>>;
            fn r#getInterfaceVersion<'a, >(&'a self) -> binder::BoxFuture<'a, binder::Result<i32>> {
              Box::pin(async move { Ok(VERSION) })
            }
            fn r#getInterfaceHash<'a, >(&'a self) -> binder::BoxFuture<'a, binder::Result<String>> {
              Box::pin(async move { Ok(HASH.into()) })
            }
          }
          #[::async_trait::async_trait]
          pub trait IKeystoreOperationAsyncServer: binder::Interface + Send {
            fn get_descriptor() -> &'static str where Self: Sized { "android.system.keystore2.IKeystoreOperation" }
            async fn r#updateAad<'a, 'l1, >(&'a self, _arg_aadInput: &'l1 [u8]) -> binder::Result<()>;
            async fn r#update<'a, 'l1, >(&'a self, _arg_input: &'l1 [u8]) -> binder::Result<Option<Vec<u8>>>;
            async fn r#finish<'a, 'l1, 'l2, >(&'a self, _arg_input: Option<&'l1 [u8]>, _arg_signature: Option<&'l2 [u8]>) -> binder::Result<Option<Vec<u8>>>;
            async fn r#abort<'a, >(&'a self) -> binder::Result<()>;
          }
          impl BnKeystoreOperation {
            /// Create a new async binder service.
            pub fn new_async_binder<T, R>(inner: T, rt: R, features: binder::BinderFeatures) -> binder::Strong<dyn IKeystoreOperation>
            where
              T: IKeystoreOperationAsyncServer + binder::Interface + Send + Sync + 'static,
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
              impl<T, R> IKeystoreOperation for Wrapper<T, R>
              where
                T: IKeystoreOperationAsyncServer + Send + Sync + 'static,
                R: binder::binder_impl::BinderAsyncRuntime + Send + Sync + 'static,
              {
                fn r#updateAad<'a, 'l1, >(&'a self, _arg_aadInput: &'l1 [u8]) -> binder::Result<()> {
                  self._rt.block_on(self._inner.r#updateAad(_arg_aadInput))
                }
                fn r#update<'a, 'l1, >(&'a self, _arg_input: &'l1 [u8]) -> binder::Result<Option<Vec<u8>>> {
                  self._rt.block_on(self._inner.r#update(_arg_input))
                }
                fn r#finish<'a, 'l1, 'l2, >(&'a self, _arg_input: Option<&'l1 [u8]>, _arg_signature: Option<&'l2 [u8]>) -> binder::Result<Option<Vec<u8>>> {
                  self._rt.block_on(self._inner.r#finish(_arg_input, _arg_signature))
                }
                fn r#abort<'a, >(&'a self) -> binder::Result<()> {
                  self._rt.block_on(self._inner.r#abort())
                }
                fn try_as_async_server(&self) -> Option<&(dyn IKeystoreOperationAsyncServer + Send + Sync)> {
                  Some(&self._inner)
                }
              }
              let wrapped = Wrapper { _inner: inner, _rt: rt };
              Self::new_binder(wrapped, features)
            }
            pub fn try_into_local_async<P: binder::BinderAsyncPool + 'static>(_native: binder::binder_impl::Binder<Self>) -> Option<binder::Strong<dyn IKeystoreOperationAsync<P>>> {
              struct Wrapper {
                _native: binder::binder_impl::Binder<BnKeystoreOperation>
              }
              impl binder::Interface for Wrapper {}
              impl<P: binder::BinderAsyncPool> IKeystoreOperationAsync<P> for Wrapper {
                fn r#updateAad<'a, >(&'a self, _arg_aadInput: &'a [u8]) -> binder::BoxFuture<'a, binder::Result<()>> {
                  Box::pin(self._native.try_as_async_server().unwrap().r#updateAad(_arg_aadInput))
                }
                fn r#update<'a, >(&'a self, _arg_input: &'a [u8]) -> binder::BoxFuture<'a, binder::Result<Option<Vec<u8>>>> {
                  Box::pin(self._native.try_as_async_server().unwrap().r#update(_arg_input))
                }
                fn r#finish<'a, >(&'a self, _arg_input: Option<&'a [u8]>, _arg_signature: Option<&'a [u8]>) -> binder::BoxFuture<'a, binder::Result<Option<Vec<u8>>>> {
                  Box::pin(self._native.try_as_async_server().unwrap().r#finish(_arg_input, _arg_signature))
                }
                fn r#abort<'a, >(&'a self) -> binder::BoxFuture<'a, binder::Result<()>> {
                  Box::pin(self._native.try_as_async_server().unwrap().r#abort())
                }
              }
              if _native.try_as_async_server().is_some() {
                Some(binder::Strong::new(Box::new(Wrapper { _native }) as Box<dyn IKeystoreOperationAsync<P>>))
              } else {
                None
              }
            }
          }
          pub trait IKeystoreOperationDefault: Send + Sync {
            fn r#updateAad<'a, 'l1, >(&'a self, _arg_aadInput: &'l1 [u8]) -> binder::Result<()> {
              Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
            }
            fn r#update<'a, 'l1, >(&'a self, _arg_input: &'l1 [u8]) -> binder::Result<Option<Vec<u8>>> {
              Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
            }
            fn r#finish<'a, 'l1, 'l2, >(&'a self, _arg_input: Option<&'l1 [u8]>, _arg_signature: Option<&'l2 [u8]>) -> binder::Result<Option<Vec<u8>>> {
              Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
            }
            fn r#abort<'a, >(&'a self) -> binder::Result<()> {
              Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
            }
          }
          pub mod transactions {
            pub const r#updateAad: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 0;
            pub const r#update: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 1;
            pub const r#finish: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 2;
            pub const r#abort: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 3;
            pub const r#getInterfaceVersion: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 16777214;
            pub const r#getInterfaceHash: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 16777213;
          }
          pub type IKeystoreOperationDefaultRef = Option<std::sync::Arc<dyn IKeystoreOperationDefault>>;
          static DEFAULT_IMPL: std::sync::Mutex<IKeystoreOperationDefaultRef> = std::sync::Mutex::new(None);
          pub const VERSION: i32 = 5;
          pub const HASH: &str = "98d815116c190250e9e5a1d9182cea8126fd0e97";
          impl BpKeystoreOperation {
            fn build_parcel_updateAad(&self, _arg_aadInput: &[u8]) -> binder::Result<binder::binder_impl::Parcel> {
              let mut aidl_data = self.binder.prepare_transact()?;
              aidl_data.mark_sensitive();
              aidl_data.write(_arg_aadInput)?;
              Ok(aidl_data)
            }
            fn read_response_updateAad(&self, _arg_aadInput: &[u8], _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
              if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                if let Some(_aidl_default_impl) = <Self as IKeystoreOperation>::getDefaultImpl() {
                  return _aidl_default_impl.r#updateAad(_arg_aadInput);
                }
              }
              let _aidl_reply = _aidl_reply?;
              let _aidl_status: binder::Status = _aidl_reply.read()?;
              if !_aidl_status.is_ok() { return Err(_aidl_status); }
              Ok(())
            }
            fn build_parcel_update(&self, _arg_input: &[u8]) -> binder::Result<binder::binder_impl::Parcel> {
              let mut aidl_data = self.binder.prepare_transact()?;
              aidl_data.mark_sensitive();
              aidl_data.write(_arg_input)?;
              Ok(aidl_data)
            }
            fn read_response_update(&self, _arg_input: &[u8], _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<Option<Vec<u8>>> {
              if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                if let Some(_aidl_default_impl) = <Self as IKeystoreOperation>::getDefaultImpl() {
                  return _aidl_default_impl.r#update(_arg_input);
                }
              }
              let _aidl_reply = _aidl_reply?;
              let _aidl_status: binder::Status = _aidl_reply.read()?;
              if !_aidl_status.is_ok() { return Err(_aidl_status); }
              let _aidl_return: Option<Vec<u8>> = _aidl_reply.read()?;
              Ok(_aidl_return)
            }
            fn build_parcel_finish(&self, _arg_input: Option<&[u8]>, _arg_signature: Option<&[u8]>) -> binder::Result<binder::binder_impl::Parcel> {
              let mut aidl_data = self.binder.prepare_transact()?;
              aidl_data.mark_sensitive();
              aidl_data.write(&_arg_input)?;
              aidl_data.write(&_arg_signature)?;
              Ok(aidl_data)
            }
            fn read_response_finish(&self, _arg_input: Option<&[u8]>, _arg_signature: Option<&[u8]>, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<Option<Vec<u8>>> {
              if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                if let Some(_aidl_default_impl) = <Self as IKeystoreOperation>::getDefaultImpl() {
                  return _aidl_default_impl.r#finish(_arg_input, _arg_signature);
                }
              }
              let _aidl_reply = _aidl_reply?;
              let _aidl_status: binder::Status = _aidl_reply.read()?;
              if !_aidl_status.is_ok() { return Err(_aidl_status); }
              let _aidl_return: Option<Vec<u8>> = _aidl_reply.read()?;
              Ok(_aidl_return)
            }
            fn build_parcel_abort(&self) -> binder::Result<binder::binder_impl::Parcel> {
              let mut aidl_data = self.binder.prepare_transact()?;
              aidl_data.mark_sensitive();
              Ok(aidl_data)
            }
            fn read_response_abort(&self, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
              if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                if let Some(_aidl_default_impl) = <Self as IKeystoreOperation>::getDefaultImpl() {
                  return _aidl_default_impl.r#abort();
                }
              }
              let _aidl_reply = _aidl_reply?;
              let _aidl_status: binder::Status = _aidl_reply.read()?;
              if !_aidl_status.is_ok() { return Err(_aidl_status); }
              Ok(())
            }
            fn build_parcel_getInterfaceVersion(&self) -> binder::Result<binder::binder_impl::Parcel> {
              let mut aidl_data = self.binder.prepare_transact()?;
              aidl_data.mark_sensitive();
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
              aidl_data.mark_sensitive();
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
          impl IKeystoreOperation for BpKeystoreOperation {
            fn r#updateAad<'a, 'l1, >(&'a self, _arg_aadInput: &'l1 [u8]) -> binder::Result<()> {
              let _aidl_data = self.build_parcel_updateAad(_arg_aadInput)?;
              let _aidl_reply = self.binder.submit_transact(transactions::r#updateAad, _aidl_data, binder::binder_impl::FLAG_CLEAR_BUF | FLAG_PRIVATE_LOCAL);
              self.read_response_updateAad(_arg_aadInput, _aidl_reply)
            }
            fn r#update<'a, 'l1, >(&'a self, _arg_input: &'l1 [u8]) -> binder::Result<Option<Vec<u8>>> {
              let _aidl_data = self.build_parcel_update(_arg_input)?;
              let _aidl_reply = self.binder.submit_transact(transactions::r#update, _aidl_data, binder::binder_impl::FLAG_CLEAR_BUF | FLAG_PRIVATE_LOCAL);
              self.read_response_update(_arg_input, _aidl_reply)
            }
            fn r#finish<'a, 'l1, 'l2, >(&'a self, _arg_input: Option<&'l1 [u8]>, _arg_signature: Option<&'l2 [u8]>) -> binder::Result<Option<Vec<u8>>> {
              let _aidl_data = self.build_parcel_finish(_arg_input, _arg_signature)?;
              let _aidl_reply = self.binder.submit_transact(transactions::r#finish, _aidl_data, binder::binder_impl::FLAG_CLEAR_BUF | FLAG_PRIVATE_LOCAL);
              self.read_response_finish(_arg_input, _arg_signature, _aidl_reply)
            }
            fn r#abort<'a, >(&'a self) -> binder::Result<()> {
              let _aidl_data = self.build_parcel_abort()?;
              let _aidl_reply = self.binder.submit_transact(transactions::r#abort, _aidl_data, binder::binder_impl::FLAG_CLEAR_BUF | FLAG_PRIVATE_LOCAL);
              self.read_response_abort(_aidl_reply)
            }
            fn r#getInterfaceVersion<'a, >(&'a self) -> binder::Result<i32> {
              let _aidl_version = self.cached_version.load(std::sync::atomic::Ordering::Relaxed);
              if _aidl_version != -1 { return Ok(_aidl_version); }
              let _aidl_data = self.build_parcel_getInterfaceVersion()?;
              let _aidl_reply = self.binder.submit_transact(transactions::r#getInterfaceVersion, _aidl_data, binder::binder_impl::FLAG_CLEAR_BUF | FLAG_PRIVATE_LOCAL);
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
              let _aidl_reply = self.binder.submit_transact(transactions::r#getInterfaceHash, _aidl_data, binder::binder_impl::FLAG_CLEAR_BUF | FLAG_PRIVATE_LOCAL);
              self.read_response_getInterfaceHash(_aidl_reply)
            }
          }
          impl<P: binder::BinderAsyncPool> IKeystoreOperationAsync<P> for BpKeystoreOperation {
            fn r#updateAad<'a, >(&'a self, _arg_aadInput: &'a [u8]) -> binder::BoxFuture<'a, binder::Result<()>> {
              let _aidl_data = match self.build_parcel_updateAad(_arg_aadInput) {
                Ok(_aidl_data) => _aidl_data,
                Err(err) => return Box::pin(std::future::ready(Err(err))),
              };
              let binder = self.binder.clone();
              P::spawn(
                move || binder.submit_transact(transactions::r#updateAad, _aidl_data, binder::binder_impl::FLAG_CLEAR_BUF | FLAG_PRIVATE_LOCAL),
                move |_aidl_reply| async move {
                  self.read_response_updateAad(_arg_aadInput, _aidl_reply)
                }
              )
            }
            fn r#update<'a, >(&'a self, _arg_input: &'a [u8]) -> binder::BoxFuture<'a, binder::Result<Option<Vec<u8>>>> {
              let _aidl_data = match self.build_parcel_update(_arg_input) {
                Ok(_aidl_data) => _aidl_data,
                Err(err) => return Box::pin(std::future::ready(Err(err))),
              };
              let binder = self.binder.clone();
              P::spawn(
                move || binder.submit_transact(transactions::r#update, _aidl_data, binder::binder_impl::FLAG_CLEAR_BUF | FLAG_PRIVATE_LOCAL),
                move |_aidl_reply| async move {
                  self.read_response_update(_arg_input, _aidl_reply)
                }
              )
            }
            fn r#finish<'a, >(&'a self, _arg_input: Option<&'a [u8]>, _arg_signature: Option<&'a [u8]>) -> binder::BoxFuture<'a, binder::Result<Option<Vec<u8>>>> {
              let _aidl_data = match self.build_parcel_finish(_arg_input, _arg_signature) {
                Ok(_aidl_data) => _aidl_data,
                Err(err) => return Box::pin(std::future::ready(Err(err))),
              };
              let binder = self.binder.clone();
              P::spawn(
                move || binder.submit_transact(transactions::r#finish, _aidl_data, binder::binder_impl::FLAG_CLEAR_BUF | FLAG_PRIVATE_LOCAL),
                move |_aidl_reply| async move {
                  self.read_response_finish(_arg_input, _arg_signature, _aidl_reply)
                }
              )
            }
            fn r#abort<'a, >(&'a self) -> binder::BoxFuture<'a, binder::Result<()>> {
              let _aidl_data = match self.build_parcel_abort() {
                Ok(_aidl_data) => _aidl_data,
                Err(err) => return Box::pin(std::future::ready(Err(err))),
              };
              let binder = self.binder.clone();
              P::spawn(
                move || binder.submit_transact(transactions::r#abort, _aidl_data, binder::binder_impl::FLAG_CLEAR_BUF | FLAG_PRIVATE_LOCAL),
                move |_aidl_reply| async move {
                  self.read_response_abort(_aidl_reply)
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
                move || binder.submit_transact(transactions::r#getInterfaceVersion, _aidl_data, binder::binder_impl::FLAG_CLEAR_BUF | FLAG_PRIVATE_LOCAL),
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
                move || binder.submit_transact(transactions::r#getInterfaceHash, _aidl_data, binder::binder_impl::FLAG_CLEAR_BUF | FLAG_PRIVATE_LOCAL),
                move |_aidl_reply| async move {
                  self.read_response_getInterfaceHash(_aidl_reply)
                }
              )
            }
          }
          impl IKeystoreOperation for binder::binder_impl::Binder<BnKeystoreOperation> {
            fn r#updateAad<'a, 'l1, >(&'a self, _arg_aadInput: &'l1 [u8]) -> binder::Result<()> { self.0.r#updateAad(_arg_aadInput) }
            fn r#update<'a, 'l1, >(&'a self, _arg_input: &'l1 [u8]) -> binder::Result<Option<Vec<u8>>> { self.0.r#update(_arg_input) }
            fn r#finish<'a, 'l1, 'l2, >(&'a self, _arg_input: Option<&'l1 [u8]>, _arg_signature: Option<&'l2 [u8]>) -> binder::Result<Option<Vec<u8>>> { self.0.r#finish(_arg_input, _arg_signature) }
            fn r#abort<'a, >(&'a self) -> binder::Result<()> { self.0.r#abort() }
            fn r#getInterfaceVersion<'a, >(&'a self) -> binder::Result<i32> { self.0.r#getInterfaceVersion() }
            fn r#getInterfaceHash<'a, >(&'a self) -> binder::Result<String> { self.0.r#getInterfaceHash() }
          }
          fn on_transact(_aidl_service: &dyn IKeystoreOperation, _aidl_code: binder::binder_impl::TransactionCode, _aidl_data: &binder::binder_impl::BorrowedParcel<'_>, _aidl_reply: &mut binder::binder_impl::BorrowedParcel<'_>) -> std::result::Result<(), binder::StatusCode> {
            match _aidl_code {
              transactions::r#updateAad => {
                let _arg_aadInput: Vec<u8> = _aidl_data.read()?;
                let _aidl_return = _aidl_service.r#updateAad(&_arg_aadInput);
                match &_aidl_return {
                  Ok(_aidl_return) => {
                    _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
                  }
                  Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
                }
                Ok(())
              }
              transactions::r#update => {
                let _arg_input: Vec<u8> = _aidl_data.read()?;
                let _aidl_return = _aidl_service.r#update(&_arg_input);
                match &_aidl_return {
                  Ok(_aidl_return) => {
                    _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
                    _aidl_reply.write(_aidl_return)?;
                  }
                  Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
                }
                Ok(())
              }
              transactions::r#finish => {
                let _arg_input: Option<Vec<u8>> = _aidl_data.read()?;
                let _arg_signature: Option<Vec<u8>> = _aidl_data.read()?;
                let _aidl_return = _aidl_service.r#finish(_arg_input.as_deref(), _arg_signature.as_deref());
                match &_aidl_return {
                  Ok(_aidl_return) => {
                    _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
                    _aidl_reply.write(_aidl_return)?;
                  }
                  Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
                }
                Ok(())
              }
              transactions::r#abort => {
                let _aidl_return = _aidl_service.r#abort();
                match &_aidl_return {
                  Ok(_aidl_return) => {
                    _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
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
           pub use super::r#IKeystoreOperation as _7_android_6_system_9_keystore2_18_IKeystoreOperation;
          }
        }
        pub mod IKeystoreSecurityLevel {
          /*
           * This file is auto-generated.  DO NOT MODIFY.
           * Using: out/host/linux-x86/bin/aidl --lang=rust --structured --version 5 --hash 98d815116c190250e9e5a1d9182cea8126fd0e97 -t --stability vintf --min_sdk_version current -pout/soong/.intermediates/hardware/interfaces/security/keymint/aidl/android.hardware.security.keymint_interface/4/preprocessed.aidl --ninja -d out/soong/.intermediates/system/hardware/interfaces/keystore2/aidl/android.system.keystore2-V5-rust-source/gen/android/system/keystore2/IKeystoreSecurityLevel.rs.d -o out/soong/.intermediates/system/hardware/interfaces/keystore2/aidl/android.system.keystore2-V5-rust-source/gen -Nsystem/hardware/interfaces/keystore2/aidl/aidl_api/android.system.keystore2/5 system/hardware/interfaces/keystore2/aidl/aidl_api/android.system.keystore2/5/android/system/keystore2/IKeystoreSecurityLevel.aidl
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
            IKeystoreSecurityLevel["android.system.keystore2.IKeystoreSecurityLevel"] {
              native: BnKeystoreSecurityLevel(on_transact),
              proxy: BpKeystoreSecurityLevel {
                cached_version: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(-1),
                cached_hash: std::sync::Mutex<Option<String>> = std::sync::Mutex::new(None)
              },
              async: IKeystoreSecurityLevelAsync(try_into_local_async),
              stability: binder::binder_impl::Stability::Vintf,
            }
          }
          pub trait IKeystoreSecurityLevel: binder::Interface + Send {
            fn get_descriptor() -> &'static str where Self: Sized { "android.system.keystore2.IKeystoreSecurityLevel" }
            fn r#createOperation<'a, 'l1, 'l2, >(&'a self, _arg_key: &'l1 crate::mangled::_7_android_6_system_9_keystore2_13_KeyDescriptor, _arg_operationParameters: &'l2 [crate::mangled::_7_android_8_hardware_8_security_7_keymint_12_KeyParameter], _arg_forced: bool) -> binder::Result<crate::mangled::_7_android_6_system_9_keystore2_23_CreateOperationResponse>;
            fn r#generateKey<'a, 'l1, 'l2, 'l3, 'l4, >(&'a self, _arg_key: &'l1 crate::mangled::_7_android_6_system_9_keystore2_13_KeyDescriptor, _arg_attestationKey: Option<&'l2 crate::mangled::_7_android_6_system_9_keystore2_13_KeyDescriptor>, _arg_params: &'l3 [crate::mangled::_7_android_8_hardware_8_security_7_keymint_12_KeyParameter], _arg_flags: i32, _arg_entropy: &'l4 [u8]) -> binder::Result<crate::mangled::_7_android_6_system_9_keystore2_11_KeyMetadata>;
            fn r#importKey<'a, 'l1, 'l2, 'l3, 'l4, >(&'a self, _arg_key: &'l1 crate::mangled::_7_android_6_system_9_keystore2_13_KeyDescriptor, _arg_attestationKey: Option<&'l2 crate::mangled::_7_android_6_system_9_keystore2_13_KeyDescriptor>, _arg_params: &'l3 [crate::mangled::_7_android_8_hardware_8_security_7_keymint_12_KeyParameter], _arg_flags: i32, _arg_keyData: &'l4 [u8]) -> binder::Result<crate::mangled::_7_android_6_system_9_keystore2_11_KeyMetadata>;
            fn r#importWrappedKey<'a, 'l1, 'l2, 'l3, 'l4, 'l5, >(&'a self, _arg_key: &'l1 crate::mangled::_7_android_6_system_9_keystore2_13_KeyDescriptor, _arg_wrappingKey: &'l2 crate::mangled::_7_android_6_system_9_keystore2_13_KeyDescriptor, _arg_maskingKey: Option<&'l3 [u8]>, _arg_params: &'l4 [crate::mangled::_7_android_8_hardware_8_security_7_keymint_12_KeyParameter], _arg_authenticators: &'l5 [crate::mangled::_7_android_6_system_9_keystore2_17_AuthenticatorSpec]) -> binder::Result<crate::mangled::_7_android_6_system_9_keystore2_11_KeyMetadata>;
            fn r#convertStorageKeyToEphemeral<'a, 'l1, >(&'a self, _arg_storageKey: &'l1 crate::mangled::_7_android_6_system_9_keystore2_13_KeyDescriptor) -> binder::Result<crate::mangled::_7_android_6_system_9_keystore2_27_EphemeralStorageKeyResponse>;
            fn r#deleteKey<'a, 'l1, >(&'a self, _arg_key: &'l1 crate::mangled::_7_android_6_system_9_keystore2_13_KeyDescriptor) -> binder::Result<()>;
            fn r#getInterfaceVersion<'a, >(&'a self) -> binder::Result<i32> {
              Ok(VERSION)
            }
            fn r#getInterfaceHash<'a, >(&'a self) -> binder::Result<String> {
              Ok(HASH.into())
            }
            fn getDefaultImpl() -> IKeystoreSecurityLevelDefaultRef where Self: Sized {
              DEFAULT_IMPL.lock().unwrap().clone()
            }
            fn setDefaultImpl(d: IKeystoreSecurityLevelDefaultRef) -> IKeystoreSecurityLevelDefaultRef where Self: Sized {
              std::mem::replace(&mut *DEFAULT_IMPL.lock().unwrap(), d)
            }
            fn try_as_async_server<'a>(&'a self) -> Option<&'a (dyn IKeystoreSecurityLevelAsyncServer + Send + Sync)> {
              None
            }
          }
          pub trait IKeystoreSecurityLevelAsync<P>: binder::Interface + Send {
            fn get_descriptor() -> &'static str where Self: Sized { "android.system.keystore2.IKeystoreSecurityLevel" }
            fn r#createOperation<'a, >(&'a self, _arg_key: &'a crate::mangled::_7_android_6_system_9_keystore2_13_KeyDescriptor, _arg_operationParameters: &'a [crate::mangled::_7_android_8_hardware_8_security_7_keymint_12_KeyParameter], _arg_forced: bool) -> binder::BoxFuture<'a, binder::Result<crate::mangled::_7_android_6_system_9_keystore2_23_CreateOperationResponse>>;
            fn r#generateKey<'a, >(&'a self, _arg_key: &'a crate::mangled::_7_android_6_system_9_keystore2_13_KeyDescriptor, _arg_attestationKey: Option<&'a crate::mangled::_7_android_6_system_9_keystore2_13_KeyDescriptor>, _arg_params: &'a [crate::mangled::_7_android_8_hardware_8_security_7_keymint_12_KeyParameter], _arg_flags: i32, _arg_entropy: &'a [u8]) -> binder::BoxFuture<'a, binder::Result<crate::mangled::_7_android_6_system_9_keystore2_11_KeyMetadata>>;
            fn r#importKey<'a, >(&'a self, _arg_key: &'a crate::mangled::_7_android_6_system_9_keystore2_13_KeyDescriptor, _arg_attestationKey: Option<&'a crate::mangled::_7_android_6_system_9_keystore2_13_KeyDescriptor>, _arg_params: &'a [crate::mangled::_7_android_8_hardware_8_security_7_keymint_12_KeyParameter], _arg_flags: i32, _arg_keyData: &'a [u8]) -> binder::BoxFuture<'a, binder::Result<crate::mangled::_7_android_6_system_9_keystore2_11_KeyMetadata>>;
            fn r#importWrappedKey<'a, >(&'a self, _arg_key: &'a crate::mangled::_7_android_6_system_9_keystore2_13_KeyDescriptor, _arg_wrappingKey: &'a crate::mangled::_7_android_6_system_9_keystore2_13_KeyDescriptor, _arg_maskingKey: Option<&'a [u8]>, _arg_params: &'a [crate::mangled::_7_android_8_hardware_8_security_7_keymint_12_KeyParameter], _arg_authenticators: &'a [crate::mangled::_7_android_6_system_9_keystore2_17_AuthenticatorSpec]) -> binder::BoxFuture<'a, binder::Result<crate::mangled::_7_android_6_system_9_keystore2_11_KeyMetadata>>;
            fn r#convertStorageKeyToEphemeral<'a, >(&'a self, _arg_storageKey: &'a crate::mangled::_7_android_6_system_9_keystore2_13_KeyDescriptor) -> binder::BoxFuture<'a, binder::Result<crate::mangled::_7_android_6_system_9_keystore2_27_EphemeralStorageKeyResponse>>;
            fn r#deleteKey<'a, >(&'a self, _arg_key: &'a crate::mangled::_7_android_6_system_9_keystore2_13_KeyDescriptor) -> binder::BoxFuture<'a, binder::Result<()>>;
            fn r#getInterfaceVersion<'a, >(&'a self) -> binder::BoxFuture<'a, binder::Result<i32>> {
              Box::pin(async move { Ok(VERSION) })
            }
            fn r#getInterfaceHash<'a, >(&'a self) -> binder::BoxFuture<'a, binder::Result<String>> {
              Box::pin(async move { Ok(HASH.into()) })
            }
          }
          #[::async_trait::async_trait]
          pub trait IKeystoreSecurityLevelAsyncServer: binder::Interface + Send {
            fn get_descriptor() -> &'static str where Self: Sized { "android.system.keystore2.IKeystoreSecurityLevel" }
            async fn r#createOperation<'a, 'l1, 'l2, >(&'a self, _arg_key: &'l1 crate::mangled::_7_android_6_system_9_keystore2_13_KeyDescriptor, _arg_operationParameters: &'l2 [crate::mangled::_7_android_8_hardware_8_security_7_keymint_12_KeyParameter], _arg_forced: bool) -> binder::Result<crate::mangled::_7_android_6_system_9_keystore2_23_CreateOperationResponse>;
            async fn r#generateKey<'a, 'l1, 'l2, 'l3, 'l4, >(&'a self, _arg_key: &'l1 crate::mangled::_7_android_6_system_9_keystore2_13_KeyDescriptor, _arg_attestationKey: Option<&'l2 crate::mangled::_7_android_6_system_9_keystore2_13_KeyDescriptor>, _arg_params: &'l3 [crate::mangled::_7_android_8_hardware_8_security_7_keymint_12_KeyParameter], _arg_flags: i32, _arg_entropy: &'l4 [u8]) -> binder::Result<crate::mangled::_7_android_6_system_9_keystore2_11_KeyMetadata>;
            async fn r#importKey<'a, 'l1, 'l2, 'l3, 'l4, >(&'a self, _arg_key: &'l1 crate::mangled::_7_android_6_system_9_keystore2_13_KeyDescriptor, _arg_attestationKey: Option<&'l2 crate::mangled::_7_android_6_system_9_keystore2_13_KeyDescriptor>, _arg_params: &'l3 [crate::mangled::_7_android_8_hardware_8_security_7_keymint_12_KeyParameter], _arg_flags: i32, _arg_keyData: &'l4 [u8]) -> binder::Result<crate::mangled::_7_android_6_system_9_keystore2_11_KeyMetadata>;
            async fn r#importWrappedKey<'a, 'l1, 'l2, 'l3, 'l4, 'l5, >(&'a self, _arg_key: &'l1 crate::mangled::_7_android_6_system_9_keystore2_13_KeyDescriptor, _arg_wrappingKey: &'l2 crate::mangled::_7_android_6_system_9_keystore2_13_KeyDescriptor, _arg_maskingKey: Option<&'l3 [u8]>, _arg_params: &'l4 [crate::mangled::_7_android_8_hardware_8_security_7_keymint_12_KeyParameter], _arg_authenticators: &'l5 [crate::mangled::_7_android_6_system_9_keystore2_17_AuthenticatorSpec]) -> binder::Result<crate::mangled::_7_android_6_system_9_keystore2_11_KeyMetadata>;
            async fn r#convertStorageKeyToEphemeral<'a, 'l1, >(&'a self, _arg_storageKey: &'l1 crate::mangled::_7_android_6_system_9_keystore2_13_KeyDescriptor) -> binder::Result<crate::mangled::_7_android_6_system_9_keystore2_27_EphemeralStorageKeyResponse>;
            async fn r#deleteKey<'a, 'l1, >(&'a self, _arg_key: &'l1 crate::mangled::_7_android_6_system_9_keystore2_13_KeyDescriptor) -> binder::Result<()>;
          }
          impl BnKeystoreSecurityLevel {
            /// Create a new async binder service.
            pub fn new_async_binder<T, R>(inner: T, rt: R, features: binder::BinderFeatures) -> binder::Strong<dyn IKeystoreSecurityLevel>
            where
              T: IKeystoreSecurityLevelAsyncServer + binder::Interface + Send + Sync + 'static,
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
              impl<T, R> IKeystoreSecurityLevel for Wrapper<T, R>
              where
                T: IKeystoreSecurityLevelAsyncServer + Send + Sync + 'static,
                R: binder::binder_impl::BinderAsyncRuntime + Send + Sync + 'static,
              {
                fn r#createOperation<'a, 'l1, 'l2, >(&'a self, _arg_key: &'l1 crate::mangled::_7_android_6_system_9_keystore2_13_KeyDescriptor, _arg_operationParameters: &'l2 [crate::mangled::_7_android_8_hardware_8_security_7_keymint_12_KeyParameter], _arg_forced: bool) -> binder::Result<crate::mangled::_7_android_6_system_9_keystore2_23_CreateOperationResponse> {
                  self._rt.block_on(self._inner.r#createOperation(_arg_key, _arg_operationParameters, _arg_forced))
                }
                fn r#generateKey<'a, 'l1, 'l2, 'l3, 'l4, >(&'a self, _arg_key: &'l1 crate::mangled::_7_android_6_system_9_keystore2_13_KeyDescriptor, _arg_attestationKey: Option<&'l2 crate::mangled::_7_android_6_system_9_keystore2_13_KeyDescriptor>, _arg_params: &'l3 [crate::mangled::_7_android_8_hardware_8_security_7_keymint_12_KeyParameter], _arg_flags: i32, _arg_entropy: &'l4 [u8]) -> binder::Result<crate::mangled::_7_android_6_system_9_keystore2_11_KeyMetadata> {
                  self._rt.block_on(self._inner.r#generateKey(_arg_key, _arg_attestationKey, _arg_params, _arg_flags, _arg_entropy))
                }
                fn r#importKey<'a, 'l1, 'l2, 'l3, 'l4, >(&'a self, _arg_key: &'l1 crate::mangled::_7_android_6_system_9_keystore2_13_KeyDescriptor, _arg_attestationKey: Option<&'l2 crate::mangled::_7_android_6_system_9_keystore2_13_KeyDescriptor>, _arg_params: &'l3 [crate::mangled::_7_android_8_hardware_8_security_7_keymint_12_KeyParameter], _arg_flags: i32, _arg_keyData: &'l4 [u8]) -> binder::Result<crate::mangled::_7_android_6_system_9_keystore2_11_KeyMetadata> {
                  self._rt.block_on(self._inner.r#importKey(_arg_key, _arg_attestationKey, _arg_params, _arg_flags, _arg_keyData))
                }
                fn r#importWrappedKey<'a, 'l1, 'l2, 'l3, 'l4, 'l5, >(&'a self, _arg_key: &'l1 crate::mangled::_7_android_6_system_9_keystore2_13_KeyDescriptor, _arg_wrappingKey: &'l2 crate::mangled::_7_android_6_system_9_keystore2_13_KeyDescriptor, _arg_maskingKey: Option<&'l3 [u8]>, _arg_params: &'l4 [crate::mangled::_7_android_8_hardware_8_security_7_keymint_12_KeyParameter], _arg_authenticators: &'l5 [crate::mangled::_7_android_6_system_9_keystore2_17_AuthenticatorSpec]) -> binder::Result<crate::mangled::_7_android_6_system_9_keystore2_11_KeyMetadata> {
                  self._rt.block_on(self._inner.r#importWrappedKey(_arg_key, _arg_wrappingKey, _arg_maskingKey, _arg_params, _arg_authenticators))
                }
                fn r#convertStorageKeyToEphemeral<'a, 'l1, >(&'a self, _arg_storageKey: &'l1 crate::mangled::_7_android_6_system_9_keystore2_13_KeyDescriptor) -> binder::Result<crate::mangled::_7_android_6_system_9_keystore2_27_EphemeralStorageKeyResponse> {
                  self._rt.block_on(self._inner.r#convertStorageKeyToEphemeral(_arg_storageKey))
                }
                fn r#deleteKey<'a, 'l1, >(&'a self, _arg_key: &'l1 crate::mangled::_7_android_6_system_9_keystore2_13_KeyDescriptor) -> binder::Result<()> {
                  self._rt.block_on(self._inner.r#deleteKey(_arg_key))
                }
                fn try_as_async_server(&self) -> Option<&(dyn IKeystoreSecurityLevelAsyncServer + Send + Sync)> {
                  Some(&self._inner)
                }
              }
              let wrapped = Wrapper { _inner: inner, _rt: rt };
              Self::new_binder(wrapped, features)
            }
            pub fn try_into_local_async<P: binder::BinderAsyncPool + 'static>(_native: binder::binder_impl::Binder<Self>) -> Option<binder::Strong<dyn IKeystoreSecurityLevelAsync<P>>> {
              struct Wrapper {
                _native: binder::binder_impl::Binder<BnKeystoreSecurityLevel>
              }
              impl binder::Interface for Wrapper {}
              impl<P: binder::BinderAsyncPool> IKeystoreSecurityLevelAsync<P> for Wrapper {
                fn r#createOperation<'a, >(&'a self, _arg_key: &'a crate::mangled::_7_android_6_system_9_keystore2_13_KeyDescriptor, _arg_operationParameters: &'a [crate::mangled::_7_android_8_hardware_8_security_7_keymint_12_KeyParameter], _arg_forced: bool) -> binder::BoxFuture<'a, binder::Result<crate::mangled::_7_android_6_system_9_keystore2_23_CreateOperationResponse>> {
                  Box::pin(self._native.try_as_async_server().unwrap().r#createOperation(_arg_key, _arg_operationParameters, _arg_forced))
                }
                fn r#generateKey<'a, >(&'a self, _arg_key: &'a crate::mangled::_7_android_6_system_9_keystore2_13_KeyDescriptor, _arg_attestationKey: Option<&'a crate::mangled::_7_android_6_system_9_keystore2_13_KeyDescriptor>, _arg_params: &'a [crate::mangled::_7_android_8_hardware_8_security_7_keymint_12_KeyParameter], _arg_flags: i32, _arg_entropy: &'a [u8]) -> binder::BoxFuture<'a, binder::Result<crate::mangled::_7_android_6_system_9_keystore2_11_KeyMetadata>> {
                  Box::pin(self._native.try_as_async_server().unwrap().r#generateKey(_arg_key, _arg_attestationKey, _arg_params, _arg_flags, _arg_entropy))
                }
                fn r#importKey<'a, >(&'a self, _arg_key: &'a crate::mangled::_7_android_6_system_9_keystore2_13_KeyDescriptor, _arg_attestationKey: Option<&'a crate::mangled::_7_android_6_system_9_keystore2_13_KeyDescriptor>, _arg_params: &'a [crate::mangled::_7_android_8_hardware_8_security_7_keymint_12_KeyParameter], _arg_flags: i32, _arg_keyData: &'a [u8]) -> binder::BoxFuture<'a, binder::Result<crate::mangled::_7_android_6_system_9_keystore2_11_KeyMetadata>> {
                  Box::pin(self._native.try_as_async_server().unwrap().r#importKey(_arg_key, _arg_attestationKey, _arg_params, _arg_flags, _arg_keyData))
                }
                fn r#importWrappedKey<'a, >(&'a self, _arg_key: &'a crate::mangled::_7_android_6_system_9_keystore2_13_KeyDescriptor, _arg_wrappingKey: &'a crate::mangled::_7_android_6_system_9_keystore2_13_KeyDescriptor, _arg_maskingKey: Option<&'a [u8]>, _arg_params: &'a [crate::mangled::_7_android_8_hardware_8_security_7_keymint_12_KeyParameter], _arg_authenticators: &'a [crate::mangled::_7_android_6_system_9_keystore2_17_AuthenticatorSpec]) -> binder::BoxFuture<'a, binder::Result<crate::mangled::_7_android_6_system_9_keystore2_11_KeyMetadata>> {
                  Box::pin(self._native.try_as_async_server().unwrap().r#importWrappedKey(_arg_key, _arg_wrappingKey, _arg_maskingKey, _arg_params, _arg_authenticators))
                }
                fn r#convertStorageKeyToEphemeral<'a, >(&'a self, _arg_storageKey: &'a crate::mangled::_7_android_6_system_9_keystore2_13_KeyDescriptor) -> binder::BoxFuture<'a, binder::Result<crate::mangled::_7_android_6_system_9_keystore2_27_EphemeralStorageKeyResponse>> {
                  Box::pin(self._native.try_as_async_server().unwrap().r#convertStorageKeyToEphemeral(_arg_storageKey))
                }
                fn r#deleteKey<'a, >(&'a self, _arg_key: &'a crate::mangled::_7_android_6_system_9_keystore2_13_KeyDescriptor) -> binder::BoxFuture<'a, binder::Result<()>> {
                  Box::pin(self._native.try_as_async_server().unwrap().r#deleteKey(_arg_key))
                }
              }
              if _native.try_as_async_server().is_some() {
                Some(binder::Strong::new(Box::new(Wrapper { _native }) as Box<dyn IKeystoreSecurityLevelAsync<P>>))
              } else {
                None
              }
            }
          }
          pub trait IKeystoreSecurityLevelDefault: Send + Sync {
            fn r#createOperation<'a, 'l1, 'l2, >(&'a self, _arg_key: &'l1 crate::mangled::_7_android_6_system_9_keystore2_13_KeyDescriptor, _arg_operationParameters: &'l2 [crate::mangled::_7_android_8_hardware_8_security_7_keymint_12_KeyParameter], _arg_forced: bool) -> binder::Result<crate::mangled::_7_android_6_system_9_keystore2_23_CreateOperationResponse> {
              Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
            }
            fn r#generateKey<'a, 'l1, 'l2, 'l3, 'l4, >(&'a self, _arg_key: &'l1 crate::mangled::_7_android_6_system_9_keystore2_13_KeyDescriptor, _arg_attestationKey: Option<&'l2 crate::mangled::_7_android_6_system_9_keystore2_13_KeyDescriptor>, _arg_params: &'l3 [crate::mangled::_7_android_8_hardware_8_security_7_keymint_12_KeyParameter], _arg_flags: i32, _arg_entropy: &'l4 [u8]) -> binder::Result<crate::mangled::_7_android_6_system_9_keystore2_11_KeyMetadata> {
              Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
            }
            fn r#importKey<'a, 'l1, 'l2, 'l3, 'l4, >(&'a self, _arg_key: &'l1 crate::mangled::_7_android_6_system_9_keystore2_13_KeyDescriptor, _arg_attestationKey: Option<&'l2 crate::mangled::_7_android_6_system_9_keystore2_13_KeyDescriptor>, _arg_params: &'l3 [crate::mangled::_7_android_8_hardware_8_security_7_keymint_12_KeyParameter], _arg_flags: i32, _arg_keyData: &'l4 [u8]) -> binder::Result<crate::mangled::_7_android_6_system_9_keystore2_11_KeyMetadata> {
              Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
            }
            fn r#importWrappedKey<'a, 'l1, 'l2, 'l3, 'l4, 'l5, >(&'a self, _arg_key: &'l1 crate::mangled::_7_android_6_system_9_keystore2_13_KeyDescriptor, _arg_wrappingKey: &'l2 crate::mangled::_7_android_6_system_9_keystore2_13_KeyDescriptor, _arg_maskingKey: Option<&'l3 [u8]>, _arg_params: &'l4 [crate::mangled::_7_android_8_hardware_8_security_7_keymint_12_KeyParameter], _arg_authenticators: &'l5 [crate::mangled::_7_android_6_system_9_keystore2_17_AuthenticatorSpec]) -> binder::Result<crate::mangled::_7_android_6_system_9_keystore2_11_KeyMetadata> {
              Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
            }
            fn r#convertStorageKeyToEphemeral<'a, 'l1, >(&'a self, _arg_storageKey: &'l1 crate::mangled::_7_android_6_system_9_keystore2_13_KeyDescriptor) -> binder::Result<crate::mangled::_7_android_6_system_9_keystore2_27_EphemeralStorageKeyResponse> {
              Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
            }
            fn r#deleteKey<'a, 'l1, >(&'a self, _arg_key: &'l1 crate::mangled::_7_android_6_system_9_keystore2_13_KeyDescriptor) -> binder::Result<()> {
              Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
            }
          }
          pub mod transactions {
            pub const r#createOperation: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 0;
            pub const r#generateKey: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 1;
            pub const r#importKey: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 2;
            pub const r#importWrappedKey: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 3;
            pub const r#convertStorageKeyToEphemeral: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 4;
            pub const r#deleteKey: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 5;
            pub const r#getInterfaceVersion: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 16777214;
            pub const r#getInterfaceHash: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 16777213;
          }
          pub type IKeystoreSecurityLevelDefaultRef = Option<std::sync::Arc<dyn IKeystoreSecurityLevelDefault>>;
          static DEFAULT_IMPL: std::sync::Mutex<IKeystoreSecurityLevelDefaultRef> = std::sync::Mutex::new(None);
          pub const r#KEY_FLAG_AUTH_BOUND_WITHOUT_CRYPTOGRAPHIC_LSKF_BINDING: i32 = 1;
          pub const VERSION: i32 = 5;
          pub const HASH: &str = "98d815116c190250e9e5a1d9182cea8126fd0e97";
          impl BpKeystoreSecurityLevel {
            fn build_parcel_createOperation(&self, _arg_key: &crate::mangled::_7_android_6_system_9_keystore2_13_KeyDescriptor, _arg_operationParameters: &[crate::mangled::_7_android_8_hardware_8_security_7_keymint_12_KeyParameter], _arg_forced: bool) -> binder::Result<binder::binder_impl::Parcel> {
              let mut aidl_data = self.binder.prepare_transact()?;
              aidl_data.mark_sensitive();
              aidl_data.write(_arg_key)?;
              aidl_data.write(_arg_operationParameters)?;
              aidl_data.write(&_arg_forced)?;
              Ok(aidl_data)
            }
            fn read_response_createOperation(&self, _arg_key: &crate::mangled::_7_android_6_system_9_keystore2_13_KeyDescriptor, _arg_operationParameters: &[crate::mangled::_7_android_8_hardware_8_security_7_keymint_12_KeyParameter], _arg_forced: bool, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<crate::mangled::_7_android_6_system_9_keystore2_23_CreateOperationResponse> {
              if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                if let Some(_aidl_default_impl) = <Self as IKeystoreSecurityLevel>::getDefaultImpl() {
                  return _aidl_default_impl.r#createOperation(_arg_key, _arg_operationParameters, _arg_forced);
                }
              }
              let _aidl_reply = _aidl_reply?;
              let _aidl_status: binder::Status = _aidl_reply.read()?;
              if !_aidl_status.is_ok() { return Err(_aidl_status); }
              let _aidl_return: crate::mangled::_7_android_6_system_9_keystore2_23_CreateOperationResponse = _aidl_reply.read()?;
              Ok(_aidl_return)
            }
            fn build_parcel_generateKey(&self, _arg_key: &crate::mangled::_7_android_6_system_9_keystore2_13_KeyDescriptor, _arg_attestationKey: Option<&crate::mangled::_7_android_6_system_9_keystore2_13_KeyDescriptor>, _arg_params: &[crate::mangled::_7_android_8_hardware_8_security_7_keymint_12_KeyParameter], _arg_flags: i32, _arg_entropy: &[u8]) -> binder::Result<binder::binder_impl::Parcel> {
              let mut aidl_data = self.binder.prepare_transact()?;
              aidl_data.mark_sensitive();
              aidl_data.write(_arg_key)?;
              aidl_data.write(&_arg_attestationKey)?;
              aidl_data.write(_arg_params)?;
              aidl_data.write(&_arg_flags)?;
              aidl_data.write(_arg_entropy)?;
              Ok(aidl_data)
            }
            fn read_response_generateKey(&self, _arg_key: &crate::mangled::_7_android_6_system_9_keystore2_13_KeyDescriptor, _arg_attestationKey: Option<&crate::mangled::_7_android_6_system_9_keystore2_13_KeyDescriptor>, _arg_params: &[crate::mangled::_7_android_8_hardware_8_security_7_keymint_12_KeyParameter], _arg_flags: i32, _arg_entropy: &[u8], _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<crate::mangled::_7_android_6_system_9_keystore2_11_KeyMetadata> {
              if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                if let Some(_aidl_default_impl) = <Self as IKeystoreSecurityLevel>::getDefaultImpl() {
                  return _aidl_default_impl.r#generateKey(_arg_key, _arg_attestationKey, _arg_params, _arg_flags, _arg_entropy);
                }
              }
              let _aidl_reply = _aidl_reply?;
              let _aidl_status: binder::Status = _aidl_reply.read()?;
              if !_aidl_status.is_ok() { return Err(_aidl_status); }
              let _aidl_return: crate::mangled::_7_android_6_system_9_keystore2_11_KeyMetadata = _aidl_reply.read()?;
              Ok(_aidl_return)
            }
            fn build_parcel_importKey(&self, _arg_key: &crate::mangled::_7_android_6_system_9_keystore2_13_KeyDescriptor, _arg_attestationKey: Option<&crate::mangled::_7_android_6_system_9_keystore2_13_KeyDescriptor>, _arg_params: &[crate::mangled::_7_android_8_hardware_8_security_7_keymint_12_KeyParameter], _arg_flags: i32, _arg_keyData: &[u8]) -> binder::Result<binder::binder_impl::Parcel> {
              let mut aidl_data = self.binder.prepare_transact()?;
              aidl_data.mark_sensitive();
              aidl_data.write(_arg_key)?;
              aidl_data.write(&_arg_attestationKey)?;
              aidl_data.write(_arg_params)?;
              aidl_data.write(&_arg_flags)?;
              aidl_data.write(_arg_keyData)?;
              Ok(aidl_data)
            }
            fn read_response_importKey(&self, _arg_key: &crate::mangled::_7_android_6_system_9_keystore2_13_KeyDescriptor, _arg_attestationKey: Option<&crate::mangled::_7_android_6_system_9_keystore2_13_KeyDescriptor>, _arg_params: &[crate::mangled::_7_android_8_hardware_8_security_7_keymint_12_KeyParameter], _arg_flags: i32, _arg_keyData: &[u8], _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<crate::mangled::_7_android_6_system_9_keystore2_11_KeyMetadata> {
              if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                if let Some(_aidl_default_impl) = <Self as IKeystoreSecurityLevel>::getDefaultImpl() {
                  return _aidl_default_impl.r#importKey(_arg_key, _arg_attestationKey, _arg_params, _arg_flags, _arg_keyData);
                }
              }
              let _aidl_reply = _aidl_reply?;
              let _aidl_status: binder::Status = _aidl_reply.read()?;
              if !_aidl_status.is_ok() { return Err(_aidl_status); }
              let _aidl_return: crate::mangled::_7_android_6_system_9_keystore2_11_KeyMetadata = _aidl_reply.read()?;
              Ok(_aidl_return)
            }
            fn build_parcel_importWrappedKey(&self, _arg_key: &crate::mangled::_7_android_6_system_9_keystore2_13_KeyDescriptor, _arg_wrappingKey: &crate::mangled::_7_android_6_system_9_keystore2_13_KeyDescriptor, _arg_maskingKey: Option<&[u8]>, _arg_params: &[crate::mangled::_7_android_8_hardware_8_security_7_keymint_12_KeyParameter], _arg_authenticators: &[crate::mangled::_7_android_6_system_9_keystore2_17_AuthenticatorSpec]) -> binder::Result<binder::binder_impl::Parcel> {
              let mut aidl_data = self.binder.prepare_transact()?;
              aidl_data.mark_sensitive();
              aidl_data.write(_arg_key)?;
              aidl_data.write(_arg_wrappingKey)?;
              aidl_data.write(&_arg_maskingKey)?;
              aidl_data.write(_arg_params)?;
              aidl_data.write(_arg_authenticators)?;
              Ok(aidl_data)
            }
            fn read_response_importWrappedKey(&self, _arg_key: &crate::mangled::_7_android_6_system_9_keystore2_13_KeyDescriptor, _arg_wrappingKey: &crate::mangled::_7_android_6_system_9_keystore2_13_KeyDescriptor, _arg_maskingKey: Option<&[u8]>, _arg_params: &[crate::mangled::_7_android_8_hardware_8_security_7_keymint_12_KeyParameter], _arg_authenticators: &[crate::mangled::_7_android_6_system_9_keystore2_17_AuthenticatorSpec], _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<crate::mangled::_7_android_6_system_9_keystore2_11_KeyMetadata> {
              if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                if let Some(_aidl_default_impl) = <Self as IKeystoreSecurityLevel>::getDefaultImpl() {
                  return _aidl_default_impl.r#importWrappedKey(_arg_key, _arg_wrappingKey, _arg_maskingKey, _arg_params, _arg_authenticators);
                }
              }
              let _aidl_reply = _aidl_reply?;
              let _aidl_status: binder::Status = _aidl_reply.read()?;
              if !_aidl_status.is_ok() { return Err(_aidl_status); }
              let _aidl_return: crate::mangled::_7_android_6_system_9_keystore2_11_KeyMetadata = _aidl_reply.read()?;
              Ok(_aidl_return)
            }
            fn build_parcel_convertStorageKeyToEphemeral(&self, _arg_storageKey: &crate::mangled::_7_android_6_system_9_keystore2_13_KeyDescriptor) -> binder::Result<binder::binder_impl::Parcel> {
              let mut aidl_data = self.binder.prepare_transact()?;
              aidl_data.mark_sensitive();
              aidl_data.write(_arg_storageKey)?;
              Ok(aidl_data)
            }
            fn read_response_convertStorageKeyToEphemeral(&self, _arg_storageKey: &crate::mangled::_7_android_6_system_9_keystore2_13_KeyDescriptor, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<crate::mangled::_7_android_6_system_9_keystore2_27_EphemeralStorageKeyResponse> {
              if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                if let Some(_aidl_default_impl) = <Self as IKeystoreSecurityLevel>::getDefaultImpl() {
                  return _aidl_default_impl.r#convertStorageKeyToEphemeral(_arg_storageKey);
                }
              }
              let _aidl_reply = _aidl_reply?;
              let _aidl_status: binder::Status = _aidl_reply.read()?;
              if !_aidl_status.is_ok() { return Err(_aidl_status); }
              let _aidl_return: crate::mangled::_7_android_6_system_9_keystore2_27_EphemeralStorageKeyResponse = _aidl_reply.read()?;
              Ok(_aidl_return)
            }
            fn build_parcel_deleteKey(&self, _arg_key: &crate::mangled::_7_android_6_system_9_keystore2_13_KeyDescriptor) -> binder::Result<binder::binder_impl::Parcel> {
              let mut aidl_data = self.binder.prepare_transact()?;
              aidl_data.mark_sensitive();
              aidl_data.write(_arg_key)?;
              Ok(aidl_data)
            }
            fn read_response_deleteKey(&self, _arg_key: &crate::mangled::_7_android_6_system_9_keystore2_13_KeyDescriptor, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
              if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                if let Some(_aidl_default_impl) = <Self as IKeystoreSecurityLevel>::getDefaultImpl() {
                  return _aidl_default_impl.r#deleteKey(_arg_key);
                }
              }
              let _aidl_reply = _aidl_reply?;
              let _aidl_status: binder::Status = _aidl_reply.read()?;
              if !_aidl_status.is_ok() { return Err(_aidl_status); }
              Ok(())
            }
            fn build_parcel_getInterfaceVersion(&self) -> binder::Result<binder::binder_impl::Parcel> {
              let mut aidl_data = self.binder.prepare_transact()?;
              aidl_data.mark_sensitive();
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
              aidl_data.mark_sensitive();
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
          impl IKeystoreSecurityLevel for BpKeystoreSecurityLevel {
            fn r#createOperation<'a, 'l1, 'l2, >(&'a self, _arg_key: &'l1 crate::mangled::_7_android_6_system_9_keystore2_13_KeyDescriptor, _arg_operationParameters: &'l2 [crate::mangled::_7_android_8_hardware_8_security_7_keymint_12_KeyParameter], _arg_forced: bool) -> binder::Result<crate::mangled::_7_android_6_system_9_keystore2_23_CreateOperationResponse> {
              let _aidl_data = self.build_parcel_createOperation(_arg_key, _arg_operationParameters, _arg_forced)?;
              let _aidl_reply = self.binder.submit_transact(transactions::r#createOperation, _aidl_data, binder::binder_impl::FLAG_CLEAR_BUF | FLAG_PRIVATE_LOCAL);
              self.read_response_createOperation(_arg_key, _arg_operationParameters, _arg_forced, _aidl_reply)
            }
            fn r#generateKey<'a, 'l1, 'l2, 'l3, 'l4, >(&'a self, _arg_key: &'l1 crate::mangled::_7_android_6_system_9_keystore2_13_KeyDescriptor, _arg_attestationKey: Option<&'l2 crate::mangled::_7_android_6_system_9_keystore2_13_KeyDescriptor>, _arg_params: &'l3 [crate::mangled::_7_android_8_hardware_8_security_7_keymint_12_KeyParameter], _arg_flags: i32, _arg_entropy: &'l4 [u8]) -> binder::Result<crate::mangled::_7_android_6_system_9_keystore2_11_KeyMetadata> {
              let _aidl_data = self.build_parcel_generateKey(_arg_key, _arg_attestationKey, _arg_params, _arg_flags, _arg_entropy)?;
              let _aidl_reply = self.binder.submit_transact(transactions::r#generateKey, _aidl_data, binder::binder_impl::FLAG_CLEAR_BUF | FLAG_PRIVATE_LOCAL);
              self.read_response_generateKey(_arg_key, _arg_attestationKey, _arg_params, _arg_flags, _arg_entropy, _aidl_reply)
            }
            fn r#importKey<'a, 'l1, 'l2, 'l3, 'l4, >(&'a self, _arg_key: &'l1 crate::mangled::_7_android_6_system_9_keystore2_13_KeyDescriptor, _arg_attestationKey: Option<&'l2 crate::mangled::_7_android_6_system_9_keystore2_13_KeyDescriptor>, _arg_params: &'l3 [crate::mangled::_7_android_8_hardware_8_security_7_keymint_12_KeyParameter], _arg_flags: i32, _arg_keyData: &'l4 [u8]) -> binder::Result<crate::mangled::_7_android_6_system_9_keystore2_11_KeyMetadata> {
              let _aidl_data = self.build_parcel_importKey(_arg_key, _arg_attestationKey, _arg_params, _arg_flags, _arg_keyData)?;
              let _aidl_reply = self.binder.submit_transact(transactions::r#importKey, _aidl_data, binder::binder_impl::FLAG_CLEAR_BUF | FLAG_PRIVATE_LOCAL);
              self.read_response_importKey(_arg_key, _arg_attestationKey, _arg_params, _arg_flags, _arg_keyData, _aidl_reply)
            }
            fn r#importWrappedKey<'a, 'l1, 'l2, 'l3, 'l4, 'l5, >(&'a self, _arg_key: &'l1 crate::mangled::_7_android_6_system_9_keystore2_13_KeyDescriptor, _arg_wrappingKey: &'l2 crate::mangled::_7_android_6_system_9_keystore2_13_KeyDescriptor, _arg_maskingKey: Option<&'l3 [u8]>, _arg_params: &'l4 [crate::mangled::_7_android_8_hardware_8_security_7_keymint_12_KeyParameter], _arg_authenticators: &'l5 [crate::mangled::_7_android_6_system_9_keystore2_17_AuthenticatorSpec]) -> binder::Result<crate::mangled::_7_android_6_system_9_keystore2_11_KeyMetadata> {
              let _aidl_data = self.build_parcel_importWrappedKey(_arg_key, _arg_wrappingKey, _arg_maskingKey, _arg_params, _arg_authenticators)?;
              let _aidl_reply = self.binder.submit_transact(transactions::r#importWrappedKey, _aidl_data, binder::binder_impl::FLAG_CLEAR_BUF | FLAG_PRIVATE_LOCAL);
              self.read_response_importWrappedKey(_arg_key, _arg_wrappingKey, _arg_maskingKey, _arg_params, _arg_authenticators, _aidl_reply)
            }
            fn r#convertStorageKeyToEphemeral<'a, 'l1, >(&'a self, _arg_storageKey: &'l1 crate::mangled::_7_android_6_system_9_keystore2_13_KeyDescriptor) -> binder::Result<crate::mangled::_7_android_6_system_9_keystore2_27_EphemeralStorageKeyResponse> {
              let _aidl_data = self.build_parcel_convertStorageKeyToEphemeral(_arg_storageKey)?;
              let _aidl_reply = self.binder.submit_transact(transactions::r#convertStorageKeyToEphemeral, _aidl_data, binder::binder_impl::FLAG_CLEAR_BUF | FLAG_PRIVATE_LOCAL);
              self.read_response_convertStorageKeyToEphemeral(_arg_storageKey, _aidl_reply)
            }
            fn r#deleteKey<'a, 'l1, >(&'a self, _arg_key: &'l1 crate::mangled::_7_android_6_system_9_keystore2_13_KeyDescriptor) -> binder::Result<()> {
              let _aidl_data = self.build_parcel_deleteKey(_arg_key)?;
              let _aidl_reply = self.binder.submit_transact(transactions::r#deleteKey, _aidl_data, binder::binder_impl::FLAG_CLEAR_BUF | FLAG_PRIVATE_LOCAL);
              self.read_response_deleteKey(_arg_key, _aidl_reply)
            }
            fn r#getInterfaceVersion<'a, >(&'a self) -> binder::Result<i32> {
              let _aidl_version = self.cached_version.load(std::sync::atomic::Ordering::Relaxed);
              if _aidl_version != -1 { return Ok(_aidl_version); }
              let _aidl_data = self.build_parcel_getInterfaceVersion()?;
              let _aidl_reply = self.binder.submit_transact(transactions::r#getInterfaceVersion, _aidl_data, binder::binder_impl::FLAG_CLEAR_BUF | FLAG_PRIVATE_LOCAL);
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
              let _aidl_reply = self.binder.submit_transact(transactions::r#getInterfaceHash, _aidl_data, binder::binder_impl::FLAG_CLEAR_BUF | FLAG_PRIVATE_LOCAL);
              self.read_response_getInterfaceHash(_aidl_reply)
            }
          }
          impl<P: binder::BinderAsyncPool> IKeystoreSecurityLevelAsync<P> for BpKeystoreSecurityLevel {
            fn r#createOperation<'a, >(&'a self, _arg_key: &'a crate::mangled::_7_android_6_system_9_keystore2_13_KeyDescriptor, _arg_operationParameters: &'a [crate::mangled::_7_android_8_hardware_8_security_7_keymint_12_KeyParameter], _arg_forced: bool) -> binder::BoxFuture<'a, binder::Result<crate::mangled::_7_android_6_system_9_keystore2_23_CreateOperationResponse>> {
              let _aidl_data = match self.build_parcel_createOperation(_arg_key, _arg_operationParameters, _arg_forced) {
                Ok(_aidl_data) => _aidl_data,
                Err(err) => return Box::pin(std::future::ready(Err(err))),
              };
              let binder = self.binder.clone();
              P::spawn(
                move || binder.submit_transact(transactions::r#createOperation, _aidl_data, binder::binder_impl::FLAG_CLEAR_BUF | FLAG_PRIVATE_LOCAL),
                move |_aidl_reply| async move {
                  self.read_response_createOperation(_arg_key, _arg_operationParameters, _arg_forced, _aidl_reply)
                }
              )
            }
            fn r#generateKey<'a, >(&'a self, _arg_key: &'a crate::mangled::_7_android_6_system_9_keystore2_13_KeyDescriptor, _arg_attestationKey: Option<&'a crate::mangled::_7_android_6_system_9_keystore2_13_KeyDescriptor>, _arg_params: &'a [crate::mangled::_7_android_8_hardware_8_security_7_keymint_12_KeyParameter], _arg_flags: i32, _arg_entropy: &'a [u8]) -> binder::BoxFuture<'a, binder::Result<crate::mangled::_7_android_6_system_9_keystore2_11_KeyMetadata>> {
              let _aidl_data = match self.build_parcel_generateKey(_arg_key, _arg_attestationKey, _arg_params, _arg_flags, _arg_entropy) {
                Ok(_aidl_data) => _aidl_data,
                Err(err) => return Box::pin(std::future::ready(Err(err))),
              };
              let binder = self.binder.clone();
              P::spawn(
                move || binder.submit_transact(transactions::r#generateKey, _aidl_data, binder::binder_impl::FLAG_CLEAR_BUF | FLAG_PRIVATE_LOCAL),
                move |_aidl_reply| async move {
                  self.read_response_generateKey(_arg_key, _arg_attestationKey, _arg_params, _arg_flags, _arg_entropy, _aidl_reply)
                }
              )
            }
            fn r#importKey<'a, >(&'a self, _arg_key: &'a crate::mangled::_7_android_6_system_9_keystore2_13_KeyDescriptor, _arg_attestationKey: Option<&'a crate::mangled::_7_android_6_system_9_keystore2_13_KeyDescriptor>, _arg_params: &'a [crate::mangled::_7_android_8_hardware_8_security_7_keymint_12_KeyParameter], _arg_flags: i32, _arg_keyData: &'a [u8]) -> binder::BoxFuture<'a, binder::Result<crate::mangled::_7_android_6_system_9_keystore2_11_KeyMetadata>> {
              let _aidl_data = match self.build_parcel_importKey(_arg_key, _arg_attestationKey, _arg_params, _arg_flags, _arg_keyData) {
                Ok(_aidl_data) => _aidl_data,
                Err(err) => return Box::pin(std::future::ready(Err(err))),
              };
              let binder = self.binder.clone();
              P::spawn(
                move || binder.submit_transact(transactions::r#importKey, _aidl_data, binder::binder_impl::FLAG_CLEAR_BUF | FLAG_PRIVATE_LOCAL),
                move |_aidl_reply| async move {
                  self.read_response_importKey(_arg_key, _arg_attestationKey, _arg_params, _arg_flags, _arg_keyData, _aidl_reply)
                }
              )
            }
            fn r#importWrappedKey<'a, >(&'a self, _arg_key: &'a crate::mangled::_7_android_6_system_9_keystore2_13_KeyDescriptor, _arg_wrappingKey: &'a crate::mangled::_7_android_6_system_9_keystore2_13_KeyDescriptor, _arg_maskingKey: Option<&'a [u8]>, _arg_params: &'a [crate::mangled::_7_android_8_hardware_8_security_7_keymint_12_KeyParameter], _arg_authenticators: &'a [crate::mangled::_7_android_6_system_9_keystore2_17_AuthenticatorSpec]) -> binder::BoxFuture<'a, binder::Result<crate::mangled::_7_android_6_system_9_keystore2_11_KeyMetadata>> {
              let _aidl_data = match self.build_parcel_importWrappedKey(_arg_key, _arg_wrappingKey, _arg_maskingKey, _arg_params, _arg_authenticators) {
                Ok(_aidl_data) => _aidl_data,
                Err(err) => return Box::pin(std::future::ready(Err(err))),
              };
              let binder = self.binder.clone();
              P::spawn(
                move || binder.submit_transact(transactions::r#importWrappedKey, _aidl_data, binder::binder_impl::FLAG_CLEAR_BUF | FLAG_PRIVATE_LOCAL),
                move |_aidl_reply| async move {
                  self.read_response_importWrappedKey(_arg_key, _arg_wrappingKey, _arg_maskingKey, _arg_params, _arg_authenticators, _aidl_reply)
                }
              )
            }
            fn r#convertStorageKeyToEphemeral<'a, >(&'a self, _arg_storageKey: &'a crate::mangled::_7_android_6_system_9_keystore2_13_KeyDescriptor) -> binder::BoxFuture<'a, binder::Result<crate::mangled::_7_android_6_system_9_keystore2_27_EphemeralStorageKeyResponse>> {
              let _aidl_data = match self.build_parcel_convertStorageKeyToEphemeral(_arg_storageKey) {
                Ok(_aidl_data) => _aidl_data,
                Err(err) => return Box::pin(std::future::ready(Err(err))),
              };
              let binder = self.binder.clone();
              P::spawn(
                move || binder.submit_transact(transactions::r#convertStorageKeyToEphemeral, _aidl_data, binder::binder_impl::FLAG_CLEAR_BUF | FLAG_PRIVATE_LOCAL),
                move |_aidl_reply| async move {
                  self.read_response_convertStorageKeyToEphemeral(_arg_storageKey, _aidl_reply)
                }
              )
            }
            fn r#deleteKey<'a, >(&'a self, _arg_key: &'a crate::mangled::_7_android_6_system_9_keystore2_13_KeyDescriptor) -> binder::BoxFuture<'a, binder::Result<()>> {
              let _aidl_data = match self.build_parcel_deleteKey(_arg_key) {
                Ok(_aidl_data) => _aidl_data,
                Err(err) => return Box::pin(std::future::ready(Err(err))),
              };
              let binder = self.binder.clone();
              P::spawn(
                move || binder.submit_transact(transactions::r#deleteKey, _aidl_data, binder::binder_impl::FLAG_CLEAR_BUF | FLAG_PRIVATE_LOCAL),
                move |_aidl_reply| async move {
                  self.read_response_deleteKey(_arg_key, _aidl_reply)
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
                move || binder.submit_transact(transactions::r#getInterfaceVersion, _aidl_data, binder::binder_impl::FLAG_CLEAR_BUF | FLAG_PRIVATE_LOCAL),
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
                move || binder.submit_transact(transactions::r#getInterfaceHash, _aidl_data, binder::binder_impl::FLAG_CLEAR_BUF | FLAG_PRIVATE_LOCAL),
                move |_aidl_reply| async move {
                  self.read_response_getInterfaceHash(_aidl_reply)
                }
              )
            }
          }
          impl IKeystoreSecurityLevel for binder::binder_impl::Binder<BnKeystoreSecurityLevel> {
            fn r#createOperation<'a, 'l1, 'l2, >(&'a self, _arg_key: &'l1 crate::mangled::_7_android_6_system_9_keystore2_13_KeyDescriptor, _arg_operationParameters: &'l2 [crate::mangled::_7_android_8_hardware_8_security_7_keymint_12_KeyParameter], _arg_forced: bool) -> binder::Result<crate::mangled::_7_android_6_system_9_keystore2_23_CreateOperationResponse> { self.0.r#createOperation(_arg_key, _arg_operationParameters, _arg_forced) }
            fn r#generateKey<'a, 'l1, 'l2, 'l3, 'l4, >(&'a self, _arg_key: &'l1 crate::mangled::_7_android_6_system_9_keystore2_13_KeyDescriptor, _arg_attestationKey: Option<&'l2 crate::mangled::_7_android_6_system_9_keystore2_13_KeyDescriptor>, _arg_params: &'l3 [crate::mangled::_7_android_8_hardware_8_security_7_keymint_12_KeyParameter], _arg_flags: i32, _arg_entropy: &'l4 [u8]) -> binder::Result<crate::mangled::_7_android_6_system_9_keystore2_11_KeyMetadata> { self.0.r#generateKey(_arg_key, _arg_attestationKey, _arg_params, _arg_flags, _arg_entropy) }
            fn r#importKey<'a, 'l1, 'l2, 'l3, 'l4, >(&'a self, _arg_key: &'l1 crate::mangled::_7_android_6_system_9_keystore2_13_KeyDescriptor, _arg_attestationKey: Option<&'l2 crate::mangled::_7_android_6_system_9_keystore2_13_KeyDescriptor>, _arg_params: &'l3 [crate::mangled::_7_android_8_hardware_8_security_7_keymint_12_KeyParameter], _arg_flags: i32, _arg_keyData: &'l4 [u8]) -> binder::Result<crate::mangled::_7_android_6_system_9_keystore2_11_KeyMetadata> { self.0.r#importKey(_arg_key, _arg_attestationKey, _arg_params, _arg_flags, _arg_keyData) }
            fn r#importWrappedKey<'a, 'l1, 'l2, 'l3, 'l4, 'l5, >(&'a self, _arg_key: &'l1 crate::mangled::_7_android_6_system_9_keystore2_13_KeyDescriptor, _arg_wrappingKey: &'l2 crate::mangled::_7_android_6_system_9_keystore2_13_KeyDescriptor, _arg_maskingKey: Option<&'l3 [u8]>, _arg_params: &'l4 [crate::mangled::_7_android_8_hardware_8_security_7_keymint_12_KeyParameter], _arg_authenticators: &'l5 [crate::mangled::_7_android_6_system_9_keystore2_17_AuthenticatorSpec]) -> binder::Result<crate::mangled::_7_android_6_system_9_keystore2_11_KeyMetadata> { self.0.r#importWrappedKey(_arg_key, _arg_wrappingKey, _arg_maskingKey, _arg_params, _arg_authenticators) }
            fn r#convertStorageKeyToEphemeral<'a, 'l1, >(&'a self, _arg_storageKey: &'l1 crate::mangled::_7_android_6_system_9_keystore2_13_KeyDescriptor) -> binder::Result<crate::mangled::_7_android_6_system_9_keystore2_27_EphemeralStorageKeyResponse> { self.0.r#convertStorageKeyToEphemeral(_arg_storageKey) }
            fn r#deleteKey<'a, 'l1, >(&'a self, _arg_key: &'l1 crate::mangled::_7_android_6_system_9_keystore2_13_KeyDescriptor) -> binder::Result<()> { self.0.r#deleteKey(_arg_key) }
            fn r#getInterfaceVersion<'a, >(&'a self) -> binder::Result<i32> { self.0.r#getInterfaceVersion() }
            fn r#getInterfaceHash<'a, >(&'a self) -> binder::Result<String> { self.0.r#getInterfaceHash() }
          }
          fn on_transact(_aidl_service: &dyn IKeystoreSecurityLevel, _aidl_code: binder::binder_impl::TransactionCode, _aidl_data: &binder::binder_impl::BorrowedParcel<'_>, _aidl_reply: &mut binder::binder_impl::BorrowedParcel<'_>) -> std::result::Result<(), binder::StatusCode> {
            match _aidl_code {
              transactions::r#createOperation => {
                let _arg_key: crate::mangled::_7_android_6_system_9_keystore2_13_KeyDescriptor = _aidl_data.read()?;
                let _arg_operationParameters: Vec<crate::mangled::_7_android_8_hardware_8_security_7_keymint_12_KeyParameter> = _aidl_data.read()?;
                let _arg_forced: bool = _aidl_data.read()?;
                let _aidl_return = _aidl_service.r#createOperation(&_arg_key, &_arg_operationParameters, _arg_forced);
                match &_aidl_return {
                  Ok(_aidl_return) => {
                    _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
                    _aidl_reply.write(_aidl_return)?;
                  }
                  Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
                }
                Ok(())
              }
              transactions::r#generateKey => {
                let _arg_key: crate::mangled::_7_android_6_system_9_keystore2_13_KeyDescriptor = _aidl_data.read()?;
                let _arg_attestationKey: Option<crate::mangled::_7_android_6_system_9_keystore2_13_KeyDescriptor> = _aidl_data.read()?;
                let _arg_params: Vec<crate::mangled::_7_android_8_hardware_8_security_7_keymint_12_KeyParameter> = _aidl_data.read()?;
                let _arg_flags: i32 = _aidl_data.read()?;
                let _arg_entropy: Vec<u8> = _aidl_data.read()?;
                let _aidl_return = _aidl_service.r#generateKey(&_arg_key, _arg_attestationKey.as_ref(), &_arg_params, _arg_flags, &_arg_entropy);
                match &_aidl_return {
                  Ok(_aidl_return) => {
                    _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
                    _aidl_reply.write(_aidl_return)?;
                  }
                  Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
                }
                Ok(())
              }
              transactions::r#importKey => {
                let _arg_key: crate::mangled::_7_android_6_system_9_keystore2_13_KeyDescriptor = _aidl_data.read()?;
                let _arg_attestationKey: Option<crate::mangled::_7_android_6_system_9_keystore2_13_KeyDescriptor> = _aidl_data.read()?;
                let _arg_params: Vec<crate::mangled::_7_android_8_hardware_8_security_7_keymint_12_KeyParameter> = _aidl_data.read()?;
                let _arg_flags: i32 = _aidl_data.read()?;
                let _arg_keyData: Vec<u8> = _aidl_data.read()?;
                let _aidl_return = _aidl_service.r#importKey(&_arg_key, _arg_attestationKey.as_ref(), &_arg_params, _arg_flags, &_arg_keyData);
                match &_aidl_return {
                  Ok(_aidl_return) => {
                    _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
                    _aidl_reply.write(_aidl_return)?;
                  }
                  Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
                }
                Ok(())
              }
              transactions::r#importWrappedKey => {
                let _arg_key: crate::mangled::_7_android_6_system_9_keystore2_13_KeyDescriptor = _aidl_data.read()?;
                let _arg_wrappingKey: crate::mangled::_7_android_6_system_9_keystore2_13_KeyDescriptor = _aidl_data.read()?;
                let _arg_maskingKey: Option<Vec<u8>> = _aidl_data.read()?;
                let _arg_params: Vec<crate::mangled::_7_android_8_hardware_8_security_7_keymint_12_KeyParameter> = _aidl_data.read()?;
                let _arg_authenticators: Vec<crate::mangled::_7_android_6_system_9_keystore2_17_AuthenticatorSpec> = _aidl_data.read()?;
                let _aidl_return = _aidl_service.r#importWrappedKey(&_arg_key, &_arg_wrappingKey, _arg_maskingKey.as_deref(), &_arg_params, &_arg_authenticators);
                match &_aidl_return {
                  Ok(_aidl_return) => {
                    _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
                    _aidl_reply.write(_aidl_return)?;
                  }
                  Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
                }
                Ok(())
              }
              transactions::r#convertStorageKeyToEphemeral => {
                let _arg_storageKey: crate::mangled::_7_android_6_system_9_keystore2_13_KeyDescriptor = _aidl_data.read()?;
                let _aidl_return = _aidl_service.r#convertStorageKeyToEphemeral(&_arg_storageKey);
                match &_aidl_return {
                  Ok(_aidl_return) => {
                    _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
                    _aidl_reply.write(_aidl_return)?;
                  }
                  Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
                }
                Ok(())
              }
              transactions::r#deleteKey => {
                let _arg_key: crate::mangled::_7_android_6_system_9_keystore2_13_KeyDescriptor = _aidl_data.read()?;
                let _aidl_return = _aidl_service.r#deleteKey(&_arg_key);
                match &_aidl_return {
                  Ok(_aidl_return) => {
                    _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
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
           pub use super::r#IKeystoreSecurityLevel as _7_android_6_system_9_keystore2_22_IKeystoreSecurityLevel;
          }
        }
        pub mod IKeystoreService {
          /*
           * This file is auto-generated.  DO NOT MODIFY.
           * Using: out/host/linux-x86/bin/aidl --lang=rust --structured --version 5 --hash 98d815116c190250e9e5a1d9182cea8126fd0e97 -t --stability vintf --min_sdk_version current -pout/soong/.intermediates/hardware/interfaces/security/keymint/aidl/android.hardware.security.keymint_interface/4/preprocessed.aidl --ninja -d out/soong/.intermediates/system/hardware/interfaces/keystore2/aidl/android.system.keystore2-V5-rust-source/gen/android/system/keystore2/IKeystoreService.rs.d -o out/soong/.intermediates/system/hardware/interfaces/keystore2/aidl/android.system.keystore2-V5-rust-source/gen -Nsystem/hardware/interfaces/keystore2/aidl/aidl_api/android.system.keystore2/5 system/hardware/interfaces/keystore2/aidl/aidl_api/android.system.keystore2/5/android/system/keystore2/IKeystoreService.aidl
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
            IKeystoreService["android.system.keystore2.IKeystoreService"] {
              native: BnKeystoreService(on_transact),
              proxy: BpKeystoreService {
                cached_version: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(-1),
                cached_hash: std::sync::Mutex<Option<String>> = std::sync::Mutex::new(None)
              },
              async: IKeystoreServiceAsync(try_into_local_async),
              stability: binder::binder_impl::Stability::Vintf,
            }
          }
          pub trait IKeystoreService: binder::Interface + Send {
            fn get_descriptor() -> &'static str where Self: Sized { "android.system.keystore2.IKeystoreService" }
            fn r#getSecurityLevel<'a, >(&'a self, _arg_securityLevel: crate::mangled::_7_android_8_hardware_8_security_7_keymint_13_SecurityLevel) -> binder::Result<binder::Strong<dyn crate::mangled::_7_android_6_system_9_keystore2_22_IKeystoreSecurityLevel>>;
            fn r#getKeyEntry<'a, 'l1, >(&'a self, _arg_key: &'l1 crate::mangled::_7_android_6_system_9_keystore2_13_KeyDescriptor) -> binder::Result<crate::mangled::_7_android_6_system_9_keystore2_16_KeyEntryResponse>;
            fn r#updateSubcomponent<'a, 'l1, 'l2, 'l3, >(&'a self, _arg_key: &'l1 crate::mangled::_7_android_6_system_9_keystore2_13_KeyDescriptor, _arg_publicCert: Option<&'l2 [u8]>, _arg_certificateChain: Option<&'l3 [u8]>) -> binder::Result<()>;
            #[deprecated = "use listEntriesBatched instead."]
            fn r#listEntries<'a, >(&'a self, _arg_domain: crate::mangled::_7_android_6_system_9_keystore2_6_Domain, _arg_nspace: i64) -> binder::Result<Vec<crate::mangled::_7_android_6_system_9_keystore2_13_KeyDescriptor>>;
            fn r#deleteKey<'a, 'l1, >(&'a self, _arg_key: &'l1 crate::mangled::_7_android_6_system_9_keystore2_13_KeyDescriptor) -> binder::Result<()>;
            fn r#grant<'a, 'l1, >(&'a self, _arg_key: &'l1 crate::mangled::_7_android_6_system_9_keystore2_13_KeyDescriptor, _arg_granteeUid: i32, _arg_accessVector: i32) -> binder::Result<crate::mangled::_7_android_6_system_9_keystore2_13_KeyDescriptor>;
            fn r#ungrant<'a, 'l1, >(&'a self, _arg_key: &'l1 crate::mangled::_7_android_6_system_9_keystore2_13_KeyDescriptor, _arg_granteeUid: i32) -> binder::Result<()>;
            fn r#getNumberOfEntries<'a, >(&'a self, _arg_domain: crate::mangled::_7_android_6_system_9_keystore2_6_Domain, _arg_nspace: i64) -> binder::Result<i32>;
            fn r#listEntriesBatched<'a, 'l1, >(&'a self, _arg_domain: crate::mangled::_7_android_6_system_9_keystore2_6_Domain, _arg_nspace: i64, _arg_startingPastAlias: Option<&'l1 str>) -> binder::Result<Vec<crate::mangled::_7_android_6_system_9_keystore2_13_KeyDescriptor>>;
            fn r#getSupplementaryAttestationInfo<'a, >(&'a self, _arg_tag: crate::mangled::_7_android_8_hardware_8_security_7_keymint_3_Tag) -> binder::Result<Vec<u8>>;
            fn r#getInterfaceVersion<'a, >(&'a self) -> binder::Result<i32> {
              Ok(VERSION)
            }
            fn r#getInterfaceHash<'a, >(&'a self) -> binder::Result<String> {
              Ok(HASH.into())
            }
            fn getDefaultImpl() -> IKeystoreServiceDefaultRef where Self: Sized {
              DEFAULT_IMPL.lock().unwrap().clone()
            }
            fn setDefaultImpl(d: IKeystoreServiceDefaultRef) -> IKeystoreServiceDefaultRef where Self: Sized {
              std::mem::replace(&mut *DEFAULT_IMPL.lock().unwrap(), d)
            }
            fn try_as_async_server<'a>(&'a self) -> Option<&'a (dyn IKeystoreServiceAsyncServer + Send + Sync)> {
              None
            }
          }
          pub trait IKeystoreServiceAsync<P>: binder::Interface + Send {
            fn get_descriptor() -> &'static str where Self: Sized { "android.system.keystore2.IKeystoreService" }
            fn r#getSecurityLevel<'a, >(&'a self, _arg_securityLevel: crate::mangled::_7_android_8_hardware_8_security_7_keymint_13_SecurityLevel) -> binder::BoxFuture<'a, binder::Result<binder::Strong<dyn crate::mangled::_7_android_6_system_9_keystore2_22_IKeystoreSecurityLevel>>>;
            fn r#getKeyEntry<'a, >(&'a self, _arg_key: &'a crate::mangled::_7_android_6_system_9_keystore2_13_KeyDescriptor) -> binder::BoxFuture<'a, binder::Result<crate::mangled::_7_android_6_system_9_keystore2_16_KeyEntryResponse>>;
            fn r#updateSubcomponent<'a, >(&'a self, _arg_key: &'a crate::mangled::_7_android_6_system_9_keystore2_13_KeyDescriptor, _arg_publicCert: Option<&'a [u8]>, _arg_certificateChain: Option<&'a [u8]>) -> binder::BoxFuture<'a, binder::Result<()>>;
            #[deprecated = "use listEntriesBatched instead."]
            fn r#listEntries<'a, >(&'a self, _arg_domain: crate::mangled::_7_android_6_system_9_keystore2_6_Domain, _arg_nspace: i64) -> binder::BoxFuture<'a, binder::Result<Vec<crate::mangled::_7_android_6_system_9_keystore2_13_KeyDescriptor>>>;
            fn r#deleteKey<'a, >(&'a self, _arg_key: &'a crate::mangled::_7_android_6_system_9_keystore2_13_KeyDescriptor) -> binder::BoxFuture<'a, binder::Result<()>>;
            fn r#grant<'a, >(&'a self, _arg_key: &'a crate::mangled::_7_android_6_system_9_keystore2_13_KeyDescriptor, _arg_granteeUid: i32, _arg_accessVector: i32) -> binder::BoxFuture<'a, binder::Result<crate::mangled::_7_android_6_system_9_keystore2_13_KeyDescriptor>>;
            fn r#ungrant<'a, >(&'a self, _arg_key: &'a crate::mangled::_7_android_6_system_9_keystore2_13_KeyDescriptor, _arg_granteeUid: i32) -> binder::BoxFuture<'a, binder::Result<()>>;
            fn r#getNumberOfEntries<'a, >(&'a self, _arg_domain: crate::mangled::_7_android_6_system_9_keystore2_6_Domain, _arg_nspace: i64) -> binder::BoxFuture<'a, binder::Result<i32>>;
            fn r#listEntriesBatched<'a, >(&'a self, _arg_domain: crate::mangled::_7_android_6_system_9_keystore2_6_Domain, _arg_nspace: i64, _arg_startingPastAlias: Option<&'a str>) -> binder::BoxFuture<'a, binder::Result<Vec<crate::mangled::_7_android_6_system_9_keystore2_13_KeyDescriptor>>>;
            fn r#getSupplementaryAttestationInfo<'a, >(&'a self, _arg_tag: crate::mangled::_7_android_8_hardware_8_security_7_keymint_3_Tag) -> binder::BoxFuture<'a, binder::Result<Vec<u8>>>;
            fn r#getInterfaceVersion<'a, >(&'a self) -> binder::BoxFuture<'a, binder::Result<i32>> {
              Box::pin(async move { Ok(VERSION) })
            }
            fn r#getInterfaceHash<'a, >(&'a self) -> binder::BoxFuture<'a, binder::Result<String>> {
              Box::pin(async move { Ok(HASH.into()) })
            }
          }
          #[::async_trait::async_trait]
          pub trait IKeystoreServiceAsyncServer: binder::Interface + Send {
            fn get_descriptor() -> &'static str where Self: Sized { "android.system.keystore2.IKeystoreService" }
            async fn r#getSecurityLevel<'a, >(&'a self, _arg_securityLevel: crate::mangled::_7_android_8_hardware_8_security_7_keymint_13_SecurityLevel) -> binder::Result<binder::Strong<dyn crate::mangled::_7_android_6_system_9_keystore2_22_IKeystoreSecurityLevel>>;
            async fn r#getKeyEntry<'a, 'l1, >(&'a self, _arg_key: &'l1 crate::mangled::_7_android_6_system_9_keystore2_13_KeyDescriptor) -> binder::Result<crate::mangled::_7_android_6_system_9_keystore2_16_KeyEntryResponse>;
            async fn r#updateSubcomponent<'a, 'l1, 'l2, 'l3, >(&'a self, _arg_key: &'l1 crate::mangled::_7_android_6_system_9_keystore2_13_KeyDescriptor, _arg_publicCert: Option<&'l2 [u8]>, _arg_certificateChain: Option<&'l3 [u8]>) -> binder::Result<()>;
            #[deprecated = "use listEntriesBatched instead."]
            async fn r#listEntries<'a, >(&'a self, _arg_domain: crate::mangled::_7_android_6_system_9_keystore2_6_Domain, _arg_nspace: i64) -> binder::Result<Vec<crate::mangled::_7_android_6_system_9_keystore2_13_KeyDescriptor>>;
            async fn r#deleteKey<'a, 'l1, >(&'a self, _arg_key: &'l1 crate::mangled::_7_android_6_system_9_keystore2_13_KeyDescriptor) -> binder::Result<()>;
            async fn r#grant<'a, 'l1, >(&'a self, _arg_key: &'l1 crate::mangled::_7_android_6_system_9_keystore2_13_KeyDescriptor, _arg_granteeUid: i32, _arg_accessVector: i32) -> binder::Result<crate::mangled::_7_android_6_system_9_keystore2_13_KeyDescriptor>;
            async fn r#ungrant<'a, 'l1, >(&'a self, _arg_key: &'l1 crate::mangled::_7_android_6_system_9_keystore2_13_KeyDescriptor, _arg_granteeUid: i32) -> binder::Result<()>;
            async fn r#getNumberOfEntries<'a, >(&'a self, _arg_domain: crate::mangled::_7_android_6_system_9_keystore2_6_Domain, _arg_nspace: i64) -> binder::Result<i32>;
            async fn r#listEntriesBatched<'a, 'l1, >(&'a self, _arg_domain: crate::mangled::_7_android_6_system_9_keystore2_6_Domain, _arg_nspace: i64, _arg_startingPastAlias: Option<&'l1 str>) -> binder::Result<Vec<crate::mangled::_7_android_6_system_9_keystore2_13_KeyDescriptor>>;
            async fn r#getSupplementaryAttestationInfo<'a, >(&'a self, _arg_tag: crate::mangled::_7_android_8_hardware_8_security_7_keymint_3_Tag) -> binder::Result<Vec<u8>>;
          }
          impl BnKeystoreService {
            /// Create a new async binder service.
            pub fn new_async_binder<T, R>(inner: T, rt: R, features: binder::BinderFeatures) -> binder::Strong<dyn IKeystoreService>
            where
              T: IKeystoreServiceAsyncServer + binder::Interface + Send + Sync + 'static,
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
              impl<T, R> IKeystoreService for Wrapper<T, R>
              where
                T: IKeystoreServiceAsyncServer + Send + Sync + 'static,
                R: binder::binder_impl::BinderAsyncRuntime + Send + Sync + 'static,
              {
                fn r#getSecurityLevel<'a, >(&'a self, _arg_securityLevel: crate::mangled::_7_android_8_hardware_8_security_7_keymint_13_SecurityLevel) -> binder::Result<binder::Strong<dyn crate::mangled::_7_android_6_system_9_keystore2_22_IKeystoreSecurityLevel>> {
                  self._rt.block_on(self._inner.r#getSecurityLevel(_arg_securityLevel))
                }
                fn r#getKeyEntry<'a, 'l1, >(&'a self, _arg_key: &'l1 crate::mangled::_7_android_6_system_9_keystore2_13_KeyDescriptor) -> binder::Result<crate::mangled::_7_android_6_system_9_keystore2_16_KeyEntryResponse> {
                  self._rt.block_on(self._inner.r#getKeyEntry(_arg_key))
                }
                fn r#updateSubcomponent<'a, 'l1, 'l2, 'l3, >(&'a self, _arg_key: &'l1 crate::mangled::_7_android_6_system_9_keystore2_13_KeyDescriptor, _arg_publicCert: Option<&'l2 [u8]>, _arg_certificateChain: Option<&'l3 [u8]>) -> binder::Result<()> {
                  self._rt.block_on(self._inner.r#updateSubcomponent(_arg_key, _arg_publicCert, _arg_certificateChain))
                }
                fn r#listEntries<'a, >(&'a self, _arg_domain: crate::mangled::_7_android_6_system_9_keystore2_6_Domain, _arg_nspace: i64) -> binder::Result<Vec<crate::mangled::_7_android_6_system_9_keystore2_13_KeyDescriptor>> {
                  self._rt.block_on(self._inner.r#listEntries(_arg_domain, _arg_nspace))
                }
                fn r#deleteKey<'a, 'l1, >(&'a self, _arg_key: &'l1 crate::mangled::_7_android_6_system_9_keystore2_13_KeyDescriptor) -> binder::Result<()> {
                  self._rt.block_on(self._inner.r#deleteKey(_arg_key))
                }
                fn r#grant<'a, 'l1, >(&'a self, _arg_key: &'l1 crate::mangled::_7_android_6_system_9_keystore2_13_KeyDescriptor, _arg_granteeUid: i32, _arg_accessVector: i32) -> binder::Result<crate::mangled::_7_android_6_system_9_keystore2_13_KeyDescriptor> {
                  self._rt.block_on(self._inner.r#grant(_arg_key, _arg_granteeUid, _arg_accessVector))
                }
                fn r#ungrant<'a, 'l1, >(&'a self, _arg_key: &'l1 crate::mangled::_7_android_6_system_9_keystore2_13_KeyDescriptor, _arg_granteeUid: i32) -> binder::Result<()> {
                  self._rt.block_on(self._inner.r#ungrant(_arg_key, _arg_granteeUid))
                }
                fn r#getNumberOfEntries<'a, >(&'a self, _arg_domain: crate::mangled::_7_android_6_system_9_keystore2_6_Domain, _arg_nspace: i64) -> binder::Result<i32> {
                  self._rt.block_on(self._inner.r#getNumberOfEntries(_arg_domain, _arg_nspace))
                }
                fn r#listEntriesBatched<'a, 'l1, >(&'a self, _arg_domain: crate::mangled::_7_android_6_system_9_keystore2_6_Domain, _arg_nspace: i64, _arg_startingPastAlias: Option<&'l1 str>) -> binder::Result<Vec<crate::mangled::_7_android_6_system_9_keystore2_13_KeyDescriptor>> {
                  self._rt.block_on(self._inner.r#listEntriesBatched(_arg_domain, _arg_nspace, _arg_startingPastAlias))
                }
                fn r#getSupplementaryAttestationInfo<'a, >(&'a self, _arg_tag: crate::mangled::_7_android_8_hardware_8_security_7_keymint_3_Tag) -> binder::Result<Vec<u8>> {
                  self._rt.block_on(self._inner.r#getSupplementaryAttestationInfo(_arg_tag))
                }
                fn try_as_async_server(&self) -> Option<&(dyn IKeystoreServiceAsyncServer + Send + Sync)> {
                  Some(&self._inner)
                }
              }
              let wrapped = Wrapper { _inner: inner, _rt: rt };
              Self::new_binder(wrapped, features)
            }
            pub fn try_into_local_async<P: binder::BinderAsyncPool + 'static>(_native: binder::binder_impl::Binder<Self>) -> Option<binder::Strong<dyn IKeystoreServiceAsync<P>>> {
              struct Wrapper {
                _native: binder::binder_impl::Binder<BnKeystoreService>
              }
              impl binder::Interface for Wrapper {}
              impl<P: binder::BinderAsyncPool> IKeystoreServiceAsync<P> for Wrapper {
                fn r#getSecurityLevel<'a, >(&'a self, _arg_securityLevel: crate::mangled::_7_android_8_hardware_8_security_7_keymint_13_SecurityLevel) -> binder::BoxFuture<'a, binder::Result<binder::Strong<dyn crate::mangled::_7_android_6_system_9_keystore2_22_IKeystoreSecurityLevel>>> {
                  Box::pin(self._native.try_as_async_server().unwrap().r#getSecurityLevel(_arg_securityLevel))
                }
                fn r#getKeyEntry<'a, >(&'a self, _arg_key: &'a crate::mangled::_7_android_6_system_9_keystore2_13_KeyDescriptor) -> binder::BoxFuture<'a, binder::Result<crate::mangled::_7_android_6_system_9_keystore2_16_KeyEntryResponse>> {
                  Box::pin(self._native.try_as_async_server().unwrap().r#getKeyEntry(_arg_key))
                }
                fn r#updateSubcomponent<'a, >(&'a self, _arg_key: &'a crate::mangled::_7_android_6_system_9_keystore2_13_KeyDescriptor, _arg_publicCert: Option<&'a [u8]>, _arg_certificateChain: Option<&'a [u8]>) -> binder::BoxFuture<'a, binder::Result<()>> {
                  Box::pin(self._native.try_as_async_server().unwrap().r#updateSubcomponent(_arg_key, _arg_publicCert, _arg_certificateChain))
                }
                fn r#listEntries<'a, >(&'a self, _arg_domain: crate::mangled::_7_android_6_system_9_keystore2_6_Domain, _arg_nspace: i64) -> binder::BoxFuture<'a, binder::Result<Vec<crate::mangled::_7_android_6_system_9_keystore2_13_KeyDescriptor>>> {
                  Box::pin(self._native.try_as_async_server().unwrap().r#listEntries(_arg_domain, _arg_nspace))
                }
                fn r#deleteKey<'a, >(&'a self, _arg_key: &'a crate::mangled::_7_android_6_system_9_keystore2_13_KeyDescriptor) -> binder::BoxFuture<'a, binder::Result<()>> {
                  Box::pin(self._native.try_as_async_server().unwrap().r#deleteKey(_arg_key))
                }
                fn r#grant<'a, >(&'a self, _arg_key: &'a crate::mangled::_7_android_6_system_9_keystore2_13_KeyDescriptor, _arg_granteeUid: i32, _arg_accessVector: i32) -> binder::BoxFuture<'a, binder::Result<crate::mangled::_7_android_6_system_9_keystore2_13_KeyDescriptor>> {
                  Box::pin(self._native.try_as_async_server().unwrap().r#grant(_arg_key, _arg_granteeUid, _arg_accessVector))
                }
                fn r#ungrant<'a, >(&'a self, _arg_key: &'a crate::mangled::_7_android_6_system_9_keystore2_13_KeyDescriptor, _arg_granteeUid: i32) -> binder::BoxFuture<'a, binder::Result<()>> {
                  Box::pin(self._native.try_as_async_server().unwrap().r#ungrant(_arg_key, _arg_granteeUid))
                }
                fn r#getNumberOfEntries<'a, >(&'a self, _arg_domain: crate::mangled::_7_android_6_system_9_keystore2_6_Domain, _arg_nspace: i64) -> binder::BoxFuture<'a, binder::Result<i32>> {
                  Box::pin(self._native.try_as_async_server().unwrap().r#getNumberOfEntries(_arg_domain, _arg_nspace))
                }
                fn r#listEntriesBatched<'a, >(&'a self, _arg_domain: crate::mangled::_7_android_6_system_9_keystore2_6_Domain, _arg_nspace: i64, _arg_startingPastAlias: Option<&'a str>) -> binder::BoxFuture<'a, binder::Result<Vec<crate::mangled::_7_android_6_system_9_keystore2_13_KeyDescriptor>>> {
                  Box::pin(self._native.try_as_async_server().unwrap().r#listEntriesBatched(_arg_domain, _arg_nspace, _arg_startingPastAlias))
                }
                fn r#getSupplementaryAttestationInfo<'a, >(&'a self, _arg_tag: crate::mangled::_7_android_8_hardware_8_security_7_keymint_3_Tag) -> binder::BoxFuture<'a, binder::Result<Vec<u8>>> {
                  Box::pin(self._native.try_as_async_server().unwrap().r#getSupplementaryAttestationInfo(_arg_tag))
                }
              }
              if _native.try_as_async_server().is_some() {
                Some(binder::Strong::new(Box::new(Wrapper { _native }) as Box<dyn IKeystoreServiceAsync<P>>))
              } else {
                None
              }
            }
          }
          pub trait IKeystoreServiceDefault: Send + Sync {
            fn r#getSecurityLevel<'a, >(&'a self, _arg_securityLevel: crate::mangled::_7_android_8_hardware_8_security_7_keymint_13_SecurityLevel) -> binder::Result<binder::Strong<dyn crate::mangled::_7_android_6_system_9_keystore2_22_IKeystoreSecurityLevel>> {
              Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
            }
            fn r#getKeyEntry<'a, 'l1, >(&'a self, _arg_key: &'l1 crate::mangled::_7_android_6_system_9_keystore2_13_KeyDescriptor) -> binder::Result<crate::mangled::_7_android_6_system_9_keystore2_16_KeyEntryResponse> {
              Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
            }
            fn r#updateSubcomponent<'a, 'l1, 'l2, 'l3, >(&'a self, _arg_key: &'l1 crate::mangled::_7_android_6_system_9_keystore2_13_KeyDescriptor, _arg_publicCert: Option<&'l2 [u8]>, _arg_certificateChain: Option<&'l3 [u8]>) -> binder::Result<()> {
              Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
            }
            fn r#listEntries<'a, >(&'a self, _arg_domain: crate::mangled::_7_android_6_system_9_keystore2_6_Domain, _arg_nspace: i64) -> binder::Result<Vec<crate::mangled::_7_android_6_system_9_keystore2_13_KeyDescriptor>> {
              Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
            }
            fn r#deleteKey<'a, 'l1, >(&'a self, _arg_key: &'l1 crate::mangled::_7_android_6_system_9_keystore2_13_KeyDescriptor) -> binder::Result<()> {
              Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
            }
            fn r#grant<'a, 'l1, >(&'a self, _arg_key: &'l1 crate::mangled::_7_android_6_system_9_keystore2_13_KeyDescriptor, _arg_granteeUid: i32, _arg_accessVector: i32) -> binder::Result<crate::mangled::_7_android_6_system_9_keystore2_13_KeyDescriptor> {
              Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
            }
            fn r#ungrant<'a, 'l1, >(&'a self, _arg_key: &'l1 crate::mangled::_7_android_6_system_9_keystore2_13_KeyDescriptor, _arg_granteeUid: i32) -> binder::Result<()> {
              Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
            }
            fn r#getNumberOfEntries<'a, >(&'a self, _arg_domain: crate::mangled::_7_android_6_system_9_keystore2_6_Domain, _arg_nspace: i64) -> binder::Result<i32> {
              Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
            }
            fn r#listEntriesBatched<'a, 'l1, >(&'a self, _arg_domain: crate::mangled::_7_android_6_system_9_keystore2_6_Domain, _arg_nspace: i64, _arg_startingPastAlias: Option<&'l1 str>) -> binder::Result<Vec<crate::mangled::_7_android_6_system_9_keystore2_13_KeyDescriptor>> {
              Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
            }
            fn r#getSupplementaryAttestationInfo<'a, >(&'a self, _arg_tag: crate::mangled::_7_android_8_hardware_8_security_7_keymint_3_Tag) -> binder::Result<Vec<u8>> {
              Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
            }
          }
          pub mod transactions {
            pub const r#getSecurityLevel: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 0;
            pub const r#getKeyEntry: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 1;
            pub const r#updateSubcomponent: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 2;
            pub const r#listEntries: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 3;
            pub const r#deleteKey: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 4;
            pub const r#grant: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 5;
            pub const r#ungrant: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 6;
            pub const r#getNumberOfEntries: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 7;
            pub const r#listEntriesBatched: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 8;
            pub const r#getSupplementaryAttestationInfo: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 9;
            pub const r#getInterfaceVersion: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 16777214;
            pub const r#getInterfaceHash: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 16777213;
          }
          pub type IKeystoreServiceDefaultRef = Option<std::sync::Arc<dyn IKeystoreServiceDefault>>;
          static DEFAULT_IMPL: std::sync::Mutex<IKeystoreServiceDefaultRef> = std::sync::Mutex::new(None);
          pub const VERSION: i32 = 5;
          pub const HASH: &str = "98d815116c190250e9e5a1d9182cea8126fd0e97";
          impl BpKeystoreService {
            fn build_parcel_getSecurityLevel(&self, _arg_securityLevel: crate::mangled::_7_android_8_hardware_8_security_7_keymint_13_SecurityLevel) -> binder::Result<binder::binder_impl::Parcel> {
              let mut aidl_data = self.binder.prepare_transact()?;
              aidl_data.write(&_arg_securityLevel)?;
              Ok(aidl_data)
            }
            fn read_response_getSecurityLevel(&self, _arg_securityLevel: crate::mangled::_7_android_8_hardware_8_security_7_keymint_13_SecurityLevel, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<binder::Strong<dyn crate::mangled::_7_android_6_system_9_keystore2_22_IKeystoreSecurityLevel>> {
              if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                if let Some(_aidl_default_impl) = <Self as IKeystoreService>::getDefaultImpl() {
                  return _aidl_default_impl.r#getSecurityLevel(_arg_securityLevel);
                }
              }
              let _aidl_reply = _aidl_reply?;
              let _aidl_status: binder::Status = _aidl_reply.read()?;
              if !_aidl_status.is_ok() { return Err(_aidl_status); }
              let _aidl_return: binder::Strong<dyn crate::mangled::_7_android_6_system_9_keystore2_22_IKeystoreSecurityLevel> = _aidl_reply.read()?;
              Ok(_aidl_return)
            }
            fn build_parcel_getKeyEntry(&self, _arg_key: &crate::mangled::_7_android_6_system_9_keystore2_13_KeyDescriptor) -> binder::Result<binder::binder_impl::Parcel> {
              let mut aidl_data = self.binder.prepare_transact()?;
              aidl_data.write(_arg_key)?;
              Ok(aidl_data)
            }
            fn read_response_getKeyEntry(&self, _arg_key: &crate::mangled::_7_android_6_system_9_keystore2_13_KeyDescriptor, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<crate::mangled::_7_android_6_system_9_keystore2_16_KeyEntryResponse> {
              if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                if let Some(_aidl_default_impl) = <Self as IKeystoreService>::getDefaultImpl() {
                  return _aidl_default_impl.r#getKeyEntry(_arg_key);
                }
              }
              let _aidl_reply = _aidl_reply?;
              let _aidl_status: binder::Status = _aidl_reply.read()?;
              if !_aidl_status.is_ok() { return Err(_aidl_status); }
              let _aidl_return: crate::mangled::_7_android_6_system_9_keystore2_16_KeyEntryResponse = _aidl_reply.read()?;
              Ok(_aidl_return)
            }
            fn build_parcel_updateSubcomponent(&self, _arg_key: &crate::mangled::_7_android_6_system_9_keystore2_13_KeyDescriptor, _arg_publicCert: Option<&[u8]>, _arg_certificateChain: Option<&[u8]>) -> binder::Result<binder::binder_impl::Parcel> {
              let mut aidl_data = self.binder.prepare_transact()?;
              aidl_data.write(_arg_key)?;
              aidl_data.write(&_arg_publicCert)?;
              aidl_data.write(&_arg_certificateChain)?;
              Ok(aidl_data)
            }
            fn read_response_updateSubcomponent(&self, _arg_key: &crate::mangled::_7_android_6_system_9_keystore2_13_KeyDescriptor, _arg_publicCert: Option<&[u8]>, _arg_certificateChain: Option<&[u8]>, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
              if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                if let Some(_aidl_default_impl) = <Self as IKeystoreService>::getDefaultImpl() {
                  return _aidl_default_impl.r#updateSubcomponent(_arg_key, _arg_publicCert, _arg_certificateChain);
                }
              }
              let _aidl_reply = _aidl_reply?;
              let _aidl_status: binder::Status = _aidl_reply.read()?;
              if !_aidl_status.is_ok() { return Err(_aidl_status); }
              Ok(())
            }
            fn build_parcel_listEntries(&self, _arg_domain: crate::mangled::_7_android_6_system_9_keystore2_6_Domain, _arg_nspace: i64) -> binder::Result<binder::binder_impl::Parcel> {
              let mut aidl_data = self.binder.prepare_transact()?;
              aidl_data.write(&_arg_domain)?;
              aidl_data.write(&_arg_nspace)?;
              Ok(aidl_data)
            }
            fn read_response_listEntries(&self, _arg_domain: crate::mangled::_7_android_6_system_9_keystore2_6_Domain, _arg_nspace: i64, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<Vec<crate::mangled::_7_android_6_system_9_keystore2_13_KeyDescriptor>> {
              if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                if let Some(_aidl_default_impl) = <Self as IKeystoreService>::getDefaultImpl() {
                  return _aidl_default_impl.r#listEntries(_arg_domain, _arg_nspace);
                }
              }
              let _aidl_reply = _aidl_reply?;
              let _aidl_status: binder::Status = _aidl_reply.read()?;
              if !_aidl_status.is_ok() { return Err(_aidl_status); }
              let _aidl_return: Vec<crate::mangled::_7_android_6_system_9_keystore2_13_KeyDescriptor> = _aidl_reply.read()?;
              Ok(_aidl_return)
            }
            fn build_parcel_deleteKey(&self, _arg_key: &crate::mangled::_7_android_6_system_9_keystore2_13_KeyDescriptor) -> binder::Result<binder::binder_impl::Parcel> {
              let mut aidl_data = self.binder.prepare_transact()?;
              aidl_data.write(_arg_key)?;
              Ok(aidl_data)
            }
            fn read_response_deleteKey(&self, _arg_key: &crate::mangled::_7_android_6_system_9_keystore2_13_KeyDescriptor, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
              if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                if let Some(_aidl_default_impl) = <Self as IKeystoreService>::getDefaultImpl() {
                  return _aidl_default_impl.r#deleteKey(_arg_key);
                }
              }
              let _aidl_reply = _aidl_reply?;
              let _aidl_status: binder::Status = _aidl_reply.read()?;
              if !_aidl_status.is_ok() { return Err(_aidl_status); }
              Ok(())
            }
            fn build_parcel_grant(&self, _arg_key: &crate::mangled::_7_android_6_system_9_keystore2_13_KeyDescriptor, _arg_granteeUid: i32, _arg_accessVector: i32) -> binder::Result<binder::binder_impl::Parcel> {
              let mut aidl_data = self.binder.prepare_transact()?;
              aidl_data.write(_arg_key)?;
              aidl_data.write(&_arg_granteeUid)?;
              aidl_data.write(&_arg_accessVector)?;
              Ok(aidl_data)
            }
            fn read_response_grant(&self, _arg_key: &crate::mangled::_7_android_6_system_9_keystore2_13_KeyDescriptor, _arg_granteeUid: i32, _arg_accessVector: i32, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<crate::mangled::_7_android_6_system_9_keystore2_13_KeyDescriptor> {
              if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                if let Some(_aidl_default_impl) = <Self as IKeystoreService>::getDefaultImpl() {
                  return _aidl_default_impl.r#grant(_arg_key, _arg_granteeUid, _arg_accessVector);
                }
              }
              let _aidl_reply = _aidl_reply?;
              let _aidl_status: binder::Status = _aidl_reply.read()?;
              if !_aidl_status.is_ok() { return Err(_aidl_status); }
              let _aidl_return: crate::mangled::_7_android_6_system_9_keystore2_13_KeyDescriptor = _aidl_reply.read()?;
              Ok(_aidl_return)
            }
            fn build_parcel_ungrant(&self, _arg_key: &crate::mangled::_7_android_6_system_9_keystore2_13_KeyDescriptor, _arg_granteeUid: i32) -> binder::Result<binder::binder_impl::Parcel> {
              let mut aidl_data = self.binder.prepare_transact()?;
              aidl_data.write(_arg_key)?;
              aidl_data.write(&_arg_granteeUid)?;
              Ok(aidl_data)
            }
            fn read_response_ungrant(&self, _arg_key: &crate::mangled::_7_android_6_system_9_keystore2_13_KeyDescriptor, _arg_granteeUid: i32, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
              if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                if let Some(_aidl_default_impl) = <Self as IKeystoreService>::getDefaultImpl() {
                  return _aidl_default_impl.r#ungrant(_arg_key, _arg_granteeUid);
                }
              }
              let _aidl_reply = _aidl_reply?;
              let _aidl_status: binder::Status = _aidl_reply.read()?;
              if !_aidl_status.is_ok() { return Err(_aidl_status); }
              Ok(())
            }
            fn build_parcel_getNumberOfEntries(&self, _arg_domain: crate::mangled::_7_android_6_system_9_keystore2_6_Domain, _arg_nspace: i64) -> binder::Result<binder::binder_impl::Parcel> {
              let mut aidl_data = self.binder.prepare_transact()?;
              aidl_data.write(&_arg_domain)?;
              aidl_data.write(&_arg_nspace)?;
              Ok(aidl_data)
            }
            fn read_response_getNumberOfEntries(&self, _arg_domain: crate::mangled::_7_android_6_system_9_keystore2_6_Domain, _arg_nspace: i64, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<i32> {
              if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                if let Some(_aidl_default_impl) = <Self as IKeystoreService>::getDefaultImpl() {
                  return _aidl_default_impl.r#getNumberOfEntries(_arg_domain, _arg_nspace);
                }
              }
              let _aidl_reply = _aidl_reply?;
              let _aidl_status: binder::Status = _aidl_reply.read()?;
              if !_aidl_status.is_ok() { return Err(_aidl_status); }
              let _aidl_return: i32 = _aidl_reply.read()?;
              Ok(_aidl_return)
            }
            fn build_parcel_listEntriesBatched(&self, _arg_domain: crate::mangled::_7_android_6_system_9_keystore2_6_Domain, _arg_nspace: i64, _arg_startingPastAlias: Option<&str>) -> binder::Result<binder::binder_impl::Parcel> {
              let mut aidl_data = self.binder.prepare_transact()?;
              aidl_data.write(&_arg_domain)?;
              aidl_data.write(&_arg_nspace)?;
              aidl_data.write(&_arg_startingPastAlias)?;
              Ok(aidl_data)
            }
            fn read_response_listEntriesBatched(&self, _arg_domain: crate::mangled::_7_android_6_system_9_keystore2_6_Domain, _arg_nspace: i64, _arg_startingPastAlias: Option<&str>, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<Vec<crate::mangled::_7_android_6_system_9_keystore2_13_KeyDescriptor>> {
              if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                if let Some(_aidl_default_impl) = <Self as IKeystoreService>::getDefaultImpl() {
                  return _aidl_default_impl.r#listEntriesBatched(_arg_domain, _arg_nspace, _arg_startingPastAlias);
                }
              }
              let _aidl_reply = _aidl_reply?;
              let _aidl_status: binder::Status = _aidl_reply.read()?;
              if !_aidl_status.is_ok() { return Err(_aidl_status); }
              let _aidl_return: Vec<crate::mangled::_7_android_6_system_9_keystore2_13_KeyDescriptor> = _aidl_reply.read()?;
              Ok(_aidl_return)
            }
            fn build_parcel_getSupplementaryAttestationInfo(&self, _arg_tag: crate::mangled::_7_android_8_hardware_8_security_7_keymint_3_Tag) -> binder::Result<binder::binder_impl::Parcel> {
              let mut aidl_data = self.binder.prepare_transact()?;
              aidl_data.write(&_arg_tag)?;
              Ok(aidl_data)
            }
            fn read_response_getSupplementaryAttestationInfo(&self, _arg_tag: crate::mangled::_7_android_8_hardware_8_security_7_keymint_3_Tag, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<Vec<u8>> {
              if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                if let Some(_aidl_default_impl) = <Self as IKeystoreService>::getDefaultImpl() {
                  return _aidl_default_impl.r#getSupplementaryAttestationInfo(_arg_tag);
                }
              }
              let _aidl_reply = _aidl_reply?;
              let _aidl_status: binder::Status = _aidl_reply.read()?;
              if !_aidl_status.is_ok() { return Err(_aidl_status); }
              let _aidl_return: Vec<u8> = _aidl_reply.read()?;
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
          impl IKeystoreService for BpKeystoreService {
            fn r#getSecurityLevel<'a, >(&'a self, _arg_securityLevel: crate::mangled::_7_android_8_hardware_8_security_7_keymint_13_SecurityLevel) -> binder::Result<binder::Strong<dyn crate::mangled::_7_android_6_system_9_keystore2_22_IKeystoreSecurityLevel>> {
              let _aidl_data = self.build_parcel_getSecurityLevel(_arg_securityLevel)?;
              let _aidl_reply = self.binder.submit_transact(transactions::r#getSecurityLevel, _aidl_data, FLAG_PRIVATE_LOCAL);
              self.read_response_getSecurityLevel(_arg_securityLevel, _aidl_reply)
            }
            fn r#getKeyEntry<'a, 'l1, >(&'a self, _arg_key: &'l1 crate::mangled::_7_android_6_system_9_keystore2_13_KeyDescriptor) -> binder::Result<crate::mangled::_7_android_6_system_9_keystore2_16_KeyEntryResponse> {
              let _aidl_data = self.build_parcel_getKeyEntry(_arg_key)?;
              let _aidl_reply = self.binder.submit_transact(transactions::r#getKeyEntry, _aidl_data, FLAG_PRIVATE_LOCAL);
              self.read_response_getKeyEntry(_arg_key, _aidl_reply)
            }
            fn r#updateSubcomponent<'a, 'l1, 'l2, 'l3, >(&'a self, _arg_key: &'l1 crate::mangled::_7_android_6_system_9_keystore2_13_KeyDescriptor, _arg_publicCert: Option<&'l2 [u8]>, _arg_certificateChain: Option<&'l3 [u8]>) -> binder::Result<()> {
              let _aidl_data = self.build_parcel_updateSubcomponent(_arg_key, _arg_publicCert, _arg_certificateChain)?;
              let _aidl_reply = self.binder.submit_transact(transactions::r#updateSubcomponent, _aidl_data, FLAG_PRIVATE_LOCAL);
              self.read_response_updateSubcomponent(_arg_key, _arg_publicCert, _arg_certificateChain, _aidl_reply)
            }
            fn r#listEntries<'a, >(&'a self, _arg_domain: crate::mangled::_7_android_6_system_9_keystore2_6_Domain, _arg_nspace: i64) -> binder::Result<Vec<crate::mangled::_7_android_6_system_9_keystore2_13_KeyDescriptor>> {
              let _aidl_data = self.build_parcel_listEntries(_arg_domain, _arg_nspace)?;
              let _aidl_reply = self.binder.submit_transact(transactions::r#listEntries, _aidl_data, FLAG_PRIVATE_LOCAL);
              self.read_response_listEntries(_arg_domain, _arg_nspace, _aidl_reply)
            }
            fn r#deleteKey<'a, 'l1, >(&'a self, _arg_key: &'l1 crate::mangled::_7_android_6_system_9_keystore2_13_KeyDescriptor) -> binder::Result<()> {
              let _aidl_data = self.build_parcel_deleteKey(_arg_key)?;
              let _aidl_reply = self.binder.submit_transact(transactions::r#deleteKey, _aidl_data, FLAG_PRIVATE_LOCAL);
              self.read_response_deleteKey(_arg_key, _aidl_reply)
            }
            fn r#grant<'a, 'l1, >(&'a self, _arg_key: &'l1 crate::mangled::_7_android_6_system_9_keystore2_13_KeyDescriptor, _arg_granteeUid: i32, _arg_accessVector: i32) -> binder::Result<crate::mangled::_7_android_6_system_9_keystore2_13_KeyDescriptor> {
              let _aidl_data = self.build_parcel_grant(_arg_key, _arg_granteeUid, _arg_accessVector)?;
              let _aidl_reply = self.binder.submit_transact(transactions::r#grant, _aidl_data, FLAG_PRIVATE_LOCAL);
              self.read_response_grant(_arg_key, _arg_granteeUid, _arg_accessVector, _aidl_reply)
            }
            fn r#ungrant<'a, 'l1, >(&'a self, _arg_key: &'l1 crate::mangled::_7_android_6_system_9_keystore2_13_KeyDescriptor, _arg_granteeUid: i32) -> binder::Result<()> {
              let _aidl_data = self.build_parcel_ungrant(_arg_key, _arg_granteeUid)?;
              let _aidl_reply = self.binder.submit_transact(transactions::r#ungrant, _aidl_data, FLAG_PRIVATE_LOCAL);
              self.read_response_ungrant(_arg_key, _arg_granteeUid, _aidl_reply)
            }
            fn r#getNumberOfEntries<'a, >(&'a self, _arg_domain: crate::mangled::_7_android_6_system_9_keystore2_6_Domain, _arg_nspace: i64) -> binder::Result<i32> {
              let _aidl_data = self.build_parcel_getNumberOfEntries(_arg_domain, _arg_nspace)?;
              let _aidl_reply = self.binder.submit_transact(transactions::r#getNumberOfEntries, _aidl_data, FLAG_PRIVATE_LOCAL);
              self.read_response_getNumberOfEntries(_arg_domain, _arg_nspace, _aidl_reply)
            }
            fn r#listEntriesBatched<'a, 'l1, >(&'a self, _arg_domain: crate::mangled::_7_android_6_system_9_keystore2_6_Domain, _arg_nspace: i64, _arg_startingPastAlias: Option<&'l1 str>) -> binder::Result<Vec<crate::mangled::_7_android_6_system_9_keystore2_13_KeyDescriptor>> {
              let _aidl_data = self.build_parcel_listEntriesBatched(_arg_domain, _arg_nspace, _arg_startingPastAlias)?;
              let _aidl_reply = self.binder.submit_transact(transactions::r#listEntriesBatched, _aidl_data, FLAG_PRIVATE_LOCAL);
              self.read_response_listEntriesBatched(_arg_domain, _arg_nspace, _arg_startingPastAlias, _aidl_reply)
            }
            fn r#getSupplementaryAttestationInfo<'a, >(&'a self, _arg_tag: crate::mangled::_7_android_8_hardware_8_security_7_keymint_3_Tag) -> binder::Result<Vec<u8>> {
              let _aidl_data = self.build_parcel_getSupplementaryAttestationInfo(_arg_tag)?;
              let _aidl_reply = self.binder.submit_transact(transactions::r#getSupplementaryAttestationInfo, _aidl_data, FLAG_PRIVATE_LOCAL);
              self.read_response_getSupplementaryAttestationInfo(_arg_tag, _aidl_reply)
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
          impl<P: binder::BinderAsyncPool> IKeystoreServiceAsync<P> for BpKeystoreService {
            fn r#getSecurityLevel<'a, >(&'a self, _arg_securityLevel: crate::mangled::_7_android_8_hardware_8_security_7_keymint_13_SecurityLevel) -> binder::BoxFuture<'a, binder::Result<binder::Strong<dyn crate::mangled::_7_android_6_system_9_keystore2_22_IKeystoreSecurityLevel>>> {
              let _aidl_data = match self.build_parcel_getSecurityLevel(_arg_securityLevel) {
                Ok(_aidl_data) => _aidl_data,
                Err(err) => return Box::pin(std::future::ready(Err(err))),
              };
              let binder = self.binder.clone();
              P::spawn(
                move || binder.submit_transact(transactions::r#getSecurityLevel, _aidl_data, FLAG_PRIVATE_LOCAL),
                move |_aidl_reply| async move {
                  self.read_response_getSecurityLevel(_arg_securityLevel, _aidl_reply)
                }
              )
            }
            fn r#getKeyEntry<'a, >(&'a self, _arg_key: &'a crate::mangled::_7_android_6_system_9_keystore2_13_KeyDescriptor) -> binder::BoxFuture<'a, binder::Result<crate::mangled::_7_android_6_system_9_keystore2_16_KeyEntryResponse>> {
              let _aidl_data = match self.build_parcel_getKeyEntry(_arg_key) {
                Ok(_aidl_data) => _aidl_data,
                Err(err) => return Box::pin(std::future::ready(Err(err))),
              };
              let binder = self.binder.clone();
              P::spawn(
                move || binder.submit_transact(transactions::r#getKeyEntry, _aidl_data, FLAG_PRIVATE_LOCAL),
                move |_aidl_reply| async move {
                  self.read_response_getKeyEntry(_arg_key, _aidl_reply)
                }
              )
            }
            fn r#updateSubcomponent<'a, >(&'a self, _arg_key: &'a crate::mangled::_7_android_6_system_9_keystore2_13_KeyDescriptor, _arg_publicCert: Option<&'a [u8]>, _arg_certificateChain: Option<&'a [u8]>) -> binder::BoxFuture<'a, binder::Result<()>> {
              let _aidl_data = match self.build_parcel_updateSubcomponent(_arg_key, _arg_publicCert, _arg_certificateChain) {
                Ok(_aidl_data) => _aidl_data,
                Err(err) => return Box::pin(std::future::ready(Err(err))),
              };
              let binder = self.binder.clone();
              P::spawn(
                move || binder.submit_transact(transactions::r#updateSubcomponent, _aidl_data, FLAG_PRIVATE_LOCAL),
                move |_aidl_reply| async move {
                  self.read_response_updateSubcomponent(_arg_key, _arg_publicCert, _arg_certificateChain, _aidl_reply)
                }
              )
            }
            fn r#listEntries<'a, >(&'a self, _arg_domain: crate::mangled::_7_android_6_system_9_keystore2_6_Domain, _arg_nspace: i64) -> binder::BoxFuture<'a, binder::Result<Vec<crate::mangled::_7_android_6_system_9_keystore2_13_KeyDescriptor>>> {
              let _aidl_data = match self.build_parcel_listEntries(_arg_domain, _arg_nspace) {
                Ok(_aidl_data) => _aidl_data,
                Err(err) => return Box::pin(std::future::ready(Err(err))),
              };
              let binder = self.binder.clone();
              P::spawn(
                move || binder.submit_transact(transactions::r#listEntries, _aidl_data, FLAG_PRIVATE_LOCAL),
                move |_aidl_reply| async move {
                  self.read_response_listEntries(_arg_domain, _arg_nspace, _aidl_reply)
                }
              )
            }
            fn r#deleteKey<'a, >(&'a self, _arg_key: &'a crate::mangled::_7_android_6_system_9_keystore2_13_KeyDescriptor) -> binder::BoxFuture<'a, binder::Result<()>> {
              let _aidl_data = match self.build_parcel_deleteKey(_arg_key) {
                Ok(_aidl_data) => _aidl_data,
                Err(err) => return Box::pin(std::future::ready(Err(err))),
              };
              let binder = self.binder.clone();
              P::spawn(
                move || binder.submit_transact(transactions::r#deleteKey, _aidl_data, FLAG_PRIVATE_LOCAL),
                move |_aidl_reply| async move {
                  self.read_response_deleteKey(_arg_key, _aidl_reply)
                }
              )
            }
            fn r#grant<'a, >(&'a self, _arg_key: &'a crate::mangled::_7_android_6_system_9_keystore2_13_KeyDescriptor, _arg_granteeUid: i32, _arg_accessVector: i32) -> binder::BoxFuture<'a, binder::Result<crate::mangled::_7_android_6_system_9_keystore2_13_KeyDescriptor>> {
              let _aidl_data = match self.build_parcel_grant(_arg_key, _arg_granteeUid, _arg_accessVector) {
                Ok(_aidl_data) => _aidl_data,
                Err(err) => return Box::pin(std::future::ready(Err(err))),
              };
              let binder = self.binder.clone();
              P::spawn(
                move || binder.submit_transact(transactions::r#grant, _aidl_data, FLAG_PRIVATE_LOCAL),
                move |_aidl_reply| async move {
                  self.read_response_grant(_arg_key, _arg_granteeUid, _arg_accessVector, _aidl_reply)
                }
              )
            }
            fn r#ungrant<'a, >(&'a self, _arg_key: &'a crate::mangled::_7_android_6_system_9_keystore2_13_KeyDescriptor, _arg_granteeUid: i32) -> binder::BoxFuture<'a, binder::Result<()>> {
              let _aidl_data = match self.build_parcel_ungrant(_arg_key, _arg_granteeUid) {
                Ok(_aidl_data) => _aidl_data,
                Err(err) => return Box::pin(std::future::ready(Err(err))),
              };
              let binder = self.binder.clone();
              P::spawn(
                move || binder.submit_transact(transactions::r#ungrant, _aidl_data, FLAG_PRIVATE_LOCAL),
                move |_aidl_reply| async move {
                  self.read_response_ungrant(_arg_key, _arg_granteeUid, _aidl_reply)
                }
              )
            }
            fn r#getNumberOfEntries<'a, >(&'a self, _arg_domain: crate::mangled::_7_android_6_system_9_keystore2_6_Domain, _arg_nspace: i64) -> binder::BoxFuture<'a, binder::Result<i32>> {
              let _aidl_data = match self.build_parcel_getNumberOfEntries(_arg_domain, _arg_nspace) {
                Ok(_aidl_data) => _aidl_data,
                Err(err) => return Box::pin(std::future::ready(Err(err))),
              };
              let binder = self.binder.clone();
              P::spawn(
                move || binder.submit_transact(transactions::r#getNumberOfEntries, _aidl_data, FLAG_PRIVATE_LOCAL),
                move |_aidl_reply| async move {
                  self.read_response_getNumberOfEntries(_arg_domain, _arg_nspace, _aidl_reply)
                }
              )
            }
            fn r#listEntriesBatched<'a, >(&'a self, _arg_domain: crate::mangled::_7_android_6_system_9_keystore2_6_Domain, _arg_nspace: i64, _arg_startingPastAlias: Option<&'a str>) -> binder::BoxFuture<'a, binder::Result<Vec<crate::mangled::_7_android_6_system_9_keystore2_13_KeyDescriptor>>> {
              let _aidl_data = match self.build_parcel_listEntriesBatched(_arg_domain, _arg_nspace, _arg_startingPastAlias) {
                Ok(_aidl_data) => _aidl_data,
                Err(err) => return Box::pin(std::future::ready(Err(err))),
              };
              let binder = self.binder.clone();
              P::spawn(
                move || binder.submit_transact(transactions::r#listEntriesBatched, _aidl_data, FLAG_PRIVATE_LOCAL),
                move |_aidl_reply| async move {
                  self.read_response_listEntriesBatched(_arg_domain, _arg_nspace, _arg_startingPastAlias, _aidl_reply)
                }
              )
            }
            fn r#getSupplementaryAttestationInfo<'a, >(&'a self, _arg_tag: crate::mangled::_7_android_8_hardware_8_security_7_keymint_3_Tag) -> binder::BoxFuture<'a, binder::Result<Vec<u8>>> {
              let _aidl_data = match self.build_parcel_getSupplementaryAttestationInfo(_arg_tag) {
                Ok(_aidl_data) => _aidl_data,
                Err(err) => return Box::pin(std::future::ready(Err(err))),
              };
              let binder = self.binder.clone();
              P::spawn(
                move || binder.submit_transact(transactions::r#getSupplementaryAttestationInfo, _aidl_data, FLAG_PRIVATE_LOCAL),
                move |_aidl_reply| async move {
                  self.read_response_getSupplementaryAttestationInfo(_arg_tag, _aidl_reply)
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
          impl IKeystoreService for binder::binder_impl::Binder<BnKeystoreService> {
            fn r#getSecurityLevel<'a, >(&'a self, _arg_securityLevel: crate::mangled::_7_android_8_hardware_8_security_7_keymint_13_SecurityLevel) -> binder::Result<binder::Strong<dyn crate::mangled::_7_android_6_system_9_keystore2_22_IKeystoreSecurityLevel>> { self.0.r#getSecurityLevel(_arg_securityLevel) }
            fn r#getKeyEntry<'a, 'l1, >(&'a self, _arg_key: &'l1 crate::mangled::_7_android_6_system_9_keystore2_13_KeyDescriptor) -> binder::Result<crate::mangled::_7_android_6_system_9_keystore2_16_KeyEntryResponse> { self.0.r#getKeyEntry(_arg_key) }
            fn r#updateSubcomponent<'a, 'l1, 'l2, 'l3, >(&'a self, _arg_key: &'l1 crate::mangled::_7_android_6_system_9_keystore2_13_KeyDescriptor, _arg_publicCert: Option<&'l2 [u8]>, _arg_certificateChain: Option<&'l3 [u8]>) -> binder::Result<()> { self.0.r#updateSubcomponent(_arg_key, _arg_publicCert, _arg_certificateChain) }
            fn r#listEntries<'a, >(&'a self, _arg_domain: crate::mangled::_7_android_6_system_9_keystore2_6_Domain, _arg_nspace: i64) -> binder::Result<Vec<crate::mangled::_7_android_6_system_9_keystore2_13_KeyDescriptor>> { self.0.r#listEntries(_arg_domain, _arg_nspace) }
            fn r#deleteKey<'a, 'l1, >(&'a self, _arg_key: &'l1 crate::mangled::_7_android_6_system_9_keystore2_13_KeyDescriptor) -> binder::Result<()> { self.0.r#deleteKey(_arg_key) }
            fn r#grant<'a, 'l1, >(&'a self, _arg_key: &'l1 crate::mangled::_7_android_6_system_9_keystore2_13_KeyDescriptor, _arg_granteeUid: i32, _arg_accessVector: i32) -> binder::Result<crate::mangled::_7_android_6_system_9_keystore2_13_KeyDescriptor> { self.0.r#grant(_arg_key, _arg_granteeUid, _arg_accessVector) }
            fn r#ungrant<'a, 'l1, >(&'a self, _arg_key: &'l1 crate::mangled::_7_android_6_system_9_keystore2_13_KeyDescriptor, _arg_granteeUid: i32) -> binder::Result<()> { self.0.r#ungrant(_arg_key, _arg_granteeUid) }
            fn r#getNumberOfEntries<'a, >(&'a self, _arg_domain: crate::mangled::_7_android_6_system_9_keystore2_6_Domain, _arg_nspace: i64) -> binder::Result<i32> { self.0.r#getNumberOfEntries(_arg_domain, _arg_nspace) }
            fn r#listEntriesBatched<'a, 'l1, >(&'a self, _arg_domain: crate::mangled::_7_android_6_system_9_keystore2_6_Domain, _arg_nspace: i64, _arg_startingPastAlias: Option<&'l1 str>) -> binder::Result<Vec<crate::mangled::_7_android_6_system_9_keystore2_13_KeyDescriptor>> { self.0.r#listEntriesBatched(_arg_domain, _arg_nspace, _arg_startingPastAlias) }
            fn r#getSupplementaryAttestationInfo<'a, >(&'a self, _arg_tag: crate::mangled::_7_android_8_hardware_8_security_7_keymint_3_Tag) -> binder::Result<Vec<u8>> { self.0.r#getSupplementaryAttestationInfo(_arg_tag) }
            fn r#getInterfaceVersion<'a, >(&'a self) -> binder::Result<i32> { self.0.r#getInterfaceVersion() }
            fn r#getInterfaceHash<'a, >(&'a self) -> binder::Result<String> { self.0.r#getInterfaceHash() }
          }
          fn on_transact(_aidl_service: &dyn IKeystoreService, _aidl_code: binder::binder_impl::TransactionCode, _aidl_data: &binder::binder_impl::BorrowedParcel<'_>, _aidl_reply: &mut binder::binder_impl::BorrowedParcel<'_>) -> std::result::Result<(), binder::StatusCode> {
            match _aidl_code {
              transactions::r#getSecurityLevel => {
                let _arg_securityLevel: crate::mangled::_7_android_8_hardware_8_security_7_keymint_13_SecurityLevel = _aidl_data.read()?;
                let _aidl_return = _aidl_service.r#getSecurityLevel(_arg_securityLevel);
                match &_aidl_return {
                  Ok(_aidl_return) => {
                    _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
                    _aidl_reply.write(_aidl_return)?;
                  }
                  Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
                }
                Ok(())
              }
              transactions::r#getKeyEntry => {
                let _arg_key: crate::mangled::_7_android_6_system_9_keystore2_13_KeyDescriptor = _aidl_data.read()?;
                let _aidl_return = _aidl_service.r#getKeyEntry(&_arg_key);
                match &_aidl_return {
                  Ok(_aidl_return) => {
                    _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
                    _aidl_reply.write(_aidl_return)?;
                  }
                  Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
                }
                Ok(())
              }
              transactions::r#updateSubcomponent => {
                let _arg_key: crate::mangled::_7_android_6_system_9_keystore2_13_KeyDescriptor = _aidl_data.read()?;
                let _arg_publicCert: Option<Vec<u8>> = _aidl_data.read()?;
                let _arg_certificateChain: Option<Vec<u8>> = _aidl_data.read()?;
                let _aidl_return = _aidl_service.r#updateSubcomponent(&_arg_key, _arg_publicCert.as_deref(), _arg_certificateChain.as_deref());
                match &_aidl_return {
                  Ok(_aidl_return) => {
                    _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
                  }
                  Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
                }
                Ok(())
              }
              transactions::r#listEntries => {
                let _arg_domain: crate::mangled::_7_android_6_system_9_keystore2_6_Domain = _aidl_data.read()?;
                let _arg_nspace: i64 = _aidl_data.read()?;
                let _aidl_return = _aidl_service.r#listEntries(_arg_domain, _arg_nspace);
                match &_aidl_return {
                  Ok(_aidl_return) => {
                    _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
                    _aidl_reply.write(_aidl_return)?;
                  }
                  Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
                }
                Ok(())
              }
              transactions::r#deleteKey => {
                let _arg_key: crate::mangled::_7_android_6_system_9_keystore2_13_KeyDescriptor = _aidl_data.read()?;
                let _aidl_return = _aidl_service.r#deleteKey(&_arg_key);
                match &_aidl_return {
                  Ok(_aidl_return) => {
                    _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
                  }
                  Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
                }
                Ok(())
              }
              transactions::r#grant => {
                let _arg_key: crate::mangled::_7_android_6_system_9_keystore2_13_KeyDescriptor = _aidl_data.read()?;
                let _arg_granteeUid: i32 = _aidl_data.read()?;
                let _arg_accessVector: i32 = _aidl_data.read()?;
                let _aidl_return = _aidl_service.r#grant(&_arg_key, _arg_granteeUid, _arg_accessVector);
                match &_aidl_return {
                  Ok(_aidl_return) => {
                    _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
                    _aidl_reply.write(_aidl_return)?;
                  }
                  Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
                }
                Ok(())
              }
              transactions::r#ungrant => {
                let _arg_key: crate::mangled::_7_android_6_system_9_keystore2_13_KeyDescriptor = _aidl_data.read()?;
                let _arg_granteeUid: i32 = _aidl_data.read()?;
                let _aidl_return = _aidl_service.r#ungrant(&_arg_key, _arg_granteeUid);
                match &_aidl_return {
                  Ok(_aidl_return) => {
                    _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
                  }
                  Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
                }
                Ok(())
              }
              transactions::r#getNumberOfEntries => {
                let _arg_domain: crate::mangled::_7_android_6_system_9_keystore2_6_Domain = _aidl_data.read()?;
                let _arg_nspace: i64 = _aidl_data.read()?;
                let _aidl_return = _aidl_service.r#getNumberOfEntries(_arg_domain, _arg_nspace);
                match &_aidl_return {
                  Ok(_aidl_return) => {
                    _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
                    _aidl_reply.write(_aidl_return)?;
                  }
                  Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
                }
                Ok(())
              }
              transactions::r#listEntriesBatched => {
                let _arg_domain: crate::mangled::_7_android_6_system_9_keystore2_6_Domain = _aidl_data.read()?;
                let _arg_nspace: i64 = _aidl_data.read()?;
                let _arg_startingPastAlias: Option<String> = _aidl_data.read()?;
                let _aidl_return = _aidl_service.r#listEntriesBatched(_arg_domain, _arg_nspace, _arg_startingPastAlias.as_deref());
                match &_aidl_return {
                  Ok(_aidl_return) => {
                    _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
                    _aidl_reply.write(_aidl_return)?;
                  }
                  Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
                }
                Ok(())
              }
              transactions::r#getSupplementaryAttestationInfo => {
                let _arg_tag: crate::mangled::_7_android_8_hardware_8_security_7_keymint_3_Tag = _aidl_data.read()?;
                let _aidl_return = _aidl_service.r#getSupplementaryAttestationInfo(_arg_tag);
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
           pub use super::r#IKeystoreService as _7_android_6_system_9_keystore2_16_IKeystoreService;
          }
        }
        pub mod KeyDescriptor {
          /*
           * This file is auto-generated.  DO NOT MODIFY.
           * Using: out/host/linux-x86/bin/aidl --lang=rust --structured --version 5 --hash 98d815116c190250e9e5a1d9182cea8126fd0e97 -t --stability vintf --min_sdk_version current -pout/soong/.intermediates/hardware/interfaces/security/keymint/aidl/android.hardware.security.keymint_interface/4/preprocessed.aidl --ninja -d out/soong/.intermediates/system/hardware/interfaces/keystore2/aidl/android.system.keystore2-V5-rust-source/gen/android/system/keystore2/KeyDescriptor.rs.d -o out/soong/.intermediates/system/hardware/interfaces/keystore2/aidl/android.system.keystore2-V5-rust-source/gen -Nsystem/hardware/interfaces/keystore2/aidl/aidl_api/android.system.keystore2/5 system/hardware/interfaces/keystore2/aidl/aidl_api/android.system.keystore2/5/android/system/keystore2/KeyDescriptor.aidl
           *
           * DO NOT CHECK THIS FILE INTO A CODE TREE (e.g. git, etc..).
           * ALWAYS GENERATE THIS FILE FROM UPDATED AIDL COMPILER
           * AS A BUILD INTERMEDIATE ONLY. THIS IS NOT SOURCE CODE.
           */
          #![forbid(unsafe_code)]
          #![cfg_attr(rustfmt, rustfmt_skip)]
          #[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd)]
          pub struct r#KeyDescriptor {
            pub r#domain: crate::mangled::_7_android_6_system_9_keystore2_6_Domain,
            pub r#nspace: i64,
            pub r#alias: Option<String>,
            pub r#blob: Option<Vec<u8>>,
          }
          impl Default for r#KeyDescriptor {
            fn default() -> Self {
              Self {
                r#domain: crate::mangled::_7_android_6_system_9_keystore2_6_Domain::APP,
                r#nspace: 0,
                r#alias: Default::default(),
                r#blob: Default::default(),
              }
            }
          }
          impl binder::Parcelable for r#KeyDescriptor {
            fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
              parcel.sized_write(|subparcel| {
                subparcel.write(&self.r#domain)?;
                subparcel.write(&self.r#nspace)?;
                subparcel.write(&self.r#alias)?;
                subparcel.write(&self.r#blob)?;
                Ok(())
              })
            }
            fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
              parcel.sized_read(|subparcel| {
                if subparcel.has_more_data() {
                  self.r#domain = subparcel.read()?;
                }
                if subparcel.has_more_data() {
                  self.r#nspace = subparcel.read()?;
                }
                if subparcel.has_more_data() {
                  self.r#alias = subparcel.read()?;
                }
                if subparcel.has_more_data() {
                  self.r#blob = subparcel.read()?;
                }
                Ok(())
              })
            }
          }
          binder::impl_serialize_for_parcelable!(r#KeyDescriptor);
          binder::impl_deserialize_for_parcelable!(r#KeyDescriptor);
          impl binder::binder_impl::ParcelableMetadata for r#KeyDescriptor {
            fn get_descriptor() -> &'static str { "android.system.keystore2.KeyDescriptor" }
            fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
          }
          pub(crate) mod mangled {
           pub use super::r#KeyDescriptor as _7_android_6_system_9_keystore2_13_KeyDescriptor;
          }
        }
        pub mod KeyEntryResponse {
          /*
           * This file is auto-generated.  DO NOT MODIFY.
           * Using: out/host/linux-x86/bin/aidl --lang=rust --structured --version 5 --hash 98d815116c190250e9e5a1d9182cea8126fd0e97 -t --stability vintf --min_sdk_version current -pout/soong/.intermediates/hardware/interfaces/security/keymint/aidl/android.hardware.security.keymint_interface/4/preprocessed.aidl --ninja -d out/soong/.intermediates/system/hardware/interfaces/keystore2/aidl/android.system.keystore2-V5-rust-source/gen/android/system/keystore2/KeyEntryResponse.rs.d -o out/soong/.intermediates/system/hardware/interfaces/keystore2/aidl/android.system.keystore2-V5-rust-source/gen -Nsystem/hardware/interfaces/keystore2/aidl/aidl_api/android.system.keystore2/5 system/hardware/interfaces/keystore2/aidl/aidl_api/android.system.keystore2/5/android/system/keystore2/KeyEntryResponse.aidl
           *
           * DO NOT CHECK THIS FILE INTO A CODE TREE (e.g. git, etc..).
           * ALWAYS GENERATE THIS FILE FROM UPDATED AIDL COMPILER
           * AS A BUILD INTERMEDIATE ONLY. THIS IS NOT SOURCE CODE.
           */
          #![forbid(unsafe_code)]
          #![cfg_attr(rustfmt, rustfmt_skip)]
          #[derive(Debug)]
          pub struct r#KeyEntryResponse {
            pub r#iSecurityLevel: Option<binder::Strong<dyn crate::mangled::_7_android_6_system_9_keystore2_22_IKeystoreSecurityLevel>>,
            pub r#metadata: crate::mangled::_7_android_6_system_9_keystore2_11_KeyMetadata,
          }
          impl Default for r#KeyEntryResponse {
            fn default() -> Self {
              Self {
                r#iSecurityLevel: Default::default(),
                r#metadata: Default::default(),
              }
            }
          }
          impl binder::Parcelable for r#KeyEntryResponse {
            fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
              parcel.sized_write(|subparcel| {
                subparcel.write(&self.r#iSecurityLevel)?;
                subparcel.write(&self.r#metadata)?;
                Ok(())
              })
            }
            fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
              parcel.sized_read(|subparcel| {
                if subparcel.has_more_data() {
                  self.r#iSecurityLevel = subparcel.read()?;
                }
                if subparcel.has_more_data() {
                  self.r#metadata = subparcel.read()?;
                }
                Ok(())
              })
            }
          }
          binder::impl_serialize_for_parcelable!(r#KeyEntryResponse);
          binder::impl_deserialize_for_parcelable!(r#KeyEntryResponse);
          impl binder::binder_impl::ParcelableMetadata for r#KeyEntryResponse {
            fn get_descriptor() -> &'static str { "android.system.keystore2.KeyEntryResponse" }
            fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
          }
          pub(crate) mod mangled {
           pub use super::r#KeyEntryResponse as _7_android_6_system_9_keystore2_16_KeyEntryResponse;
          }
        }
        pub mod KeyMetadata {
          /*
           * This file is auto-generated.  DO NOT MODIFY.
           * Using: out/host/linux-x86/bin/aidl --lang=rust --structured --version 5 --hash 98d815116c190250e9e5a1d9182cea8126fd0e97 -t --stability vintf --min_sdk_version current -pout/soong/.intermediates/hardware/interfaces/security/keymint/aidl/android.hardware.security.keymint_interface/4/preprocessed.aidl --ninja -d out/soong/.intermediates/system/hardware/interfaces/keystore2/aidl/android.system.keystore2-V5-rust-source/gen/android/system/keystore2/KeyMetadata.rs.d -o out/soong/.intermediates/system/hardware/interfaces/keystore2/aidl/android.system.keystore2-V5-rust-source/gen -Nsystem/hardware/interfaces/keystore2/aidl/aidl_api/android.system.keystore2/5 system/hardware/interfaces/keystore2/aidl/aidl_api/android.system.keystore2/5/android/system/keystore2/KeyMetadata.aidl
           *
           * DO NOT CHECK THIS FILE INTO A CODE TREE (e.g. git, etc..).
           * ALWAYS GENERATE THIS FILE FROM UPDATED AIDL COMPILER
           * AS A BUILD INTERMEDIATE ONLY. THIS IS NOT SOURCE CODE.
           */
          #![forbid(unsafe_code)]
          #![cfg_attr(rustfmt, rustfmt_skip)]
          #[derive(Debug)]
          pub struct r#KeyMetadata {
            pub r#key: crate::mangled::_7_android_6_system_9_keystore2_13_KeyDescriptor,
            pub r#keySecurityLevel: crate::mangled::_7_android_8_hardware_8_security_7_keymint_13_SecurityLevel,
            pub r#authorizations: Vec<crate::mangled::_7_android_6_system_9_keystore2_13_Authorization>,
            pub r#certificate: Option<Vec<u8>>,
            pub r#certificateChain: Option<Vec<u8>>,
            pub r#modificationTimeMs: i64,
          }
          impl Default for r#KeyMetadata {
            fn default() -> Self {
              Self {
                r#key: Default::default(),
                r#keySecurityLevel: crate::mangled::_7_android_8_hardware_8_security_7_keymint_13_SecurityLevel::SOFTWARE,
                r#authorizations: Default::default(),
                r#certificate: Default::default(),
                r#certificateChain: Default::default(),
                r#modificationTimeMs: 0,
              }
            }
          }
          impl binder::Parcelable for r#KeyMetadata {
            fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
              parcel.sized_write(|subparcel| {
                subparcel.write(&self.r#key)?;
                subparcel.write(&self.r#keySecurityLevel)?;
                subparcel.write(&self.r#authorizations)?;
                subparcel.write(&self.r#certificate)?;
                subparcel.write(&self.r#certificateChain)?;
                subparcel.write(&self.r#modificationTimeMs)?;
                Ok(())
              })
            }
            fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
              parcel.sized_read(|subparcel| {
                if subparcel.has_more_data() {
                  self.r#key = subparcel.read()?;
                }
                if subparcel.has_more_data() {
                  self.r#keySecurityLevel = subparcel.read()?;
                }
                if subparcel.has_more_data() {
                  self.r#authorizations = subparcel.read()?;
                }
                if subparcel.has_more_data() {
                  self.r#certificate = subparcel.read()?;
                }
                if subparcel.has_more_data() {
                  self.r#certificateChain = subparcel.read()?;
                }
                if subparcel.has_more_data() {
                  self.r#modificationTimeMs = subparcel.read()?;
                }
                Ok(())
              })
            }
          }
          binder::impl_serialize_for_parcelable!(r#KeyMetadata);
          binder::impl_deserialize_for_parcelable!(r#KeyMetadata);
          impl binder::binder_impl::ParcelableMetadata for r#KeyMetadata {
            fn get_descriptor() -> &'static str { "android.system.keystore2.KeyMetadata" }
            fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
          }
          pub(crate) mod mangled {
           pub use super::r#KeyMetadata as _7_android_6_system_9_keystore2_11_KeyMetadata;
          }
        }
        pub mod KeyParameters {
          /*
           * This file is auto-generated.  DO NOT MODIFY.
           * Using: out/host/linux-x86/bin/aidl --lang=rust --structured --version 5 --hash 98d815116c190250e9e5a1d9182cea8126fd0e97 -t --stability vintf --min_sdk_version current -pout/soong/.intermediates/hardware/interfaces/security/keymint/aidl/android.hardware.security.keymint_interface/4/preprocessed.aidl --ninja -d out/soong/.intermediates/system/hardware/interfaces/keystore2/aidl/android.system.keystore2-V5-rust-source/gen/android/system/keystore2/KeyParameters.rs.d -o out/soong/.intermediates/system/hardware/interfaces/keystore2/aidl/android.system.keystore2-V5-rust-source/gen -Nsystem/hardware/interfaces/keystore2/aidl/aidl_api/android.system.keystore2/5 system/hardware/interfaces/keystore2/aidl/aidl_api/android.system.keystore2/5/android/system/keystore2/KeyParameters.aidl
           *
           * DO NOT CHECK THIS FILE INTO A CODE TREE (e.g. git, etc..).
           * ALWAYS GENERATE THIS FILE FROM UPDATED AIDL COMPILER
           * AS A BUILD INTERMEDIATE ONLY. THIS IS NOT SOURCE CODE.
           */
          #![forbid(unsafe_code)]
          #![cfg_attr(rustfmt, rustfmt_skip)]
          #[derive(Debug)]
          pub struct r#KeyParameters {
            pub r#keyParameter: Vec<crate::mangled::_7_android_8_hardware_8_security_7_keymint_12_KeyParameter>,
          }
          impl Default for r#KeyParameters {
            fn default() -> Self {
              Self {
                r#keyParameter: Default::default(),
              }
            }
          }
          impl binder::Parcelable for r#KeyParameters {
            fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
              parcel.sized_write(|subparcel| {
                subparcel.write(&self.r#keyParameter)?;
                Ok(())
              })
            }
            fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
              parcel.sized_read(|subparcel| {
                if subparcel.has_more_data() {
                  self.r#keyParameter = subparcel.read()?;
                }
                Ok(())
              })
            }
          }
          binder::impl_serialize_for_parcelable!(r#KeyParameters);
          binder::impl_deserialize_for_parcelable!(r#KeyParameters);
          impl binder::binder_impl::ParcelableMetadata for r#KeyParameters {
            fn get_descriptor() -> &'static str { "android.system.keystore2.KeyParameters" }
            fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
          }
          pub(crate) mod mangled {
           pub use super::r#KeyParameters as _7_android_6_system_9_keystore2_13_KeyParameters;
          }
        }
        pub mod KeyPermission {
          /*
           * This file is auto-generated.  DO NOT MODIFY.
           * Using: out/host/linux-x86/bin/aidl --lang=rust --structured --version 5 --hash 98d815116c190250e9e5a1d9182cea8126fd0e97 -t --stability vintf --min_sdk_version current -pout/soong/.intermediates/hardware/interfaces/security/keymint/aidl/android.hardware.security.keymint_interface/4/preprocessed.aidl --ninja -d out/soong/.intermediates/system/hardware/interfaces/keystore2/aidl/android.system.keystore2-V5-rust-source/gen/android/system/keystore2/KeyPermission.rs.d -o out/soong/.intermediates/system/hardware/interfaces/keystore2/aidl/android.system.keystore2-V5-rust-source/gen -Nsystem/hardware/interfaces/keystore2/aidl/aidl_api/android.system.keystore2/5 system/hardware/interfaces/keystore2/aidl/aidl_api/android.system.keystore2/5/android/system/keystore2/KeyPermission.aidl
           *
           * DO NOT CHECK THIS FILE INTO A CODE TREE (e.g. git, etc..).
           * ALWAYS GENERATE THIS FILE FROM UPDATED AIDL COMPILER
           * AS A BUILD INTERMEDIATE ONLY. THIS IS NOT SOURCE CODE.
           */
          #![forbid(unsafe_code)]
          #![cfg_attr(rustfmt, rustfmt_skip)]
          #![allow(non_upper_case_globals)]
          use binder::declare_binder_enum;
          declare_binder_enum! {
            #[repr(C, align(4))]
            r#KeyPermission : [i32; 13] {
              r#NONE = 0,
              r#DELETE = 1,
              r#GEN_UNIQUE_ID = 2,
              r#GET_INFO = 4,
              r#GRANT = 8,
              r#MANAGE_BLOB = 16,
              r#REBIND = 32,
              r#REQ_FORCED_OP = 64,
              r#UPDATE = 128,
              r#USE = 256,
              r#USE_DEV_ID = 512,
              r#USE_NO_LSKF_BINDING = 1024,
              r#CONVERT_STORAGE_KEY_TO_EPHEMERAL = 2048,
            }
          }
          pub(crate) mod mangled {
           pub use super::r#KeyPermission as _7_android_6_system_9_keystore2_13_KeyPermission;
          }
        }
        pub mod OperationChallenge {
          /*
           * This file is auto-generated.  DO NOT MODIFY.
           * Using: out/host/linux-x86/bin/aidl --lang=rust --structured --version 5 --hash 98d815116c190250e9e5a1d9182cea8126fd0e97 -t --stability vintf --min_sdk_version current -pout/soong/.intermediates/hardware/interfaces/security/keymint/aidl/android.hardware.security.keymint_interface/4/preprocessed.aidl --ninja -d out/soong/.intermediates/system/hardware/interfaces/keystore2/aidl/android.system.keystore2-V5-rust-source/gen/android/system/keystore2/OperationChallenge.rs.d -o out/soong/.intermediates/system/hardware/interfaces/keystore2/aidl/android.system.keystore2-V5-rust-source/gen -Nsystem/hardware/interfaces/keystore2/aidl/aidl_api/android.system.keystore2/5 system/hardware/interfaces/keystore2/aidl/aidl_api/android.system.keystore2/5/android/system/keystore2/OperationChallenge.aidl
           *
           * DO NOT CHECK THIS FILE INTO A CODE TREE (e.g. git, etc..).
           * ALWAYS GENERATE THIS FILE FROM UPDATED AIDL COMPILER
           * AS A BUILD INTERMEDIATE ONLY. THIS IS NOT SOURCE CODE.
           */
          #![forbid(unsafe_code)]
          #![cfg_attr(rustfmt, rustfmt_skip)]
          #[derive(Debug)]
          pub struct r#OperationChallenge {
            pub r#challenge: i64,
          }
          impl Default for r#OperationChallenge {
            fn default() -> Self {
              Self {
                r#challenge: 0,
              }
            }
          }
          impl binder::Parcelable for r#OperationChallenge {
            fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
              parcel.sized_write(|subparcel| {
                subparcel.write(&self.r#challenge)?;
                Ok(())
              })
            }
            fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
              parcel.sized_read(|subparcel| {
                if subparcel.has_more_data() {
                  self.r#challenge = subparcel.read()?;
                }
                Ok(())
              })
            }
          }
          binder::impl_serialize_for_parcelable!(r#OperationChallenge);
          binder::impl_deserialize_for_parcelable!(r#OperationChallenge);
          impl binder::binder_impl::ParcelableMetadata for r#OperationChallenge {
            fn get_descriptor() -> &'static str { "android.system.keystore2.OperationChallenge" }
            fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
          }
          pub(crate) mod mangled {
           pub use super::r#OperationChallenge as _7_android_6_system_9_keystore2_18_OperationChallenge;
          }
        }
        pub mod ResponseCode {
          /*
           * This file is auto-generated.  DO NOT MODIFY.
           * Using: out/host/linux-x86/bin/aidl --lang=rust --structured --version 5 --hash 98d815116c190250e9e5a1d9182cea8126fd0e97 -t --stability vintf --min_sdk_version current -pout/soong/.intermediates/hardware/interfaces/security/keymint/aidl/android.hardware.security.keymint_interface/4/preprocessed.aidl --ninja -d out/soong/.intermediates/system/hardware/interfaces/keystore2/aidl/android.system.keystore2-V5-rust-source/gen/android/system/keystore2/ResponseCode.rs.d -o out/soong/.intermediates/system/hardware/interfaces/keystore2/aidl/android.system.keystore2-V5-rust-source/gen -Nsystem/hardware/interfaces/keystore2/aidl/aidl_api/android.system.keystore2/5 system/hardware/interfaces/keystore2/aidl/aidl_api/android.system.keystore2/5/android/system/keystore2/ResponseCode.aidl
           *
           * DO NOT CHECK THIS FILE INTO A CODE TREE (e.g. git, etc..).
           * ALWAYS GENERATE THIS FILE FROM UPDATED AIDL COMPILER
           * AS A BUILD INTERMEDIATE ONLY. THIS IS NOT SOURCE CODE.
           */
          #![forbid(unsafe_code)]
          #![cfg_attr(rustfmt, rustfmt_skip)]
          #![allow(non_upper_case_globals)]
          use binder::declare_binder_enum;
          declare_binder_enum! {
            #[repr(C, align(4))]
            r#ResponseCode : [i32; 18] {
              r#LOCKED = 2,
              r#UNINITIALIZED = 3,
              r#SYSTEM_ERROR = 4,
              r#PERMISSION_DENIED = 6,
              r#KEY_NOT_FOUND = 7,
              r#VALUE_CORRUPTED = 8,
              r#KEY_PERMANENTLY_INVALIDATED = 17,
              r#BACKEND_BUSY = 18,
              r#OPERATION_BUSY = 19,
              r#INVALID_ARGUMENT = 20,
              r#TOO_MUCH_DATA = 21,
              #[deprecated = "replaced by other OUT_OF_KEYS_* errors below"]
              r#OUT_OF_KEYS = 22,
              r#OUT_OF_KEYS_REQUIRES_SYSTEM_UPGRADE = 23,
              r#OUT_OF_KEYS_PENDING_INTERNET_CONNECTIVITY = 24,
              r#OUT_OF_KEYS_TRANSIENT_ERROR = 25,
              r#OUT_OF_KEYS_PERMANENT_ERROR = 26,
              r#GET_ATTESTATION_APPLICATION_ID_FAILED = 27,
              r#INFO_NOT_AVAILABLE = 28,
            }
          }
          pub(crate) mod mangled {
           pub use super::r#ResponseCode as _7_android_6_system_9_keystore2_12_ResponseCode;
          }
        }
      }
    }
  }
}
pub mod mangled {
  pub use super::aidl::android::system::keystore2::AuthenticatorSpec::mangled::*;
  pub use super::aidl::android::system::keystore2::Authorization::mangled::*;
  pub use super::aidl::android::system::keystore2::CreateOperationResponse::mangled::*;
  pub use super::aidl::android::system::keystore2::Domain::mangled::*;
  pub use super::aidl::android::system::keystore2::EphemeralStorageKeyResponse::mangled::*;
  pub use super::aidl::android::system::keystore2::IKeystoreOperation::mangled::*;
  pub use super::aidl::android::system::keystore2::IKeystoreSecurityLevel::mangled::*;
  pub use super::aidl::android::system::keystore2::IKeystoreService::mangled::*;
  pub use super::aidl::android::system::keystore2::KeyDescriptor::mangled::*;
  pub use super::aidl::android::system::keystore2::KeyEntryResponse::mangled::*;
  pub use super::aidl::android::system::keystore2::KeyMetadata::mangled::*;
  pub use super::aidl::android::system::keystore2::KeyParameters::mangled::*;
  pub use super::aidl::android::system::keystore2::KeyPermission::mangled::*;
  pub use super::aidl::android::system::keystore2::OperationChallenge::mangled::*;
  pub use super::aidl::android::system::keystore2::ResponseCode::mangled::*;
  pub(crate) use crate::android_hardware_security_keymint::mangled::*;
}

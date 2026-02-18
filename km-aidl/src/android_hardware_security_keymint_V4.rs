#![allow(non_snake_case)]
#![allow(missing_docs)]
#[deprecated(note = "Please access via libbinder_rs binder::")]
pub use binder;
pub mod aidl {
  pub mod android {
    pub mod hardware {
      pub mod security {
        pub mod keymint {
          pub mod Algorithm {
            /*
             * This file is auto-generated.  DO NOT MODIFY.
             * Using: out/host/linux-x86/bin/aidl --lang=rust --structured --version 4 --hash a05c8079586139db45b0762a528cdd9745ad15ce -t --stability vintf --min_sdk_version 35 -pout/soong/.intermediates/hardware/interfaces/security/secureclock/aidl/android.hardware.security.secureclock_interface/1/preprocessed.aidl --ninja -d out/soong/.intermediates/hardware/interfaces/security/keymint/aidl/android.hardware.security.keymint-V4-rust-source/gen/android/hardware/security/keymint/Algorithm.rs.d -o out/soong/.intermediates/hardware/interfaces/security/keymint/aidl/android.hardware.security.keymint-V4-rust-source/gen -Nhardware/interfaces/security/keymint/aidl/aidl_api/android.hardware.security.keymint/4 hardware/interfaces/security/keymint/aidl/aidl_api/android.hardware.security.keymint/4/android/hardware/security/keymint/Algorithm.aidl
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
              r#Algorithm : [i32; 5] {
                r#RSA = 1,
                r#EC = 3,
                r#AES = 32,
                r#TRIPLE_DES = 33,
                r#HMAC = 128,
              }
            }
            pub(crate) mod mangled {
             pub use super::r#Algorithm as _7_android_8_hardware_8_security_7_keymint_9_Algorithm;
            }
          }
          pub mod AttestationKey {
            /*
             * This file is auto-generated.  DO NOT MODIFY.
             * Using: out/host/linux-x86/bin/aidl --lang=rust --structured --version 4 --hash a05c8079586139db45b0762a528cdd9745ad15ce -t --stability vintf --min_sdk_version 35 -pout/soong/.intermediates/hardware/interfaces/security/secureclock/aidl/android.hardware.security.secureclock_interface/1/preprocessed.aidl --ninja -d out/soong/.intermediates/hardware/interfaces/security/keymint/aidl/android.hardware.security.keymint-V4-rust-source/gen/android/hardware/security/keymint/AttestationKey.rs.d -o out/soong/.intermediates/hardware/interfaces/security/keymint/aidl/android.hardware.security.keymint-V4-rust-source/gen -Nhardware/interfaces/security/keymint/aidl/aidl_api/android.hardware.security.keymint/4 hardware/interfaces/security/keymint/aidl/aidl_api/android.hardware.security.keymint/4/android/hardware/security/keymint/AttestationKey.aidl
             *
             * DO NOT CHECK THIS FILE INTO A CODE TREE (e.g. git, etc..).
             * ALWAYS GENERATE THIS FILE FROM UPDATED AIDL COMPILER
             * AS A BUILD INTERMEDIATE ONLY. THIS IS NOT SOURCE CODE.
             */
            #![forbid(unsafe_code)]
            #![cfg_attr(rustfmt, rustfmt_skip)]
            #[derive(Debug, Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
            pub struct r#AttestationKey {
              pub r#keyBlob: Vec<u8>,
              pub r#attestKeyParams: Vec<crate::mangled::_7_android_8_hardware_8_security_7_keymint_12_KeyParameter>,
              pub r#issuerSubjectName: Vec<u8>,
            }
            impl Default for r#AttestationKey {
              fn default() -> Self {
                Self {
                  r#keyBlob: Default::default(),
                  r#attestKeyParams: Default::default(),
                  r#issuerSubjectName: Default::default(),
                }
              }
            }
            impl binder::Parcelable for r#AttestationKey {
              fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_write(|subparcel| {
                  subparcel.write(&self.r#keyBlob)?;
                  subparcel.write(&self.r#attestKeyParams)?;
                  subparcel.write(&self.r#issuerSubjectName)?;
                  Ok(())
                })
              }
              fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_read(|subparcel| {
                  if subparcel.has_more_data() {
                    self.r#keyBlob = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#attestKeyParams = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#issuerSubjectName = subparcel.read()?;
                  }
                  Ok(())
                })
              }
            }
            binder::impl_serialize_for_parcelable!(r#AttestationKey);
            binder::impl_deserialize_for_parcelable!(r#AttestationKey);
            impl binder::binder_impl::ParcelableMetadata for r#AttestationKey {
              fn get_descriptor() -> &'static str { "android.hardware.security.keymint.AttestationKey" }
              fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
            }
            pub(crate) mod mangled {
             pub use super::r#AttestationKey as _7_android_8_hardware_8_security_7_keymint_14_AttestationKey;
            }
          }
          pub mod BeginResult {
            /*
             * This file is auto-generated.  DO NOT MODIFY.
             * Using: out/host/linux-x86/bin/aidl --lang=rust --structured --version 4 --hash a05c8079586139db45b0762a528cdd9745ad15ce -t --stability vintf --min_sdk_version 35 -pout/soong/.intermediates/hardware/interfaces/security/secureclock/aidl/android.hardware.security.secureclock_interface/1/preprocessed.aidl --ninja -d out/soong/.intermediates/hardware/interfaces/security/keymint/aidl/android.hardware.security.keymint-V4-rust-source/gen/android/hardware/security/keymint/BeginResult.rs.d -o out/soong/.intermediates/hardware/interfaces/security/keymint/aidl/android.hardware.security.keymint-V4-rust-source/gen -Nhardware/interfaces/security/keymint/aidl/aidl_api/android.hardware.security.keymint/4 hardware/interfaces/security/keymint/aidl/aidl_api/android.hardware.security.keymint/4/android/hardware/security/keymint/BeginResult.aidl
             *
             * DO NOT CHECK THIS FILE INTO A CODE TREE (e.g. git, etc..).
             * ALWAYS GENERATE THIS FILE FROM UPDATED AIDL COMPILER
             * AS A BUILD INTERMEDIATE ONLY. THIS IS NOT SOURCE CODE.
             */
            #![forbid(unsafe_code)]
            #![cfg_attr(rustfmt, rustfmt_skip)]
            #[derive(Debug)]
            pub struct r#BeginResult {
              pub r#challenge: i64,
              pub r#params: Vec<crate::mangled::_7_android_8_hardware_8_security_7_keymint_12_KeyParameter>,
              pub r#operation: Option<binder::Strong<dyn crate::mangled::_7_android_8_hardware_8_security_7_keymint_17_IKeyMintOperation>>,
            }
            impl Default for r#BeginResult {
              fn default() -> Self {
                Self {
                  r#challenge: 0,
                  r#params: Default::default(),
                  r#operation: Default::default(),
                }
              }
            }
            impl binder::Parcelable for r#BeginResult {
              fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_write(|subparcel| {
                  subparcel.write(&self.r#challenge)?;
                  subparcel.write(&self.r#params)?;
                  let __field_ref = self.r#operation.as_ref().ok_or(binder::StatusCode::UNEXPECTED_NULL)?;
                  subparcel.write(__field_ref)?;
                  Ok(())
                })
              }
              fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_read(|subparcel| {
                  if subparcel.has_more_data() {
                    self.r#challenge = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#params = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#operation = Some(subparcel.read()?);
                  }
                  Ok(())
                })
              }
            }
            binder::impl_serialize_for_parcelable!(r#BeginResult);
            binder::impl_deserialize_for_parcelable!(r#BeginResult);
            impl binder::binder_impl::ParcelableMetadata for r#BeginResult {
              fn get_descriptor() -> &'static str { "android.hardware.security.keymint.BeginResult" }
              fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
            }
            pub(crate) mod mangled {
             pub use super::r#BeginResult as _7_android_8_hardware_8_security_7_keymint_11_BeginResult;
            }
          }
          pub mod BlockMode {
            /*
             * This file is auto-generated.  DO NOT MODIFY.
             * Using: out/host/linux-x86/bin/aidl --lang=rust --structured --version 4 --hash a05c8079586139db45b0762a528cdd9745ad15ce -t --stability vintf --min_sdk_version 35 -pout/soong/.intermediates/hardware/interfaces/security/secureclock/aidl/android.hardware.security.secureclock_interface/1/preprocessed.aidl --ninja -d out/soong/.intermediates/hardware/interfaces/security/keymint/aidl/android.hardware.security.keymint-V4-rust-source/gen/android/hardware/security/keymint/BlockMode.rs.d -o out/soong/.intermediates/hardware/interfaces/security/keymint/aidl/android.hardware.security.keymint-V4-rust-source/gen -Nhardware/interfaces/security/keymint/aidl/aidl_api/android.hardware.security.keymint/4 hardware/interfaces/security/keymint/aidl/aidl_api/android.hardware.security.keymint/4/android/hardware/security/keymint/BlockMode.aidl
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
              r#BlockMode : [i32; 4] {
                r#ECB = 1,
                r#CBC = 2,
                r#CTR = 3,
                r#GCM = 32,
              }
            }
            pub(crate) mod mangled {
             pub use super::r#BlockMode as _7_android_8_hardware_8_security_7_keymint_9_BlockMode;
            }
          }
          pub mod Certificate {
            /*
             * This file is auto-generated.  DO NOT MODIFY.
             * Using: out/host/linux-x86/bin/aidl --lang=rust --structured --version 4 --hash a05c8079586139db45b0762a528cdd9745ad15ce -t --stability vintf --min_sdk_version 35 -pout/soong/.intermediates/hardware/interfaces/security/secureclock/aidl/android.hardware.security.secureclock_interface/1/preprocessed.aidl --ninja -d out/soong/.intermediates/hardware/interfaces/security/keymint/aidl/android.hardware.security.keymint-V4-rust-source/gen/android/hardware/security/keymint/Certificate.rs.d -o out/soong/.intermediates/hardware/interfaces/security/keymint/aidl/android.hardware.security.keymint-V4-rust-source/gen -Nhardware/interfaces/security/keymint/aidl/aidl_api/android.hardware.security.keymint/4 hardware/interfaces/security/keymint/aidl/aidl_api/android.hardware.security.keymint/4/android/hardware/security/keymint/Certificate.aidl
             *
             * DO NOT CHECK THIS FILE INTO A CODE TREE (e.g. git, etc..).
             * ALWAYS GENERATE THIS FILE FROM UPDATED AIDL COMPILER
             * AS A BUILD INTERMEDIATE ONLY. THIS IS NOT SOURCE CODE.
             */
            #![forbid(unsafe_code)]
            #![cfg_attr(rustfmt, rustfmt_skip)]
            #[derive(Debug)]
            pub struct r#Certificate {
              pub r#encodedCertificate: Vec<u8>,
            }
            impl Default for r#Certificate {
              fn default() -> Self {
                Self {
                  r#encodedCertificate: Default::default(),
                }
              }
            }
            impl binder::Parcelable for r#Certificate {
              fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_write(|subparcel| {
                  subparcel.write(&self.r#encodedCertificate)?;
                  Ok(())
                })
              }
              fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_read(|subparcel| {
                  if subparcel.has_more_data() {
                    self.r#encodedCertificate = subparcel.read()?;
                  }
                  Ok(())
                })
              }
            }
            binder::impl_serialize_for_parcelable!(r#Certificate);
            binder::impl_deserialize_for_parcelable!(r#Certificate);
            impl binder::binder_impl::ParcelableMetadata for r#Certificate {
              fn get_descriptor() -> &'static str { "android.hardware.security.keymint.Certificate" }
              fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
            }
            pub(crate) mod mangled {
             pub use super::r#Certificate as _7_android_8_hardware_8_security_7_keymint_11_Certificate;
            }
          }
          pub mod Digest {
            /*
             * This file is auto-generated.  DO NOT MODIFY.
             * Using: out/host/linux-x86/bin/aidl --lang=rust --structured --version 4 --hash a05c8079586139db45b0762a528cdd9745ad15ce -t --stability vintf --min_sdk_version 35 -pout/soong/.intermediates/hardware/interfaces/security/secureclock/aidl/android.hardware.security.secureclock_interface/1/preprocessed.aidl --ninja -d out/soong/.intermediates/hardware/interfaces/security/keymint/aidl/android.hardware.security.keymint-V4-rust-source/gen/android/hardware/security/keymint/Digest.rs.d -o out/soong/.intermediates/hardware/interfaces/security/keymint/aidl/android.hardware.security.keymint-V4-rust-source/gen -Nhardware/interfaces/security/keymint/aidl/aidl_api/android.hardware.security.keymint/4 hardware/interfaces/security/keymint/aidl/aidl_api/android.hardware.security.keymint/4/android/hardware/security/keymint/Digest.aidl
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
              r#Digest : [i32; 7] {
                r#NONE = 0,
                r#MD5 = 1,
                r#SHA1 = 2,
                r#SHA_2_224 = 3,
                r#SHA_2_256 = 4,
                r#SHA_2_384 = 5,
                r#SHA_2_512 = 6,
              }
            }
            pub(crate) mod mangled {
             pub use super::r#Digest as _7_android_8_hardware_8_security_7_keymint_6_Digest;
            }
          }
          pub mod EcCurve {
            /*
             * This file is auto-generated.  DO NOT MODIFY.
             * Using: out/host/linux-x86/bin/aidl --lang=rust --structured --version 4 --hash a05c8079586139db45b0762a528cdd9745ad15ce -t --stability vintf --min_sdk_version 35 -pout/soong/.intermediates/hardware/interfaces/security/secureclock/aidl/android.hardware.security.secureclock_interface/1/preprocessed.aidl --ninja -d out/soong/.intermediates/hardware/interfaces/security/keymint/aidl/android.hardware.security.keymint-V4-rust-source/gen/android/hardware/security/keymint/EcCurve.rs.d -o out/soong/.intermediates/hardware/interfaces/security/keymint/aidl/android.hardware.security.keymint-V4-rust-source/gen -Nhardware/interfaces/security/keymint/aidl/aidl_api/android.hardware.security.keymint/4 hardware/interfaces/security/keymint/aidl/aidl_api/android.hardware.security.keymint/4/android/hardware/security/keymint/EcCurve.aidl
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
              r#EcCurve : [i32; 5] {
                r#P_224 = 0,
                r#P_256 = 1,
                r#P_384 = 2,
                r#P_521 = 3,
                r#CURVE_25519 = 4,
              }
            }
            pub(crate) mod mangled {
             pub use super::r#EcCurve as _7_android_8_hardware_8_security_7_keymint_7_EcCurve;
            }
          }
          pub mod ErrorCode {
            /*
             * This file is auto-generated.  DO NOT MODIFY.
             * Using: out/host/linux-x86/bin/aidl --lang=rust --structured --version 4 --hash a05c8079586139db45b0762a528cdd9745ad15ce -t --stability vintf --min_sdk_version 35 -pout/soong/.intermediates/hardware/interfaces/security/secureclock/aidl/android.hardware.security.secureclock_interface/1/preprocessed.aidl --ninja -d out/soong/.intermediates/hardware/interfaces/security/keymint/aidl/android.hardware.security.keymint-V4-rust-source/gen/android/hardware/security/keymint/ErrorCode.rs.d -o out/soong/.intermediates/hardware/interfaces/security/keymint/aidl/android.hardware.security.keymint-V4-rust-source/gen -Nhardware/interfaces/security/keymint/aidl/aidl_api/android.hardware.security.keymint/4 hardware/interfaces/security/keymint/aidl/aidl_api/android.hardware.security.keymint/4/android/hardware/security/keymint/ErrorCode.aidl
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
              r#ErrorCode : [i32; 88] {
                r#OK = 0,
                r#ROOT_OF_TRUST_ALREADY_SET = -1,
                r#UNSUPPORTED_PURPOSE = -2,
                r#INCOMPATIBLE_PURPOSE = -3,
                r#UNSUPPORTED_ALGORITHM = -4,
                r#INCOMPATIBLE_ALGORITHM = -5,
                r#UNSUPPORTED_KEY_SIZE = -6,
                r#UNSUPPORTED_BLOCK_MODE = -7,
                r#INCOMPATIBLE_BLOCK_MODE = -8,
                r#UNSUPPORTED_MAC_LENGTH = -9,
                r#UNSUPPORTED_PADDING_MODE = -10,
                r#INCOMPATIBLE_PADDING_MODE = -11,
                r#UNSUPPORTED_DIGEST = -12,
                r#INCOMPATIBLE_DIGEST = -13,
                r#INVALID_EXPIRATION_TIME = -14,
                r#INVALID_USER_ID = -15,
                r#INVALID_AUTHORIZATION_TIMEOUT = -16,
                r#UNSUPPORTED_KEY_FORMAT = -17,
                r#INCOMPATIBLE_KEY_FORMAT = -18,
                r#UNSUPPORTED_KEY_ENCRYPTION_ALGORITHM = -19,
                r#UNSUPPORTED_KEY_VERIFICATION_ALGORITHM = -20,
                r#INVALID_INPUT_LENGTH = -21,
                r#KEY_EXPORT_OPTIONS_INVALID = -22,
                r#DELEGATION_NOT_ALLOWED = -23,
                r#KEY_NOT_YET_VALID = -24,
                r#KEY_EXPIRED = -25,
                r#KEY_USER_NOT_AUTHENTICATED = -26,
                r#OUTPUT_PARAMETER_NULL = -27,
                r#INVALID_OPERATION_HANDLE = -28,
                r#INSUFFICIENT_BUFFER_SPACE = -29,
                r#VERIFICATION_FAILED = -30,
                r#TOO_MANY_OPERATIONS = -31,
                r#UNEXPECTED_NULL_POINTER = -32,
                r#INVALID_KEY_BLOB = -33,
                r#IMPORTED_KEY_NOT_ENCRYPTED = -34,
                r#IMPORTED_KEY_DECRYPTION_FAILED = -35,
                r#IMPORTED_KEY_NOT_SIGNED = -36,
                r#IMPORTED_KEY_VERIFICATION_FAILED = -37,
                r#INVALID_ARGUMENT = -38,
                r#UNSUPPORTED_TAG = -39,
                r#INVALID_TAG = -40,
                r#MEMORY_ALLOCATION_FAILED = -41,
                r#IMPORT_PARAMETER_MISMATCH = -44,
                r#SECURE_HW_ACCESS_DENIED = -45,
                r#OPERATION_CANCELLED = -46,
                r#CONCURRENT_ACCESS_CONFLICT = -47,
                r#SECURE_HW_BUSY = -48,
                r#SECURE_HW_COMMUNICATION_FAILED = -49,
                r#UNSUPPORTED_EC_FIELD = -50,
                r#MISSING_NONCE = -51,
                r#INVALID_NONCE = -52,
                r#MISSING_MAC_LENGTH = -53,
                r#KEY_RATE_LIMIT_EXCEEDED = -54,
                r#CALLER_NONCE_PROHIBITED = -55,
                r#KEY_MAX_OPS_EXCEEDED = -56,
                r#INVALID_MAC_LENGTH = -57,
                r#MISSING_MIN_MAC_LENGTH = -58,
                r#UNSUPPORTED_MIN_MAC_LENGTH = -59,
                r#UNSUPPORTED_KDF = -60,
                r#UNSUPPORTED_EC_CURVE = -61,
                r#KEY_REQUIRES_UPGRADE = -62,
                r#ATTESTATION_CHALLENGE_MISSING = -63,
                r#KEYMINT_NOT_CONFIGURED = -64,
                r#ATTESTATION_APPLICATION_ID_MISSING = -65,
                r#CANNOT_ATTEST_IDS = -66,
                r#ROLLBACK_RESISTANCE_UNAVAILABLE = -67,
                r#HARDWARE_TYPE_UNAVAILABLE = -68,
                r#PROOF_OF_PRESENCE_REQUIRED = -69,
                r#CONCURRENT_PROOF_OF_PRESENCE_REQUESTED = -70,
                r#NO_USER_CONFIRMATION = -71,
                r#DEVICE_LOCKED = -72,
                r#EARLY_BOOT_ENDED = -73,
                r#ATTESTATION_KEYS_NOT_PROVISIONED = -74,
                r#ATTESTATION_IDS_NOT_PROVISIONED = -75,
                r#INVALID_OPERATION = -76,
                r#STORAGE_KEY_UNSUPPORTED = -77,
                r#INCOMPATIBLE_MGF_DIGEST = -78,
                r#UNSUPPORTED_MGF_DIGEST = -79,
                r#MISSING_NOT_BEFORE = -80,
                r#MISSING_NOT_AFTER = -81,
                r#MISSING_ISSUER_SUBJECT = -82,
                r#INVALID_ISSUER_SUBJECT = -83,
                r#BOOT_LEVEL_EXCEEDED = -84,
                r#HARDWARE_NOT_YET_AVAILABLE = -85,
                r#MODULE_HASH_ALREADY_SET = -86,
                r#UNIMPLEMENTED = -100,
                r#VERSION_MISMATCH = -101,
                r#UNKNOWN_ERROR = -1000,
              }
            }
            pub(crate) mod mangled {
             pub use super::r#ErrorCode as _7_android_8_hardware_8_security_7_keymint_9_ErrorCode;
            }
          }
          pub mod HardwareAuthToken {
            /*
             * This file is auto-generated.  DO NOT MODIFY.
             * Using: out/host/linux-x86/bin/aidl --lang=rust --structured --version 4 --hash a05c8079586139db45b0762a528cdd9745ad15ce -t --stability vintf --min_sdk_version 35 -pout/soong/.intermediates/hardware/interfaces/security/secureclock/aidl/android.hardware.security.secureclock_interface/1/preprocessed.aidl --ninja -d out/soong/.intermediates/hardware/interfaces/security/keymint/aidl/android.hardware.security.keymint-V4-rust-source/gen/android/hardware/security/keymint/HardwareAuthToken.rs.d -o out/soong/.intermediates/hardware/interfaces/security/keymint/aidl/android.hardware.security.keymint-V4-rust-source/gen -Nhardware/interfaces/security/keymint/aidl/aidl_api/android.hardware.security.keymint/4 hardware/interfaces/security/keymint/aidl/aidl_api/android.hardware.security.keymint/4/android/hardware/security/keymint/HardwareAuthToken.aidl
             *
             * DO NOT CHECK THIS FILE INTO A CODE TREE (e.g. git, etc..).
             * ALWAYS GENERATE THIS FILE FROM UPDATED AIDL COMPILER
             * AS A BUILD INTERMEDIATE ONLY. THIS IS NOT SOURCE CODE.
             */
            #![forbid(unsafe_code)]
            #![cfg_attr(rustfmt, rustfmt_skip)]
            #[derive(Debug, Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
            pub struct r#HardwareAuthToken {
              pub r#challenge: i64,
              pub r#userId: i64,
              pub r#authenticatorId: i64,
              pub r#authenticatorType: crate::mangled::_7_android_8_hardware_8_security_7_keymint_25_HardwareAuthenticatorType,
              pub r#timestamp: crate::mangled::_7_android_8_hardware_8_security_11_secureclock_9_Timestamp,
              pub r#mac: Vec<u8>,
            }
            impl Default for r#HardwareAuthToken {
              fn default() -> Self {
                Self {
                  r#challenge: 0,
                  r#userId: 0,
                  r#authenticatorId: 0,
                  r#authenticatorType: crate::mangled::_7_android_8_hardware_8_security_7_keymint_25_HardwareAuthenticatorType::NONE,
                  r#timestamp: Default::default(),
                  r#mac: Default::default(),
                }
              }
            }
            impl binder::Parcelable for r#HardwareAuthToken {
              fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_write(|subparcel| {
                  subparcel.write(&self.r#challenge)?;
                  subparcel.write(&self.r#userId)?;
                  subparcel.write(&self.r#authenticatorId)?;
                  subparcel.write(&self.r#authenticatorType)?;
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
                    self.r#userId = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#authenticatorId = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#authenticatorType = subparcel.read()?;
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
            binder::impl_serialize_for_parcelable!(r#HardwareAuthToken);
            binder::impl_deserialize_for_parcelable!(r#HardwareAuthToken);
            impl binder::binder_impl::ParcelableMetadata for r#HardwareAuthToken {
              fn get_descriptor() -> &'static str { "android.hardware.security.keymint.HardwareAuthToken" }
              fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
            }
            pub(crate) mod mangled {
             pub use super::r#HardwareAuthToken as _7_android_8_hardware_8_security_7_keymint_17_HardwareAuthToken;
            }
          }
          pub mod HardwareAuthenticatorType {
            /*
             * This file is auto-generated.  DO NOT MODIFY.
             * Using: out/host/linux-x86/bin/aidl --lang=rust --structured --version 4 --hash a05c8079586139db45b0762a528cdd9745ad15ce -t --stability vintf --min_sdk_version 35 -pout/soong/.intermediates/hardware/interfaces/security/secureclock/aidl/android.hardware.security.secureclock_interface/1/preprocessed.aidl --ninja -d out/soong/.intermediates/hardware/interfaces/security/keymint/aidl/android.hardware.security.keymint-V4-rust-source/gen/android/hardware/security/keymint/HardwareAuthenticatorType.rs.d -o out/soong/.intermediates/hardware/interfaces/security/keymint/aidl/android.hardware.security.keymint-V4-rust-source/gen -Nhardware/interfaces/security/keymint/aidl/aidl_api/android.hardware.security.keymint/4 hardware/interfaces/security/keymint/aidl/aidl_api/android.hardware.security.keymint/4/android/hardware/security/keymint/HardwareAuthenticatorType.aidl
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
              r#HardwareAuthenticatorType : [i32; 4] {
                r#NONE = 0,
                r#PASSWORD = 1,
                r#FINGERPRINT = 2,
                r#ANY = -1,
              }
            }
            pub(crate) mod mangled {
             pub use super::r#HardwareAuthenticatorType as _7_android_8_hardware_8_security_7_keymint_25_HardwareAuthenticatorType;
            }
          }
          pub mod IKeyMintDevice {
            /*
             * This file is auto-generated.  DO NOT MODIFY.
             * Using: out/host/linux-x86/bin/aidl --lang=rust --structured --version 4 --hash a05c8079586139db45b0762a528cdd9745ad15ce -t --stability vintf --min_sdk_version 35 -pout/soong/.intermediates/hardware/interfaces/security/secureclock/aidl/android.hardware.security.secureclock_interface/1/preprocessed.aidl --ninja -d out/soong/.intermediates/hardware/interfaces/security/keymint/aidl/android.hardware.security.keymint-V4-rust-source/gen/android/hardware/security/keymint/IKeyMintDevice.rs.d -o out/soong/.intermediates/hardware/interfaces/security/keymint/aidl/android.hardware.security.keymint-V4-rust-source/gen -Nhardware/interfaces/security/keymint/aidl/aidl_api/android.hardware.security.keymint/4 hardware/interfaces/security/keymint/aidl/aidl_api/android.hardware.security.keymint/4/android/hardware/security/keymint/IKeyMintDevice.aidl
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
              IKeyMintDevice["android.hardware.security.keymint.IKeyMintDevice"] {
                native: BnKeyMintDevice(on_transact),
                proxy: BpKeyMintDevice {
                  cached_version: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(-1),
                  cached_hash: std::sync::Mutex<Option<String>> = std::sync::Mutex::new(None)
                },
                async: IKeyMintDeviceAsync(try_into_local_async),
                stability: binder::binder_impl::Stability::Vintf,
              }
            }
            pub trait IKeyMintDevice: binder::Interface + Send {
              fn get_descriptor() -> &'static str where Self: Sized { "android.hardware.security.keymint.IKeyMintDevice" }
              fn r#getHardwareInfo<'a, >(&'a self) -> binder::Result<crate::mangled::_7_android_8_hardware_8_security_7_keymint_19_KeyMintHardwareInfo>;
              fn r#addRngEntropy<'a, 'l1, >(&'a self, _arg_data: &'l1 [u8]) -> binder::Result<()>;
              fn r#generateKey<'a, 'l1, 'l2, >(&'a self, _arg_keyParams: &'l1 [crate::mangled::_7_android_8_hardware_8_security_7_keymint_12_KeyParameter], _arg_attestationKey: Option<&'l2 crate::mangled::_7_android_8_hardware_8_security_7_keymint_14_AttestationKey>) -> binder::Result<crate::mangled::_7_android_8_hardware_8_security_7_keymint_17_KeyCreationResult>;
              fn r#importKey<'a, 'l1, 'l2, 'l3, >(&'a self, _arg_keyParams: &'l1 [crate::mangled::_7_android_8_hardware_8_security_7_keymint_12_KeyParameter], _arg_keyFormat: crate::mangled::_7_android_8_hardware_8_security_7_keymint_9_KeyFormat, _arg_keyData: &'l2 [u8], _arg_attestationKey: Option<&'l3 crate::mangled::_7_android_8_hardware_8_security_7_keymint_14_AttestationKey>) -> binder::Result<crate::mangled::_7_android_8_hardware_8_security_7_keymint_17_KeyCreationResult>;
              fn r#importWrappedKey<'a, 'l1, 'l2, 'l3, 'l4, >(&'a self, _arg_wrappedKeyData: &'l1 [u8], _arg_wrappingKeyBlob: &'l2 [u8], _arg_maskingKey: &'l3 [u8], _arg_unwrappingParams: &'l4 [crate::mangled::_7_android_8_hardware_8_security_7_keymint_12_KeyParameter], _arg_passwordSid: i64, _arg_biometricSid: i64) -> binder::Result<crate::mangled::_7_android_8_hardware_8_security_7_keymint_17_KeyCreationResult>;
              fn r#upgradeKey<'a, 'l1, 'l2, >(&'a self, _arg_keyBlobToUpgrade: &'l1 [u8], _arg_upgradeParams: &'l2 [crate::mangled::_7_android_8_hardware_8_security_7_keymint_12_KeyParameter]) -> binder::Result<Vec<u8>>;
              fn r#deleteKey<'a, 'l1, >(&'a self, _arg_keyBlob: &'l1 [u8]) -> binder::Result<()>;
              fn r#deleteAllKeys<'a, >(&'a self) -> binder::Result<()>;
              fn r#destroyAttestationIds<'a, >(&'a self) -> binder::Result<()>;
              fn r#begin<'a, 'l1, 'l2, 'l3, >(&'a self, _arg_purpose: crate::mangled::_7_android_8_hardware_8_security_7_keymint_10_KeyPurpose, _arg_keyBlob: &'l1 [u8], _arg_params: &'l2 [crate::mangled::_7_android_8_hardware_8_security_7_keymint_12_KeyParameter], _arg_authToken: Option<&'l3 crate::mangled::_7_android_8_hardware_8_security_7_keymint_17_HardwareAuthToken>) -> binder::Result<crate::mangled::_7_android_8_hardware_8_security_7_keymint_11_BeginResult>;
              #[deprecated = "Method has never been used due to design limitations"]
              fn r#deviceLocked<'a, 'l1, >(&'a self, _arg_passwordOnly: bool, _arg_timestampToken: Option<&'l1 crate::mangled::_7_android_8_hardware_8_security_11_secureclock_14_TimeStampToken>) -> binder::Result<()>;
              fn r#earlyBootEnded<'a, >(&'a self) -> binder::Result<()>;
              fn r#convertStorageKeyToEphemeral<'a, 'l1, >(&'a self, _arg_storageKeyBlob: &'l1 [u8]) -> binder::Result<Vec<u8>>;
              fn r#getKeyCharacteristics<'a, 'l1, 'l2, 'l3, >(&'a self, _arg_keyBlob: &'l1 [u8], _arg_appId: &'l2 [u8], _arg_appData: &'l3 [u8]) -> binder::Result<Vec<crate::mangled::_7_android_8_hardware_8_security_7_keymint_18_KeyCharacteristics>>;
              fn r#getRootOfTrustChallenge<'a, >(&'a self) -> binder::Result<[u8; 16]>;
              fn r#getRootOfTrust<'a, 'l1, >(&'a self, _arg_challenge: &'l1 [u8; 16]) -> binder::Result<Vec<u8>>;
              fn r#sendRootOfTrust<'a, 'l1, >(&'a self, _arg_rootOfTrust: &'l1 [u8]) -> binder::Result<()>;
              fn r#setAdditionalAttestationInfo<'a, 'l1, >(&'a self, _arg_info: &'l1 [crate::mangled::_7_android_8_hardware_8_security_7_keymint_12_KeyParameter]) -> binder::Result<()>;
              fn r#getInterfaceVersion<'a, >(&'a self) -> binder::Result<i32> {
                Ok(VERSION)
              }
              fn r#getInterfaceHash<'a, >(&'a self) -> binder::Result<String> {
                Ok(HASH.into())
              }
              fn getDefaultImpl() -> IKeyMintDeviceDefaultRef where Self: Sized {
                DEFAULT_IMPL.lock().unwrap().clone()
              }
              fn setDefaultImpl(d: IKeyMintDeviceDefaultRef) -> IKeyMintDeviceDefaultRef where Self: Sized {
                std::mem::replace(&mut *DEFAULT_IMPL.lock().unwrap(), d)
              }
              fn try_as_async_server<'a>(&'a self) -> Option<&'a (dyn IKeyMintDeviceAsyncServer + Send + Sync)> {
                None
              }
            }
            pub trait IKeyMintDeviceAsync<P>: binder::Interface + Send {
              fn get_descriptor() -> &'static str where Self: Sized { "android.hardware.security.keymint.IKeyMintDevice" }
              fn r#getHardwareInfo<'a, >(&'a self) -> binder::BoxFuture<'a, binder::Result<crate::mangled::_7_android_8_hardware_8_security_7_keymint_19_KeyMintHardwareInfo>>;
              fn r#addRngEntropy<'a, >(&'a self, _arg_data: &'a [u8]) -> binder::BoxFuture<'a, binder::Result<()>>;
              fn r#generateKey<'a, >(&'a self, _arg_keyParams: &'a [crate::mangled::_7_android_8_hardware_8_security_7_keymint_12_KeyParameter], _arg_attestationKey: Option<&'a crate::mangled::_7_android_8_hardware_8_security_7_keymint_14_AttestationKey>) -> binder::BoxFuture<'a, binder::Result<crate::mangled::_7_android_8_hardware_8_security_7_keymint_17_KeyCreationResult>>;
              fn r#importKey<'a, >(&'a self, _arg_keyParams: &'a [crate::mangled::_7_android_8_hardware_8_security_7_keymint_12_KeyParameter], _arg_keyFormat: crate::mangled::_7_android_8_hardware_8_security_7_keymint_9_KeyFormat, _arg_keyData: &'a [u8], _arg_attestationKey: Option<&'a crate::mangled::_7_android_8_hardware_8_security_7_keymint_14_AttestationKey>) -> binder::BoxFuture<'a, binder::Result<crate::mangled::_7_android_8_hardware_8_security_7_keymint_17_KeyCreationResult>>;
              fn r#importWrappedKey<'a, >(&'a self, _arg_wrappedKeyData: &'a [u8], _arg_wrappingKeyBlob: &'a [u8], _arg_maskingKey: &'a [u8], _arg_unwrappingParams: &'a [crate::mangled::_7_android_8_hardware_8_security_7_keymint_12_KeyParameter], _arg_passwordSid: i64, _arg_biometricSid: i64) -> binder::BoxFuture<'a, binder::Result<crate::mangled::_7_android_8_hardware_8_security_7_keymint_17_KeyCreationResult>>;
              fn r#upgradeKey<'a, >(&'a self, _arg_keyBlobToUpgrade: &'a [u8], _arg_upgradeParams: &'a [crate::mangled::_7_android_8_hardware_8_security_7_keymint_12_KeyParameter]) -> binder::BoxFuture<'a, binder::Result<Vec<u8>>>;
              fn r#deleteKey<'a, >(&'a self, _arg_keyBlob: &'a [u8]) -> binder::BoxFuture<'a, binder::Result<()>>;
              fn r#deleteAllKeys<'a, >(&'a self) -> binder::BoxFuture<'a, binder::Result<()>>;
              fn r#destroyAttestationIds<'a, >(&'a self) -> binder::BoxFuture<'a, binder::Result<()>>;
              fn r#begin<'a, >(&'a self, _arg_purpose: crate::mangled::_7_android_8_hardware_8_security_7_keymint_10_KeyPurpose, _arg_keyBlob: &'a [u8], _arg_params: &'a [crate::mangled::_7_android_8_hardware_8_security_7_keymint_12_KeyParameter], _arg_authToken: Option<&'a crate::mangled::_7_android_8_hardware_8_security_7_keymint_17_HardwareAuthToken>) -> binder::BoxFuture<'a, binder::Result<crate::mangled::_7_android_8_hardware_8_security_7_keymint_11_BeginResult>>;
              #[deprecated = "Method has never been used due to design limitations"]
              fn r#deviceLocked<'a, >(&'a self, _arg_passwordOnly: bool, _arg_timestampToken: Option<&'a crate::mangled::_7_android_8_hardware_8_security_11_secureclock_14_TimeStampToken>) -> binder::BoxFuture<'a, binder::Result<()>>;
              fn r#earlyBootEnded<'a, >(&'a self) -> binder::BoxFuture<'a, binder::Result<()>>;
              fn r#convertStorageKeyToEphemeral<'a, >(&'a self, _arg_storageKeyBlob: &'a [u8]) -> binder::BoxFuture<'a, binder::Result<Vec<u8>>>;
              fn r#getKeyCharacteristics<'a, >(&'a self, _arg_keyBlob: &'a [u8], _arg_appId: &'a [u8], _arg_appData: &'a [u8]) -> binder::BoxFuture<'a, binder::Result<Vec<crate::mangled::_7_android_8_hardware_8_security_7_keymint_18_KeyCharacteristics>>>;
              fn r#getRootOfTrustChallenge<'a, >(&'a self) -> binder::BoxFuture<'a, binder::Result<[u8; 16]>>;
              fn r#getRootOfTrust<'a, >(&'a self, _arg_challenge: &'a [u8; 16]) -> binder::BoxFuture<'a, binder::Result<Vec<u8>>>;
              fn r#sendRootOfTrust<'a, >(&'a self, _arg_rootOfTrust: &'a [u8]) -> binder::BoxFuture<'a, binder::Result<()>>;
              fn r#setAdditionalAttestationInfo<'a, >(&'a self, _arg_info: &'a [crate::mangled::_7_android_8_hardware_8_security_7_keymint_12_KeyParameter]) -> binder::BoxFuture<'a, binder::Result<()>>;
              fn r#getInterfaceVersion<'a, >(&'a self) -> binder::BoxFuture<'a, binder::Result<i32>> {
                Box::pin(async move { Ok(VERSION) })
              }
              fn r#getInterfaceHash<'a, >(&'a self) -> binder::BoxFuture<'a, binder::Result<String>> {
                Box::pin(async move { Ok(HASH.into()) })
              }
            }
            #[::async_trait::async_trait]
            pub trait IKeyMintDeviceAsyncServer: binder::Interface + Send {
              fn get_descriptor() -> &'static str where Self: Sized { "android.hardware.security.keymint.IKeyMintDevice" }
              async fn r#getHardwareInfo<'a, >(&'a self) -> binder::Result<crate::mangled::_7_android_8_hardware_8_security_7_keymint_19_KeyMintHardwareInfo>;
              async fn r#addRngEntropy<'a, 'l1, >(&'a self, _arg_data: &'l1 [u8]) -> binder::Result<()>;
              async fn r#generateKey<'a, 'l1, 'l2, >(&'a self, _arg_keyParams: &'l1 [crate::mangled::_7_android_8_hardware_8_security_7_keymint_12_KeyParameter], _arg_attestationKey: Option<&'l2 crate::mangled::_7_android_8_hardware_8_security_7_keymint_14_AttestationKey>) -> binder::Result<crate::mangled::_7_android_8_hardware_8_security_7_keymint_17_KeyCreationResult>;
              async fn r#importKey<'a, 'l1, 'l2, 'l3, >(&'a self, _arg_keyParams: &'l1 [crate::mangled::_7_android_8_hardware_8_security_7_keymint_12_KeyParameter], _arg_keyFormat: crate::mangled::_7_android_8_hardware_8_security_7_keymint_9_KeyFormat, _arg_keyData: &'l2 [u8], _arg_attestationKey: Option<&'l3 crate::mangled::_7_android_8_hardware_8_security_7_keymint_14_AttestationKey>) -> binder::Result<crate::mangled::_7_android_8_hardware_8_security_7_keymint_17_KeyCreationResult>;
              async fn r#importWrappedKey<'a, 'l1, 'l2, 'l3, 'l4, >(&'a self, _arg_wrappedKeyData: &'l1 [u8], _arg_wrappingKeyBlob: &'l2 [u8], _arg_maskingKey: &'l3 [u8], _arg_unwrappingParams: &'l4 [crate::mangled::_7_android_8_hardware_8_security_7_keymint_12_KeyParameter], _arg_passwordSid: i64, _arg_biometricSid: i64) -> binder::Result<crate::mangled::_7_android_8_hardware_8_security_7_keymint_17_KeyCreationResult>;
              async fn r#upgradeKey<'a, 'l1, 'l2, >(&'a self, _arg_keyBlobToUpgrade: &'l1 [u8], _arg_upgradeParams: &'l2 [crate::mangled::_7_android_8_hardware_8_security_7_keymint_12_KeyParameter]) -> binder::Result<Vec<u8>>;
              async fn r#deleteKey<'a, 'l1, >(&'a self, _arg_keyBlob: &'l1 [u8]) -> binder::Result<()>;
              async fn r#deleteAllKeys<'a, >(&'a self) -> binder::Result<()>;
              async fn r#destroyAttestationIds<'a, >(&'a self) -> binder::Result<()>;
              async fn r#begin<'a, 'l1, 'l2, 'l3, >(&'a self, _arg_purpose: crate::mangled::_7_android_8_hardware_8_security_7_keymint_10_KeyPurpose, _arg_keyBlob: &'l1 [u8], _arg_params: &'l2 [crate::mangled::_7_android_8_hardware_8_security_7_keymint_12_KeyParameter], _arg_authToken: Option<&'l3 crate::mangled::_7_android_8_hardware_8_security_7_keymint_17_HardwareAuthToken>) -> binder::Result<crate::mangled::_7_android_8_hardware_8_security_7_keymint_11_BeginResult>;
              #[deprecated = "Method has never been used due to design limitations"]
              async fn r#deviceLocked<'a, 'l1, >(&'a self, _arg_passwordOnly: bool, _arg_timestampToken: Option<&'l1 crate::mangled::_7_android_8_hardware_8_security_11_secureclock_14_TimeStampToken>) -> binder::Result<()>;
              async fn r#earlyBootEnded<'a, >(&'a self) -> binder::Result<()>;
              async fn r#convertStorageKeyToEphemeral<'a, 'l1, >(&'a self, _arg_storageKeyBlob: &'l1 [u8]) -> binder::Result<Vec<u8>>;
              async fn r#getKeyCharacteristics<'a, 'l1, 'l2, 'l3, >(&'a self, _arg_keyBlob: &'l1 [u8], _arg_appId: &'l2 [u8], _arg_appData: &'l3 [u8]) -> binder::Result<Vec<crate::mangled::_7_android_8_hardware_8_security_7_keymint_18_KeyCharacteristics>>;
              async fn r#getRootOfTrustChallenge<'a, >(&'a self) -> binder::Result<[u8; 16]>;
              async fn r#getRootOfTrust<'a, 'l1, >(&'a self, _arg_challenge: &'l1 [u8; 16]) -> binder::Result<Vec<u8>>;
              async fn r#sendRootOfTrust<'a, 'l1, >(&'a self, _arg_rootOfTrust: &'l1 [u8]) -> binder::Result<()>;
              async fn r#setAdditionalAttestationInfo<'a, 'l1, >(&'a self, _arg_info: &'l1 [crate::mangled::_7_android_8_hardware_8_security_7_keymint_12_KeyParameter]) -> binder::Result<()>;
            }
            impl BnKeyMintDevice {
              /// Create a new async binder service.
              pub fn new_async_binder<T, R>(inner: T, rt: R, features: binder::BinderFeatures) -> binder::Strong<dyn IKeyMintDevice>
              where
                T: IKeyMintDeviceAsyncServer + binder::Interface + Send + Sync + 'static,
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
                impl<T, R> IKeyMintDevice for Wrapper<T, R>
                where
                  T: IKeyMintDeviceAsyncServer + Send + Sync + 'static,
                  R: binder::binder_impl::BinderAsyncRuntime + Send + Sync + 'static,
                {
                  fn r#getHardwareInfo<'a, >(&'a self) -> binder::Result<crate::mangled::_7_android_8_hardware_8_security_7_keymint_19_KeyMintHardwareInfo> {
                    self._rt.block_on(self._inner.r#getHardwareInfo())
                  }
                  fn r#addRngEntropy<'a, 'l1, >(&'a self, _arg_data: &'l1 [u8]) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#addRngEntropy(_arg_data))
                  }
                  fn r#generateKey<'a, 'l1, 'l2, >(&'a self, _arg_keyParams: &'l1 [crate::mangled::_7_android_8_hardware_8_security_7_keymint_12_KeyParameter], _arg_attestationKey: Option<&'l2 crate::mangled::_7_android_8_hardware_8_security_7_keymint_14_AttestationKey>) -> binder::Result<crate::mangled::_7_android_8_hardware_8_security_7_keymint_17_KeyCreationResult> {
                    self._rt.block_on(self._inner.r#generateKey(_arg_keyParams, _arg_attestationKey))
                  }
                  fn r#importKey<'a, 'l1, 'l2, 'l3, >(&'a self, _arg_keyParams: &'l1 [crate::mangled::_7_android_8_hardware_8_security_7_keymint_12_KeyParameter], _arg_keyFormat: crate::mangled::_7_android_8_hardware_8_security_7_keymint_9_KeyFormat, _arg_keyData: &'l2 [u8], _arg_attestationKey: Option<&'l3 crate::mangled::_7_android_8_hardware_8_security_7_keymint_14_AttestationKey>) -> binder::Result<crate::mangled::_7_android_8_hardware_8_security_7_keymint_17_KeyCreationResult> {
                    self._rt.block_on(self._inner.r#importKey(_arg_keyParams, _arg_keyFormat, _arg_keyData, _arg_attestationKey))
                  }
                  fn r#importWrappedKey<'a, 'l1, 'l2, 'l3, 'l4, >(&'a self, _arg_wrappedKeyData: &'l1 [u8], _arg_wrappingKeyBlob: &'l2 [u8], _arg_maskingKey: &'l3 [u8], _arg_unwrappingParams: &'l4 [crate::mangled::_7_android_8_hardware_8_security_7_keymint_12_KeyParameter], _arg_passwordSid: i64, _arg_biometricSid: i64) -> binder::Result<crate::mangled::_7_android_8_hardware_8_security_7_keymint_17_KeyCreationResult> {
                    self._rt.block_on(self._inner.r#importWrappedKey(_arg_wrappedKeyData, _arg_wrappingKeyBlob, _arg_maskingKey, _arg_unwrappingParams, _arg_passwordSid, _arg_biometricSid))
                  }
                  fn r#upgradeKey<'a, 'l1, 'l2, >(&'a self, _arg_keyBlobToUpgrade: &'l1 [u8], _arg_upgradeParams: &'l2 [crate::mangled::_7_android_8_hardware_8_security_7_keymint_12_KeyParameter]) -> binder::Result<Vec<u8>> {
                    self._rt.block_on(self._inner.r#upgradeKey(_arg_keyBlobToUpgrade, _arg_upgradeParams))
                  }
                  fn r#deleteKey<'a, 'l1, >(&'a self, _arg_keyBlob: &'l1 [u8]) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#deleteKey(_arg_keyBlob))
                  }
                  fn r#deleteAllKeys<'a, >(&'a self) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#deleteAllKeys())
                  }
                  fn r#destroyAttestationIds<'a, >(&'a self) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#destroyAttestationIds())
                  }
                  fn r#begin<'a, 'l1, 'l2, 'l3, >(&'a self, _arg_purpose: crate::mangled::_7_android_8_hardware_8_security_7_keymint_10_KeyPurpose, _arg_keyBlob: &'l1 [u8], _arg_params: &'l2 [crate::mangled::_7_android_8_hardware_8_security_7_keymint_12_KeyParameter], _arg_authToken: Option<&'l3 crate::mangled::_7_android_8_hardware_8_security_7_keymint_17_HardwareAuthToken>) -> binder::Result<crate::mangled::_7_android_8_hardware_8_security_7_keymint_11_BeginResult> {
                    self._rt.block_on(self._inner.r#begin(_arg_purpose, _arg_keyBlob, _arg_params, _arg_authToken))
                  }
                  fn r#deviceLocked<'a, 'l1, >(&'a self, _arg_passwordOnly: bool, _arg_timestampToken: Option<&'l1 crate::mangled::_7_android_8_hardware_8_security_11_secureclock_14_TimeStampToken>) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#deviceLocked(_arg_passwordOnly, _arg_timestampToken))
                  }
                  fn r#earlyBootEnded<'a, >(&'a self) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#earlyBootEnded())
                  }
                  fn r#convertStorageKeyToEphemeral<'a, 'l1, >(&'a self, _arg_storageKeyBlob: &'l1 [u8]) -> binder::Result<Vec<u8>> {
                    self._rt.block_on(self._inner.r#convertStorageKeyToEphemeral(_arg_storageKeyBlob))
                  }
                  fn r#getKeyCharacteristics<'a, 'l1, 'l2, 'l3, >(&'a self, _arg_keyBlob: &'l1 [u8], _arg_appId: &'l2 [u8], _arg_appData: &'l3 [u8]) -> binder::Result<Vec<crate::mangled::_7_android_8_hardware_8_security_7_keymint_18_KeyCharacteristics>> {
                    self._rt.block_on(self._inner.r#getKeyCharacteristics(_arg_keyBlob, _arg_appId, _arg_appData))
                  }
                  fn r#getRootOfTrustChallenge<'a, >(&'a self) -> binder::Result<[u8; 16]> {
                    self._rt.block_on(self._inner.r#getRootOfTrustChallenge())
                  }
                  fn r#getRootOfTrust<'a, 'l1, >(&'a self, _arg_challenge: &'l1 [u8; 16]) -> binder::Result<Vec<u8>> {
                    self._rt.block_on(self._inner.r#getRootOfTrust(_arg_challenge))
                  }
                  fn r#sendRootOfTrust<'a, 'l1, >(&'a self, _arg_rootOfTrust: &'l1 [u8]) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#sendRootOfTrust(_arg_rootOfTrust))
                  }
                  fn r#setAdditionalAttestationInfo<'a, 'l1, >(&'a self, _arg_info: &'l1 [crate::mangled::_7_android_8_hardware_8_security_7_keymint_12_KeyParameter]) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#setAdditionalAttestationInfo(_arg_info))
                  }
                  fn try_as_async_server(&self) -> Option<&(dyn IKeyMintDeviceAsyncServer + Send + Sync)> {
                    Some(&self._inner)
                  }
                }
                let wrapped = Wrapper { _inner: inner, _rt: rt };
                Self::new_binder(wrapped, features)
              }
              pub fn try_into_local_async<P: binder::BinderAsyncPool + 'static>(_native: binder::binder_impl::Binder<Self>) -> Option<binder::Strong<dyn IKeyMintDeviceAsync<P>>> {
                struct Wrapper {
                  _native: binder::binder_impl::Binder<BnKeyMintDevice>
                }
                impl binder::Interface for Wrapper {}
                impl<P: binder::BinderAsyncPool> IKeyMintDeviceAsync<P> for Wrapper {
                  fn r#getHardwareInfo<'a, >(&'a self) -> binder::BoxFuture<'a, binder::Result<crate::mangled::_7_android_8_hardware_8_security_7_keymint_19_KeyMintHardwareInfo>> {
                    Box::pin(self._native.try_as_async_server().unwrap().r#getHardwareInfo())
                  }
                  fn r#addRngEntropy<'a, >(&'a self, _arg_data: &'a [u8]) -> binder::BoxFuture<'a, binder::Result<()>> {
                    Box::pin(self._native.try_as_async_server().unwrap().r#addRngEntropy(_arg_data))
                  }
                  fn r#generateKey<'a, >(&'a self, _arg_keyParams: &'a [crate::mangled::_7_android_8_hardware_8_security_7_keymint_12_KeyParameter], _arg_attestationKey: Option<&'a crate::mangled::_7_android_8_hardware_8_security_7_keymint_14_AttestationKey>) -> binder::BoxFuture<'a, binder::Result<crate::mangled::_7_android_8_hardware_8_security_7_keymint_17_KeyCreationResult>> {
                    Box::pin(self._native.try_as_async_server().unwrap().r#generateKey(_arg_keyParams, _arg_attestationKey))
                  }
                  fn r#importKey<'a, >(&'a self, _arg_keyParams: &'a [crate::mangled::_7_android_8_hardware_8_security_7_keymint_12_KeyParameter], _arg_keyFormat: crate::mangled::_7_android_8_hardware_8_security_7_keymint_9_KeyFormat, _arg_keyData: &'a [u8], _arg_attestationKey: Option<&'a crate::mangled::_7_android_8_hardware_8_security_7_keymint_14_AttestationKey>) -> binder::BoxFuture<'a, binder::Result<crate::mangled::_7_android_8_hardware_8_security_7_keymint_17_KeyCreationResult>> {
                    Box::pin(self._native.try_as_async_server().unwrap().r#importKey(_arg_keyParams, _arg_keyFormat, _arg_keyData, _arg_attestationKey))
                  }
                  fn r#importWrappedKey<'a, >(&'a self, _arg_wrappedKeyData: &'a [u8], _arg_wrappingKeyBlob: &'a [u8], _arg_maskingKey: &'a [u8], _arg_unwrappingParams: &'a [crate::mangled::_7_android_8_hardware_8_security_7_keymint_12_KeyParameter], _arg_passwordSid: i64, _arg_biometricSid: i64) -> binder::BoxFuture<'a, binder::Result<crate::mangled::_7_android_8_hardware_8_security_7_keymint_17_KeyCreationResult>> {
                    Box::pin(self._native.try_as_async_server().unwrap().r#importWrappedKey(_arg_wrappedKeyData, _arg_wrappingKeyBlob, _arg_maskingKey, _arg_unwrappingParams, _arg_passwordSid, _arg_biometricSid))
                  }
                  fn r#upgradeKey<'a, >(&'a self, _arg_keyBlobToUpgrade: &'a [u8], _arg_upgradeParams: &'a [crate::mangled::_7_android_8_hardware_8_security_7_keymint_12_KeyParameter]) -> binder::BoxFuture<'a, binder::Result<Vec<u8>>> {
                    Box::pin(self._native.try_as_async_server().unwrap().r#upgradeKey(_arg_keyBlobToUpgrade, _arg_upgradeParams))
                  }
                  fn r#deleteKey<'a, >(&'a self, _arg_keyBlob: &'a [u8]) -> binder::BoxFuture<'a, binder::Result<()>> {
                    Box::pin(self._native.try_as_async_server().unwrap().r#deleteKey(_arg_keyBlob))
                  }
                  fn r#deleteAllKeys<'a, >(&'a self) -> binder::BoxFuture<'a, binder::Result<()>> {
                    Box::pin(self._native.try_as_async_server().unwrap().r#deleteAllKeys())
                  }
                  fn r#destroyAttestationIds<'a, >(&'a self) -> binder::BoxFuture<'a, binder::Result<()>> {
                    Box::pin(self._native.try_as_async_server().unwrap().r#destroyAttestationIds())
                  }
                  fn r#begin<'a, >(&'a self, _arg_purpose: crate::mangled::_7_android_8_hardware_8_security_7_keymint_10_KeyPurpose, _arg_keyBlob: &'a [u8], _arg_params: &'a [crate::mangled::_7_android_8_hardware_8_security_7_keymint_12_KeyParameter], _arg_authToken: Option<&'a crate::mangled::_7_android_8_hardware_8_security_7_keymint_17_HardwareAuthToken>) -> binder::BoxFuture<'a, binder::Result<crate::mangled::_7_android_8_hardware_8_security_7_keymint_11_BeginResult>> {
                    Box::pin(self._native.try_as_async_server().unwrap().r#begin(_arg_purpose, _arg_keyBlob, _arg_params, _arg_authToken))
                  }
                  fn r#deviceLocked<'a, >(&'a self, _arg_passwordOnly: bool, _arg_timestampToken: Option<&'a crate::mangled::_7_android_8_hardware_8_security_11_secureclock_14_TimeStampToken>) -> binder::BoxFuture<'a, binder::Result<()>> {
                    Box::pin(self._native.try_as_async_server().unwrap().r#deviceLocked(_arg_passwordOnly, _arg_timestampToken))
                  }
                  fn r#earlyBootEnded<'a, >(&'a self) -> binder::BoxFuture<'a, binder::Result<()>> {
                    Box::pin(self._native.try_as_async_server().unwrap().r#earlyBootEnded())
                  }
                  fn r#convertStorageKeyToEphemeral<'a, >(&'a self, _arg_storageKeyBlob: &'a [u8]) -> binder::BoxFuture<'a, binder::Result<Vec<u8>>> {
                    Box::pin(self._native.try_as_async_server().unwrap().r#convertStorageKeyToEphemeral(_arg_storageKeyBlob))
                  }
                  fn r#getKeyCharacteristics<'a, >(&'a self, _arg_keyBlob: &'a [u8], _arg_appId: &'a [u8], _arg_appData: &'a [u8]) -> binder::BoxFuture<'a, binder::Result<Vec<crate::mangled::_7_android_8_hardware_8_security_7_keymint_18_KeyCharacteristics>>> {
                    Box::pin(self._native.try_as_async_server().unwrap().r#getKeyCharacteristics(_arg_keyBlob, _arg_appId, _arg_appData))
                  }
                  fn r#getRootOfTrustChallenge<'a, >(&'a self) -> binder::BoxFuture<'a, binder::Result<[u8; 16]>> {
                    Box::pin(self._native.try_as_async_server().unwrap().r#getRootOfTrustChallenge())
                  }
                  fn r#getRootOfTrust<'a, >(&'a self, _arg_challenge: &'a [u8; 16]) -> binder::BoxFuture<'a, binder::Result<Vec<u8>>> {
                    Box::pin(self._native.try_as_async_server().unwrap().r#getRootOfTrust(_arg_challenge))
                  }
                  fn r#sendRootOfTrust<'a, >(&'a self, _arg_rootOfTrust: &'a [u8]) -> binder::BoxFuture<'a, binder::Result<()>> {
                    Box::pin(self._native.try_as_async_server().unwrap().r#sendRootOfTrust(_arg_rootOfTrust))
                  }
                  fn r#setAdditionalAttestationInfo<'a, >(&'a self, _arg_info: &'a [crate::mangled::_7_android_8_hardware_8_security_7_keymint_12_KeyParameter]) -> binder::BoxFuture<'a, binder::Result<()>> {
                    Box::pin(self._native.try_as_async_server().unwrap().r#setAdditionalAttestationInfo(_arg_info))
                  }
                }
                if _native.try_as_async_server().is_some() {
                  Some(binder::Strong::new(Box::new(Wrapper { _native }) as Box<dyn IKeyMintDeviceAsync<P>>))
                } else {
                  None
                }
              }
            }
            pub trait IKeyMintDeviceDefault: Send + Sync {
              fn r#getHardwareInfo<'a, >(&'a self) -> binder::Result<crate::mangled::_7_android_8_hardware_8_security_7_keymint_19_KeyMintHardwareInfo> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#addRngEntropy<'a, 'l1, >(&'a self, _arg_data: &'l1 [u8]) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#generateKey<'a, 'l1, 'l2, >(&'a self, _arg_keyParams: &'l1 [crate::mangled::_7_android_8_hardware_8_security_7_keymint_12_KeyParameter], _arg_attestationKey: Option<&'l2 crate::mangled::_7_android_8_hardware_8_security_7_keymint_14_AttestationKey>) -> binder::Result<crate::mangled::_7_android_8_hardware_8_security_7_keymint_17_KeyCreationResult> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#importKey<'a, 'l1, 'l2, 'l3, >(&'a self, _arg_keyParams: &'l1 [crate::mangled::_7_android_8_hardware_8_security_7_keymint_12_KeyParameter], _arg_keyFormat: crate::mangled::_7_android_8_hardware_8_security_7_keymint_9_KeyFormat, _arg_keyData: &'l2 [u8], _arg_attestationKey: Option<&'l3 crate::mangled::_7_android_8_hardware_8_security_7_keymint_14_AttestationKey>) -> binder::Result<crate::mangled::_7_android_8_hardware_8_security_7_keymint_17_KeyCreationResult> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#importWrappedKey<'a, 'l1, 'l2, 'l3, 'l4, >(&'a self, _arg_wrappedKeyData: &'l1 [u8], _arg_wrappingKeyBlob: &'l2 [u8], _arg_maskingKey: &'l3 [u8], _arg_unwrappingParams: &'l4 [crate::mangled::_7_android_8_hardware_8_security_7_keymint_12_KeyParameter], _arg_passwordSid: i64, _arg_biometricSid: i64) -> binder::Result<crate::mangled::_7_android_8_hardware_8_security_7_keymint_17_KeyCreationResult> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#upgradeKey<'a, 'l1, 'l2, >(&'a self, _arg_keyBlobToUpgrade: &'l1 [u8], _arg_upgradeParams: &'l2 [crate::mangled::_7_android_8_hardware_8_security_7_keymint_12_KeyParameter]) -> binder::Result<Vec<u8>> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#deleteKey<'a, 'l1, >(&'a self, _arg_keyBlob: &'l1 [u8]) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#deleteAllKeys<'a, >(&'a self) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#destroyAttestationIds<'a, >(&'a self) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#begin<'a, 'l1, 'l2, 'l3, >(&'a self, _arg_purpose: crate::mangled::_7_android_8_hardware_8_security_7_keymint_10_KeyPurpose, _arg_keyBlob: &'l1 [u8], _arg_params: &'l2 [crate::mangled::_7_android_8_hardware_8_security_7_keymint_12_KeyParameter], _arg_authToken: Option<&'l3 crate::mangled::_7_android_8_hardware_8_security_7_keymint_17_HardwareAuthToken>) -> binder::Result<crate::mangled::_7_android_8_hardware_8_security_7_keymint_11_BeginResult> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#deviceLocked<'a, 'l1, >(&'a self, _arg_passwordOnly: bool, _arg_timestampToken: Option<&'l1 crate::mangled::_7_android_8_hardware_8_security_11_secureclock_14_TimeStampToken>) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#earlyBootEnded<'a, >(&'a self) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#convertStorageKeyToEphemeral<'a, 'l1, >(&'a self, _arg_storageKeyBlob: &'l1 [u8]) -> binder::Result<Vec<u8>> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#getKeyCharacteristics<'a, 'l1, 'l2, 'l3, >(&'a self, _arg_keyBlob: &'l1 [u8], _arg_appId: &'l2 [u8], _arg_appData: &'l3 [u8]) -> binder::Result<Vec<crate::mangled::_7_android_8_hardware_8_security_7_keymint_18_KeyCharacteristics>> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#getRootOfTrustChallenge<'a, >(&'a self) -> binder::Result<[u8; 16]> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#getRootOfTrust<'a, 'l1, >(&'a self, _arg_challenge: &'l1 [u8; 16]) -> binder::Result<Vec<u8>> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#sendRootOfTrust<'a, 'l1, >(&'a self, _arg_rootOfTrust: &'l1 [u8]) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#setAdditionalAttestationInfo<'a, 'l1, >(&'a self, _arg_info: &'l1 [crate::mangled::_7_android_8_hardware_8_security_7_keymint_12_KeyParameter]) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
            }
            pub mod transactions {
              pub const r#getHardwareInfo: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 0;
              pub const r#addRngEntropy: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 1;
              pub const r#generateKey: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 2;
              pub const r#importKey: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 3;
              pub const r#importWrappedKey: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 4;
              pub const r#upgradeKey: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 5;
              pub const r#deleteKey: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 6;
              pub const r#deleteAllKeys: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 7;
              pub const r#destroyAttestationIds: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 8;
              pub const r#begin: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 9;
              pub const r#deviceLocked: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 10;
              pub const r#earlyBootEnded: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 11;
              pub const r#convertStorageKeyToEphemeral: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 12;
              pub const r#getKeyCharacteristics: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 13;
              pub const r#getRootOfTrustChallenge: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 14;
              pub const r#getRootOfTrust: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 15;
              pub const r#sendRootOfTrust: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 16;
              pub const r#setAdditionalAttestationInfo: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 17;
              pub const r#getInterfaceVersion: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 16777214;
              pub const r#getInterfaceHash: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 16777213;
            }
            pub type IKeyMintDeviceDefaultRef = Option<std::sync::Arc<dyn IKeyMintDeviceDefault>>;
            static DEFAULT_IMPL: std::sync::Mutex<IKeyMintDeviceDefaultRef> = std::sync::Mutex::new(None);
            pub const r#AUTH_TOKEN_MAC_LENGTH: i32 = 32;
            pub const VERSION: i32 = 4;
            pub const HASH: &str = "a05c8079586139db45b0762a528cdd9745ad15ce";
            impl BpKeyMintDevice {
              fn build_parcel_getHardwareInfo(&self) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.mark_sensitive();
                Ok(aidl_data)
              }
              fn read_response_getHardwareInfo(&self, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<crate::mangled::_7_android_8_hardware_8_security_7_keymint_19_KeyMintHardwareInfo> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IKeyMintDevice>::getDefaultImpl() {
                    return _aidl_default_impl.r#getHardwareInfo();
                  }
                }
                let _aidl_reply = _aidl_reply?;
                let _aidl_status: binder::Status = _aidl_reply.read()?;
                if !_aidl_status.is_ok() { return Err(_aidl_status); }
                let _aidl_return: crate::mangled::_7_android_8_hardware_8_security_7_keymint_19_KeyMintHardwareInfo = _aidl_reply.read()?;
                Ok(_aidl_return)
              }
              fn build_parcel_addRngEntropy(&self, _arg_data: &[u8]) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.mark_sensitive();
                aidl_data.write(_arg_data)?;
                Ok(aidl_data)
              }
              fn read_response_addRngEntropy(&self, _arg_data: &[u8], _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IKeyMintDevice>::getDefaultImpl() {
                    return _aidl_default_impl.r#addRngEntropy(_arg_data);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                let _aidl_status: binder::Status = _aidl_reply.read()?;
                if !_aidl_status.is_ok() { return Err(_aidl_status); }
                Ok(())
              }
              fn build_parcel_generateKey(&self, _arg_keyParams: &[crate::mangled::_7_android_8_hardware_8_security_7_keymint_12_KeyParameter], _arg_attestationKey: Option<&crate::mangled::_7_android_8_hardware_8_security_7_keymint_14_AttestationKey>) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.mark_sensitive();
                aidl_data.write(_arg_keyParams)?;
                aidl_data.write(&_arg_attestationKey)?;
                Ok(aidl_data)
              }
              fn read_response_generateKey(&self, _arg_keyParams: &[crate::mangled::_7_android_8_hardware_8_security_7_keymint_12_KeyParameter], _arg_attestationKey: Option<&crate::mangled::_7_android_8_hardware_8_security_7_keymint_14_AttestationKey>, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<crate::mangled::_7_android_8_hardware_8_security_7_keymint_17_KeyCreationResult> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IKeyMintDevice>::getDefaultImpl() {
                    return _aidl_default_impl.r#generateKey(_arg_keyParams, _arg_attestationKey);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                let _aidl_status: binder::Status = _aidl_reply.read()?;
                if !_aidl_status.is_ok() { return Err(_aidl_status); }
                let _aidl_return: crate::mangled::_7_android_8_hardware_8_security_7_keymint_17_KeyCreationResult = _aidl_reply.read()?;
                Ok(_aidl_return)
              }
              fn build_parcel_importKey(&self, _arg_keyParams: &[crate::mangled::_7_android_8_hardware_8_security_7_keymint_12_KeyParameter], _arg_keyFormat: crate::mangled::_7_android_8_hardware_8_security_7_keymint_9_KeyFormat, _arg_keyData: &[u8], _arg_attestationKey: Option<&crate::mangled::_7_android_8_hardware_8_security_7_keymint_14_AttestationKey>) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.mark_sensitive();
                aidl_data.write(_arg_keyParams)?;
                aidl_data.write(&_arg_keyFormat)?;
                aidl_data.write(_arg_keyData)?;
                aidl_data.write(&_arg_attestationKey)?;
                Ok(aidl_data)
              }
              fn read_response_importKey(&self, _arg_keyParams: &[crate::mangled::_7_android_8_hardware_8_security_7_keymint_12_KeyParameter], _arg_keyFormat: crate::mangled::_7_android_8_hardware_8_security_7_keymint_9_KeyFormat, _arg_keyData: &[u8], _arg_attestationKey: Option<&crate::mangled::_7_android_8_hardware_8_security_7_keymint_14_AttestationKey>, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<crate::mangled::_7_android_8_hardware_8_security_7_keymint_17_KeyCreationResult> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IKeyMintDevice>::getDefaultImpl() {
                    return _aidl_default_impl.r#importKey(_arg_keyParams, _arg_keyFormat, _arg_keyData, _arg_attestationKey);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                let _aidl_status: binder::Status = _aidl_reply.read()?;
                if !_aidl_status.is_ok() { return Err(_aidl_status); }
                let _aidl_return: crate::mangled::_7_android_8_hardware_8_security_7_keymint_17_KeyCreationResult = _aidl_reply.read()?;
                Ok(_aidl_return)
              }
              fn build_parcel_importWrappedKey(&self, _arg_wrappedKeyData: &[u8], _arg_wrappingKeyBlob: &[u8], _arg_maskingKey: &[u8], _arg_unwrappingParams: &[crate::mangled::_7_android_8_hardware_8_security_7_keymint_12_KeyParameter], _arg_passwordSid: i64, _arg_biometricSid: i64) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.mark_sensitive();
                aidl_data.write(_arg_wrappedKeyData)?;
                aidl_data.write(_arg_wrappingKeyBlob)?;
                aidl_data.write(_arg_maskingKey)?;
                aidl_data.write(_arg_unwrappingParams)?;
                aidl_data.write(&_arg_passwordSid)?;
                aidl_data.write(&_arg_biometricSid)?;
                Ok(aidl_data)
              }
              fn read_response_importWrappedKey(&self, _arg_wrappedKeyData: &[u8], _arg_wrappingKeyBlob: &[u8], _arg_maskingKey: &[u8], _arg_unwrappingParams: &[crate::mangled::_7_android_8_hardware_8_security_7_keymint_12_KeyParameter], _arg_passwordSid: i64, _arg_biometricSid: i64, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<crate::mangled::_7_android_8_hardware_8_security_7_keymint_17_KeyCreationResult> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IKeyMintDevice>::getDefaultImpl() {
                    return _aidl_default_impl.r#importWrappedKey(_arg_wrappedKeyData, _arg_wrappingKeyBlob, _arg_maskingKey, _arg_unwrappingParams, _arg_passwordSid, _arg_biometricSid);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                let _aidl_status: binder::Status = _aidl_reply.read()?;
                if !_aidl_status.is_ok() { return Err(_aidl_status); }
                let _aidl_return: crate::mangled::_7_android_8_hardware_8_security_7_keymint_17_KeyCreationResult = _aidl_reply.read()?;
                Ok(_aidl_return)
              }
              fn build_parcel_upgradeKey(&self, _arg_keyBlobToUpgrade: &[u8], _arg_upgradeParams: &[crate::mangled::_7_android_8_hardware_8_security_7_keymint_12_KeyParameter]) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.mark_sensitive();
                aidl_data.write(_arg_keyBlobToUpgrade)?;
                aidl_data.write(_arg_upgradeParams)?;
                Ok(aidl_data)
              }
              fn read_response_upgradeKey(&self, _arg_keyBlobToUpgrade: &[u8], _arg_upgradeParams: &[crate::mangled::_7_android_8_hardware_8_security_7_keymint_12_KeyParameter], _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<Vec<u8>> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IKeyMintDevice>::getDefaultImpl() {
                    return _aidl_default_impl.r#upgradeKey(_arg_keyBlobToUpgrade, _arg_upgradeParams);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                let _aidl_status: binder::Status = _aidl_reply.read()?;
                if !_aidl_status.is_ok() { return Err(_aidl_status); }
                let _aidl_return: Vec<u8> = _aidl_reply.read()?;
                Ok(_aidl_return)
              }
              fn build_parcel_deleteKey(&self, _arg_keyBlob: &[u8]) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.mark_sensitive();
                aidl_data.write(_arg_keyBlob)?;
                Ok(aidl_data)
              }
              fn read_response_deleteKey(&self, _arg_keyBlob: &[u8], _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IKeyMintDevice>::getDefaultImpl() {
                    return _aidl_default_impl.r#deleteKey(_arg_keyBlob);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                let _aidl_status: binder::Status = _aidl_reply.read()?;
                if !_aidl_status.is_ok() { return Err(_aidl_status); }
                Ok(())
              }
              fn build_parcel_deleteAllKeys(&self) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.mark_sensitive();
                Ok(aidl_data)
              }
              fn read_response_deleteAllKeys(&self, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IKeyMintDevice>::getDefaultImpl() {
                    return _aidl_default_impl.r#deleteAllKeys();
                  }
                }
                let _aidl_reply = _aidl_reply?;
                let _aidl_status: binder::Status = _aidl_reply.read()?;
                if !_aidl_status.is_ok() { return Err(_aidl_status); }
                Ok(())
              }
              fn build_parcel_destroyAttestationIds(&self) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.mark_sensitive();
                Ok(aidl_data)
              }
              fn read_response_destroyAttestationIds(&self, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IKeyMintDevice>::getDefaultImpl() {
                    return _aidl_default_impl.r#destroyAttestationIds();
                  }
                }
                let _aidl_reply = _aidl_reply?;
                let _aidl_status: binder::Status = _aidl_reply.read()?;
                if !_aidl_status.is_ok() { return Err(_aidl_status); }
                Ok(())
              }
              fn build_parcel_begin(&self, _arg_purpose: crate::mangled::_7_android_8_hardware_8_security_7_keymint_10_KeyPurpose, _arg_keyBlob: &[u8], _arg_params: &[crate::mangled::_7_android_8_hardware_8_security_7_keymint_12_KeyParameter], _arg_authToken: Option<&crate::mangled::_7_android_8_hardware_8_security_7_keymint_17_HardwareAuthToken>) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.mark_sensitive();
                aidl_data.write(&_arg_purpose)?;
                aidl_data.write(_arg_keyBlob)?;
                aidl_data.write(_arg_params)?;
                aidl_data.write(&_arg_authToken)?;
                Ok(aidl_data)
              }
              fn read_response_begin(&self, _arg_purpose: crate::mangled::_7_android_8_hardware_8_security_7_keymint_10_KeyPurpose, _arg_keyBlob: &[u8], _arg_params: &[crate::mangled::_7_android_8_hardware_8_security_7_keymint_12_KeyParameter], _arg_authToken: Option<&crate::mangled::_7_android_8_hardware_8_security_7_keymint_17_HardwareAuthToken>, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<crate::mangled::_7_android_8_hardware_8_security_7_keymint_11_BeginResult> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IKeyMintDevice>::getDefaultImpl() {
                    return _aidl_default_impl.r#begin(_arg_purpose, _arg_keyBlob, _arg_params, _arg_authToken);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                let _aidl_status: binder::Status = _aidl_reply.read()?;
                if !_aidl_status.is_ok() { return Err(_aidl_status); }
                let _aidl_return: crate::mangled::_7_android_8_hardware_8_security_7_keymint_11_BeginResult = _aidl_reply.read()?;
                Ok(_aidl_return)
              }
              fn build_parcel_deviceLocked(&self, _arg_passwordOnly: bool, _arg_timestampToken: Option<&crate::mangled::_7_android_8_hardware_8_security_11_secureclock_14_TimeStampToken>) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.mark_sensitive();
                aidl_data.write(&_arg_passwordOnly)?;
                aidl_data.write(&_arg_timestampToken)?;
                Ok(aidl_data)
              }
              fn read_response_deviceLocked(&self, _arg_passwordOnly: bool, _arg_timestampToken: Option<&crate::mangled::_7_android_8_hardware_8_security_11_secureclock_14_TimeStampToken>, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IKeyMintDevice>::getDefaultImpl() {
                    return _aidl_default_impl.r#deviceLocked(_arg_passwordOnly, _arg_timestampToken);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                let _aidl_status: binder::Status = _aidl_reply.read()?;
                if !_aidl_status.is_ok() { return Err(_aidl_status); }
                Ok(())
              }
              fn build_parcel_earlyBootEnded(&self) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.mark_sensitive();
                Ok(aidl_data)
              }
              fn read_response_earlyBootEnded(&self, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IKeyMintDevice>::getDefaultImpl() {
                    return _aidl_default_impl.r#earlyBootEnded();
                  }
                }
                let _aidl_reply = _aidl_reply?;
                let _aidl_status: binder::Status = _aidl_reply.read()?;
                if !_aidl_status.is_ok() { return Err(_aidl_status); }
                Ok(())
              }
              fn build_parcel_convertStorageKeyToEphemeral(&self, _arg_storageKeyBlob: &[u8]) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.mark_sensitive();
                aidl_data.write(_arg_storageKeyBlob)?;
                Ok(aidl_data)
              }
              fn read_response_convertStorageKeyToEphemeral(&self, _arg_storageKeyBlob: &[u8], _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<Vec<u8>> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IKeyMintDevice>::getDefaultImpl() {
                    return _aidl_default_impl.r#convertStorageKeyToEphemeral(_arg_storageKeyBlob);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                let _aidl_status: binder::Status = _aidl_reply.read()?;
                if !_aidl_status.is_ok() { return Err(_aidl_status); }
                let _aidl_return: Vec<u8> = _aidl_reply.read()?;
                Ok(_aidl_return)
              }
              fn build_parcel_getKeyCharacteristics(&self, _arg_keyBlob: &[u8], _arg_appId: &[u8], _arg_appData: &[u8]) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.mark_sensitive();
                aidl_data.write(_arg_keyBlob)?;
                aidl_data.write(_arg_appId)?;
                aidl_data.write(_arg_appData)?;
                Ok(aidl_data)
              }
              fn read_response_getKeyCharacteristics(&self, _arg_keyBlob: &[u8], _arg_appId: &[u8], _arg_appData: &[u8], _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<Vec<crate::mangled::_7_android_8_hardware_8_security_7_keymint_18_KeyCharacteristics>> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IKeyMintDevice>::getDefaultImpl() {
                    return _aidl_default_impl.r#getKeyCharacteristics(_arg_keyBlob, _arg_appId, _arg_appData);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                let _aidl_status: binder::Status = _aidl_reply.read()?;
                if !_aidl_status.is_ok() { return Err(_aidl_status); }
                let _aidl_return: Vec<crate::mangled::_7_android_8_hardware_8_security_7_keymint_18_KeyCharacteristics> = _aidl_reply.read()?;
                Ok(_aidl_return)
              }
              fn build_parcel_getRootOfTrustChallenge(&self) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.mark_sensitive();
                Ok(aidl_data)
              }
              fn read_response_getRootOfTrustChallenge(&self, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<[u8; 16]> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IKeyMintDevice>::getDefaultImpl() {
                    return _aidl_default_impl.r#getRootOfTrustChallenge();
                  }
                }
                let _aidl_reply = _aidl_reply?;
                let _aidl_status: binder::Status = _aidl_reply.read()?;
                if !_aidl_status.is_ok() { return Err(_aidl_status); }
                let _aidl_return: [u8; 16] = _aidl_reply.read()?;
                Ok(_aidl_return)
              }
              fn build_parcel_getRootOfTrust(&self, _arg_challenge: &[u8; 16]) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.mark_sensitive();
                aidl_data.write(_arg_challenge)?;
                Ok(aidl_data)
              }
              fn read_response_getRootOfTrust(&self, _arg_challenge: &[u8; 16], _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<Vec<u8>> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IKeyMintDevice>::getDefaultImpl() {
                    return _aidl_default_impl.r#getRootOfTrust(_arg_challenge);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                let _aidl_status: binder::Status = _aidl_reply.read()?;
                if !_aidl_status.is_ok() { return Err(_aidl_status); }
                let _aidl_return: Vec<u8> = _aidl_reply.read()?;
                Ok(_aidl_return)
              }
              fn build_parcel_sendRootOfTrust(&self, _arg_rootOfTrust: &[u8]) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.mark_sensitive();
                aidl_data.write(_arg_rootOfTrust)?;
                Ok(aidl_data)
              }
              fn read_response_sendRootOfTrust(&self, _arg_rootOfTrust: &[u8], _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IKeyMintDevice>::getDefaultImpl() {
                    return _aidl_default_impl.r#sendRootOfTrust(_arg_rootOfTrust);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                let _aidl_status: binder::Status = _aidl_reply.read()?;
                if !_aidl_status.is_ok() { return Err(_aidl_status); }
                Ok(())
              }
              fn build_parcel_setAdditionalAttestationInfo(&self, _arg_info: &[crate::mangled::_7_android_8_hardware_8_security_7_keymint_12_KeyParameter]) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.mark_sensitive();
                aidl_data.write(_arg_info)?;
                Ok(aidl_data)
              }
              fn read_response_setAdditionalAttestationInfo(&self, _arg_info: &[crate::mangled::_7_android_8_hardware_8_security_7_keymint_12_KeyParameter], _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IKeyMintDevice>::getDefaultImpl() {
                    return _aidl_default_impl.r#setAdditionalAttestationInfo(_arg_info);
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
            impl IKeyMintDevice for BpKeyMintDevice {
              fn r#getHardwareInfo<'a, >(&'a self) -> binder::Result<crate::mangled::_7_android_8_hardware_8_security_7_keymint_19_KeyMintHardwareInfo> {
                let _aidl_data = self.build_parcel_getHardwareInfo()?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#getHardwareInfo, _aidl_data, binder::binder_impl::FLAG_CLEAR_BUF | FLAG_PRIVATE_LOCAL);
                self.read_response_getHardwareInfo(_aidl_reply)
              }
              fn r#addRngEntropy<'a, 'l1, >(&'a self, _arg_data: &'l1 [u8]) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_addRngEntropy(_arg_data)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#addRngEntropy, _aidl_data, binder::binder_impl::FLAG_CLEAR_BUF | FLAG_PRIVATE_LOCAL);
                self.read_response_addRngEntropy(_arg_data, _aidl_reply)
              }
              fn r#generateKey<'a, 'l1, 'l2, >(&'a self, _arg_keyParams: &'l1 [crate::mangled::_7_android_8_hardware_8_security_7_keymint_12_KeyParameter], _arg_attestationKey: Option<&'l2 crate::mangled::_7_android_8_hardware_8_security_7_keymint_14_AttestationKey>) -> binder::Result<crate::mangled::_7_android_8_hardware_8_security_7_keymint_17_KeyCreationResult> {
                let _aidl_data = self.build_parcel_generateKey(_arg_keyParams, _arg_attestationKey)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#generateKey, _aidl_data, binder::binder_impl::FLAG_CLEAR_BUF | FLAG_PRIVATE_LOCAL);
                self.read_response_generateKey(_arg_keyParams, _arg_attestationKey, _aidl_reply)
              }
              fn r#importKey<'a, 'l1, 'l2, 'l3, >(&'a self, _arg_keyParams: &'l1 [crate::mangled::_7_android_8_hardware_8_security_7_keymint_12_KeyParameter], _arg_keyFormat: crate::mangled::_7_android_8_hardware_8_security_7_keymint_9_KeyFormat, _arg_keyData: &'l2 [u8], _arg_attestationKey: Option<&'l3 crate::mangled::_7_android_8_hardware_8_security_7_keymint_14_AttestationKey>) -> binder::Result<crate::mangled::_7_android_8_hardware_8_security_7_keymint_17_KeyCreationResult> {
                let _aidl_data = self.build_parcel_importKey(_arg_keyParams, _arg_keyFormat, _arg_keyData, _arg_attestationKey)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#importKey, _aidl_data, binder::binder_impl::FLAG_CLEAR_BUF | FLAG_PRIVATE_LOCAL);
                self.read_response_importKey(_arg_keyParams, _arg_keyFormat, _arg_keyData, _arg_attestationKey, _aidl_reply)
              }
              fn r#importWrappedKey<'a, 'l1, 'l2, 'l3, 'l4, >(&'a self, _arg_wrappedKeyData: &'l1 [u8], _arg_wrappingKeyBlob: &'l2 [u8], _arg_maskingKey: &'l3 [u8], _arg_unwrappingParams: &'l4 [crate::mangled::_7_android_8_hardware_8_security_7_keymint_12_KeyParameter], _arg_passwordSid: i64, _arg_biometricSid: i64) -> binder::Result<crate::mangled::_7_android_8_hardware_8_security_7_keymint_17_KeyCreationResult> {
                let _aidl_data = self.build_parcel_importWrappedKey(_arg_wrappedKeyData, _arg_wrappingKeyBlob, _arg_maskingKey, _arg_unwrappingParams, _arg_passwordSid, _arg_biometricSid)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#importWrappedKey, _aidl_data, binder::binder_impl::FLAG_CLEAR_BUF | FLAG_PRIVATE_LOCAL);
                self.read_response_importWrappedKey(_arg_wrappedKeyData, _arg_wrappingKeyBlob, _arg_maskingKey, _arg_unwrappingParams, _arg_passwordSid, _arg_biometricSid, _aidl_reply)
              }
              fn r#upgradeKey<'a, 'l1, 'l2, >(&'a self, _arg_keyBlobToUpgrade: &'l1 [u8], _arg_upgradeParams: &'l2 [crate::mangled::_7_android_8_hardware_8_security_7_keymint_12_KeyParameter]) -> binder::Result<Vec<u8>> {
                let _aidl_data = self.build_parcel_upgradeKey(_arg_keyBlobToUpgrade, _arg_upgradeParams)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#upgradeKey, _aidl_data, binder::binder_impl::FLAG_CLEAR_BUF | FLAG_PRIVATE_LOCAL);
                self.read_response_upgradeKey(_arg_keyBlobToUpgrade, _arg_upgradeParams, _aidl_reply)
              }
              fn r#deleteKey<'a, 'l1, >(&'a self, _arg_keyBlob: &'l1 [u8]) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_deleteKey(_arg_keyBlob)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#deleteKey, _aidl_data, binder::binder_impl::FLAG_CLEAR_BUF | FLAG_PRIVATE_LOCAL);
                self.read_response_deleteKey(_arg_keyBlob, _aidl_reply)
              }
              fn r#deleteAllKeys<'a, >(&'a self) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_deleteAllKeys()?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#deleteAllKeys, _aidl_data, binder::binder_impl::FLAG_CLEAR_BUF | FLAG_PRIVATE_LOCAL);
                self.read_response_deleteAllKeys(_aidl_reply)
              }
              fn r#destroyAttestationIds<'a, >(&'a self) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_destroyAttestationIds()?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#destroyAttestationIds, _aidl_data, binder::binder_impl::FLAG_CLEAR_BUF | FLAG_PRIVATE_LOCAL);
                self.read_response_destroyAttestationIds(_aidl_reply)
              }
              fn r#begin<'a, 'l1, 'l2, 'l3, >(&'a self, _arg_purpose: crate::mangled::_7_android_8_hardware_8_security_7_keymint_10_KeyPurpose, _arg_keyBlob: &'l1 [u8], _arg_params: &'l2 [crate::mangled::_7_android_8_hardware_8_security_7_keymint_12_KeyParameter], _arg_authToken: Option<&'l3 crate::mangled::_7_android_8_hardware_8_security_7_keymint_17_HardwareAuthToken>) -> binder::Result<crate::mangled::_7_android_8_hardware_8_security_7_keymint_11_BeginResult> {
                let _aidl_data = self.build_parcel_begin(_arg_purpose, _arg_keyBlob, _arg_params, _arg_authToken)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#begin, _aidl_data, binder::binder_impl::FLAG_CLEAR_BUF | FLAG_PRIVATE_LOCAL);
                self.read_response_begin(_arg_purpose, _arg_keyBlob, _arg_params, _arg_authToken, _aidl_reply)
              }
              fn r#deviceLocked<'a, 'l1, >(&'a self, _arg_passwordOnly: bool, _arg_timestampToken: Option<&'l1 crate::mangled::_7_android_8_hardware_8_security_11_secureclock_14_TimeStampToken>) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_deviceLocked(_arg_passwordOnly, _arg_timestampToken)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#deviceLocked, _aidl_data, binder::binder_impl::FLAG_CLEAR_BUF | FLAG_PRIVATE_LOCAL);
                self.read_response_deviceLocked(_arg_passwordOnly, _arg_timestampToken, _aidl_reply)
              }
              fn r#earlyBootEnded<'a, >(&'a self) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_earlyBootEnded()?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#earlyBootEnded, _aidl_data, binder::binder_impl::FLAG_CLEAR_BUF | FLAG_PRIVATE_LOCAL);
                self.read_response_earlyBootEnded(_aidl_reply)
              }
              fn r#convertStorageKeyToEphemeral<'a, 'l1, >(&'a self, _arg_storageKeyBlob: &'l1 [u8]) -> binder::Result<Vec<u8>> {
                let _aidl_data = self.build_parcel_convertStorageKeyToEphemeral(_arg_storageKeyBlob)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#convertStorageKeyToEphemeral, _aidl_data, binder::binder_impl::FLAG_CLEAR_BUF | FLAG_PRIVATE_LOCAL);
                self.read_response_convertStorageKeyToEphemeral(_arg_storageKeyBlob, _aidl_reply)
              }
              fn r#getKeyCharacteristics<'a, 'l1, 'l2, 'l3, >(&'a self, _arg_keyBlob: &'l1 [u8], _arg_appId: &'l2 [u8], _arg_appData: &'l3 [u8]) -> binder::Result<Vec<crate::mangled::_7_android_8_hardware_8_security_7_keymint_18_KeyCharacteristics>> {
                let _aidl_data = self.build_parcel_getKeyCharacteristics(_arg_keyBlob, _arg_appId, _arg_appData)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#getKeyCharacteristics, _aidl_data, binder::binder_impl::FLAG_CLEAR_BUF | FLAG_PRIVATE_LOCAL);
                self.read_response_getKeyCharacteristics(_arg_keyBlob, _arg_appId, _arg_appData, _aidl_reply)
              }
              fn r#getRootOfTrustChallenge<'a, >(&'a self) -> binder::Result<[u8; 16]> {
                let _aidl_data = self.build_parcel_getRootOfTrustChallenge()?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#getRootOfTrustChallenge, _aidl_data, binder::binder_impl::FLAG_CLEAR_BUF | FLAG_PRIVATE_LOCAL);
                self.read_response_getRootOfTrustChallenge(_aidl_reply)
              }
              fn r#getRootOfTrust<'a, 'l1, >(&'a self, _arg_challenge: &'l1 [u8; 16]) -> binder::Result<Vec<u8>> {
                let _aidl_data = self.build_parcel_getRootOfTrust(_arg_challenge)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#getRootOfTrust, _aidl_data, binder::binder_impl::FLAG_CLEAR_BUF | FLAG_PRIVATE_LOCAL);
                self.read_response_getRootOfTrust(_arg_challenge, _aidl_reply)
              }
              fn r#sendRootOfTrust<'a, 'l1, >(&'a self, _arg_rootOfTrust: &'l1 [u8]) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_sendRootOfTrust(_arg_rootOfTrust)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#sendRootOfTrust, _aidl_data, binder::binder_impl::FLAG_CLEAR_BUF | FLAG_PRIVATE_LOCAL);
                self.read_response_sendRootOfTrust(_arg_rootOfTrust, _aidl_reply)
              }
              fn r#setAdditionalAttestationInfo<'a, 'l1, >(&'a self, _arg_info: &'l1 [crate::mangled::_7_android_8_hardware_8_security_7_keymint_12_KeyParameter]) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_setAdditionalAttestationInfo(_arg_info)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#setAdditionalAttestationInfo, _aidl_data, binder::binder_impl::FLAG_CLEAR_BUF | FLAG_PRIVATE_LOCAL);
                self.read_response_setAdditionalAttestationInfo(_arg_info, _aidl_reply)
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
            impl<P: binder::BinderAsyncPool> IKeyMintDeviceAsync<P> for BpKeyMintDevice {
              fn r#getHardwareInfo<'a, >(&'a self) -> binder::BoxFuture<'a, binder::Result<crate::mangled::_7_android_8_hardware_8_security_7_keymint_19_KeyMintHardwareInfo>> {
                let _aidl_data = match self.build_parcel_getHardwareInfo() {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return Box::pin(std::future::ready(Err(err))),
                };
                let binder = self.binder.clone();
                P::spawn(
                  move || binder.submit_transact(transactions::r#getHardwareInfo, _aidl_data, binder::binder_impl::FLAG_CLEAR_BUF | FLAG_PRIVATE_LOCAL),
                  move |_aidl_reply| async move {
                    self.read_response_getHardwareInfo(_aidl_reply)
                  }
                )
              }
              fn r#addRngEntropy<'a, >(&'a self, _arg_data: &'a [u8]) -> binder::BoxFuture<'a, binder::Result<()>> {
                let _aidl_data = match self.build_parcel_addRngEntropy(_arg_data) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return Box::pin(std::future::ready(Err(err))),
                };
                let binder = self.binder.clone();
                P::spawn(
                  move || binder.submit_transact(transactions::r#addRngEntropy, _aidl_data, binder::binder_impl::FLAG_CLEAR_BUF | FLAG_PRIVATE_LOCAL),
                  move |_aidl_reply| async move {
                    self.read_response_addRngEntropy(_arg_data, _aidl_reply)
                  }
                )
              }
              fn r#generateKey<'a, >(&'a self, _arg_keyParams: &'a [crate::mangled::_7_android_8_hardware_8_security_7_keymint_12_KeyParameter], _arg_attestationKey: Option<&'a crate::mangled::_7_android_8_hardware_8_security_7_keymint_14_AttestationKey>) -> binder::BoxFuture<'a, binder::Result<crate::mangled::_7_android_8_hardware_8_security_7_keymint_17_KeyCreationResult>> {
                let _aidl_data = match self.build_parcel_generateKey(_arg_keyParams, _arg_attestationKey) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return Box::pin(std::future::ready(Err(err))),
                };
                let binder = self.binder.clone();
                P::spawn(
                  move || binder.submit_transact(transactions::r#generateKey, _aidl_data, binder::binder_impl::FLAG_CLEAR_BUF | FLAG_PRIVATE_LOCAL),
                  move |_aidl_reply| async move {
                    self.read_response_generateKey(_arg_keyParams, _arg_attestationKey, _aidl_reply)
                  }
                )
              }
              fn r#importKey<'a, >(&'a self, _arg_keyParams: &'a [crate::mangled::_7_android_8_hardware_8_security_7_keymint_12_KeyParameter], _arg_keyFormat: crate::mangled::_7_android_8_hardware_8_security_7_keymint_9_KeyFormat, _arg_keyData: &'a [u8], _arg_attestationKey: Option<&'a crate::mangled::_7_android_8_hardware_8_security_7_keymint_14_AttestationKey>) -> binder::BoxFuture<'a, binder::Result<crate::mangled::_7_android_8_hardware_8_security_7_keymint_17_KeyCreationResult>> {
                let _aidl_data = match self.build_parcel_importKey(_arg_keyParams, _arg_keyFormat, _arg_keyData, _arg_attestationKey) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return Box::pin(std::future::ready(Err(err))),
                };
                let binder = self.binder.clone();
                P::spawn(
                  move || binder.submit_transact(transactions::r#importKey, _aidl_data, binder::binder_impl::FLAG_CLEAR_BUF | FLAG_PRIVATE_LOCAL),
                  move |_aidl_reply| async move {
                    self.read_response_importKey(_arg_keyParams, _arg_keyFormat, _arg_keyData, _arg_attestationKey, _aidl_reply)
                  }
                )
              }
              fn r#importWrappedKey<'a, >(&'a self, _arg_wrappedKeyData: &'a [u8], _arg_wrappingKeyBlob: &'a [u8], _arg_maskingKey: &'a [u8], _arg_unwrappingParams: &'a [crate::mangled::_7_android_8_hardware_8_security_7_keymint_12_KeyParameter], _arg_passwordSid: i64, _arg_biometricSid: i64) -> binder::BoxFuture<'a, binder::Result<crate::mangled::_7_android_8_hardware_8_security_7_keymint_17_KeyCreationResult>> {
                let _aidl_data = match self.build_parcel_importWrappedKey(_arg_wrappedKeyData, _arg_wrappingKeyBlob, _arg_maskingKey, _arg_unwrappingParams, _arg_passwordSid, _arg_biometricSid) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return Box::pin(std::future::ready(Err(err))),
                };
                let binder = self.binder.clone();
                P::spawn(
                  move || binder.submit_transact(transactions::r#importWrappedKey, _aidl_data, binder::binder_impl::FLAG_CLEAR_BUF | FLAG_PRIVATE_LOCAL),
                  move |_aidl_reply| async move {
                    self.read_response_importWrappedKey(_arg_wrappedKeyData, _arg_wrappingKeyBlob, _arg_maskingKey, _arg_unwrappingParams, _arg_passwordSid, _arg_biometricSid, _aidl_reply)
                  }
                )
              }
              fn r#upgradeKey<'a, >(&'a self, _arg_keyBlobToUpgrade: &'a [u8], _arg_upgradeParams: &'a [crate::mangled::_7_android_8_hardware_8_security_7_keymint_12_KeyParameter]) -> binder::BoxFuture<'a, binder::Result<Vec<u8>>> {
                let _aidl_data = match self.build_parcel_upgradeKey(_arg_keyBlobToUpgrade, _arg_upgradeParams) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return Box::pin(std::future::ready(Err(err))),
                };
                let binder = self.binder.clone();
                P::spawn(
                  move || binder.submit_transact(transactions::r#upgradeKey, _aidl_data, binder::binder_impl::FLAG_CLEAR_BUF | FLAG_PRIVATE_LOCAL),
                  move |_aidl_reply| async move {
                    self.read_response_upgradeKey(_arg_keyBlobToUpgrade, _arg_upgradeParams, _aidl_reply)
                  }
                )
              }
              fn r#deleteKey<'a, >(&'a self, _arg_keyBlob: &'a [u8]) -> binder::BoxFuture<'a, binder::Result<()>> {
                let _aidl_data = match self.build_parcel_deleteKey(_arg_keyBlob) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return Box::pin(std::future::ready(Err(err))),
                };
                let binder = self.binder.clone();
                P::spawn(
                  move || binder.submit_transact(transactions::r#deleteKey, _aidl_data, binder::binder_impl::FLAG_CLEAR_BUF | FLAG_PRIVATE_LOCAL),
                  move |_aidl_reply| async move {
                    self.read_response_deleteKey(_arg_keyBlob, _aidl_reply)
                  }
                )
              }
              fn r#deleteAllKeys<'a, >(&'a self) -> binder::BoxFuture<'a, binder::Result<()>> {
                let _aidl_data = match self.build_parcel_deleteAllKeys() {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return Box::pin(std::future::ready(Err(err))),
                };
                let binder = self.binder.clone();
                P::spawn(
                  move || binder.submit_transact(transactions::r#deleteAllKeys, _aidl_data, binder::binder_impl::FLAG_CLEAR_BUF | FLAG_PRIVATE_LOCAL),
                  move |_aidl_reply| async move {
                    self.read_response_deleteAllKeys(_aidl_reply)
                  }
                )
              }
              fn r#destroyAttestationIds<'a, >(&'a self) -> binder::BoxFuture<'a, binder::Result<()>> {
                let _aidl_data = match self.build_parcel_destroyAttestationIds() {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return Box::pin(std::future::ready(Err(err))),
                };
                let binder = self.binder.clone();
                P::spawn(
                  move || binder.submit_transact(transactions::r#destroyAttestationIds, _aidl_data, binder::binder_impl::FLAG_CLEAR_BUF | FLAG_PRIVATE_LOCAL),
                  move |_aidl_reply| async move {
                    self.read_response_destroyAttestationIds(_aidl_reply)
                  }
                )
              }
              fn r#begin<'a, >(&'a self, _arg_purpose: crate::mangled::_7_android_8_hardware_8_security_7_keymint_10_KeyPurpose, _arg_keyBlob: &'a [u8], _arg_params: &'a [crate::mangled::_7_android_8_hardware_8_security_7_keymint_12_KeyParameter], _arg_authToken: Option<&'a crate::mangled::_7_android_8_hardware_8_security_7_keymint_17_HardwareAuthToken>) -> binder::BoxFuture<'a, binder::Result<crate::mangled::_7_android_8_hardware_8_security_7_keymint_11_BeginResult>> {
                let _aidl_data = match self.build_parcel_begin(_arg_purpose, _arg_keyBlob, _arg_params, _arg_authToken) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return Box::pin(std::future::ready(Err(err))),
                };
                let binder = self.binder.clone();
                P::spawn(
                  move || binder.submit_transact(transactions::r#begin, _aidl_data, binder::binder_impl::FLAG_CLEAR_BUF | FLAG_PRIVATE_LOCAL),
                  move |_aidl_reply| async move {
                    self.read_response_begin(_arg_purpose, _arg_keyBlob, _arg_params, _arg_authToken, _aidl_reply)
                  }
                )
              }
              fn r#deviceLocked<'a, >(&'a self, _arg_passwordOnly: bool, _arg_timestampToken: Option<&'a crate::mangled::_7_android_8_hardware_8_security_11_secureclock_14_TimeStampToken>) -> binder::BoxFuture<'a, binder::Result<()>> {
                let _aidl_data = match self.build_parcel_deviceLocked(_arg_passwordOnly, _arg_timestampToken) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return Box::pin(std::future::ready(Err(err))),
                };
                let binder = self.binder.clone();
                P::spawn(
                  move || binder.submit_transact(transactions::r#deviceLocked, _aidl_data, binder::binder_impl::FLAG_CLEAR_BUF | FLAG_PRIVATE_LOCAL),
                  move |_aidl_reply| async move {
                    self.read_response_deviceLocked(_arg_passwordOnly, _arg_timestampToken, _aidl_reply)
                  }
                )
              }
              fn r#earlyBootEnded<'a, >(&'a self) -> binder::BoxFuture<'a, binder::Result<()>> {
                let _aidl_data = match self.build_parcel_earlyBootEnded() {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return Box::pin(std::future::ready(Err(err))),
                };
                let binder = self.binder.clone();
                P::spawn(
                  move || binder.submit_transact(transactions::r#earlyBootEnded, _aidl_data, binder::binder_impl::FLAG_CLEAR_BUF | FLAG_PRIVATE_LOCAL),
                  move |_aidl_reply| async move {
                    self.read_response_earlyBootEnded(_aidl_reply)
                  }
                )
              }
              fn r#convertStorageKeyToEphemeral<'a, >(&'a self, _arg_storageKeyBlob: &'a [u8]) -> binder::BoxFuture<'a, binder::Result<Vec<u8>>> {
                let _aidl_data = match self.build_parcel_convertStorageKeyToEphemeral(_arg_storageKeyBlob) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return Box::pin(std::future::ready(Err(err))),
                };
                let binder = self.binder.clone();
                P::spawn(
                  move || binder.submit_transact(transactions::r#convertStorageKeyToEphemeral, _aidl_data, binder::binder_impl::FLAG_CLEAR_BUF | FLAG_PRIVATE_LOCAL),
                  move |_aidl_reply| async move {
                    self.read_response_convertStorageKeyToEphemeral(_arg_storageKeyBlob, _aidl_reply)
                  }
                )
              }
              fn r#getKeyCharacteristics<'a, >(&'a self, _arg_keyBlob: &'a [u8], _arg_appId: &'a [u8], _arg_appData: &'a [u8]) -> binder::BoxFuture<'a, binder::Result<Vec<crate::mangled::_7_android_8_hardware_8_security_7_keymint_18_KeyCharacteristics>>> {
                let _aidl_data = match self.build_parcel_getKeyCharacteristics(_arg_keyBlob, _arg_appId, _arg_appData) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return Box::pin(std::future::ready(Err(err))),
                };
                let binder = self.binder.clone();
                P::spawn(
                  move || binder.submit_transact(transactions::r#getKeyCharacteristics, _aidl_data, binder::binder_impl::FLAG_CLEAR_BUF | FLAG_PRIVATE_LOCAL),
                  move |_aidl_reply| async move {
                    self.read_response_getKeyCharacteristics(_arg_keyBlob, _arg_appId, _arg_appData, _aidl_reply)
                  }
                )
              }
              fn r#getRootOfTrustChallenge<'a, >(&'a self) -> binder::BoxFuture<'a, binder::Result<[u8; 16]>> {
                let _aidl_data = match self.build_parcel_getRootOfTrustChallenge() {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return Box::pin(std::future::ready(Err(err))),
                };
                let binder = self.binder.clone();
                P::spawn(
                  move || binder.submit_transact(transactions::r#getRootOfTrustChallenge, _aidl_data, binder::binder_impl::FLAG_CLEAR_BUF | FLAG_PRIVATE_LOCAL),
                  move |_aidl_reply| async move {
                    self.read_response_getRootOfTrustChallenge(_aidl_reply)
                  }
                )
              }
              fn r#getRootOfTrust<'a, >(&'a self, _arg_challenge: &'a [u8; 16]) -> binder::BoxFuture<'a, binder::Result<Vec<u8>>> {
                let _aidl_data = match self.build_parcel_getRootOfTrust(_arg_challenge) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return Box::pin(std::future::ready(Err(err))),
                };
                let binder = self.binder.clone();
                P::spawn(
                  move || binder.submit_transact(transactions::r#getRootOfTrust, _aidl_data, binder::binder_impl::FLAG_CLEAR_BUF | FLAG_PRIVATE_LOCAL),
                  move |_aidl_reply| async move {
                    self.read_response_getRootOfTrust(_arg_challenge, _aidl_reply)
                  }
                )
              }
              fn r#sendRootOfTrust<'a, >(&'a self, _arg_rootOfTrust: &'a [u8]) -> binder::BoxFuture<'a, binder::Result<()>> {
                let _aidl_data = match self.build_parcel_sendRootOfTrust(_arg_rootOfTrust) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return Box::pin(std::future::ready(Err(err))),
                };
                let binder = self.binder.clone();
                P::spawn(
                  move || binder.submit_transact(transactions::r#sendRootOfTrust, _aidl_data, binder::binder_impl::FLAG_CLEAR_BUF | FLAG_PRIVATE_LOCAL),
                  move |_aidl_reply| async move {
                    self.read_response_sendRootOfTrust(_arg_rootOfTrust, _aidl_reply)
                  }
                )
              }
              fn r#setAdditionalAttestationInfo<'a, >(&'a self, _arg_info: &'a [crate::mangled::_7_android_8_hardware_8_security_7_keymint_12_KeyParameter]) -> binder::BoxFuture<'a, binder::Result<()>> {
                let _aidl_data = match self.build_parcel_setAdditionalAttestationInfo(_arg_info) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return Box::pin(std::future::ready(Err(err))),
                };
                let binder = self.binder.clone();
                P::spawn(
                  move || binder.submit_transact(transactions::r#setAdditionalAttestationInfo, _aidl_data, binder::binder_impl::FLAG_CLEAR_BUF | FLAG_PRIVATE_LOCAL),
                  move |_aidl_reply| async move {
                    self.read_response_setAdditionalAttestationInfo(_arg_info, _aidl_reply)
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
            impl IKeyMintDevice for binder::binder_impl::Binder<BnKeyMintDevice> {
              fn r#getHardwareInfo<'a, >(&'a self) -> binder::Result<crate::mangled::_7_android_8_hardware_8_security_7_keymint_19_KeyMintHardwareInfo> { self.0.r#getHardwareInfo() }
              fn r#addRngEntropy<'a, 'l1, >(&'a self, _arg_data: &'l1 [u8]) -> binder::Result<()> { self.0.r#addRngEntropy(_arg_data) }
              fn r#generateKey<'a, 'l1, 'l2, >(&'a self, _arg_keyParams: &'l1 [crate::mangled::_7_android_8_hardware_8_security_7_keymint_12_KeyParameter], _arg_attestationKey: Option<&'l2 crate::mangled::_7_android_8_hardware_8_security_7_keymint_14_AttestationKey>) -> binder::Result<crate::mangled::_7_android_8_hardware_8_security_7_keymint_17_KeyCreationResult> { self.0.r#generateKey(_arg_keyParams, _arg_attestationKey) }
              fn r#importKey<'a, 'l1, 'l2, 'l3, >(&'a self, _arg_keyParams: &'l1 [crate::mangled::_7_android_8_hardware_8_security_7_keymint_12_KeyParameter], _arg_keyFormat: crate::mangled::_7_android_8_hardware_8_security_7_keymint_9_KeyFormat, _arg_keyData: &'l2 [u8], _arg_attestationKey: Option<&'l3 crate::mangled::_7_android_8_hardware_8_security_7_keymint_14_AttestationKey>) -> binder::Result<crate::mangled::_7_android_8_hardware_8_security_7_keymint_17_KeyCreationResult> { self.0.r#importKey(_arg_keyParams, _arg_keyFormat, _arg_keyData, _arg_attestationKey) }
              fn r#importWrappedKey<'a, 'l1, 'l2, 'l3, 'l4, >(&'a self, _arg_wrappedKeyData: &'l1 [u8], _arg_wrappingKeyBlob: &'l2 [u8], _arg_maskingKey: &'l3 [u8], _arg_unwrappingParams: &'l4 [crate::mangled::_7_android_8_hardware_8_security_7_keymint_12_KeyParameter], _arg_passwordSid: i64, _arg_biometricSid: i64) -> binder::Result<crate::mangled::_7_android_8_hardware_8_security_7_keymint_17_KeyCreationResult> { self.0.r#importWrappedKey(_arg_wrappedKeyData, _arg_wrappingKeyBlob, _arg_maskingKey, _arg_unwrappingParams, _arg_passwordSid, _arg_biometricSid) }
              fn r#upgradeKey<'a, 'l1, 'l2, >(&'a self, _arg_keyBlobToUpgrade: &'l1 [u8], _arg_upgradeParams: &'l2 [crate::mangled::_7_android_8_hardware_8_security_7_keymint_12_KeyParameter]) -> binder::Result<Vec<u8>> { self.0.r#upgradeKey(_arg_keyBlobToUpgrade, _arg_upgradeParams) }
              fn r#deleteKey<'a, 'l1, >(&'a self, _arg_keyBlob: &'l1 [u8]) -> binder::Result<()> { self.0.r#deleteKey(_arg_keyBlob) }
              fn r#deleteAllKeys<'a, >(&'a self) -> binder::Result<()> { self.0.r#deleteAllKeys() }
              fn r#destroyAttestationIds<'a, >(&'a self) -> binder::Result<()> { self.0.r#destroyAttestationIds() }
              fn r#begin<'a, 'l1, 'l2, 'l3, >(&'a self, _arg_purpose: crate::mangled::_7_android_8_hardware_8_security_7_keymint_10_KeyPurpose, _arg_keyBlob: &'l1 [u8], _arg_params: &'l2 [crate::mangled::_7_android_8_hardware_8_security_7_keymint_12_KeyParameter], _arg_authToken: Option<&'l3 crate::mangled::_7_android_8_hardware_8_security_7_keymint_17_HardwareAuthToken>) -> binder::Result<crate::mangled::_7_android_8_hardware_8_security_7_keymint_11_BeginResult> { self.0.r#begin(_arg_purpose, _arg_keyBlob, _arg_params, _arg_authToken) }
              fn r#deviceLocked<'a, 'l1, >(&'a self, _arg_passwordOnly: bool, _arg_timestampToken: Option<&'l1 crate::mangled::_7_android_8_hardware_8_security_11_secureclock_14_TimeStampToken>) -> binder::Result<()> { self.0.r#deviceLocked(_arg_passwordOnly, _arg_timestampToken) }
              fn r#earlyBootEnded<'a, >(&'a self) -> binder::Result<()> { self.0.r#earlyBootEnded() }
              fn r#convertStorageKeyToEphemeral<'a, 'l1, >(&'a self, _arg_storageKeyBlob: &'l1 [u8]) -> binder::Result<Vec<u8>> { self.0.r#convertStorageKeyToEphemeral(_arg_storageKeyBlob) }
              fn r#getKeyCharacteristics<'a, 'l1, 'l2, 'l3, >(&'a self, _arg_keyBlob: &'l1 [u8], _arg_appId: &'l2 [u8], _arg_appData: &'l3 [u8]) -> binder::Result<Vec<crate::mangled::_7_android_8_hardware_8_security_7_keymint_18_KeyCharacteristics>> { self.0.r#getKeyCharacteristics(_arg_keyBlob, _arg_appId, _arg_appData) }
              fn r#getRootOfTrustChallenge<'a, >(&'a self) -> binder::Result<[u8; 16]> { self.0.r#getRootOfTrustChallenge() }
              fn r#getRootOfTrust<'a, 'l1, >(&'a self, _arg_challenge: &'l1 [u8; 16]) -> binder::Result<Vec<u8>> { self.0.r#getRootOfTrust(_arg_challenge) }
              fn r#sendRootOfTrust<'a, 'l1, >(&'a self, _arg_rootOfTrust: &'l1 [u8]) -> binder::Result<()> { self.0.r#sendRootOfTrust(_arg_rootOfTrust) }
              fn r#setAdditionalAttestationInfo<'a, 'l1, >(&'a self, _arg_info: &'l1 [crate::mangled::_7_android_8_hardware_8_security_7_keymint_12_KeyParameter]) -> binder::Result<()> { self.0.r#setAdditionalAttestationInfo(_arg_info) }
              fn r#getInterfaceVersion<'a, >(&'a self) -> binder::Result<i32> { self.0.r#getInterfaceVersion() }
              fn r#getInterfaceHash<'a, >(&'a self) -> binder::Result<String> { self.0.r#getInterfaceHash() }
            }
            fn on_transact(_aidl_service: &dyn IKeyMintDevice, _aidl_code: binder::binder_impl::TransactionCode, _aidl_data: &binder::binder_impl::BorrowedParcel<'_>, _aidl_reply: &mut binder::binder_impl::BorrowedParcel<'_>) -> std::result::Result<(), binder::StatusCode> {
              match _aidl_code {
                transactions::r#getHardwareInfo => {
                  let _aidl_return = _aidl_service.r#getHardwareInfo();
                  match &_aidl_return {
                    Ok(_aidl_return) => {
                      _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
                      _aidl_reply.write(_aidl_return)?;
                    }
                    Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
                  }
                  Ok(())
                }
                transactions::r#addRngEntropy => {
                  let _arg_data: Vec<u8> = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#addRngEntropy(&_arg_data);
                  match &_aidl_return {
                    Ok(_aidl_return) => {
                      _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
                    }
                    Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
                  }
                  Ok(())
                }
                transactions::r#generateKey => {
                  let _arg_keyParams: Vec<crate::mangled::_7_android_8_hardware_8_security_7_keymint_12_KeyParameter> = _aidl_data.read()?;
                  let _arg_attestationKey: Option<crate::mangled::_7_android_8_hardware_8_security_7_keymint_14_AttestationKey> = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#generateKey(&_arg_keyParams, _arg_attestationKey.as_ref());
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
                  let _arg_keyParams: Vec<crate::mangled::_7_android_8_hardware_8_security_7_keymint_12_KeyParameter> = _aidl_data.read()?;
                  let _arg_keyFormat: crate::mangled::_7_android_8_hardware_8_security_7_keymint_9_KeyFormat = _aidl_data.read()?;
                  let _arg_keyData: Vec<u8> = _aidl_data.read()?;
                  let _arg_attestationKey: Option<crate::mangled::_7_android_8_hardware_8_security_7_keymint_14_AttestationKey> = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#importKey(&_arg_keyParams, _arg_keyFormat, &_arg_keyData, _arg_attestationKey.as_ref());
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
                  let _arg_wrappedKeyData: Vec<u8> = _aidl_data.read()?;
                  let _arg_wrappingKeyBlob: Vec<u8> = _aidl_data.read()?;
                  let _arg_maskingKey: Vec<u8> = _aidl_data.read()?;
                  let _arg_unwrappingParams: Vec<crate::mangled::_7_android_8_hardware_8_security_7_keymint_12_KeyParameter> = _aidl_data.read()?;
                  let _arg_passwordSid: i64 = _aidl_data.read()?;
                  let _arg_biometricSid: i64 = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#importWrappedKey(&_arg_wrappedKeyData, &_arg_wrappingKeyBlob, &_arg_maskingKey, &_arg_unwrappingParams, _arg_passwordSid, _arg_biometricSid);
                  match &_aidl_return {
                    Ok(_aidl_return) => {
                      _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
                      _aidl_reply.write(_aidl_return)?;
                    }
                    Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
                  }
                  Ok(())
                }
                transactions::r#upgradeKey => {
                  let _arg_keyBlobToUpgrade: Vec<u8> = _aidl_data.read()?;
                  let _arg_upgradeParams: Vec<crate::mangled::_7_android_8_hardware_8_security_7_keymint_12_KeyParameter> = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#upgradeKey(&_arg_keyBlobToUpgrade, &_arg_upgradeParams);
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
                  let _arg_keyBlob: Vec<u8> = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#deleteKey(&_arg_keyBlob);
                  match &_aidl_return {
                    Ok(_aidl_return) => {
                      _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
                    }
                    Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
                  }
                  Ok(())
                }
                transactions::r#deleteAllKeys => {
                  let _aidl_return = _aidl_service.r#deleteAllKeys();
                  match &_aidl_return {
                    Ok(_aidl_return) => {
                      _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
                    }
                    Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
                  }
                  Ok(())
                }
                transactions::r#destroyAttestationIds => {
                  let _aidl_return = _aidl_service.r#destroyAttestationIds();
                  match &_aidl_return {
                    Ok(_aidl_return) => {
                      _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
                    }
                    Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
                  }
                  Ok(())
                }
                transactions::r#begin => {
                  let _arg_purpose: crate::mangled::_7_android_8_hardware_8_security_7_keymint_10_KeyPurpose = _aidl_data.read()?;
                  let _arg_keyBlob: Vec<u8> = _aidl_data.read()?;
                  let _arg_params: Vec<crate::mangled::_7_android_8_hardware_8_security_7_keymint_12_KeyParameter> = _aidl_data.read()?;
                  let _arg_authToken: Option<crate::mangled::_7_android_8_hardware_8_security_7_keymint_17_HardwareAuthToken> = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#begin(_arg_purpose, &_arg_keyBlob, &_arg_params, _arg_authToken.as_ref());
                  match &_aidl_return {
                    Ok(_aidl_return) => {
                      _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
                      _aidl_reply.write(_aidl_return)?;
                    }
                    Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
                  }
                  Ok(())
                }
                transactions::r#deviceLocked => {
                  let _arg_passwordOnly: bool = _aidl_data.read()?;
                  let _arg_timestampToken: Option<crate::mangled::_7_android_8_hardware_8_security_11_secureclock_14_TimeStampToken> = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#deviceLocked(_arg_passwordOnly, _arg_timestampToken.as_ref());
                  match &_aidl_return {
                    Ok(_aidl_return) => {
                      _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
                    }
                    Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
                  }
                  Ok(())
                }
                transactions::r#earlyBootEnded => {
                  let _aidl_return = _aidl_service.r#earlyBootEnded();
                  match &_aidl_return {
                    Ok(_aidl_return) => {
                      _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
                    }
                    Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
                  }
                  Ok(())
                }
                transactions::r#convertStorageKeyToEphemeral => {
                  let _arg_storageKeyBlob: Vec<u8> = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#convertStorageKeyToEphemeral(&_arg_storageKeyBlob);
                  match &_aidl_return {
                    Ok(_aidl_return) => {
                      _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
                      _aidl_reply.write(_aidl_return)?;
                    }
                    Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
                  }
                  Ok(())
                }
                transactions::r#getKeyCharacteristics => {
                  let _arg_keyBlob: Vec<u8> = _aidl_data.read()?;
                  let _arg_appId: Vec<u8> = _aidl_data.read()?;
                  let _arg_appData: Vec<u8> = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#getKeyCharacteristics(&_arg_keyBlob, &_arg_appId, &_arg_appData);
                  match &_aidl_return {
                    Ok(_aidl_return) => {
                      _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
                      _aidl_reply.write(_aidl_return)?;
                    }
                    Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
                  }
                  Ok(())
                }
                transactions::r#getRootOfTrustChallenge => {
                  let _aidl_return = _aidl_service.r#getRootOfTrustChallenge();
                  match &_aidl_return {
                    Ok(_aidl_return) => {
                      _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
                      _aidl_reply.write(_aidl_return)?;
                    }
                    Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
                  }
                  Ok(())
                }
                transactions::r#getRootOfTrust => {
                  let _arg_challenge: [u8; 16] = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#getRootOfTrust(&_arg_challenge);
                  match &_aidl_return {
                    Ok(_aidl_return) => {
                      _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
                      _aidl_reply.write(_aidl_return)?;
                    }
                    Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
                  }
                  Ok(())
                }
                transactions::r#sendRootOfTrust => {
                  let _arg_rootOfTrust: Vec<u8> = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#sendRootOfTrust(&_arg_rootOfTrust);
                  match &_aidl_return {
                    Ok(_aidl_return) => {
                      _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
                    }
                    Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
                  }
                  Ok(())
                }
                transactions::r#setAdditionalAttestationInfo => {
                  let _arg_info: Vec<crate::mangled::_7_android_8_hardware_8_security_7_keymint_12_KeyParameter> = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#setAdditionalAttestationInfo(&_arg_info);
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
             pub use super::r#IKeyMintDevice as _7_android_8_hardware_8_security_7_keymint_14_IKeyMintDevice;
            }
          }
          pub mod IKeyMintOperation {
            /*
             * This file is auto-generated.  DO NOT MODIFY.
             * Using: out/host/linux-x86/bin/aidl --lang=rust --structured --version 4 --hash a05c8079586139db45b0762a528cdd9745ad15ce -t --stability vintf --min_sdk_version 35 -pout/soong/.intermediates/hardware/interfaces/security/secureclock/aidl/android.hardware.security.secureclock_interface/1/preprocessed.aidl --ninja -d out/soong/.intermediates/hardware/interfaces/security/keymint/aidl/android.hardware.security.keymint-V4-rust-source/gen/android/hardware/security/keymint/IKeyMintOperation.rs.d -o out/soong/.intermediates/hardware/interfaces/security/keymint/aidl/android.hardware.security.keymint-V4-rust-source/gen -Nhardware/interfaces/security/keymint/aidl/aidl_api/android.hardware.security.keymint/4 hardware/interfaces/security/keymint/aidl/aidl_api/android.hardware.security.keymint/4/android/hardware/security/keymint/IKeyMintOperation.aidl
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
              IKeyMintOperation["android.hardware.security.keymint.IKeyMintOperation"] {
                native: BnKeyMintOperation(on_transact),
                proxy: BpKeyMintOperation {
                  cached_version: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(-1),
                  cached_hash: std::sync::Mutex<Option<String>> = std::sync::Mutex::new(None)
                },
                async: IKeyMintOperationAsync(try_into_local_async),
                stability: binder::binder_impl::Stability::Vintf,
              }
            }
            pub trait IKeyMintOperation: binder::Interface + Send {
              fn get_descriptor() -> &'static str where Self: Sized { "android.hardware.security.keymint.IKeyMintOperation" }
              fn r#updateAad<'a, 'l1, 'l2, 'l3, >(&'a self, _arg_input: &'l1 [u8], _arg_authToken: Option<&'l2 crate::mangled::_7_android_8_hardware_8_security_7_keymint_17_HardwareAuthToken>, _arg_timeStampToken: Option<&'l3 crate::mangled::_7_android_8_hardware_8_security_11_secureclock_14_TimeStampToken>) -> binder::Result<()>;
              fn r#update<'a, 'l1, 'l2, 'l3, >(&'a self, _arg_input: &'l1 [u8], _arg_authToken: Option<&'l2 crate::mangled::_7_android_8_hardware_8_security_7_keymint_17_HardwareAuthToken>, _arg_timeStampToken: Option<&'l3 crate::mangled::_7_android_8_hardware_8_security_11_secureclock_14_TimeStampToken>) -> binder::Result<Vec<u8>>;
              fn r#finish<'a, 'l1, 'l2, 'l3, 'l4, 'l5, >(&'a self, _arg_input: Option<&'l1 [u8]>, _arg_signature: Option<&'l2 [u8]>, _arg_authToken: Option<&'l3 crate::mangled::_7_android_8_hardware_8_security_7_keymint_17_HardwareAuthToken>, _arg_timestampToken: Option<&'l4 crate::mangled::_7_android_8_hardware_8_security_11_secureclock_14_TimeStampToken>, _arg_confirmationToken: Option<&'l5 [u8]>) -> binder::Result<Vec<u8>>;
              fn r#abort<'a, >(&'a self) -> binder::Result<()>;
              fn r#getInterfaceVersion<'a, >(&'a self) -> binder::Result<i32> {
                Ok(VERSION)
              }
              fn r#getInterfaceHash<'a, >(&'a self) -> binder::Result<String> {
                Ok(HASH.into())
              }
              fn getDefaultImpl() -> IKeyMintOperationDefaultRef where Self: Sized {
                DEFAULT_IMPL.lock().unwrap().clone()
              }
              fn setDefaultImpl(d: IKeyMintOperationDefaultRef) -> IKeyMintOperationDefaultRef where Self: Sized {
                std::mem::replace(&mut *DEFAULT_IMPL.lock().unwrap(), d)
              }
              fn try_as_async_server<'a>(&'a self) -> Option<&'a (dyn IKeyMintOperationAsyncServer + Send + Sync)> {
                None
              }
            }
            pub trait IKeyMintOperationAsync<P>: binder::Interface + Send {
              fn get_descriptor() -> &'static str where Self: Sized { "android.hardware.security.keymint.IKeyMintOperation" }
              fn r#updateAad<'a, >(&'a self, _arg_input: &'a [u8], _arg_authToken: Option<&'a crate::mangled::_7_android_8_hardware_8_security_7_keymint_17_HardwareAuthToken>, _arg_timeStampToken: Option<&'a crate::mangled::_7_android_8_hardware_8_security_11_secureclock_14_TimeStampToken>) -> binder::BoxFuture<'a, binder::Result<()>>;
              fn r#update<'a, >(&'a self, _arg_input: &'a [u8], _arg_authToken: Option<&'a crate::mangled::_7_android_8_hardware_8_security_7_keymint_17_HardwareAuthToken>, _arg_timeStampToken: Option<&'a crate::mangled::_7_android_8_hardware_8_security_11_secureclock_14_TimeStampToken>) -> binder::BoxFuture<'a, binder::Result<Vec<u8>>>;
              fn r#finish<'a, >(&'a self, _arg_input: Option<&'a [u8]>, _arg_signature: Option<&'a [u8]>, _arg_authToken: Option<&'a crate::mangled::_7_android_8_hardware_8_security_7_keymint_17_HardwareAuthToken>, _arg_timestampToken: Option<&'a crate::mangled::_7_android_8_hardware_8_security_11_secureclock_14_TimeStampToken>, _arg_confirmationToken: Option<&'a [u8]>) -> binder::BoxFuture<'a, binder::Result<Vec<u8>>>;
              fn r#abort<'a, >(&'a self) -> binder::BoxFuture<'a, binder::Result<()>>;
              fn r#getInterfaceVersion<'a, >(&'a self) -> binder::BoxFuture<'a, binder::Result<i32>> {
                Box::pin(async move { Ok(VERSION) })
              }
              fn r#getInterfaceHash<'a, >(&'a self) -> binder::BoxFuture<'a, binder::Result<String>> {
                Box::pin(async move { Ok(HASH.into()) })
              }
            }
            #[::async_trait::async_trait]
            pub trait IKeyMintOperationAsyncServer: binder::Interface + Send {
              fn get_descriptor() -> &'static str where Self: Sized { "android.hardware.security.keymint.IKeyMintOperation" }
              async fn r#updateAad<'a, 'l1, 'l2, 'l3, >(&'a self, _arg_input: &'l1 [u8], _arg_authToken: Option<&'l2 crate::mangled::_7_android_8_hardware_8_security_7_keymint_17_HardwareAuthToken>, _arg_timeStampToken: Option<&'l3 crate::mangled::_7_android_8_hardware_8_security_11_secureclock_14_TimeStampToken>) -> binder::Result<()>;
              async fn r#update<'a, 'l1, 'l2, 'l3, >(&'a self, _arg_input: &'l1 [u8], _arg_authToken: Option<&'l2 crate::mangled::_7_android_8_hardware_8_security_7_keymint_17_HardwareAuthToken>, _arg_timeStampToken: Option<&'l3 crate::mangled::_7_android_8_hardware_8_security_11_secureclock_14_TimeStampToken>) -> binder::Result<Vec<u8>>;
              async fn r#finish<'a, 'l1, 'l2, 'l3, 'l4, 'l5, >(&'a self, _arg_input: Option<&'l1 [u8]>, _arg_signature: Option<&'l2 [u8]>, _arg_authToken: Option<&'l3 crate::mangled::_7_android_8_hardware_8_security_7_keymint_17_HardwareAuthToken>, _arg_timestampToken: Option<&'l4 crate::mangled::_7_android_8_hardware_8_security_11_secureclock_14_TimeStampToken>, _arg_confirmationToken: Option<&'l5 [u8]>) -> binder::Result<Vec<u8>>;
              async fn r#abort<'a, >(&'a self) -> binder::Result<()>;
            }
            impl BnKeyMintOperation {
              /// Create a new async binder service.
              pub fn new_async_binder<T, R>(inner: T, rt: R, features: binder::BinderFeatures) -> binder::Strong<dyn IKeyMintOperation>
              where
                T: IKeyMintOperationAsyncServer + binder::Interface + Send + Sync + 'static,
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
                impl<T, R> IKeyMintOperation for Wrapper<T, R>
                where
                  T: IKeyMintOperationAsyncServer + Send + Sync + 'static,
                  R: binder::binder_impl::BinderAsyncRuntime + Send + Sync + 'static,
                {
                  fn r#updateAad<'a, 'l1, 'l2, 'l3, >(&'a self, _arg_input: &'l1 [u8], _arg_authToken: Option<&'l2 crate::mangled::_7_android_8_hardware_8_security_7_keymint_17_HardwareAuthToken>, _arg_timeStampToken: Option<&'l3 crate::mangled::_7_android_8_hardware_8_security_11_secureclock_14_TimeStampToken>) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#updateAad(_arg_input, _arg_authToken, _arg_timeStampToken))
                  }
                  fn r#update<'a, 'l1, 'l2, 'l3, >(&'a self, _arg_input: &'l1 [u8], _arg_authToken: Option<&'l2 crate::mangled::_7_android_8_hardware_8_security_7_keymint_17_HardwareAuthToken>, _arg_timeStampToken: Option<&'l3 crate::mangled::_7_android_8_hardware_8_security_11_secureclock_14_TimeStampToken>) -> binder::Result<Vec<u8>> {
                    self._rt.block_on(self._inner.r#update(_arg_input, _arg_authToken, _arg_timeStampToken))
                  }
                  fn r#finish<'a, 'l1, 'l2, 'l3, 'l4, 'l5, >(&'a self, _arg_input: Option<&'l1 [u8]>, _arg_signature: Option<&'l2 [u8]>, _arg_authToken: Option<&'l3 crate::mangled::_7_android_8_hardware_8_security_7_keymint_17_HardwareAuthToken>, _arg_timestampToken: Option<&'l4 crate::mangled::_7_android_8_hardware_8_security_11_secureclock_14_TimeStampToken>, _arg_confirmationToken: Option<&'l5 [u8]>) -> binder::Result<Vec<u8>> {
                    self._rt.block_on(self._inner.r#finish(_arg_input, _arg_signature, _arg_authToken, _arg_timestampToken, _arg_confirmationToken))
                  }
                  fn r#abort<'a, >(&'a self) -> binder::Result<()> {
                    self._rt.block_on(self._inner.r#abort())
                  }
                  fn try_as_async_server(&self) -> Option<&(dyn IKeyMintOperationAsyncServer + Send + Sync)> {
                    Some(&self._inner)
                  }
                }
                let wrapped = Wrapper { _inner: inner, _rt: rt };
                Self::new_binder(wrapped, features)
              }
              pub fn try_into_local_async<P: binder::BinderAsyncPool + 'static>(_native: binder::binder_impl::Binder<Self>) -> Option<binder::Strong<dyn IKeyMintOperationAsync<P>>> {
                struct Wrapper {
                  _native: binder::binder_impl::Binder<BnKeyMintOperation>
                }
                impl binder::Interface for Wrapper {}
                impl<P: binder::BinderAsyncPool> IKeyMintOperationAsync<P> for Wrapper {
                  fn r#updateAad<'a, >(&'a self, _arg_input: &'a [u8], _arg_authToken: Option<&'a crate::mangled::_7_android_8_hardware_8_security_7_keymint_17_HardwareAuthToken>, _arg_timeStampToken: Option<&'a crate::mangled::_7_android_8_hardware_8_security_11_secureclock_14_TimeStampToken>) -> binder::BoxFuture<'a, binder::Result<()>> {
                    Box::pin(self._native.try_as_async_server().unwrap().r#updateAad(_arg_input, _arg_authToken, _arg_timeStampToken))
                  }
                  fn r#update<'a, >(&'a self, _arg_input: &'a [u8], _arg_authToken: Option<&'a crate::mangled::_7_android_8_hardware_8_security_7_keymint_17_HardwareAuthToken>, _arg_timeStampToken: Option<&'a crate::mangled::_7_android_8_hardware_8_security_11_secureclock_14_TimeStampToken>) -> binder::BoxFuture<'a, binder::Result<Vec<u8>>> {
                    Box::pin(self._native.try_as_async_server().unwrap().r#update(_arg_input, _arg_authToken, _arg_timeStampToken))
                  }
                  fn r#finish<'a, >(&'a self, _arg_input: Option<&'a [u8]>, _arg_signature: Option<&'a [u8]>, _arg_authToken: Option<&'a crate::mangled::_7_android_8_hardware_8_security_7_keymint_17_HardwareAuthToken>, _arg_timestampToken: Option<&'a crate::mangled::_7_android_8_hardware_8_security_11_secureclock_14_TimeStampToken>, _arg_confirmationToken: Option<&'a [u8]>) -> binder::BoxFuture<'a, binder::Result<Vec<u8>>> {
                    Box::pin(self._native.try_as_async_server().unwrap().r#finish(_arg_input, _arg_signature, _arg_authToken, _arg_timestampToken, _arg_confirmationToken))
                  }
                  fn r#abort<'a, >(&'a self) -> binder::BoxFuture<'a, binder::Result<()>> {
                    Box::pin(self._native.try_as_async_server().unwrap().r#abort())
                  }
                }
                if _native.try_as_async_server().is_some() {
                  Some(binder::Strong::new(Box::new(Wrapper { _native }) as Box<dyn IKeyMintOperationAsync<P>>))
                } else {
                  None
                }
              }
            }
            pub trait IKeyMintOperationDefault: Send + Sync {
              fn r#updateAad<'a, 'l1, 'l2, 'l3, >(&'a self, _arg_input: &'l1 [u8], _arg_authToken: Option<&'l2 crate::mangled::_7_android_8_hardware_8_security_7_keymint_17_HardwareAuthToken>, _arg_timeStampToken: Option<&'l3 crate::mangled::_7_android_8_hardware_8_security_11_secureclock_14_TimeStampToken>) -> binder::Result<()> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#update<'a, 'l1, 'l2, 'l3, >(&'a self, _arg_input: &'l1 [u8], _arg_authToken: Option<&'l2 crate::mangled::_7_android_8_hardware_8_security_7_keymint_17_HardwareAuthToken>, _arg_timeStampToken: Option<&'l3 crate::mangled::_7_android_8_hardware_8_security_11_secureclock_14_TimeStampToken>) -> binder::Result<Vec<u8>> {
                Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
              }
              fn r#finish<'a, 'l1, 'l2, 'l3, 'l4, 'l5, >(&'a self, _arg_input: Option<&'l1 [u8]>, _arg_signature: Option<&'l2 [u8]>, _arg_authToken: Option<&'l3 crate::mangled::_7_android_8_hardware_8_security_7_keymint_17_HardwareAuthToken>, _arg_timestampToken: Option<&'l4 crate::mangled::_7_android_8_hardware_8_security_11_secureclock_14_TimeStampToken>, _arg_confirmationToken: Option<&'l5 [u8]>) -> binder::Result<Vec<u8>> {
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
            pub type IKeyMintOperationDefaultRef = Option<std::sync::Arc<dyn IKeyMintOperationDefault>>;
            static DEFAULT_IMPL: std::sync::Mutex<IKeyMintOperationDefaultRef> = std::sync::Mutex::new(None);
            pub const VERSION: i32 = 4;
            pub const HASH: &str = "a05c8079586139db45b0762a528cdd9745ad15ce";
            impl BpKeyMintOperation {
              fn build_parcel_updateAad(&self, _arg_input: &[u8], _arg_authToken: Option<&crate::mangled::_7_android_8_hardware_8_security_7_keymint_17_HardwareAuthToken>, _arg_timeStampToken: Option<&crate::mangled::_7_android_8_hardware_8_security_11_secureclock_14_TimeStampToken>) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.mark_sensitive();
                aidl_data.write(_arg_input)?;
                aidl_data.write(&_arg_authToken)?;
                aidl_data.write(&_arg_timeStampToken)?;
                Ok(aidl_data)
              }
              fn read_response_updateAad(&self, _arg_input: &[u8], _arg_authToken: Option<&crate::mangled::_7_android_8_hardware_8_security_7_keymint_17_HardwareAuthToken>, _arg_timeStampToken: Option<&crate::mangled::_7_android_8_hardware_8_security_11_secureclock_14_TimeStampToken>, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IKeyMintOperation>::getDefaultImpl() {
                    return _aidl_default_impl.r#updateAad(_arg_input, _arg_authToken, _arg_timeStampToken);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                let _aidl_status: binder::Status = _aidl_reply.read()?;
                if !_aidl_status.is_ok() { return Err(_aidl_status); }
                Ok(())
              }
              fn build_parcel_update(&self, _arg_input: &[u8], _arg_authToken: Option<&crate::mangled::_7_android_8_hardware_8_security_7_keymint_17_HardwareAuthToken>, _arg_timeStampToken: Option<&crate::mangled::_7_android_8_hardware_8_security_11_secureclock_14_TimeStampToken>) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.mark_sensitive();
                aidl_data.write(_arg_input)?;
                aidl_data.write(&_arg_authToken)?;
                aidl_data.write(&_arg_timeStampToken)?;
                Ok(aidl_data)
              }
              fn read_response_update(&self, _arg_input: &[u8], _arg_authToken: Option<&crate::mangled::_7_android_8_hardware_8_security_7_keymint_17_HardwareAuthToken>, _arg_timeStampToken: Option<&crate::mangled::_7_android_8_hardware_8_security_11_secureclock_14_TimeStampToken>, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<Vec<u8>> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IKeyMintOperation>::getDefaultImpl() {
                    return _aidl_default_impl.r#update(_arg_input, _arg_authToken, _arg_timeStampToken);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                let _aidl_status: binder::Status = _aidl_reply.read()?;
                if !_aidl_status.is_ok() { return Err(_aidl_status); }
                let _aidl_return: Vec<u8> = _aidl_reply.read()?;
                Ok(_aidl_return)
              }
              fn build_parcel_finish(&self, _arg_input: Option<&[u8]>, _arg_signature: Option<&[u8]>, _arg_authToken: Option<&crate::mangled::_7_android_8_hardware_8_security_7_keymint_17_HardwareAuthToken>, _arg_timestampToken: Option<&crate::mangled::_7_android_8_hardware_8_security_11_secureclock_14_TimeStampToken>, _arg_confirmationToken: Option<&[u8]>) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.mark_sensitive();
                aidl_data.write(&_arg_input)?;
                aidl_data.write(&_arg_signature)?;
                aidl_data.write(&_arg_authToken)?;
                aidl_data.write(&_arg_timestampToken)?;
                aidl_data.write(&_arg_confirmationToken)?;
                Ok(aidl_data)
              }
              fn read_response_finish(&self, _arg_input: Option<&[u8]>, _arg_signature: Option<&[u8]>, _arg_authToken: Option<&crate::mangled::_7_android_8_hardware_8_security_7_keymint_17_HardwareAuthToken>, _arg_timestampToken: Option<&crate::mangled::_7_android_8_hardware_8_security_11_secureclock_14_TimeStampToken>, _arg_confirmationToken: Option<&[u8]>, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<Vec<u8>> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IKeyMintOperation>::getDefaultImpl() {
                    return _aidl_default_impl.r#finish(_arg_input, _arg_signature, _arg_authToken, _arg_timestampToken, _arg_confirmationToken);
                  }
                }
                let _aidl_reply = _aidl_reply?;
                let _aidl_status: binder::Status = _aidl_reply.read()?;
                if !_aidl_status.is_ok() { return Err(_aidl_status); }
                let _aidl_return: Vec<u8> = _aidl_reply.read()?;
                Ok(_aidl_return)
              }
              fn build_parcel_abort(&self) -> binder::Result<binder::binder_impl::Parcel> {
                let mut aidl_data = self.binder.prepare_transact()?;
                aidl_data.mark_sensitive();
                Ok(aidl_data)
              }
              fn read_response_abort(&self, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
                if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
                  if let Some(_aidl_default_impl) = <Self as IKeyMintOperation>::getDefaultImpl() {
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
            impl IKeyMintOperation for BpKeyMintOperation {
              fn r#updateAad<'a, 'l1, 'l2, 'l3, >(&'a self, _arg_input: &'l1 [u8], _arg_authToken: Option<&'l2 crate::mangled::_7_android_8_hardware_8_security_7_keymint_17_HardwareAuthToken>, _arg_timeStampToken: Option<&'l3 crate::mangled::_7_android_8_hardware_8_security_11_secureclock_14_TimeStampToken>) -> binder::Result<()> {
                let _aidl_data = self.build_parcel_updateAad(_arg_input, _arg_authToken, _arg_timeStampToken)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#updateAad, _aidl_data, binder::binder_impl::FLAG_CLEAR_BUF | FLAG_PRIVATE_LOCAL);
                self.read_response_updateAad(_arg_input, _arg_authToken, _arg_timeStampToken, _aidl_reply)
              }
              fn r#update<'a, 'l1, 'l2, 'l3, >(&'a self, _arg_input: &'l1 [u8], _arg_authToken: Option<&'l2 crate::mangled::_7_android_8_hardware_8_security_7_keymint_17_HardwareAuthToken>, _arg_timeStampToken: Option<&'l3 crate::mangled::_7_android_8_hardware_8_security_11_secureclock_14_TimeStampToken>) -> binder::Result<Vec<u8>> {
                let _aidl_data = self.build_parcel_update(_arg_input, _arg_authToken, _arg_timeStampToken)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#update, _aidl_data, binder::binder_impl::FLAG_CLEAR_BUF | FLAG_PRIVATE_LOCAL);
                self.read_response_update(_arg_input, _arg_authToken, _arg_timeStampToken, _aidl_reply)
              }
              fn r#finish<'a, 'l1, 'l2, 'l3, 'l4, 'l5, >(&'a self, _arg_input: Option<&'l1 [u8]>, _arg_signature: Option<&'l2 [u8]>, _arg_authToken: Option<&'l3 crate::mangled::_7_android_8_hardware_8_security_7_keymint_17_HardwareAuthToken>, _arg_timestampToken: Option<&'l4 crate::mangled::_7_android_8_hardware_8_security_11_secureclock_14_TimeStampToken>, _arg_confirmationToken: Option<&'l5 [u8]>) -> binder::Result<Vec<u8>> {
                let _aidl_data = self.build_parcel_finish(_arg_input, _arg_signature, _arg_authToken, _arg_timestampToken, _arg_confirmationToken)?;
                let _aidl_reply = self.binder.submit_transact(transactions::r#finish, _aidl_data, binder::binder_impl::FLAG_CLEAR_BUF | FLAG_PRIVATE_LOCAL);
                self.read_response_finish(_arg_input, _arg_signature, _arg_authToken, _arg_timestampToken, _arg_confirmationToken, _aidl_reply)
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
            impl<P: binder::BinderAsyncPool> IKeyMintOperationAsync<P> for BpKeyMintOperation {
              fn r#updateAad<'a, >(&'a self, _arg_input: &'a [u8], _arg_authToken: Option<&'a crate::mangled::_7_android_8_hardware_8_security_7_keymint_17_HardwareAuthToken>, _arg_timeStampToken: Option<&'a crate::mangled::_7_android_8_hardware_8_security_11_secureclock_14_TimeStampToken>) -> binder::BoxFuture<'a, binder::Result<()>> {
                let _aidl_data = match self.build_parcel_updateAad(_arg_input, _arg_authToken, _arg_timeStampToken) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return Box::pin(std::future::ready(Err(err))),
                };
                let binder = self.binder.clone();
                P::spawn(
                  move || binder.submit_transact(transactions::r#updateAad, _aidl_data, binder::binder_impl::FLAG_CLEAR_BUF | FLAG_PRIVATE_LOCAL),
                  move |_aidl_reply| async move {
                    self.read_response_updateAad(_arg_input, _arg_authToken, _arg_timeStampToken, _aidl_reply)
                  }
                )
              }
              fn r#update<'a, >(&'a self, _arg_input: &'a [u8], _arg_authToken: Option<&'a crate::mangled::_7_android_8_hardware_8_security_7_keymint_17_HardwareAuthToken>, _arg_timeStampToken: Option<&'a crate::mangled::_7_android_8_hardware_8_security_11_secureclock_14_TimeStampToken>) -> binder::BoxFuture<'a, binder::Result<Vec<u8>>> {
                let _aidl_data = match self.build_parcel_update(_arg_input, _arg_authToken, _arg_timeStampToken) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return Box::pin(std::future::ready(Err(err))),
                };
                let binder = self.binder.clone();
                P::spawn(
                  move || binder.submit_transact(transactions::r#update, _aidl_data, binder::binder_impl::FLAG_CLEAR_BUF | FLAG_PRIVATE_LOCAL),
                  move |_aidl_reply| async move {
                    self.read_response_update(_arg_input, _arg_authToken, _arg_timeStampToken, _aidl_reply)
                  }
                )
              }
              fn r#finish<'a, >(&'a self, _arg_input: Option<&'a [u8]>, _arg_signature: Option<&'a [u8]>, _arg_authToken: Option<&'a crate::mangled::_7_android_8_hardware_8_security_7_keymint_17_HardwareAuthToken>, _arg_timestampToken: Option<&'a crate::mangled::_7_android_8_hardware_8_security_11_secureclock_14_TimeStampToken>, _arg_confirmationToken: Option<&'a [u8]>) -> binder::BoxFuture<'a, binder::Result<Vec<u8>>> {
                let _aidl_data = match self.build_parcel_finish(_arg_input, _arg_signature, _arg_authToken, _arg_timestampToken, _arg_confirmationToken) {
                  Ok(_aidl_data) => _aidl_data,
                  Err(err) => return Box::pin(std::future::ready(Err(err))),
                };
                let binder = self.binder.clone();
                P::spawn(
                  move || binder.submit_transact(transactions::r#finish, _aidl_data, binder::binder_impl::FLAG_CLEAR_BUF | FLAG_PRIVATE_LOCAL),
                  move |_aidl_reply| async move {
                    self.read_response_finish(_arg_input, _arg_signature, _arg_authToken, _arg_timestampToken, _arg_confirmationToken, _aidl_reply)
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
            impl IKeyMintOperation for binder::binder_impl::Binder<BnKeyMintOperation> {
              fn r#updateAad<'a, 'l1, 'l2, 'l3, >(&'a self, _arg_input: &'l1 [u8], _arg_authToken: Option<&'l2 crate::mangled::_7_android_8_hardware_8_security_7_keymint_17_HardwareAuthToken>, _arg_timeStampToken: Option<&'l3 crate::mangled::_7_android_8_hardware_8_security_11_secureclock_14_TimeStampToken>) -> binder::Result<()> { self.0.r#updateAad(_arg_input, _arg_authToken, _arg_timeStampToken) }
              fn r#update<'a, 'l1, 'l2, 'l3, >(&'a self, _arg_input: &'l1 [u8], _arg_authToken: Option<&'l2 crate::mangled::_7_android_8_hardware_8_security_7_keymint_17_HardwareAuthToken>, _arg_timeStampToken: Option<&'l3 crate::mangled::_7_android_8_hardware_8_security_11_secureclock_14_TimeStampToken>) -> binder::Result<Vec<u8>> { self.0.r#update(_arg_input, _arg_authToken, _arg_timeStampToken) }
              fn r#finish<'a, 'l1, 'l2, 'l3, 'l4, 'l5, >(&'a self, _arg_input: Option<&'l1 [u8]>, _arg_signature: Option<&'l2 [u8]>, _arg_authToken: Option<&'l3 crate::mangled::_7_android_8_hardware_8_security_7_keymint_17_HardwareAuthToken>, _arg_timestampToken: Option<&'l4 crate::mangled::_7_android_8_hardware_8_security_11_secureclock_14_TimeStampToken>, _arg_confirmationToken: Option<&'l5 [u8]>) -> binder::Result<Vec<u8>> { self.0.r#finish(_arg_input, _arg_signature, _arg_authToken, _arg_timestampToken, _arg_confirmationToken) }
              fn r#abort<'a, >(&'a self) -> binder::Result<()> { self.0.r#abort() }
              fn r#getInterfaceVersion<'a, >(&'a self) -> binder::Result<i32> { self.0.r#getInterfaceVersion() }
              fn r#getInterfaceHash<'a, >(&'a self) -> binder::Result<String> { self.0.r#getInterfaceHash() }
            }
            fn on_transact(_aidl_service: &dyn IKeyMintOperation, _aidl_code: binder::binder_impl::TransactionCode, _aidl_data: &binder::binder_impl::BorrowedParcel<'_>, _aidl_reply: &mut binder::binder_impl::BorrowedParcel<'_>) -> std::result::Result<(), binder::StatusCode> {
              match _aidl_code {
                transactions::r#updateAad => {
                  let _arg_input: Vec<u8> = _aidl_data.read()?;
                  let _arg_authToken: Option<crate::mangled::_7_android_8_hardware_8_security_7_keymint_17_HardwareAuthToken> = _aidl_data.read()?;
                  let _arg_timeStampToken: Option<crate::mangled::_7_android_8_hardware_8_security_11_secureclock_14_TimeStampToken> = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#updateAad(&_arg_input, _arg_authToken.as_ref(), _arg_timeStampToken.as_ref());
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
                  let _arg_authToken: Option<crate::mangled::_7_android_8_hardware_8_security_7_keymint_17_HardwareAuthToken> = _aidl_data.read()?;
                  let _arg_timeStampToken: Option<crate::mangled::_7_android_8_hardware_8_security_11_secureclock_14_TimeStampToken> = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#update(&_arg_input, _arg_authToken.as_ref(), _arg_timeStampToken.as_ref());
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
                  let _arg_authToken: Option<crate::mangled::_7_android_8_hardware_8_security_7_keymint_17_HardwareAuthToken> = _aidl_data.read()?;
                  let _arg_timestampToken: Option<crate::mangled::_7_android_8_hardware_8_security_11_secureclock_14_TimeStampToken> = _aidl_data.read()?;
                  let _arg_confirmationToken: Option<Vec<u8>> = _aidl_data.read()?;
                  let _aidl_return = _aidl_service.r#finish(_arg_input.as_deref(), _arg_signature.as_deref(), _arg_authToken.as_ref(), _arg_timestampToken.as_ref(), _arg_confirmationToken.as_deref());
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
             pub use super::r#IKeyMintOperation as _7_android_8_hardware_8_security_7_keymint_17_IKeyMintOperation;
            }
          }
          pub mod KeyCharacteristics {
            /*
             * This file is auto-generated.  DO NOT MODIFY.
             * Using: out/host/linux-x86/bin/aidl --lang=rust --structured --version 4 --hash a05c8079586139db45b0762a528cdd9745ad15ce -t --stability vintf --min_sdk_version 35 -pout/soong/.intermediates/hardware/interfaces/security/secureclock/aidl/android.hardware.security.secureclock_interface/1/preprocessed.aidl --ninja -d out/soong/.intermediates/hardware/interfaces/security/keymint/aidl/android.hardware.security.keymint-V4-rust-source/gen/android/hardware/security/keymint/KeyCharacteristics.rs.d -o out/soong/.intermediates/hardware/interfaces/security/keymint/aidl/android.hardware.security.keymint-V4-rust-source/gen -Nhardware/interfaces/security/keymint/aidl/aidl_api/android.hardware.security.keymint/4 hardware/interfaces/security/keymint/aidl/aidl_api/android.hardware.security.keymint/4/android/hardware/security/keymint/KeyCharacteristics.aidl
             *
             * DO NOT CHECK THIS FILE INTO A CODE TREE (e.g. git, etc..).
             * ALWAYS GENERATE THIS FILE FROM UPDATED AIDL COMPILER
             * AS A BUILD INTERMEDIATE ONLY. THIS IS NOT SOURCE CODE.
             */
            #![forbid(unsafe_code)]
            #![cfg_attr(rustfmt, rustfmt_skip)]
            #[derive(Debug)]
            pub struct r#KeyCharacteristics {
              pub r#securityLevel: crate::mangled::_7_android_8_hardware_8_security_7_keymint_13_SecurityLevel,
              pub r#authorizations: Vec<crate::mangled::_7_android_8_hardware_8_security_7_keymint_12_KeyParameter>,
            }
            impl Default for r#KeyCharacteristics {
              fn default() -> Self {
                Self {
                  r#securityLevel: crate::mangled::_7_android_8_hardware_8_security_7_keymint_13_SecurityLevel::SOFTWARE,
                  r#authorizations: Default::default(),
                }
              }
            }
            impl binder::Parcelable for r#KeyCharacteristics {
              fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_write(|subparcel| {
                  subparcel.write(&self.r#securityLevel)?;
                  subparcel.write(&self.r#authorizations)?;
                  Ok(())
                })
              }
              fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_read(|subparcel| {
                  if subparcel.has_more_data() {
                    self.r#securityLevel = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#authorizations = subparcel.read()?;
                  }
                  Ok(())
                })
              }
            }
            binder::impl_serialize_for_parcelable!(r#KeyCharacteristics);
            binder::impl_deserialize_for_parcelable!(r#KeyCharacteristics);
            impl binder::binder_impl::ParcelableMetadata for r#KeyCharacteristics {
              fn get_descriptor() -> &'static str { "android.hardware.security.keymint.KeyCharacteristics" }
              fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
            }
            pub(crate) mod mangled {
             pub use super::r#KeyCharacteristics as _7_android_8_hardware_8_security_7_keymint_18_KeyCharacteristics;
            }
          }
          pub mod KeyCreationResult {
            /*
             * This file is auto-generated.  DO NOT MODIFY.
             * Using: out/host/linux-x86/bin/aidl --lang=rust --structured --version 4 --hash a05c8079586139db45b0762a528cdd9745ad15ce -t --stability vintf --min_sdk_version 35 -pout/soong/.intermediates/hardware/interfaces/security/secureclock/aidl/android.hardware.security.secureclock_interface/1/preprocessed.aidl --ninja -d out/soong/.intermediates/hardware/interfaces/security/keymint/aidl/android.hardware.security.keymint-V4-rust-source/gen/android/hardware/security/keymint/KeyCreationResult.rs.d -o out/soong/.intermediates/hardware/interfaces/security/keymint/aidl/android.hardware.security.keymint-V4-rust-source/gen -Nhardware/interfaces/security/keymint/aidl/aidl_api/android.hardware.security.keymint/4 hardware/interfaces/security/keymint/aidl/aidl_api/android.hardware.security.keymint/4/android/hardware/security/keymint/KeyCreationResult.aidl
             *
             * DO NOT CHECK THIS FILE INTO A CODE TREE (e.g. git, etc..).
             * ALWAYS GENERATE THIS FILE FROM UPDATED AIDL COMPILER
             * AS A BUILD INTERMEDIATE ONLY. THIS IS NOT SOURCE CODE.
             */
            #![forbid(unsafe_code)]
            #![cfg_attr(rustfmt, rustfmt_skip)]
            #[derive(Debug)]
            pub struct r#KeyCreationResult {
              pub r#keyBlob: Vec<u8>,
              pub r#keyCharacteristics: Vec<crate::mangled::_7_android_8_hardware_8_security_7_keymint_18_KeyCharacteristics>,
              pub r#certificateChain: Vec<crate::mangled::_7_android_8_hardware_8_security_7_keymint_11_Certificate>,
            }
            impl Default for r#KeyCreationResult {
              fn default() -> Self {
                Self {
                  r#keyBlob: Default::default(),
                  r#keyCharacteristics: Default::default(),
                  r#certificateChain: Default::default(),
                }
              }
            }
            impl binder::Parcelable for r#KeyCreationResult {
              fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_write(|subparcel| {
                  subparcel.write(&self.r#keyBlob)?;
                  subparcel.write(&self.r#keyCharacteristics)?;
                  subparcel.write(&self.r#certificateChain)?;
                  Ok(())
                })
              }
              fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_read(|subparcel| {
                  if subparcel.has_more_data() {
                    self.r#keyBlob = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#keyCharacteristics = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#certificateChain = subparcel.read()?;
                  }
                  Ok(())
                })
              }
            }
            binder::impl_serialize_for_parcelable!(r#KeyCreationResult);
            binder::impl_deserialize_for_parcelable!(r#KeyCreationResult);
            impl binder::binder_impl::ParcelableMetadata for r#KeyCreationResult {
              fn get_descriptor() -> &'static str { "android.hardware.security.keymint.KeyCreationResult" }
              fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
            }
            pub(crate) mod mangled {
             pub use super::r#KeyCreationResult as _7_android_8_hardware_8_security_7_keymint_17_KeyCreationResult;
            }
          }
          pub mod KeyFormat {
            /*
             * This file is auto-generated.  DO NOT MODIFY.
             * Using: out/host/linux-x86/bin/aidl --lang=rust --structured --version 4 --hash a05c8079586139db45b0762a528cdd9745ad15ce -t --stability vintf --min_sdk_version 35 -pout/soong/.intermediates/hardware/interfaces/security/secureclock/aidl/android.hardware.security.secureclock_interface/1/preprocessed.aidl --ninja -d out/soong/.intermediates/hardware/interfaces/security/keymint/aidl/android.hardware.security.keymint-V4-rust-source/gen/android/hardware/security/keymint/KeyFormat.rs.d -o out/soong/.intermediates/hardware/interfaces/security/keymint/aidl/android.hardware.security.keymint-V4-rust-source/gen -Nhardware/interfaces/security/keymint/aidl/aidl_api/android.hardware.security.keymint/4 hardware/interfaces/security/keymint/aidl/aidl_api/android.hardware.security.keymint/4/android/hardware/security/keymint/KeyFormat.aidl
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
              r#KeyFormat : [i32; 3] {
                r#X509 = 0,
                r#PKCS8 = 1,
                r#RAW = 3,
              }
            }
            pub(crate) mod mangled {
             pub use super::r#KeyFormat as _7_android_8_hardware_8_security_7_keymint_9_KeyFormat;
            }
          }
          pub mod KeyMintHardwareInfo {
            /*
             * This file is auto-generated.  DO NOT MODIFY.
             * Using: out/host/linux-x86/bin/aidl --lang=rust --structured --version 4 --hash a05c8079586139db45b0762a528cdd9745ad15ce -t --stability vintf --min_sdk_version 35 -pout/soong/.intermediates/hardware/interfaces/security/secureclock/aidl/android.hardware.security.secureclock_interface/1/preprocessed.aidl --ninja -d out/soong/.intermediates/hardware/interfaces/security/keymint/aidl/android.hardware.security.keymint-V4-rust-source/gen/android/hardware/security/keymint/KeyMintHardwareInfo.rs.d -o out/soong/.intermediates/hardware/interfaces/security/keymint/aidl/android.hardware.security.keymint-V4-rust-source/gen -Nhardware/interfaces/security/keymint/aidl/aidl_api/android.hardware.security.keymint/4 hardware/interfaces/security/keymint/aidl/aidl_api/android.hardware.security.keymint/4/android/hardware/security/keymint/KeyMintHardwareInfo.aidl
             *
             * DO NOT CHECK THIS FILE INTO A CODE TREE (e.g. git, etc..).
             * ALWAYS GENERATE THIS FILE FROM UPDATED AIDL COMPILER
             * AS A BUILD INTERMEDIATE ONLY. THIS IS NOT SOURCE CODE.
             */
            #![forbid(unsafe_code)]
            #![cfg_attr(rustfmt, rustfmt_skip)]
            #[derive(Debug, Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
            pub struct r#KeyMintHardwareInfo {
              pub r#versionNumber: i32,
              pub r#securityLevel: crate::mangled::_7_android_8_hardware_8_security_7_keymint_13_SecurityLevel,
              pub r#keyMintName: String,
              pub r#keyMintAuthorName: String,
              pub r#timestampTokenRequired: bool,
            }
            impl Default for r#KeyMintHardwareInfo {
              fn default() -> Self {
                Self {
                  r#versionNumber: 0,
                  r#securityLevel: crate::mangled::_7_android_8_hardware_8_security_7_keymint_13_SecurityLevel::SOFTWARE,
                  r#keyMintName: Default::default(),
                  r#keyMintAuthorName: Default::default(),
                  r#timestampTokenRequired: false,
                }
              }
            }
            impl binder::Parcelable for r#KeyMintHardwareInfo {
              fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_write(|subparcel| {
                  subparcel.write(&self.r#versionNumber)?;
                  subparcel.write(&self.r#securityLevel)?;
                  subparcel.write(&self.r#keyMintName)?;
                  subparcel.write(&self.r#keyMintAuthorName)?;
                  subparcel.write(&self.r#timestampTokenRequired)?;
                  Ok(())
                })
              }
              fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_read(|subparcel| {
                  if subparcel.has_more_data() {
                    self.r#versionNumber = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#securityLevel = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#keyMintName = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#keyMintAuthorName = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#timestampTokenRequired = subparcel.read()?;
                  }
                  Ok(())
                })
              }
            }
            binder::impl_serialize_for_parcelable!(r#KeyMintHardwareInfo);
            binder::impl_deserialize_for_parcelable!(r#KeyMintHardwareInfo);
            impl binder::binder_impl::ParcelableMetadata for r#KeyMintHardwareInfo {
              fn get_descriptor() -> &'static str { "android.hardware.security.keymint.KeyMintHardwareInfo" }
              fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
            }
            pub(crate) mod mangled {
             pub use super::r#KeyMintHardwareInfo as _7_android_8_hardware_8_security_7_keymint_19_KeyMintHardwareInfo;
            }
          }
          pub mod KeyOrigin {
            /*
             * This file is auto-generated.  DO NOT MODIFY.
             * Using: out/host/linux-x86/bin/aidl --lang=rust --structured --version 4 --hash a05c8079586139db45b0762a528cdd9745ad15ce -t --stability vintf --min_sdk_version 35 -pout/soong/.intermediates/hardware/interfaces/security/secureclock/aidl/android.hardware.security.secureclock_interface/1/preprocessed.aidl --ninja -d out/soong/.intermediates/hardware/interfaces/security/keymint/aidl/android.hardware.security.keymint-V4-rust-source/gen/android/hardware/security/keymint/KeyOrigin.rs.d -o out/soong/.intermediates/hardware/interfaces/security/keymint/aidl/android.hardware.security.keymint-V4-rust-source/gen -Nhardware/interfaces/security/keymint/aidl/aidl_api/android.hardware.security.keymint/4 hardware/interfaces/security/keymint/aidl/aidl_api/android.hardware.security.keymint/4/android/hardware/security/keymint/KeyOrigin.aidl
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
              r#KeyOrigin : [i32; 5] {
                r#GENERATED = 0,
                r#DERIVED = 1,
                r#IMPORTED = 2,
                r#RESERVED = 3,
                r#SECURELY_IMPORTED = 4,
              }
            }
            pub(crate) mod mangled {
             pub use super::r#KeyOrigin as _7_android_8_hardware_8_security_7_keymint_9_KeyOrigin;
            }
          }
          pub mod KeyParameter {
            /*
             * This file is auto-generated.  DO NOT MODIFY.
             * Using: out/host/linux-x86/bin/aidl --lang=rust --structured --version 4 --hash a05c8079586139db45b0762a528cdd9745ad15ce -t --stability vintf --min_sdk_version 35 -pout/soong/.intermediates/hardware/interfaces/security/secureclock/aidl/android.hardware.security.secureclock_interface/1/preprocessed.aidl --ninja -d out/soong/.intermediates/hardware/interfaces/security/keymint/aidl/android.hardware.security.keymint-V4-rust-source/gen/android/hardware/security/keymint/KeyParameter.rs.d -o out/soong/.intermediates/hardware/interfaces/security/keymint/aidl/android.hardware.security.keymint-V4-rust-source/gen -Nhardware/interfaces/security/keymint/aidl/aidl_api/android.hardware.security.keymint/4 hardware/interfaces/security/keymint/aidl/aidl_api/android.hardware.security.keymint/4/android/hardware/security/keymint/KeyParameter.aidl
             *
             * DO NOT CHECK THIS FILE INTO A CODE TREE (e.g. git, etc..).
             * ALWAYS GENERATE THIS FILE FROM UPDATED AIDL COMPILER
             * AS A BUILD INTERMEDIATE ONLY. THIS IS NOT SOURCE CODE.
             */
            #![forbid(unsafe_code)]
            #![cfg_attr(rustfmt, rustfmt_skip)]
            #[derive(Debug, Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
            pub struct r#KeyParameter {
              pub r#tag: crate::mangled::_7_android_8_hardware_8_security_7_keymint_3_Tag,
              pub r#value: crate::mangled::_7_android_8_hardware_8_security_7_keymint_17_KeyParameterValue,
            }
            impl Default for r#KeyParameter {
              fn default() -> Self {
                Self {
                  r#tag: crate::mangled::_7_android_8_hardware_8_security_7_keymint_3_Tag::INVALID,
                  r#value: Default::default(),
                }
              }
            }
            impl binder::Parcelable for r#KeyParameter {
              fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_write(|subparcel| {
                  subparcel.write(&self.r#tag)?;
                  subparcel.write(&self.r#value)?;
                  Ok(())
                })
              }
              fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                parcel.sized_read(|subparcel| {
                  if subparcel.has_more_data() {
                    self.r#tag = subparcel.read()?;
                  }
                  if subparcel.has_more_data() {
                    self.r#value = subparcel.read()?;
                  }
                  Ok(())
                })
              }
            }
            binder::impl_serialize_for_parcelable!(r#KeyParameter);
            binder::impl_deserialize_for_parcelable!(r#KeyParameter);
            impl binder::binder_impl::ParcelableMetadata for r#KeyParameter {
              fn get_descriptor() -> &'static str { "android.hardware.security.keymint.KeyParameter" }
              fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
            }
            pub(crate) mod mangled {
             pub use super::r#KeyParameter as _7_android_8_hardware_8_security_7_keymint_12_KeyParameter;
            }
          }
          pub mod KeyParameterValue {
            /*
             * This file is auto-generated.  DO NOT MODIFY.
             * Using: out/host/linux-x86/bin/aidl --lang=rust --structured --version 4 --hash a05c8079586139db45b0762a528cdd9745ad15ce -t --stability vintf --min_sdk_version 35 -pout/soong/.intermediates/hardware/interfaces/security/secureclock/aidl/android.hardware.security.secureclock_interface/1/preprocessed.aidl --ninja -d out/soong/.intermediates/hardware/interfaces/security/keymint/aidl/android.hardware.security.keymint-V4-rust-source/gen/android/hardware/security/keymint/KeyParameterValue.rs.d -o out/soong/.intermediates/hardware/interfaces/security/keymint/aidl/android.hardware.security.keymint-V4-rust-source/gen -Nhardware/interfaces/security/keymint/aidl/aidl_api/android.hardware.security.keymint/4 hardware/interfaces/security/keymint/aidl/aidl_api/android.hardware.security.keymint/4/android/hardware/security/keymint/KeyParameterValue.aidl
             *
             * DO NOT CHECK THIS FILE INTO A CODE TREE (e.g. git, etc..).
             * ALWAYS GENERATE THIS FILE FROM UPDATED AIDL COMPILER
             * AS A BUILD INTERMEDIATE ONLY. THIS IS NOT SOURCE CODE.
             */
            #![forbid(unsafe_code)]
            #![cfg_attr(rustfmt, rustfmt_skip)]
            #[derive(Debug, Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
            pub enum r#KeyParameterValue {
              Invalid(i32),
              Algorithm(crate::mangled::_7_android_8_hardware_8_security_7_keymint_9_Algorithm),
              BlockMode(crate::mangled::_7_android_8_hardware_8_security_7_keymint_9_BlockMode),
              PaddingMode(crate::mangled::_7_android_8_hardware_8_security_7_keymint_11_PaddingMode),
              Digest(crate::mangled::_7_android_8_hardware_8_security_7_keymint_6_Digest),
              EcCurve(crate::mangled::_7_android_8_hardware_8_security_7_keymint_7_EcCurve),
              Origin(crate::mangled::_7_android_8_hardware_8_security_7_keymint_9_KeyOrigin),
              KeyPurpose(crate::mangled::_7_android_8_hardware_8_security_7_keymint_10_KeyPurpose),
              HardwareAuthenticatorType(crate::mangled::_7_android_8_hardware_8_security_7_keymint_25_HardwareAuthenticatorType),
              SecurityLevel(crate::mangled::_7_android_8_hardware_8_security_7_keymint_13_SecurityLevel),
              BoolValue(bool),
              Integer(i32),
              LongInteger(i64),
              DateTime(i64),
              Blob(Vec<u8>),
            }
            impl Default for r#KeyParameterValue {
              fn default() -> Self {
                Self::Invalid(0)
              }
            }
            impl binder::Parcelable for r#KeyParameterValue {
              fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                match self {
                  Self::Invalid(v) => {
                    parcel.write(&0i32)?;
                    parcel.write(v)
                  }
                  Self::Algorithm(v) => {
                    parcel.write(&1i32)?;
                    parcel.write(v)
                  }
                  Self::BlockMode(v) => {
                    parcel.write(&2i32)?;
                    parcel.write(v)
                  }
                  Self::PaddingMode(v) => {
                    parcel.write(&3i32)?;
                    parcel.write(v)
                  }
                  Self::Digest(v) => {
                    parcel.write(&4i32)?;
                    parcel.write(v)
                  }
                  Self::EcCurve(v) => {
                    parcel.write(&5i32)?;
                    parcel.write(v)
                  }
                  Self::Origin(v) => {
                    parcel.write(&6i32)?;
                    parcel.write(v)
                  }
                  Self::KeyPurpose(v) => {
                    parcel.write(&7i32)?;
                    parcel.write(v)
                  }
                  Self::HardwareAuthenticatorType(v) => {
                    parcel.write(&8i32)?;
                    parcel.write(v)
                  }
                  Self::SecurityLevel(v) => {
                    parcel.write(&9i32)?;
                    parcel.write(v)
                  }
                  Self::BoolValue(v) => {
                    parcel.write(&10i32)?;
                    parcel.write(v)
                  }
                  Self::Integer(v) => {
                    parcel.write(&11i32)?;
                    parcel.write(v)
                  }
                  Self::LongInteger(v) => {
                    parcel.write(&12i32)?;
                    parcel.write(v)
                  }
                  Self::DateTime(v) => {
                    parcel.write(&13i32)?;
                    parcel.write(v)
                  }
                  Self::Blob(v) => {
                    parcel.write(&14i32)?;
                    parcel.write(v)
                  }
                }
              }
              fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
                let tag: i32 = parcel.read()?;
                match tag {
                  0 => {
                    let value: i32 = parcel.read()?;
                    *self = Self::Invalid(value);
                    Ok(())
                  }
                  1 => {
                    let value: crate::mangled::_7_android_8_hardware_8_security_7_keymint_9_Algorithm = parcel.read()?;
                    *self = Self::Algorithm(value);
                    Ok(())
                  }
                  2 => {
                    let value: crate::mangled::_7_android_8_hardware_8_security_7_keymint_9_BlockMode = parcel.read()?;
                    *self = Self::BlockMode(value);
                    Ok(())
                  }
                  3 => {
                    let value: crate::mangled::_7_android_8_hardware_8_security_7_keymint_11_PaddingMode = parcel.read()?;
                    *self = Self::PaddingMode(value);
                    Ok(())
                  }
                  4 => {
                    let value: crate::mangled::_7_android_8_hardware_8_security_7_keymint_6_Digest = parcel.read()?;
                    *self = Self::Digest(value);
                    Ok(())
                  }
                  5 => {
                    let value: crate::mangled::_7_android_8_hardware_8_security_7_keymint_7_EcCurve = parcel.read()?;
                    *self = Self::EcCurve(value);
                    Ok(())
                  }
                  6 => {
                    let value: crate::mangled::_7_android_8_hardware_8_security_7_keymint_9_KeyOrigin = parcel.read()?;
                    *self = Self::Origin(value);
                    Ok(())
                  }
                  7 => {
                    let value: crate::mangled::_7_android_8_hardware_8_security_7_keymint_10_KeyPurpose = parcel.read()?;
                    *self = Self::KeyPurpose(value);
                    Ok(())
                  }
                  8 => {
                    let value: crate::mangled::_7_android_8_hardware_8_security_7_keymint_25_HardwareAuthenticatorType = parcel.read()?;
                    *self = Self::HardwareAuthenticatorType(value);
                    Ok(())
                  }
                  9 => {
                    let value: crate::mangled::_7_android_8_hardware_8_security_7_keymint_13_SecurityLevel = parcel.read()?;
                    *self = Self::SecurityLevel(value);
                    Ok(())
                  }
                  10 => {
                    let value: bool = parcel.read()?;
                    *self = Self::BoolValue(value);
                    Ok(())
                  }
                  11 => {
                    let value: i32 = parcel.read()?;
                    *self = Self::Integer(value);
                    Ok(())
                  }
                  12 => {
                    let value: i64 = parcel.read()?;
                    *self = Self::LongInteger(value);
                    Ok(())
                  }
                  13 => {
                    let value: i64 = parcel.read()?;
                    *self = Self::DateTime(value);
                    Ok(())
                  }
                  14 => {
                    let value: Vec<u8> = parcel.read()?;
                    *self = Self::Blob(value);
                    Ok(())
                  }
                  _ => {
                    Err(binder::StatusCode::BAD_VALUE)
                  }
                }
              }
            }
            binder::impl_serialize_for_parcelable!(r#KeyParameterValue);
            binder::impl_deserialize_for_parcelable!(r#KeyParameterValue);
            impl binder::binder_impl::ParcelableMetadata for r#KeyParameterValue {
              fn get_descriptor() -> &'static str { "android.hardware.security.keymint.KeyParameterValue" }
              fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
            }
            pub mod r#Tag {
              #![allow(non_upper_case_globals)]
              use binder::declare_binder_enum;
              declare_binder_enum! {
                #[repr(C, align(4))]
                r#Tag : [i32; 15] {
                  r#invalid = 0,
                  r#algorithm = 1,
                  r#blockMode = 2,
                  r#paddingMode = 3,
                  r#digest = 4,
                  r#ecCurve = 5,
                  r#origin = 6,
                  r#keyPurpose = 7,
                  r#hardwareAuthenticatorType = 8,
                  r#securityLevel = 9,
                  r#boolValue = 10,
                  r#integer = 11,
                  r#longInteger = 12,
                  r#dateTime = 13,
                  r#blob = 14,
                }
              }
            }
            pub(crate) mod mangled {
             pub use super::r#KeyParameterValue as _7_android_8_hardware_8_security_7_keymint_17_KeyParameterValue;
             pub use super::r#Tag::r#Tag as _7_android_8_hardware_8_security_7_keymint_17_KeyParameterValue_3_Tag;
            }
          }
          pub mod KeyPurpose {
            /*
             * This file is auto-generated.  DO NOT MODIFY.
             * Using: out/host/linux-x86/bin/aidl --lang=rust --structured --version 4 --hash a05c8079586139db45b0762a528cdd9745ad15ce -t --stability vintf --min_sdk_version 35 -pout/soong/.intermediates/hardware/interfaces/security/secureclock/aidl/android.hardware.security.secureclock_interface/1/preprocessed.aidl --ninja -d out/soong/.intermediates/hardware/interfaces/security/keymint/aidl/android.hardware.security.keymint-V4-rust-source/gen/android/hardware/security/keymint/KeyPurpose.rs.d -o out/soong/.intermediates/hardware/interfaces/security/keymint/aidl/android.hardware.security.keymint-V4-rust-source/gen -Nhardware/interfaces/security/keymint/aidl/aidl_api/android.hardware.security.keymint/4 hardware/interfaces/security/keymint/aidl/aidl_api/android.hardware.security.keymint/4/android/hardware/security/keymint/KeyPurpose.aidl
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
              r#KeyPurpose : [i32; 7] {
                r#ENCRYPT = 0,
                r#DECRYPT = 1,
                r#SIGN = 2,
                r#VERIFY = 3,
                r#WRAP_KEY = 5,
                r#AGREE_KEY = 6,
                r#ATTEST_KEY = 7,
              }
            }
            pub(crate) mod mangled {
             pub use super::r#KeyPurpose as _7_android_8_hardware_8_security_7_keymint_10_KeyPurpose;
            }
          }
          pub mod PaddingMode {
            /*
             * This file is auto-generated.  DO NOT MODIFY.
             * Using: out/host/linux-x86/bin/aidl --lang=rust --structured --version 4 --hash a05c8079586139db45b0762a528cdd9745ad15ce -t --stability vintf --min_sdk_version 35 -pout/soong/.intermediates/hardware/interfaces/security/secureclock/aidl/android.hardware.security.secureclock_interface/1/preprocessed.aidl --ninja -d out/soong/.intermediates/hardware/interfaces/security/keymint/aidl/android.hardware.security.keymint-V4-rust-source/gen/android/hardware/security/keymint/PaddingMode.rs.d -o out/soong/.intermediates/hardware/interfaces/security/keymint/aidl/android.hardware.security.keymint-V4-rust-source/gen -Nhardware/interfaces/security/keymint/aidl/aidl_api/android.hardware.security.keymint/4 hardware/interfaces/security/keymint/aidl/aidl_api/android.hardware.security.keymint/4/android/hardware/security/keymint/PaddingMode.aidl
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
              r#PaddingMode : [i32; 6] {
                r#NONE = 1,
                r#RSA_OAEP = 2,
                r#RSA_PSS = 3,
                r#RSA_PKCS1_1_5_ENCRYPT = 4,
                r#RSA_PKCS1_1_5_SIGN = 5,
                r#PKCS7 = 64,
              }
            }
            pub(crate) mod mangled {
             pub use super::r#PaddingMode as _7_android_8_hardware_8_security_7_keymint_11_PaddingMode;
            }
          }
          pub mod SecurityLevel {
            /*
             * This file is auto-generated.  DO NOT MODIFY.
             * Using: out/host/linux-x86/bin/aidl --lang=rust --structured --version 4 --hash a05c8079586139db45b0762a528cdd9745ad15ce -t --stability vintf --min_sdk_version 35 -pout/soong/.intermediates/hardware/interfaces/security/secureclock/aidl/android.hardware.security.secureclock_interface/1/preprocessed.aidl --ninja -d out/soong/.intermediates/hardware/interfaces/security/keymint/aidl/android.hardware.security.keymint-V4-rust-source/gen/android/hardware/security/keymint/SecurityLevel.rs.d -o out/soong/.intermediates/hardware/interfaces/security/keymint/aidl/android.hardware.security.keymint-V4-rust-source/gen -Nhardware/interfaces/security/keymint/aidl/aidl_api/android.hardware.security.keymint/4 hardware/interfaces/security/keymint/aidl/aidl_api/android.hardware.security.keymint/4/android/hardware/security/keymint/SecurityLevel.aidl
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
              r#SecurityLevel : [i32; 4] {
                r#SOFTWARE = 0,
                r#TRUSTED_ENVIRONMENT = 1,
                r#STRONGBOX = 2,
                r#KEYSTORE = 100,
              }
            }
            pub(crate) mod mangled {
             pub use super::r#SecurityLevel as _7_android_8_hardware_8_security_7_keymint_13_SecurityLevel;
            }
          }
          pub mod Tag {
            /*
             * This file is auto-generated.  DO NOT MODIFY.
             * Using: out/host/linux-x86/bin/aidl --lang=rust --structured --version 4 --hash a05c8079586139db45b0762a528cdd9745ad15ce -t --stability vintf --min_sdk_version 35 -pout/soong/.intermediates/hardware/interfaces/security/secureclock/aidl/android.hardware.security.secureclock_interface/1/preprocessed.aidl --ninja -d out/soong/.intermediates/hardware/interfaces/security/keymint/aidl/android.hardware.security.keymint-V4-rust-source/gen/android/hardware/security/keymint/Tag.rs.d -o out/soong/.intermediates/hardware/interfaces/security/keymint/aidl/android.hardware.security.keymint-V4-rust-source/gen -Nhardware/interfaces/security/keymint/aidl/aidl_api/android.hardware.security.keymint/4 hardware/interfaces/security/keymint/aidl/aidl_api/android.hardware.security.keymint/4/android/hardware/security/keymint/Tag.aidl
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
              r#Tag : [i32; 67] {
                r#INVALID = 0,
                r#PURPOSE = 536870913,
                r#ALGORITHM = 268435458,
                r#KEY_SIZE = 805306371,
                r#BLOCK_MODE = 536870916,
                r#DIGEST = 536870917,
                r#PADDING = 536870918,
                r#CALLER_NONCE = 1879048199,
                r#MIN_MAC_LENGTH = 805306376,
                r#EC_CURVE = 268435466,
                r#RSA_PUBLIC_EXPONENT = 1342177480,
                r#INCLUDE_UNIQUE_ID = 1879048394,
                r#RSA_OAEP_MGF_DIGEST = 536871115,
                r#BOOTLOADER_ONLY = 1879048494,
                r#ROLLBACK_RESISTANCE = 1879048495,
                r#HARDWARE_TYPE = 268435760,
                r#EARLY_BOOT_ONLY = 1879048497,
                r#ACTIVE_DATETIME = 1610613136,
                r#ORIGINATION_EXPIRE_DATETIME = 1610613137,
                r#USAGE_EXPIRE_DATETIME = 1610613138,
                r#MIN_SECONDS_BETWEEN_OPS = 805306771,
                r#MAX_USES_PER_BOOT = 805306772,
                r#USAGE_COUNT_LIMIT = 805306773,
                r#USER_ID = 805306869,
                r#USER_SECURE_ID = -1610612234,
                r#NO_AUTH_REQUIRED = 1879048695,
                r#USER_AUTH_TYPE = 268435960,
                r#AUTH_TIMEOUT = 805306873,
                r#ALLOW_WHILE_ON_BODY = 1879048698,
                r#TRUSTED_USER_PRESENCE_REQUIRED = 1879048699,
                r#TRUSTED_CONFIRMATION_REQUIRED = 1879048700,
                r#UNLOCKED_DEVICE_REQUIRED = 1879048701,
                r#APPLICATION_ID = -1879047591,
                r#APPLICATION_DATA = -1879047492,
                r#CREATION_DATETIME = 1610613437,
                r#ORIGIN = 268436158,
                r#ROOT_OF_TRUST = -1879047488,
                r#OS_VERSION = 805307073,
                r#OS_PATCHLEVEL = 805307074,
                r#UNIQUE_ID = -1879047485,
                r#ATTESTATION_CHALLENGE = -1879047484,
                r#ATTESTATION_APPLICATION_ID = -1879047483,
                r#ATTESTATION_ID_BRAND = -1879047482,
                r#ATTESTATION_ID_DEVICE = -1879047481,
                r#ATTESTATION_ID_PRODUCT = -1879047480,
                r#ATTESTATION_ID_SERIAL = -1879047479,
                r#ATTESTATION_ID_IMEI = -1879047478,
                r#ATTESTATION_ID_MEID = -1879047477,
                r#ATTESTATION_ID_MANUFACTURER = -1879047476,
                r#ATTESTATION_ID_MODEL = -1879047475,
                r#VENDOR_PATCHLEVEL = 805307086,
                r#BOOT_PATCHLEVEL = 805307087,
                r#DEVICE_UNIQUE_ATTESTATION = 1879048912,
                r#IDENTITY_CREDENTIAL_KEY = 1879048913,
                r#STORAGE_KEY = 1879048914,
                r#ATTESTATION_ID_SECOND_IMEI = -1879047469,
                r#MODULE_HASH = -1879047468,
                r#ASSOCIATED_DATA = -1879047192,
                r#NONCE = -1879047191,
                r#MAC_LENGTH = 805307371,
                r#RESET_SINCE_ID_ROTATION = 1879049196,
                r#CONFIRMATION_TOKEN = -1879047187,
                r#CERTIFICATE_SERIAL = -2147482642,
                r#CERTIFICATE_SUBJECT = -1879047185,
                r#CERTIFICATE_NOT_BEFORE = 1610613744,
                r#CERTIFICATE_NOT_AFTER = 1610613745,
                r#MAX_BOOT_LEVEL = 805307378,
              }
            }
            pub(crate) mod mangled {
             pub use super::r#Tag as _7_android_8_hardware_8_security_7_keymint_3_Tag;
            }
          }
          pub mod TagType {
            /*
             * This file is auto-generated.  DO NOT MODIFY.
             * Using: out/host/linux-x86/bin/aidl --lang=rust --structured --version 4 --hash a05c8079586139db45b0762a528cdd9745ad15ce -t --stability vintf --min_sdk_version 35 -pout/soong/.intermediates/hardware/interfaces/security/secureclock/aidl/android.hardware.security.secureclock_interface/1/preprocessed.aidl --ninja -d out/soong/.intermediates/hardware/interfaces/security/keymint/aidl/android.hardware.security.keymint-V4-rust-source/gen/android/hardware/security/keymint/TagType.rs.d -o out/soong/.intermediates/hardware/interfaces/security/keymint/aidl/android.hardware.security.keymint-V4-rust-source/gen -Nhardware/interfaces/security/keymint/aidl/aidl_api/android.hardware.security.keymint/4 hardware/interfaces/security/keymint/aidl/aidl_api/android.hardware.security.keymint/4/android/hardware/security/keymint/TagType.aidl
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
              r#TagType : [i32; 11] {
                r#INVALID = 0,
                r#ENUM = 268435456,
                r#ENUM_REP = 536870912,
                r#UINT = 805306368,
                r#UINT_REP = 1073741824,
                r#ULONG = 1342177280,
                r#DATE = 1610612736,
                r#BOOL = 1879048192,
                r#BIGNUM = -2147483648,
                r#BYTES = -1879048192,
                r#ULONG_REP = -1610612736,
              }
            }
            pub(crate) mod mangled {
             pub use super::r#TagType as _7_android_8_hardware_8_security_7_keymint_7_TagType;
            }
          }
        }
      }
    }
  }
}
pub mod mangled {
  pub use super::aidl::android::hardware::security::keymint::Algorithm::mangled::*;
  pub use super::aidl::android::hardware::security::keymint::AttestationKey::mangled::*;
  pub use super::aidl::android::hardware::security::keymint::BeginResult::mangled::*;
  pub use super::aidl::android::hardware::security::keymint::BlockMode::mangled::*;
  pub use super::aidl::android::hardware::security::keymint::Certificate::mangled::*;
  pub use super::aidl::android::hardware::security::keymint::Digest::mangled::*;
  pub use super::aidl::android::hardware::security::keymint::EcCurve::mangled::*;
  pub use super::aidl::android::hardware::security::keymint::ErrorCode::mangled::*;
  pub use super::aidl::android::hardware::security::keymint::HardwareAuthToken::mangled::*;
  pub use super::aidl::android::hardware::security::keymint::HardwareAuthenticatorType::mangled::*;
  pub use super::aidl::android::hardware::security::keymint::IKeyMintDevice::mangled::*;
  pub use super::aidl::android::hardware::security::keymint::IKeyMintOperation::mangled::*;
  pub use super::aidl::android::hardware::security::keymint::KeyCharacteristics::mangled::*;
  pub use super::aidl::android::hardware::security::keymint::KeyCreationResult::mangled::*;
  pub use super::aidl::android::hardware::security::keymint::KeyFormat::mangled::*;
  pub use super::aidl::android::hardware::security::keymint::KeyMintHardwareInfo::mangled::*;
  pub use super::aidl::android::hardware::security::keymint::KeyOrigin::mangled::*;
  pub use super::aidl::android::hardware::security::keymint::KeyParameter::mangled::*;
  pub use super::aidl::android::hardware::security::keymint::KeyParameterValue::mangled::*;
  pub use super::aidl::android::hardware::security::keymint::KeyPurpose::mangled::*;
  pub use super::aidl::android::hardware::security::keymint::PaddingMode::mangled::*;
  pub use super::aidl::android::hardware::security::keymint::SecurityLevel::mangled::*;
  pub use super::aidl::android::hardware::security::keymint::Tag::mangled::*;
  pub use super::aidl::android::hardware::security::keymint::TagType::mangled::*;
  pub(crate) use crate::android_hardware_security_secureclock::mangled::*;
}

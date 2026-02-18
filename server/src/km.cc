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
#include "ka-server/include/km.h"
#include "ka-server/src/main.rs.h"
#include "rust/cxx.h"

#include <aidl/android/hardware/security/keymint/ErrorCode.h>
#include <aidl/android/hardware/security/keymint/KeyParameterValue.h>
#include <aidl/android/hardware/security/keymint/Tag.h>
#include <aidl/android/system/keystore2/Authorization.h>
#include <aidl/android/system/keystore2/KeyDescriptor.h>
#include <aidl/android/system/keystore2/KeyMetadata.h>
#include <aidl/android/system/keystore2/KeyParameters.h>
#include <android/hardware/keymaster/4.0/types.h>
#include <android/hidl/manager/1.2/IServiceManager.h>
#include <keymasterV4_1/Keymaster4.h>
#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wunused-function"
#pragma clang diagnostic ignored "-Wmissing-field-initializers"
#include "km_compat_type_conversion.h"
#pragma clang diagnostic pop

#include <android-base/logging.h>
#include <binder/IServiceManager.h>

#include <android/binder_parcel.h>
#include <android/binder_parcel_utils.h>
#include <android/binder_status.h>

#include <chrono>
#include <stdio.h>
#include <string>

using ::android::hardware::hidl_vec;
namespace keymasterNs = ::android::hardware::keymaster::V4_1;
using ::android::sp;
using keymasterNs::SecurityLevel;
using keymasterNs::support::Keymaster;

template <typename T, size_t count>
class Devices : public std::array<T, count> {
public:
  T &operator[](SecurityLevel secLevel) {
    static_assert(uint32_t(SecurityLevel::SOFTWARE) == 0 &&
                      uint32_t(SecurityLevel::TRUSTED_ENVIRONMENT) == 1 &&
                      uint32_t(SecurityLevel::STRONGBOX) == 2,
                  "Numeric values of security levels have changed");
    return std::array<T, count>::at(static_cast<uint32_t>(secLevel));
  }
  T operator[](SecurityLevel secLevel) const {
    if (static_cast<uint32_t>(secLevel) >
        static_cast<uint32_t>(SecurityLevel::STRONGBOX)) {
      LOG(ERROR) << "Invalid security level requested";
      return {};
    }
    return (*const_cast<Devices *>(this))[secLevel];
  }
};

// hexdump: dump hex value and character notations.
void hexdump(const void *data, size_t size) {
  const unsigned char *p = static_cast<const unsigned char *>(data);
  for (size_t i = 0; i < size; i += 16) {
    fprintf(stderr, "%08zx  ", i);
    for (size_t j = 0; j < 16; j++) {
      if (i + j < size)
        fprintf(stderr, "%02x ", p[i + j]);
      else
        fprintf(stderr, "   ");
    }
    fprintf(stderr, " |");
    for (size_t j = 0; j < 16; j++) {
      if (i + j < size) {
        unsigned char c = p[i + j];
        fprintf(stderr, "%c", (c >= 32 && c <= 126) ? c : '.');
      }
    }
    fprintf(stderr, "|\n");
  }
}

using KeymasterDevices = Devices<sp<Keymaster>, 3>;
using ::android::hidl::manager::V1_2::IServiceManager;
// using keymasterNs::support::Keymaster3;
using keymasterNs::support::Keymaster4;

using ::android::hardware::hidl_string;
const char *serviceName = "android.hardware.keymaster@4.0::IKeymasterDevice";
const char *serviceNameInstance =
    "android.hardware.keymaster@4.0::IKeymasterDevice/default";

template <typename Wrapper>
KeymasterDevices enumerateKeymasterDevices(IServiceManager *serviceManager) {
  KeymasterDevices result;
  serviceManager->listManifestByInterface(
      serviceName, [&](const hidl_vec<hidl_string> &names) {
        auto try_get_device = [&](const auto &name, bool fail_silent) {
          auto device = Wrapper::WrappedIKeymasterDevice::getService(name);
          if (fail_silent && !device)
            return;
          CHECK(device) << "Failed to get service for \""
                        << Wrapper::WrappedIKeymasterDevice::descriptor
                        << "\" with interface name \"" << name << "\"";

          sp<Keymaster> kmDevice(new Wrapper(device, name));
          auto halVersion = kmDevice->halVersion();
          SecurityLevel securityLevel = halVersion.securityLevel;
          std::cerr << "found " << Wrapper::WrappedIKeymasterDevice::descriptor
                    << " with interface name " << name << " and seclevel "
                    << toString(securityLevel);
          std::cerr << "\n";
          CHECK(static_cast<uint32_t>(securityLevel) < result.size())
              << "Security level of \""
              << Wrapper::WrappedIKeymasterDevice::descriptor
              << "\" with interface name \"" << name << "\" out of range";
          auto &deviceSlot = result[securityLevel];
          if (deviceSlot) {
            if (!fail_silent) {
              LOG(WARNING) << "Implementation of \""
                           << Wrapper::WrappedIKeymasterDevice::descriptor
                           << "\" with interface name \"" << name
                           << "\" and security level: "
                           << toString(securityLevel)
                           << " Masked by other implementation of Keymaster";
            }
          } else {
            deviceSlot = kmDevice;
          }
        };
        // bool has_default = false;
        // for (auto& n : names) {
        //     try_get_device(n, false);
        //     if (n == "default") has_default = true;
        // }
        //// Make sure that we always check the default device. If we enumerate
        /// only what is / known to hwservicemanager, we miss a possible
        /// passthrough HAL.
        // if (!has_default) {
        //     try_get_device("default", true /* fail_silent */);
        // }
        for (auto &n : names) {
          try_get_device(n, false);
        }
      });
  return result;
}

void keymaster_test(bool save_cert, const rust::String &cert_prefix) {
  auto serviceManager = IServiceManager::getService();
  if (!serviceManager.get()) {
    // New devices no longer have HIDL support, so failing to get
    // hwservicemanager is expected behavior.
    LOG(INFO) << "Skipping keymaster compat, this system is AIDL only.";
    return;
  }
  auto device = enumerateKeymasterDevices<Keymaster4>(serviceManager.get());
  auto d = device[SecurityLevel::TRUSTED_ENVIRONMENT];
  if (!d) {
    fprintf(stderr, "Failed to get TEE device\n");
    return;
  }
  auto v = d->halVersion();
  fprintf(stderr, "Keymaster %d.%d SecurityLevel: %d supportsEc: %d\n",
          v.majorVersion, v.minorVersion, v.securityLevel, v.supportsEc);

  d->getHardwareInfo(
      [](::android::hardware::keymaster::V4_0::SecurityLevel securityLevel,
         const ::android::hardware::hidl_string &keymasterName,
         const ::android::hardware::hidl_string &keymasterAuthorName) {
        fprintf(stderr, "Keymaster Author: %s\n", keymasterAuthorName.c_str());
      });

  std::vector<android::hardware::keymaster::V4_1::KeyParameter> keyParams;
  android::hardware::keymaster::V4_0::KeyParameter kp;

  char appId[] = {0x30, 0x36, 0x31, 0x10, 0x30, 0x0e, 0x04, 0x09, 0x74, 0x65,
                  0x73, 0x74, 0x61, 0x70, 0x70, 0x69, 0x64, 0x02, 0x01, 0x0b,
                  0x31, 0x22, 0x04, 0x20, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05,
                  0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f,
                  0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19,
                  0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f};
  bool rsa = false;
  if (rsa) {
    kp.tag = android::hardware::keymaster::V4_0::Tag::ALGORITHM;
    kp.f.algorithm = android::hardware::keymaster::V4_0::Algorithm::RSA;
    keyParams.push_back(kp);

    kp.tag = android::hardware::keymaster::V4_0::Tag::KEY_SIZE;
    kp.f.integer = 1024;
    keyParams.push_back(kp);

    kp.tag = android::hardware::keymaster::V4_0::Tag::PADDING;
    kp.f.paddingMode =
        android::hardware::keymaster::V4_0::PaddingMode::RSA_PKCS1_1_5_SIGN;
    keyParams.push_back(kp);

    kp.tag = android::hardware::keymaster::V4_0::Tag::RSA_PUBLIC_EXPONENT;
    kp.f.longInteger = 65537;
    keyParams.push_back(kp);
  } else {
    kp.tag = android::hardware::keymaster::V4_0::Tag::ALGORITHM;
    kp.f.algorithm = android::hardware::keymaster::V4_0::Algorithm::EC;
    keyParams.push_back(kp);

    kp.tag = android::hardware::keymaster::V4_0::Tag::KEY_SIZE;
    kp.f.integer = 256;
    keyParams.push_back(kp);

    kp.tag = android::hardware::keymaster::V4_0::Tag::EC_CURVE;
    kp.f.ecCurve = android::hardware::keymaster::V4_0::EcCurve::P_256;
    keyParams.push_back(kp);
  }
  kp.tag = android::hardware::keymaster::V4_0::Tag::DIGEST;
  kp.f.digest = android::hardware::keymaster::V4_0::Digest::SHA_2_256;
  keyParams.push_back(kp);

  kp.tag = android::hardware::keymaster::V4_0::Tag::PURPOSE;
  kp.f.purpose = android::hardware::keymaster::V4_0::KeyPurpose::SIGN;
  keyParams.push_back(kp);

  kp.tag = android::hardware::keymaster::V4_0::Tag::NO_AUTH_REQUIRED;
  kp.f.boolValue = 1;
  keyParams.push_back(kp);
  // kp.tag = android::hardware::keymaster::V4_0::Tag::APPLICATION_ID;
  // kp.blob = hidl_vec<uint8_t>(appId.begin(), appId.end());
  // keyParams.push_back(kp);

  bool ok = false;
  hidl_vec<uint8_t> myKeyBlob;
  d->generateKey(
      hidl_vec<android::hardware::keymaster::V4_0::KeyParameter>(keyParams),
      [&myKeyBlob,
       &ok](::android::hardware::keymaster::V4_0::ErrorCode error,
            const ::android::hardware::hidl_vec<uint8_t> &keyBlob,
            const ::android::hardware::keymaster::V4_0::KeyCharacteristics
                &keyCharacteristics) {
        fprintf(stderr, "Generated: %d %s\n", error,
                android::hardware::keymaster::V4_0::toString(error).c_str());
        if (error == android::hardware::keymaster::V4_0::ErrorCode::OK) {
          ok = true;
          myKeyBlob = keyBlob;
          fprintf(stderr, "Key Blob Size: %zu\n", keyBlob.size());
          for (auto &&kc : keyCharacteristics.softwareEnforced) {
            fprintf(
                stderr, "[SW] : %s\n",
                android::hardware::keymaster::V4_0::toString(kc.tag).c_str());
          }
          for (auto &&kc : keyCharacteristics.hardwareEnforced) {
            fprintf(
                stderr, "[HW] : %s\n",
                android::hardware::keymaster::V4_0::toString(kc.tag).c_str());
            if (kc.tag == android::hardware::keymaster::V4_0::Tag::OS_VERSION) {
              fprintf(stderr, "\t%d\n", kc.f.integer);
            } else if (kc.tag ==
                       android::hardware::keymaster::V4_0::Tag::OS_PATCHLEVEL) {
              fprintf(stderr, "\t%d\n", kc.f.integer);
            } else if (kc.tag == android::hardware::keymaster::V4_0::Tag::
                                     BOOT_PATCHLEVEL) {
              fprintf(stderr, "\t%d\n", kc.f.integer);
            } else if (kc.tag == android::hardware::keymaster::V4_0::Tag::
                                     VENDOR_PATCHLEVEL) {
              fprintf(stderr, "\t%d\n", kc.f.integer);
            } else if (kc.tag == android::hardware::keymaster::V4_0::Tag::
                                     BLOB_USAGE_REQUIREMENTS) {
              fprintf(stderr, "\t%d\n", kc.f.keyBlobUsageRequirements);
            }
          }
        }
      });

  if (!ok) {
    return;
  }

  std::vector<android::hardware::keymaster::V4_0::KeyParameter> attestParams;

  // kp.tag = android::hardware::keymaster::V4_0::Tag::APPLICATION_ID;
  // kp.blob = hidl_vec<uint8_t>(appId.begin(), appId.end());
  // attestParams.push_back(kp);
  kp.tag = android::hardware::keymaster::V4_0::Tag::ATTESTATION_APPLICATION_ID;
  kp.blob = hidl_vec<uint8_t>(appId, appId + sizeof(appId));
  attestParams.push_back(kp);
  kp.tag = android::hardware::keymaster::V4_0::Tag::ATTESTATION_CHALLENGE;
  std::string challenge = "mycallenge123";
  kp.blob = hidl_vec<uint8_t>(challenge.begin(), challenge.end());
  attestParams.push_back(kp);

  d->attestKey(
      myKeyBlob,
      hidl_vec<android::hardware::keymaster::V4_0::KeyParameter>(attestParams),
      [save_cert,
       &cert_prefix](::android::hardware::keymaster::V4_0::ErrorCode error,
                     const ::android::hardware::hidl_vec<
                         ::android::hardware::hidl_vec<uint8_t>> &certChain) {
        fprintf(stderr, "Attest result: %d %s\n", error,
                android::hardware::keymaster::V4_0::toString(error).c_str());
        if (save_cert) {
          for (size_t i = 0; i < certChain.size(); i++) {
            std::string path;
            path.insert(path.end(), cert_prefix.begin(), cert_prefix.end());
            path += std::to_string(i);
            path += ".crt";
            FILE *fp = fopen(path.c_str(), "wb");
            if (!fp) {
              fprintf(stderr, "Cannot open cert for write\n");
            } else {
              fwrite(certChain[i].data(), certChain[i].size(), 1, fp);
              fclose(fp);
              fprintf(stderr, "Wrote cert %s\n", path.c_str());
            }
          }
        }
      });
}

static std::optional<android::hardware::keymaster::V4_0::KeyParameter>
convertKeyMintToKeymaster(
    const aidl::android::hardware::security::keymint::KeyParameter &km_kp) {
  using aidl::android::hardware::security::keymint::KeyParameterValue;
  namespace km = android::hardware::keymaster::V4_0;
  km::KeyParameter hidl_kp;

  switch (km_kp.tag) {
  case aidl::android::hardware::security::keymint::Tag::CERTIFICATE_NOT_AFTER:
  case aidl::android::hardware::security::keymint::Tag::CERTIFICATE_NOT_BEFORE:
  case aidl::android::hardware::security::keymint::Tag::CERTIFICATE_SERIAL:
  case aidl::android::hardware::security::keymint::Tag::CERTIFICATE_SUBJECT:
    return std::nullopt;
  }

  hidl_kp.tag = static_cast<km::Tag>(km_kp.tag);

  switch (km_kp.value.getTag()) {
  case KeyParameterValue::invalid:
    break;
  case KeyParameterValue::algorithm:
    hidl_kp.f.algorithm = static_cast<km::Algorithm>(
        km_kp.value.template get<KeyParameterValue::algorithm>());
    break;
  case KeyParameterValue::blockMode:
    hidl_kp.f.blockMode = static_cast<km::BlockMode>(
        km_kp.value.template get<KeyParameterValue::blockMode>());
    break;
  case KeyParameterValue::paddingMode:
    hidl_kp.f.paddingMode = static_cast<km::PaddingMode>(
        km_kp.value.template get<KeyParameterValue::paddingMode>());
    break;
  case KeyParameterValue::digest:
    hidl_kp.f.digest = static_cast<km::Digest>(
        km_kp.value.template get<KeyParameterValue::digest>());
    break;
  case KeyParameterValue::ecCurve:
    hidl_kp.f.ecCurve = static_cast<km::EcCurve>(
        km_kp.value.template get<KeyParameterValue::ecCurve>());
    break;
  case KeyParameterValue::origin:
    hidl_kp.f.origin = static_cast<km::KeyOrigin>(
        km_kp.value.template get<KeyParameterValue::origin>());
    break;
  case KeyParameterValue::keyPurpose:
    hidl_kp.f.purpose = static_cast<km::KeyPurpose>(
        km_kp.value.template get<KeyParameterValue::keyPurpose>());
    break;
  case KeyParameterValue::hardwareAuthenticatorType:
    hidl_kp.f.hardwareAuthenticatorType =
        static_cast<km::HardwareAuthenticatorType>(
            km_kp.value
                .template get<KeyParameterValue::hardwareAuthenticatorType>());
    break;
  case KeyParameterValue::boolValue:
    hidl_kp.f.boolValue =
        km_kp.value.template get<KeyParameterValue::boolValue>();
    break;
  case KeyParameterValue::integer:
    hidl_kp.f.integer = km_kp.value.template get<KeyParameterValue::integer>();
    break;
  case KeyParameterValue::longInteger:
    hidl_kp.f.longInteger =
        km_kp.value.template get<KeyParameterValue::longInteger>();
    break;
  case KeyParameterValue::dateTime:
    hidl_kp.f.dateTime =
        km_kp.value.template get<KeyParameterValue::dateTime>();
    break;
  case KeyParameterValue::blob: {
    const auto &blob = km_kp.value.template get<KeyParameterValue::blob>();
    hidl_kp.blob = hidl_vec<uint8_t>(blob.begin(), blob.end());
    break;
  }
  case KeyParameterValue::securityLevel:
    // Keymaster 4.0 doesn't use SecurityLevel as a parameter type for tags.
    break;
  }
  return std::optional<km::KeyParameter>(hidl_kp);
}

bool isDeviceIdAttestationKp(
    aidl::android::hardware::security::keymint::Tag tag) {
  switch (tag) {
  case aidl::android::hardware::security::keymint::Tag::ATTESTATION_ID_BRAND:
  case aidl::android::hardware::security::keymint::Tag::ATTESTATION_ID_DEVICE:
  case aidl::android::hardware::security::keymint::Tag::ATTESTATION_ID_PRODUCT:
  case aidl::android::hardware::security::keymint::Tag::ATTESTATION_ID_SERIAL:
  case aidl::android::hardware::security::keymint::Tag::ATTESTATION_ID_IMEI:
  case aidl::android::hardware::security::keymint::Tag::ATTESTATION_ID_MEID:
  case aidl::android::hardware::security::keymint::Tag::
      ATTESTATION_ID_MANUFACTURER:
  case aidl::android::hardware::security::keymint::Tag::ATTESTATION_ID_MODEL:
    return true;
  default:
    return false;
  }
}

// std::optional<aidl::android::hardware::security::keymint::KeyParameter>
// toKeymintParameter(android::hardware::keymaster::V4_0::KeyParameter kp) {
//   switch (kp.tag) {
//     case android::hardware::keymaster::V4_0::Tag::ALGORITHM:

//   }
// }

void generate_key(const rust::Vec<rust::u8> &in_params,
                  const rust::Vec<rust::u8> &app_id,
                  rust::Vec<rust::u8> &key_blob,
                  rust::Vec<rust::u8> &metadataParceled, bool &hasError,
                  rust::Vec<rust::u8> &error_response) {
  // Decode in_params to KeyParameters
  aidl::android::system::keystore2::KeyParameters kps;
  AParcel *parcel = AParcel_create();

  // hexdump(in_params.data(), in_params.size());
  binder_status_t status =
      AParcel_unmarshal(parcel, in_params.data(), in_params.size());
  if (status != STATUS_OK) {
    fprintf(stderr, "Failed to unmarshal parcel: %d\n", status);
    AParcel_delete(parcel);
    return;
  }
  AParcel_setDataPosition(parcel, 0);

  if (kps.readFromParcel(parcel) != STATUS_OK) {
    fprintf(stderr, "Failed to read KeyParameters from parcel\n");
    AParcel_delete(parcel);
    return;
  }
  AParcel_delete(parcel);

  auto serviceManager = IServiceManager::getService();
  if (!serviceManager.get()) {
    fprintf(stderr, "Failed to get ServiceManager\n");
    return;
  }
  auto device = enumerateKeymasterDevices<Keymaster4>(serviceManager.get());
  auto d = device[SecurityLevel::TRUSTED_ENVIRONMENT];
  if (!d) {
    fprintf(stderr, "Failed to get TEE device\n");
    return;
  }
  for (const auto &kp : kps.keyParameter) {
    // Id attestation is not support on some device (Return CANNOT_ATTEST_IDS).
    if (isDeviceIdAttestationKp(kp.tag)) {
      fprintf(stderr, "Device id attestation is not supported\n");
      AParcel *parcel = AParcel_create();
      const int32_t EX_SERVICE_SPECIFIC = -8;
      AParcel_writeInt32(parcel, EX_SERVICE_SPECIFIC);

      // Dummy Empty String16
      AParcel_writeInt32(parcel, 0);
      AParcel_writeInt32(parcel, 0);

      // Stack trace header size
      AParcel_writeInt32(parcel, 0);

      // Keymint error code
      AParcel_writeInt32(
          parcel,
          static_cast<int32_t>(aidl::android::hardware::security::keymint::
                                   ErrorCode::CANNOT_ATTEST_IDS));

      size_t size = AParcel_getDataSize(parcel);
      vec_u8_set_len(error_response, size);
      AParcel_setDataPosition(parcel, 0);
      AParcel_marshal(parcel, error_response.data(), 0, size);
      AParcel_delete(parcel);
      hasError = true;
      return;
    }
  }

  std::vector<android::hardware::keymaster::V4_0::KeyParameter> hidlParams;
  for (const auto &kp : kps.keyParameter) {
    if (kp.tag == aidl::android::hardware::security::keymint::Tag::
                      ATTESTATION_CHALLENGE) {
      continue;
    }
    auto kp2 = convertKeyMintToKeymaster(kp);
    if (kp2) {
      hidlParams.push_back(*kp2);
    }
  }

  hidl_vec<uint8_t> myKeyBlob;
  bool ok = false;
  android::hardware::keymaster::V4_0::KeyCharacteristics
      generatedKeyCharacteristics;

  d->generateKey(
      hidlParams,
      [&myKeyBlob, &ok, &generatedKeyCharacteristics](
          android::hardware::keymaster::V4_0::ErrorCode error,
          const hidl_vec<uint8_t> &keyBlob,
          const android::hardware::keymaster::V4_0::KeyCharacteristics
              &keyCharacteristics) {
        fprintf(stderr, "generateKey result: %d %s\n", error,
                android::hardware::keymaster::V4_0::toString(error).c_str());
        if (error == android::hardware::keymaster::V4_0::ErrorCode::OK) {
          generatedKeyCharacteristics = std::move(keyCharacteristics);
          myKeyBlob = keyBlob;
          ok = true;
          fprintf(stderr, "Key Blob Size: %zu\n", keyBlob.size());
        }
      });

  aidl::android::system::keystore2::KeyMetadata metadata;
  metadata.keySecurityLevel = aidl::android::hardware::security::keymint::
      SecurityLevel::TRUSTED_ENVIRONMENT;
  if (ok) {
    for (auto &&kc : generatedKeyCharacteristics.softwareEnforced) {
      metadata.authorizations.push_back(
          aidl::android::system::keystore2::Authorization{
              .securityLevel = aidl::android::hardware::security::keymint::
                  SecurityLevel::KEYSTORE,
              .keyParameter = convertKeyParameterFromLegacy(kc)});
      fprintf(stderr, "[SW] : %s\n",
              android::hardware::keymaster::V4_0::toString(kc).c_str());
    }
    for (auto &&kc : generatedKeyCharacteristics.hardwareEnforced) {
      fprintf(stderr, "[HW] : %s\n",
              android::hardware::keymaster::V4_0::toString(kc).c_str());
      if (kc.tag == android::hardware::keymaster::V4_0::Tag::HARDWARE_TYPE) {
        fprintf(stderr, "Skip HARDWARE_TYPE\n");
        continue;
      }
      metadata.authorizations.push_back(
          aidl::android::system::keystore2::Authorization{
              .securityLevel = aidl::android::hardware::security::keymint::
                  SecurityLevel::TRUSTED_ENVIRONMENT,
              .keyParameter = convertKeyParameterFromLegacy(kc)});
      if (kc.tag == android::hardware::keymaster::V4_0::Tag::OS_VERSION) {
        fprintf(stderr, "\t%d\n", kc.f.integer);
      } else if (kc.tag == android::hardware::keymaster::V4_0::Tag::
                               BLOB_USAGE_REQUIREMENTS) {
        fprintf(stderr, "\t%d\n", kc.f.keyBlobUsageRequirements);
      }
    }
  }
  int32_t userId = 0;
  metadata.authorizations.push_back(
      aidl::android::system::keystore2::Authorization{
          .securityLevel = aidl::android::hardware::security::keymint::
              SecurityLevel::SOFTWARE,
          .keyParameter = {
              .tag = aidl::android::hardware::security::keymint::Tag::USER_ID,
              .value = aidl::android::hardware::security::keymint::
                  KeyParameterValue::make<
                      aidl::android::hardware::security::keymint::
                          KeyParameterValue::Tag::integer>(userId)}});

  std::vector<android::hardware::keymaster::V4_0::KeyParameter> attestParams;

  android::hardware::keymaster::V4_0::KeyParameter kp;
  kp.tag = android::hardware::keymaster::V4_0::Tag::ATTESTATION_APPLICATION_ID;
  kp.blob = hidl_vec<uint8_t>(app_id.begin(), app_id.end());
  fprintf(stderr, "Attest len: %zu\n", app_id.size());
  attestParams.push_back(kp);

  for (const auto &kpa : kps.keyParameter) {
    if (kpa.tag == aidl::android::hardware::security::keymint::Tag::
                       ATTESTATION_CHALLENGE) {
      const auto &blob =
          kpa.value.template get<aidl::android::hardware::security::keymint::
                                     KeyParameterValue::blob>();
      kp.tag = android::hardware::keymaster::V4_0::Tag::ATTESTATION_CHALLENGE;
      kp.blob = hidl_vec<uint8_t>(blob.begin(), blob.end());
      // kp.blob.data()[kp.blob.size() -1] = 't';
      fprintf(stderr, "Challenge len: %zu\n", blob.size());
      attestParams.push_back(kp);
    }
  }
  if (ok) {
    d->attestKey(
        myKeyBlob,
        hidl_vec<android::hardware::keymaster::V4_0::KeyParameter>(
            attestParams),
        [&metadata, &metadataParceled, &key_blob,
         &myKeyBlob](android::hardware::keymaster::V4_0::ErrorCode error,
                     const hidl_vec<hidl_vec<uint8_t>> &certChain) {
          fprintf(stderr, "attestKey result: %d %s\n", error,
                  android::hardware::keymaster::V4_0::toString(error).c_str());
          if (error == android::hardware::keymaster::V4_0::ErrorCode::OK) {
            fprintf(stderr, "Certificate chain size: %zu\n", certChain.size());
            metadata.certificate =
                std::vector<uint8_t>(certChain[0].begin(), certChain[0].end());
            metadata.certificateChain = std::vector<uint8_t>();
            for (size_t i = 1; i < certChain.size(); i++) {
              const auto &cert = certChain[i];
              metadata.certificateChain->insert(
                  metadata.certificateChain->end(), cert.begin(), cert.end());
            }
            metadata.modificationTimeMs = (int64_t)time(nullptr) * 1000;

            // Parcel metadata
            AParcel *parcel = AParcel_create();
            metadata.writeToParcel(parcel);
            size_t size = AParcel_getDataSize(parcel);
            vec_u8_set_len(metadataParceled, size);
            AParcel_setDataPosition(parcel, 0);
            AParcel_marshal(parcel, metadataParceled.data(), 0, size);
            AParcel_delete(parcel);
            // Copy key blob
            vec_u8_set_len(key_blob, myKeyBlob.size());
            memcpy(key_blob.data(), myKeyBlob.data(), myKeyBlob.size());
          }
        });
  }
}
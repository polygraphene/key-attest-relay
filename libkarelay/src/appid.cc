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
#include "libkarelay/include/appid.h"
#include "rust/cxx.h"

#include <keystore/keystore_attestation_id.h>

constexpr const size_t KEY_ATTESTATION_APPLICATION_ID_MAX_SIZE = 1024;
using android::security::gather_attestation_application_id;

rust::Vec<rust::u8> get_app_id(int uid, rust::u32 &result) {
  static_assert(sizeof(uint32_t) == sizeof(uid_t), "uid_t has unexpected size");
  static_assert(sizeof(uint32_t) == sizeof(android::status_t),
                "status_t has unexpected size");
  static_assert(KEY_ATTESTATION_APPLICATION_ID_MAX_SIZE ==
                    android::security::KEY_ATTESTATION_APPLICATION_ID_MAX_SIZE,
                "KEY_ATTESTATION_APPLICATION_ID_MAX_SIZE sizes don't match.");
  auto result2 = gather_attestation_application_id(uid);
  if (!result2.isOk()) {
    result = result2.status();
    return rust::Vec<rust::u8>();
  }
  if (result2.value().size() > KEY_ATTESTATION_APPLICATION_ID_MAX_SIZE) {
    result = ::android::NO_MEMORY;
    return rust::Vec<rust::u8>();
  }
  rust::Vec<rust::u8> aaid;
  for (auto c : result2.value()) {
    aaid.push_back(c);
  }
  return aaid;
}
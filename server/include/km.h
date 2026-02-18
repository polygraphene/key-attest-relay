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
#pragma once
#include "rust/cxx.h"

void keymaster_test(bool save_cert, const rust::String &cert_prefix);
void generate_key(const rust::Vec<rust::u8>& in_params, const rust::Vec<rust::u8>& app_id, rust::Vec<rust::u8>& key_blob, rust::Vec<rust::u8>& metadataParceled, bool& hasError, rust::Vec<rust::u8>& error_response);
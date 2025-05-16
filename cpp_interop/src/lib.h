#pragma once
#include <cstddef>
#include <cstdint>
#include "rust/cxx.h"
#include <memory>


std::unique_ptr<std::string> get_cstr_from_std_string(uintptr_t s);
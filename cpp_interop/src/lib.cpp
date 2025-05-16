#include <string>
#include <cstddef>
#include <cstdint>
#include "lib.h"
#include "cpp_interop/src/lib.rs.h"
#include "memory"

std::unique_ptr<std::string> get_cstr_from_std_string(uintptr_t s) {
    const std::string* str = reinterpret_cast<const std::string*>(s);
    if (!str) return nullptr;
    return std::make_unique<std::string>(*str);
}

#[cxx::bridge]
pub mod ffi {
    struct CstrResultT {
        pub ptr: *const c_char,
        pub len: usize,
    }

    unsafe extern "C++" {
        include!("cpp_interop/src/lib.h");

        fn get_cstr_from_std_string(s: usize) -> UniquePtr<CxxString>;
    }
}

#[derive(Debug)]
pub enum A {
    A,
    B,
    C((String, usize))
}

// Helper function for Rust usage
pub fn std_string_to_str(s: usize) -> A {
    let result = ffi::get_cstr_from_std_string(s);
    if result.is_null() {
        return A::A;
    }
    A::C((result.to_string(), result.len()))
}
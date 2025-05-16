fn main() {
    cxx_build::bridge("src/lib.rs")
        .file("src/lib.cpp")
        .flag_if_supported("-std=c++17")
        .compile("cpp_interop");

    println!("cargo:rerun-if-changed=src/lib.rs");
    println!("cargo:rerun-if-changed=src/lib.cpp");
    println!("cargo:rerun-if-changed=src/lib.h");
}

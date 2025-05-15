use std::{env, path::Path};

use bindgen::Builder;
use build_print::warn;

include!("./src/update.rs");
include!("./src/shuffles/mod.rs");
include!("./src/encryptions/mod.rs");

const PRE_REPLACE: [(&str, [(&str, &str); 1]); 1] = [(
    "VM/src/lobject.h",
    [(
        "uint8_t tt; uint8_t marked; uint8_t memcat",
        "LUAVM_SHUFFLE3(LUAVM_SHUFFLE_OTHER, uint8_t tt, uint8_t marked, uint8_t memcat)",
    )],
)];

const BINDINGS_REPLACE: &[(&str, &str)] = &[(
    "pub static mut Luau_list: *mut Luau_FValue<T>;",
    "pub static mut Luau_list: *mut Luau_FValue<i32>;",
)];

fn cpp_bindings() -> Builder {
    bindgen::builder()
        .clang_arg("-xc++")
        .clang_arg("-std=c++17")
        .layout_tests(false)
        .allowlist_type("(LUA|lua)(u|U)?.*")
        .allowlist_function("(LUA|lua)(u|U)?.*")
        .allowlist_var("(LUA|lua)(u|U)?.*")
        .prepend_enum_name(false)
        .size_t_is_usize(true)
        .c_naming(false)
        .disable_name_namespacing()
}

fn vm_bindings(out_dir: &Path) -> std::io::Result<()> {
    cpp_bindings()
		.header("../official_luau/VM/include/lua.h")
		.header("../official_luau/VM/include/lualib.h")
        .header("../official_luau/VM/src/lobject.h")
        .header("../official_luau/VM/src/lstate.h")
        .header("../official_luau/VM/src/lapi.h")
        .clang_args([
            "-I../official_luau/VM/include",
            "-I../official_luau/Common/include",
        ])
		.blocklist_function("luaO_pushvfstring")
        .blocklist_function("lua_pushvfstring")
		.blocklist_type("va_list")
		.generate()
		.expect("Failed to generate VM bindings")
		.write_to_file(out_dir.join("luau_vm.rs"))
}

fn compiler_bindings(out_dir: &Path) -> std::io::Result<()> {
    cpp_bindings()
        .allowlist_item(".*")
        .header("../official_luau/Common/include/Luau/BytecodeUtils.h")
        .clang_args([
            "-I../official_luau/Common/include",
        ])
		.derive_default(true)
		.derive_copy(true)
		.derive_partialeq(true)
		.derive_eq(true)
		.derive_hash(true)
		.generate()
		.expect("Failed to generate Compiler bindings")
		.write_to_file(out_dir.join("luau_compiler.rs"))
}

fn main() {
    println!("cargo:rerun-if-changed=NULL");

    // Add (and update) VM shuffles
    if do_shuffles() {
        // Do some replacements before bindgen
        let official_luau_path = PathBuf::from("../official_luau");
        for (file_path, replacements) in PRE_REPLACE {
            let file_path = official_luau_path.join(file_path);
            let mut file_content = read_to_string(&file_path).expect("failed to find file");
    
            for (from, to) in replacements {
                file_content = file_content.replace(from, to)
            }
    
            fs::write(file_path, file_content).expect("failed to write file");
        }
    }

    // Output the bindings
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let bindings_path = out_dir.join("luau_vm.rs");

    // Build raw bindings
    compiler_bindings(&out_dir).unwrap();
    vm_bindings(&out_dir).unwrap();

    // Read the generated bindings
    let mut bindings_content = fs::read_to_string(&bindings_path).expect("Couldn't read bindings!");

    // Modify the bindings to fix some issues
    for (from, to) in BINDINGS_REPLACE {
        bindings_content = bindings_content.replace(from, to);
    }

    let mut syntax_tree = syn::parse_file(&bindings_content).unwrap();
    do_encryptions(&mut syntax_tree);

    // Write the modified bindings back to the file
    fs::write(&bindings_path, prettyplease::unparse(&syntax_tree))
        .expect("Couldn't write modified bindings!");

    warn!("run");
}

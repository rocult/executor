use std::{env, path::Path};

use bindgen::Builder;

include!("./src/update.rs");
include!("./src/func_override/mod.rs");
include!("./src/shuffles/mod.rs");
include!("./src/encryptions/mod.rs");

const PRE_REPLACE: [(&str, [(&str, &str); 1]); 1] = [(
    "VM/src/lobject.h",
    [(
        "uint8_t tt; uint8_t marked; uint8_t memcat",
        "LUAU_SHUFFLE3(LUAU_SHUFFLE_OTHER, uint8_t tt, uint8_t marked, uint8_t memcat)",
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
        .clang_args(["-I../official_luau/Common/include"])
        .derive_default(true)
        .derive_copy(true)
        .derive_partialeq(true)
        .derive_eq(true)
        .derive_hash(true)
        .generate()
        .expect("Failed to generate Compiler bindings")
        .write_to_file(out_dir.join("luau_compiler.rs"))
}

fn build_cmake(official_luau_path: PathBuf) -> std::io::Result<()> {
    // Statically link the Luau VM library
    let cmake_folder = official_luau_path.join("cmake");
    println!("cargo:rustc-link-search=native={}", cmake_folder.join("build").display());
    println!("cargo:rustc-link-lib=static=Luau.VM");
    println!("cargo:rustc-link-lib=static=Luau.Compiler");
    println!("cargo:rustc-link-lib=static=Luau.Ast");

    let mut cc_build = cc::Build::new();
    cc_build.cpp(true);
    cc_build.std("c++17");

    // Initialise the cmake build
    std::fs::create_dir_all(&cmake_folder)?;
    cmake::Config::new(&official_luau_path)
        .init_cxx_cfg(cc_build.clone())
        .out_dir(&cmake_folder)
        .generator("Ninja")
        .build_target("Luau.VM;Luau.Compiler")
        .define("CMAKE_BUILD_TYPE", "RelWithDebInfo")
        .define("LUAU_STATIC_CRT", "ON")
        .static_crt(true)
        .build();

    Ok(())
}

fn main() -> std::io::Result<()> {
    println!("cargo:rerun-if-changed=NULL");

    // Add (and update) VM shuffles
    let manifest_path = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let official_luau_path = manifest_path.join("../official_luau");
    if do_shuffles() {
        // Do some replacements before bindgen
        for (file_path, replacements) in PRE_REPLACE {
            let file_path = official_luau_path.join(file_path);
            let mut file_content = read_to_string(&file_path)?;

            for (from, to) in replacements {
                file_content = file_content.replace(from, to)
            }

            fs::write(file_path, file_content)?;
        }
    }

    let vm_dir = official_luau_path.join("VM/src");
    do_func_override(&[
        (
            vm_dir.join("lgc.cpp"),
            "size_t luaC_step(lua_State* L, bool assist)",
            funcs::LUAC_STEP,
        ),
        (
            vm_dir.join("ldo.cpp"),
            "l_noret luaD_throw(lua_State* L, int errcode)",
            funcs::LUAD_THROW, // need to cover both
        ),
        (
            vm_dir.join("lapi.cpp"),
            "const TValue* luaA_toobject(lua_State* L, int idx)",
            funcs::LUAA_TOOBJECT,
        ),
        (
            vm_dir.join("laux.cpp"),
            "const char* luaL_checklstring(lua_State* L, int narg, size_t* len)",
            funcs::LUAL_CHECKLSTRING,
        ),
        (
            vm_dir.join("laux.cpp"),
            "int luaL_getmetafield(lua_State* L, int obj, const char* event)",
            funcs::LUAL_GETMETAFIELD,
        ),
        (
            vm_dir.join("laux.cpp"),
            "void luaL_register(lua_State* L, const char* libname, const luaL_Reg* l)",
            funcs::LUAL_REGISTER,
        ),
        (
            vm_dir.join("lmem.cpp"),
            "void luaM_visitgco(lua_State* L, void* context, bool (*visitor)(void* context, lua_Page* page, GCObject* gco))",
            funcs::LUAM_VISITGCO,
        ),
        (
            vm_dir.join("lobject.cpp"),
            "const char* luaO_pushvfstring(lua_State* L, const char* fmt, va_list argp)",
            funcs::LUAO_PUSHVFSTRING, 
        ),
        (
            vm_dir.join("lvmload.cpp"),
            "int luau_load(lua_State* L, const char* chunkname, const char* data, size_t size, int env)",
            funcs::LUAU_LOAD,
        ),
        (
            vm_dir.join("lvmexecute.cpp"),
            "void luau_execute(lua_State* L)",
            funcs::LUAU_EXECUTE,
        )
    ])?;

    // Output the bindings
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let bindings_path = out_dir.join("luau_vm.rs");

    // Build raw bindings
    compiler_bindings(&out_dir)?;
    vm_bindings(&out_dir)?;

    // Read the generated bindings
    let mut bindings_content = fs::read_to_string(&bindings_path)?;

    // Modify the bindings to fix some issues
    for (from, to) in BINDINGS_REPLACE {
        bindings_content = bindings_content.replace(from, to);
    }

    let mut syntax_tree = syn::parse_file(&bindings_content).unwrap();
    do_encryptions(&mut syntax_tree);

    // Write the modified bindings back to the file
    fs::write(&bindings_path, prettyplease::unparse(&syntax_tree))?;

    // Build the Luau libraries
    build_cmake(official_luau_path)
}

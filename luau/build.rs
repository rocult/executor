use std::{env, fs};
use std::path::PathBuf;

fn insert_vm_shuffles(lua_h: &mut String) {
    // Find the last #include directive
    let last_include = lua_h.lines()
        .enumerate()
        .filter(|(_, line)| line.trim_start().starts_with("#include"))
        .map(|(index, _)| index)
        .last()
        .expect("could not find any #include directives");

    // Add the definitions after the last include
    let insert_position = lua_h.lines().take(last_include + 1).map(|line| line.len() + 1).sum();
    lua_h.insert_str(insert_position, r#"
        #define LUAVM_SHUFFLE_COMMA ,
        #define LUAVM_SHUFFLE_OTHER ;

        #define LUAVM_SHUFFLE3(sep, a1, a2, a3) a1 sep a3 sep a2
        #define LUAVM_SHUFFLE4(sep, a1, a2, a3, a4) a1 sep a3 sep a2 sep a4
        #define LUAVM_SHUFFLE5(sep, a1, a2, a3, a4, a5) a3 sep a1 sep a2 sep a5 sep a4
        #define LUAVM_SHUFFLE6(sep, a1, a2, a3, a4, a5, a6) a3 sep a1 sep a2 sep a6 sep a4 sep a5
        #define LUAVM_SHUFFLE7(sep, a1, a2, a3, a4, a5, a6, a7) a2 sep a4 sep a5 sep a7 sep a6 sep a3 sep a1
        #define LUAVM_SHUFFLE8(sep, a1, a2, a3, a4, a5, a6, a7, a8) a6 sep a4 sep a7 sep a2 sep a8 sep a1 sep a5 sep a3
        #define LUAVM_SHUFFLE9(sep, a1, a2, a3, a4, a5, a6, a7, a8, a9) a4 sep a7 sep a6 sep a5 sep a2 sep a3 sep a1 sep a9 sep a8
    "#);
}

fn remove_existing_vm_shuffles(lua_h: &mut String) {
    let shuffle_definitions = [
        "#define LUAVM_SHUFFLE_COMMA",
        "#define LUAVM_SHUFFLE_OTHER",
        "#define LUAVM_SHUFFLE3",
        "#define LUAVM_SHUFFLE4",
        "#define LUAVM_SHUFFLE5",
        "#define LUAVM_SHUFFLE6",
        "#define LUAVM_SHUFFLE7",
        "#define LUAVM_SHUFFLE8",
        "#define LUAVM_SHUFFLE9",
    ];

    for definition in &shuffle_definitions {
        while let Some(start) = lua_h.find(definition) {
            if let Some(end) = lua_h[start..].find('\n') {
                lua_h.replace_range(start..start + end + 1, "");
            }
        }
    }
}

fn add_vm_shuffles() {
    let mut lua_h = fs::read_to_string("../official_luau/VM/include/lua.h").expect("could not find main include: lua.h");

    // VM shuffles already defined, only update them
    if lua_h.contains("#define LUAVM_SHUFFLE3") {
        remove_existing_vm_shuffles(&mut lua_h);
        insert_vm_shuffles(&mut lua_h);
        return;
    }

    // Insert the defintions
    insert_vm_shuffles(&mut lua_h);

    // Add the macro in the correct places
}

fn main() {
    // Add (and update) VM shuffles
    add_vm_shuffles();

    // Configure the bindgen
    let bindings = bindgen::Builder::default()
        .header("../official_luau/VM/src/lobject.h")
        .clang_args([
            "-I../official_luau/VM/include",
            "-I../official_luau/Common/include",
            "-x", "c++",
            "-std=c++11",
        ])
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    // Output the bindings
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    let bindings_path = out_path.join("bindings.rs");
    bindings
        .write_to_file(&bindings_path)
        .expect("Couldn't write bindings!");

    // Read the generated bindings
    let mut bindings_content = fs::read_to_string(&bindings_path)
        .expect("Couldn't read bindings!");

    // Modify the bindings to fix the issue with Luau_FValue
    bindings_content = bindings_content.replace(
        "pub static mut Luau_list: *mut Luau_FValue<T>;",
        "pub static mut Luau_list: *mut Luau_FValue<i32>;"
    );

    // Write the modified bindings back to the file
    fs::write(&bindings_path, bindings_content)
        .expect("Couldn't write modified bindings!");
}
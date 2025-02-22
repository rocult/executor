use std::{env, fs};
use std::path::PathBuf;

fn insert_vm_shuffles_dirs(lua_h: &mut String) {
    let byte_position = lua_h.find("\n\n\n").expect("could not find insert place");

    // Add the definitions after the found position
    lua_h.insert_str(byte_position + 4, r#"
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

fn remove_existing_vm_shuffles_dirs(lua_h: &mut String) {
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

fn add_vm_shuffles_macro(file: &mut String) {
    let mut lines = file.lines().map(str::to_string).collect::<Vec<String>>();
    let mut i = 0;

    // Iterate through all of the lines
    while i < lines.len() {
        // Check if both the current line and next line are empty, otherwise go next
        if !(lines[i].trim().is_empty() && i + 1 < lines.len() && lines[i + 1].trim().is_empty()) {
            i += 1;
            continue;
        }

        // Find the next empty line
        let mut j = i + 2;
        while j < lines.len() && !lines[j].trim().is_empty() {
            j += 1;
        }

        // Ensure the count is valid
        let count = j - (i + 2);
        if count < 3 || count > 9 {
            println!("warning: invalid shuffle detected!");
            i = j; // Move to the next section
            continue;
        }

        // Find what separator the line is using
        let char = lines[i + 2].chars().last().expect("could not get char");
        let sep = match char {
            ';' => "OTHER",
            ',' => "COMMA",
            _ => {
                println!("warning: invalid sep detected!");
                i = j; // Move to the next section
                continue;
            }
        };

        // Replace all separators between the lines with a comma
        for k in (i + 2)..j {
            if k == j {
                lines[k] = lines[k].replace(char, "");
            } else {
                lines[k] = lines[k].replace(char, ",");
            }
        }

        // Add the macro to the source
        let formatted_string = format!("LUAVM_SHUFFLE{}(LUAVM_SHUFFLE_{},", count, sep);
        lines.insert(i + 2, formatted_string);

        // Add the closing bracket
        lines.insert(j + 1, format!("){}", char));

        // Account for added lines in next iteration
        i = j + 2;
    }

    *file = lines.join("\n");
}

fn process_file(path: &PathBuf) {
    let mut data = fs::read_to_string(path).unwrap();
    add_vm_shuffles_macro(&mut data);
    fs::write(path, data).unwrap();
}

fn add_vm_shuffles() {
    let mut lua_h = fs::read_to_string("../official_luau/VM/include/lua.h").expect("could not find main include: lua.h");

    // VM shuffles already defined, only update them
    if lua_h.contains("#define LUAVM_SHUFFLE3") {
        remove_existing_vm_shuffles_dirs(&mut lua_h);
        insert_vm_shuffles_dirs(&mut lua_h);
        return;
    }

    // Insert the defintions
    insert_vm_shuffles_dirs(&mut lua_h);
    fs::write("../official_luau/VM/include/lua.h", lua_h).unwrap();

    // Add the macro in the correct places
    let vm_dir = PathBuf::from("../official_luau/VM");

    for entry in fs::read_dir(vm_dir).expect("could not read VM directory") {
        let entry = entry.expect("could not read directory entry");
        let path = entry.path();

        if path.is_file() {
            process_file(&path);
        } else if path.is_dir() {
            for sub_entry in fs::read_dir(path).expect("could not read subdirectory") {
                let sub_entry = sub_entry.expect("could not read subdirectory entry");
                let sub_path = sub_entry.path();

                if sub_path.is_file() {
                    process_file(&sub_path);
                }
            }
        }
    }
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
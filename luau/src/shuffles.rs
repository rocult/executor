use std::{fs::{self, read_to_string}, path::PathBuf};

use build_print::warn;

const LUAU_VM_PATH: &'static str = "../official_luau/VM";
const LUAU_VM_LUA_H_PATH: &'static str = concat!("../official_luau/VM", "/include/lua.h");

const SHUFFLES: &'static str = r#"
#define LUAVM_SHUFFLE_COMMA ,
#define LUAVM_SHUFFLE_OTHER ;

#define LUAVM_SHUFFLE1(sep, a1) a1
#define LUAVM_SHUFFLE2(sep, a1, a2) a1 sep a2
#define LUAVM_SHUFFLE3(sep, a1, a2, a3) a1 sep a3 sep a2
#define LUAVM_SHUFFLE4(sep, a1, a2, a3, a4) a1 sep a3 sep a2 sep a4
#define LUAVM_SHUFFLE5(sep, a1, a2, a3, a4, a5) a3 sep a1 sep a2 sep a5 sep a4
#define LUAVM_SHUFFLE6(sep, a1, a2, a3, a4, a5, a6) a3 sep a1 sep a2 sep a6 sep a4 sep a5
#define LUAVM_SHUFFLE7(sep, a1, a2, a3, a4, a5, a6, a7) a2 sep a4 sep a5 sep a7 sep a6 sep a3 sep a1
#define LUAVM_SHUFFLE8(sep, a1, a2, a3, a4, a5, a6, a7, a8) a6 sep a4 sep a7 sep a2 sep a8 sep a1 sep a5 sep a3
#define LUAVM_SHUFFLE9(sep, a1, a2, a3, a4, a5, a6, a7, a8, a9) a4 sep a7 sep a6 sep a5 sep a2 sep a3 sep a1 sep a9 sep a8
#define LUAVM_SHUFFLE10(sep, a1, a2, a3, a4, a5, a6, a7, a8, a9, a10) a1 sep a2 sep a3 sep a4 sep a5 sep a6 sep a7 sep a8 sep a9 sep a10
"#;

fn insert_directives(lua_h: &mut String) -> bool {
    if lua_h.contains("LUAVM_SHUFFLE_COMMA") {
        return false;
    }

    let byte_position = lua_h
        .find("\n\n\n")
        .or_else(|| lua_h.find("\r\n\r\n\r\n"))
        .expect("could not find a valid place to insert directives.");

    lua_h.replace_range(byte_position + 2..byte_position + 6, &SHUFFLES);
    true
}

fn insert_calls(path: &PathBuf, file: &mut String) {
    let mut lines: Vec<String> = file.lines().map(str::to_string).collect();
    let mut i = 0;

    while i < lines.len() {
        // Ensure the sequence starts with two new lines, indicating a shuffle should be placed here.
        if !(lines[i].trim().is_empty() && i + 1 < lines.len() && lines[i + 1].trim().is_empty()) {
            i += 1;
            continue;
        }

        // Find the line at which the elements end
        let mut j = i + 2;
        while j < lines.len() && (!lines[j].trim().is_empty() && !lines[j].trim().contains('}')) {
            j += 1;
        }

        // Ensure we have 3-10 elements
        let count = j - (i + 2);
        if count > 10 {
            warn!("{}:{}: invalid shuffle detected: {}", path.canonicalize().unwrap().display(), i + 2, count);
            i = j;
            continue;
        }

        // Grab the separator between elements, i.e. a comma in an enum or a semi colon in a struct
        let mut char = None;
        for k in (i + 2)..j {
            let line = lines[k].trim();
            if !line.starts_with("//") && !line.starts_with("/*") {
                let stripped_line = line.split("//").next().unwrap_or(line).trim();
                char = stripped_line.chars().last();
                break;
            }
        }

        let char = char.expect("could not get char");
        let sep = match char {
            ';' => "OTHER",
            ',' => "COMMA",
            x => {
                warn!("{}:{}: invalid sep detected: {},", path.canonicalize().unwrap().display(), i + 2, x);
                i = j;
                continue;
            }
        };

        // Replace the separator with either nothing, if at the end, or a comma
        // This fixes two cases:
        // - Properly handles when the element is the last element in the item
        // - Cases where the separator is a semi colon, but the macro expects a comma
        for k in (i + 2)..j {
            if k == j - 1 {
                lines[k] = lines[k].replace(char, "");
            } else {
                lines[k] = lines[k].replace(char, ",");
            }
        }

        // Finally, insert the macro call start and end sequence
        lines.insert(i + 2, format!("LUAVM_SHUFFLE{}(LUAVM_SHUFFLE_{},", count, sep));
        lines.insert(j + 1, format!("){}", char));

        i = j + 2;
    }

    *file = lines.join("\n");
}

fn process_file(path: &PathBuf) {
    let mut data = read_to_string(path).expect("shuffles: failed to read file");
    insert_calls(path, &mut data);
    fs::write(path, data).expect("shuffles: failed to write file")
}

pub fn do_shuffles() -> bool {
    // Insert the directives inside of lua.h
    let mut lua_h = read_to_string(LUAU_VM_LUA_H_PATH)
        .expect("failed to read lua.h");
    if !insert_directives(&mut lua_h) {
        return false
    };
    fs::write(LUAU_VM_LUA_H_PATH, lua_h)
        .expect("failed to write to lua.h");

    // Process all other VM files, adding calls
    for entry in fs::read_dir(LUAU_VM_PATH).expect("could not read VM directory") {
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

    true
}
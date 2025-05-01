use std::{
    ffi::OsStr,
    fs::{self, read_to_string, File, OpenOptions},
    io::{BufRead, BufReader, Write},
    path::PathBuf,
};

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

fn read_line(reader: &mut BufReader<File>) -> std::io::Result<Option<(String, usize, String)>> {
    let mut line = String::new();
    let mut delimiter = String::new();
    let num_bytes = reader.read_line(&mut line)?;

    match num_bytes {
        0 => Ok(None),
        n => {
            if line.ends_with('\n') {
                delimiter.push(line.pop().unwrap());
                if line.ends_with('\r') {
                    delimiter.insert(0, line.pop().unwrap());
                }
            }

            Ok(Some((line, n, delimiter)))
        }
    }
}

fn insert_calls(mut reader: BufReader<File>, mut writer: File) -> std::io::Result<()> {
    // Track consecutive empty lines.
    let mut consecutive_empty = 0;

    while let Some((line, _num_bytes, delimiter)) = read_line(&mut reader)? {
        // Update the empty-line count.
        if line.trim().is_empty() {
            consecutive_empty += 1;
        } else {
            consecutive_empty = 0;
        }

        writer.write_all(line.as_bytes())?;
        writer.write_all(delimiter.as_bytes())?;

        // When two empty lines are encountered, process the next block.
        if consecutive_empty != 2 {
            continue;
        }

        // Prepare to capture a block of non-empty lines.
        let mut buffer = Vec::new();
        let mut nonempty_count = 0;
        let mut last_char = ';'; // fallback value
        let mut last_line = None;

        // Read the following lines without writing immediately.
        while let Some((next_line, next_bytes, next_delim)) = read_line(&mut reader)? {
            let trimmed = next_line.trim();
            if trimmed.is_empty() {
                // Undo reading of this empty line.
                reader.seek_relative(-(next_bytes as i64))?;
                break;
            } else if trimmed.starts_with("//") || trimmed.starts_with("}") {
                last_line = Some(format!("{next_line}{next_delim}"));
                break;
            }

            // Obtain the code portion before any trailing comment.
            let code_part = trimmed.split("//").next().unwrap().trim();
            if let Some(ch) = code_part.chars().last() {
                last_char = ch;
            }
            buffer.push(format!("{next_line}{next_delim}"));
            nonempty_count += 1;
        }

        // Reset consecutive_empty so the next block is processed independently.
        consecutive_empty = 0;

        // If block size is not between 3 and 9, write the block as-is.
        if nonempty_count < 3 || nonempty_count > 9 {
            for buf_line in buffer {
                writer.write_all(buf_line.as_bytes())?;
            }
            if let Some(ref l) = last_line {
                writer.write_all(l.as_bytes())?;
            }
            continue;
        }

        // Determine the ending separator by inspecting the second-to-last buffered line.
        let end_sep = buffer
            .get(buffer.len().saturating_sub(2))
            .map(|s| if s.contains(';') { ';' } else { ',' })
            .unwrap_or(';');

        // Determine the shuffle flag based on the last code character.
        let sep_flag = match last_char {
            ';' => "OTHER",
            ',' => "COMMA",
            _ => unreachable!(),
        };

        // Begin the shuffle macro call.
        write!(writer, "LUAVM_SHUFFLE{}(LUAVM_SHUFFLE_{},{}", nonempty_count, sep_flag, delimiter)?;

        // Write each buffered line, replacing ';' with ',' where needed.
        let buf_last = buffer.len().saturating_sub(1);
        for (i, line) in buffer.into_iter().enumerate() {
            // Make sure we don't replace any comments
            let (code, comment) = match line.split_once("//") {
                Some((code_part, comment_part)) => (code_part, format!("//{}", comment_part)),
                None => (line.as_str(), String::new()),
            };

            let mut processed = code.replace(';', ",");
            // For the last argument, remove all commas.
            if i == buf_last {
                processed = processed.replace(',', "");
            }
            write!(writer, "{}{}", processed, comment)?;
        }

        // Close off the macro call.
        write!(writer, "){end_sep}{delimiter}")?;
        if let Some(ref l) = last_line {
            writer.write_all(l.as_bytes())?;
        }
    }
    Ok(())
}
fn append_to_path(p: impl Into<std::ffi::OsString>, s: impl AsRef<OsStr>) -> PathBuf {
    let mut p = p.into();
    p.push(s);
    p.into()
}

fn process_file(path: &PathBuf) {
    let temp_path = append_to_path(path, ".tmp");
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(path)
        .expect("shuffles: failed to open file");
    let temp_file = OpenOptions::new()
        .append(true)
        .create_new(true)
        .open(&temp_path)
        .expect("shuffles: failed to create temp file");

    let reader = BufReader::new(file);
    insert_calls(reader, temp_file).expect("shuffles: failed.");

    fs::rename(temp_path, path).expect("shuffles: failed to write new");
}

pub fn do_shuffles() -> bool {
    // Insert the directives inside of lua.h
    let mut lua_h = read_to_string(LUAU_VM_LUA_H_PATH).expect("failed to read lua.h");
    if !insert_directives(&mut lua_h) {
        return false;
    };
    fs::write(LUAU_VM_LUA_H_PATH, lua_h).expect("failed to write to lua.h");

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

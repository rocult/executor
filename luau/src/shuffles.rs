use std::{
    ffi::OsStr,
    fs::{self, read_to_string, File, OpenOptions},
    io::{BufRead, BufReader, Write},
    path::PathBuf,
};

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

fn peek_line(reader: &mut BufReader<File>) -> std::io::Result<Option<String>> {
    let Some((line, num_bytes, _)) = read_line(reader)? else {
        return Ok(None);
    };

    reader.seek_relative(-(num_bytes as i64))?;

    Ok(Some(line))
}

fn is_empty(str: &String) -> bool {
    str.trim().is_empty()
}

const IGNORED: [&str; 3] = ["lperf", "ludata", "lvmload"];

fn insert_calls_check(reader: &mut BufReader<File>) -> std::io::Result<bool> {
    // Read the second line
    let Some((line, num_bytes, _)) = read_line(reader)? else {
        return Ok(false);
    };

    // It's not empty, so unlikely to be a shuffle since it needs 2 new lines
    if !line.is_empty() {
        reader.seek_relative(-(num_bytes as i64))?;
        return Ok(false);
    }

    // Read the third line which should have code here...
    let Some((line, num_bytes2, _)) = read_line(reader)? else {
        reader.seek_relative(-(num_bytes as i64))?;
        return Ok(false);
    };

    // Ensure the third line does not start with `#`, indicating it's a directive
    if line.starts_with("#") {
        reader.seek_relative(-((num_bytes + num_bytes2) as i64))?;
        return Ok(false);
    }

    // Progress the reader, if needed
    if !line.is_empty() {
        reader.seek_relative(-(num_bytes2 as i64))?;
    }

    // i need to check the cursor stuff which empty line and stuff

    Ok(true)
}

fn insert_calls(path: &PathBuf, reader: BufReader<File>, mut writer: File) -> std::io::Result<()> {
    let mut reader = reader;
    let is_ltm = path.file_stem() == Some(&OsStr::new("ltm"));
    let x = path
        .file_stem()
        .and_then(|x| x.to_str())
        .unwrap_or_default();
    let is_ignored = IGNORED.contains(&x);

    while let Some((line, _, delimiter)) = read_line(&mut reader)? {
        if !((is_ltm && line == "    ") || (is_empty(&line) && !insert_calls_check(&mut reader)?))
            || is_ignored
        {
            writer.write(line.as_bytes())?;
            writer.write(delimiter.as_bytes())?;
            continue;
        }

        write!(writer, "{}{}", line, delimiter)?;

        let mut count = 0;
        let mut char_sep = None;
        let mut num_bytes_count = 0;

        while let Some((line2, num_bytes2, delimiter2)) = read_line(&mut reader)? {
            warn!("{line2}");
            count += 1;
            num_bytes_count += num_bytes2;
            if count > 10 {
                break;
            }

            if char_sep.is_none()
                && !(line2.starts_with("//") || line2.starts_with("/*") || line2.starts_with("#"))
            {
                let stripped_line = line2.split("//").next().unwrap_or(&line2).trim();
                if let Some(char) = stripped_line.chars().last() {
                    char_sep = match char {
                        ';' => Some((';', "OTHER")),
                        ',' => Some((',', "COMMA")),
                        x => {
                            warn!(
                                "{}: invalid sep detected: {:?}",
                                path.canonicalize().unwrap().display(),
                                x
                            );
                            write!(writer, "{}{}", line2, delimiter2)?;
                            continue;
                        }
                    };
                }
            }

            if line2.is_empty() || line2.contains('}') {
                if char_sep.is_none() {
                    count = 0;
                    continue;
                }

                break;
            }
        }

        let count = count - 1;

        reader.seek_relative(-(num_bytes_count as i64))?;
        let (char, sep) = char_sep.expect("erm");

        if count < 3 || count > 10 {
            warn!(
                "{}: invalid shuffle detected: {}",
                path.canonicalize().unwrap().display(),
                count
            );
            continue;
        }

        write!(
            writer,
            "LUAVM_SHUFFLE{}(LUAVM_SHUFFLE_{},{}",
            count, sep, delimiter
        )?;

        for i in 0..count {
            let (mut line, _, delimiter) = read_line(&mut reader)?.expect("failed to read line?");
            if i == count - 1 {
                line = line.replace(char, "");
            } else {
                line = line.replace(char, ",");
            }
            write!(writer, "{}{}", line, delimiter)?;
        }

        write!(writer, "){}{}", char, delimiter)?;
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
    insert_calls(path, reader, temp_file).expect("shuffles: failed.");

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

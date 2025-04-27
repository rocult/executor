use std::env;

include!("./src/shuffles.rs");
include!("./src/encryptions/mod.rs");

const PRE_REPLACE: [(&str, [(&str, &str); 1]); 1] = [
    (
        "VM/src/lobject.h",
        [
            (
                "uint8_t tt; uint8_t marked; uint8_t memcat",
                "LUAVM_SHUFFLE3(LUAVM_SHUFFLE_OTHER, uint8_t tt, uint8_t marked, uint8_t memcat)"
            )
        ]
    )
];

const BINDINGS_REPLACE: &[(&str, &str)] = &[
    (
        "pub static mut Luau_list: *mut Luau_FValue<T>;",
        "pub static mut Luau_list: *mut Luau_FValue<i32>;"
    ),
];

fn main() {
    // Add (and update) VM shuffles
    if !do_shuffles() {
        return
    }

    // Do some replacements before bindgen
    let official_luau_path = PathBuf::from("../official_luau");
    for (file_path, replacements) in PRE_REPLACE {
        let file_path = official_luau_path.join(file_path);
        let mut file_content = read_to_string(&file_path).expect("failed to find file");

        for (from, to) in replacements {
            file_content = file_content.replace(from, to)
        }

        fs::write(file_path, file_content)
            .expect("failed to write file");
    }

    // Configure the bindgen
    let bindings = bindgen::Builder::default()
        .header("../official_luau/VM/src/lobject.h")
        .header("../official_luau/VM/src/lstate.h")
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

    // Modify the bindings to fix some issues
    for (from, to) in BINDINGS_REPLACE {
        bindings_content = bindings_content.replace(from, to);
    }

    let mut syntax_tree = syn::parse_file(&bindings_content).unwrap();
    do_encryptions(&mut syntax_tree);

    // Write the modified bindings back to the file
    fs::write(&bindings_path, prettyplease::unparse(&syntax_tree))
        .expect("Couldn't write modified bindings!");
}
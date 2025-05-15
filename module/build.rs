fn main() -> Result<(), std::env::VarError> {
    let this_dir = std::env::var("CARGO_MANIFEST_DIR")?;
    println!("cargo:rustc-link-arg=/DEF:{this_dir}\\export.def");
    Ok(())
}

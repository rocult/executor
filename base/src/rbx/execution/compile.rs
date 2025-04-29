use std::io::{Cursor, Write};

use mlua::Error;
use xxhash_rust::xxh32::xxh32;

pub fn compile_script(source: &str) -> mlua::Result<Vec<u8>> {
    // Compile to bytecode with custom encoder
    let compiler = mlua::Compiler::new()
        .set_optimization_level(2)
        .set_debug_level(1);
    let bytecode = compiler.compile(source)?; // need to use Luau::compile 
    let bytecode_len = bytecode.len();

    // Compress the bytecode, adding RSB1 and uncompressed length, as a prefix
    let compressed = zstd::encode_all(Cursor::new(bytecode), zstd::zstd_safe::max_c_level()).unwrap();
    let mut buffer = Vec::with_capacity(12 + compressed.len());
    buffer.write(b"RSB1").map_err(|e| Error::runtime(format!("{e}")))?;
    buffer.write(&(bytecode_len as u64).to_le_bytes()).map_err(|e| Error::runtime(format!("{e}")))?;
    buffer.extend(compressed);

    // Calculate the key
    let key = xxh32(&buffer, 42); 
    let key_bytes = key.to_le_bytes();

    // XOR encrypt the bytecode using the key
    for (i, byte) in buffer.iter_mut().enumerate() {
        let xor_val = key_bytes[i % key_bytes.len()].wrapping_add((i as u8).wrapping_mul(41));
        *byte ^= xor_val;
    }
    Ok(buffer)
}
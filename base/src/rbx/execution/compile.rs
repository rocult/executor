use std::{ffi::CString, io::{Cursor, Write}};

use luau::compile::BytecodeEncoderVmt;
use mlua::Error;
use vtable_rs::VPtr;
use xxhash_rust::xxh32::xxh32;

#[derive(Default)]
#[repr(C)]
struct RustBytecodeEncoder {
    vftable: VPtr<dyn BytecodeEncoderVmt, Self>,
}
impl BytecodeEncoderVmt for RustBytecodeEncoder {
    extern "C" fn encode(&self, data: *mut u32, count: usize) {
        unsafe {
            let mut i = 0_isize;
            while i < count as isize {
                let opcode = data.offset(i);
                i += luau::getOpLength(*opcode as i32) as isize;
                *opcode *= 227;
            }
        }
    }
}

pub fn compile_script(source: &str) -> mlua::Result<Vec<u8>> {
    // Compile to bytecode with custom encoder
    let bytecode = unsafe {
        use luau::compile::*;

        let source = CString::new(source).map_err(|_| Error::runtime("invalid source"))?;
        let compile_options = CompileOptions {
            optimizationLevel: 2,
            debugLevel: 1,
            typeInfoLevel: 2,
            ..Default::default()
        };
        let parse_options = ParseOptions {
            allowDeclarationSyntax: true,
            captureComments: true,
            ..Default::default()
        };
        let encoder = {
            Box::leak(Box::new(RustBytecodeEncoder::default()))
        };
        
        compile(
            source.as_ptr(),
            &compile_options as *const CompileOptions,
            &parse_options as *const ParseOptions,
            encoder
        )
    };
    let bytecode = bytecode.as_bytes();
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
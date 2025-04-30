#[repr(C)]
pub struct BytecodeEncoderVTable {
    pub encode: unsafe extern "C" fn(*const BytecodeEncoderWrapper, *mut u32, usize),
    pub drop: unsafe extern "C" fn(*mut BytecodeEncoderWrapper),
}

#[repr(C)]
pub struct BytecodeEncoderWrapper {
    pub vtable: *const BytecodeEncoderVTable,
    pub rust_object: Box<dyn BytecodeEncoderTrait>,
}

impl BytecodeEncoderWrapper {
    pub fn new(rust_object: Box<dyn BytecodeEncoderTrait>) -> Self {
        let vtable = BytecodeEncoderVTable {
            encode: encode_fn,
            drop: drop_fn,
        };

        Self {
            vtable: Box::into_raw(Box::new(vtable)),
            rust_object,
        }
    }
}

unsafe extern "C" fn encode_fn(wrapper: *const BytecodeEncoderWrapper, data: *mut u32, count: usize) {
    let rust_object = &(*wrapper).rust_object;
    rust_object.encode(data, count);
}

unsafe extern "C" fn drop_fn(wrapper: *mut BytecodeEncoderWrapper) {
    let _ = Box::from_raw(wrapper);
}

pub trait BytecodeEncoderTrait {
    fn encode(&self, data: *mut u32, count: usize);
}

use std::ffi::{CString, NulError};

use crate::rbx::PRINT;

pub enum PrintType {
    Info = 0,
    Warning = 1,
    Error = 2,
}

pub fn println<T: AsRef<str>>(print_type: PrintType, message: T) -> Result<(), NulError> {
    let c_str = CString::new(message.as_ref())?;
    unsafe {
        PRINT(
            print_type as i32,
            c_str.as_ptr() as *const i8,
        );
    };
    Ok(())
}
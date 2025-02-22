use std::ffi::CStr;

use mlua::{ffi, prelude::*};

pub fn getnamecallmethod(state: &Lua, _: ()) -> LuaResult<String> {
    unsafe {
        let mut ptr = std::ptr::null();
        state.exec_raw::<()>((), |state: *mut ffi::lua_State| {
            ptr = ffi::lua_namecallatom(state, std::ptr::null_mut());
        })?;

        if ptr.is_null() {
            Err(LuaError::runtime("could not getnamecallmethod"))?
        }

        Ok(CStr::from_ptr(ptr).to_string_lossy().into_owned())
    }
}
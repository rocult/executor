use std::ffi::c_void;

use mlua::{ffi, prelude::*};

pub fn setnamecallmethod(state: &Lua, method: LuaString) -> LuaResult<()> {
    unsafe {
        // state.exec_raw::<()>((), |state: *mut ffi::lua_State| {
        //     let namecall_ptr = state.add(LUA_STATE_NAMECALL) as *mut *const c_void;
        //     *namecall_ptr = method.to_pointer();
        // })?;
    }

    Ok(())
}
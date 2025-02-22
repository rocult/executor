use mlua::{ffi, lua_State, prelude::*};

pub fn islclosure(state: &Lua, func: LuaFunction) -> LuaResult<bool> {
    let mut result = false;
    unsafe {
        state.exec_raw::<LuaFunction>(func, |state: *mut lua_State| {
            result = ffi::lua_isLfunction(state, -1) != 0;
        })?;
    }
    Ok(result)
}
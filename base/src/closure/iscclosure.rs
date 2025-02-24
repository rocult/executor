use mlua::prelude::*;

use crate::Closure;

pub fn iscclosure(_: &Lua, func: LuaFunction) -> LuaResult<bool> {
    let closure = Closure::new(&func);
    Ok(closure.isC != 0)
}
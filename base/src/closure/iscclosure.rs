use mlua::prelude::*;

use crate::extensions::ClosureGuard;

pub fn iscclosure(_: &Lua, func: LuaFunction) -> LuaResult<bool> {
    let closure = ClosureGuard::new(&func);
    
    Ok(closure.isC != 0)
}
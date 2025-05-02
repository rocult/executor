use mlua::prelude::*;

use crate::safe::ClosureGuard;

pub fn iscclosure(_: &Lua, func: LuaFunction) -> LuaResult<bool> {
    let closure = ClosureGuard::new(&func);
    
    Ok(closure.isC != 0)
}
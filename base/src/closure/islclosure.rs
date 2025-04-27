use mlua::prelude::*;

use crate::extensions::ClosureGuard;

pub fn islclosure(_: &Lua, func: LuaFunction) -> LuaResult<bool> {
    let closure = ClosureGuard::new(&func);
    Ok(closure.isC == 0)
}
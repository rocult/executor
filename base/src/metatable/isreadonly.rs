use mlua::prelude::*;

pub fn isreadonly(_: &Lua, object: LuaTable) -> LuaResult<bool> {
    Ok(object.is_readonly())
}
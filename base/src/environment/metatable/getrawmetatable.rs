use mlua::prelude::*;

pub fn getrawmetatable(_: &Lua, table: LuaTable) -> LuaResult<Option<LuaTable>> {
    Ok(table.metatable())
}
use mlua::prelude::*;

pub fn setrawmetatable(_: &Lua, (object, metatable): (LuaTable, LuaTable)) -> LuaResult<()> {
    object.set_metatable(Some(metatable));
    Ok(())
}
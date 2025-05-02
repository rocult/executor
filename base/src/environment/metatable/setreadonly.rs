use mlua::prelude::*;

pub fn setreadonly(_: &Lua, (object, read_only): (LuaTable, bool)) -> LuaResult<()> {
    object.set_readonly(read_only);
    Ok(())
}
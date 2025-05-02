use mlua::Lua;

mod closure;
mod debug;
mod metatable;

pub fn initialise(state: &Lua) -> mlua::Result<()> {
    closure::register(state)?;
    metatable::register(state)?;
    Ok(())
}

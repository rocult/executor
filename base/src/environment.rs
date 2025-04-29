use mlua::Lua;

use crate::{closure, metatable};

pub fn initialise(state: &Lua) -> mlua::Result<()> {
    closure::register(state)?;
    metatable::register(state)?;
    Ok(())
}

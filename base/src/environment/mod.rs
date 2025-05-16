use mlua::Lua;

mod closure;
mod debug;
mod metatable;

pub fn initialise(state: &Lua) -> mlua::Result<()> {
    closure::register(state)?;
    metatable::register(state)?;
    Ok(())
}

macro_rules! import_register {
    ($($module:tt,)*) => {
        $(
            mod $module;
            pub use $module::*;
        )*

        pub fn register(state: &::mlua::prelude::Lua) -> ::mlua::prelude::LuaResult<()> {
            let globals = state.globals();
            $(
                globals.set(
                    stringify!($module),
                    state.create_function($module::$module)?
                )?;
            )*

            Ok(())
        }
    };
}
pub(crate) use import_register;

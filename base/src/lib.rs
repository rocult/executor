#[macro_export]
macro_rules! import {
    ($($module:tt,)*) => {
        $(
            mod $module;
            pub use $module::*;
        )*
    };
}

#[macro_export]
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

import!(
    closure,
    metatable,
    offsets,

    extensions,
);
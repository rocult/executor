#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

// NOTE: some shuffles aren't properly applied.
// for example, CommonHeader needs to be done manually and stuff like stringtable, CallInfo, etc.
// bindings for stuff like lua_State and typenames aren't properly generated either.

// include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

pub mod compiler {
    include!(concat!(env!("OUT_DIR"), "/luau_compiler.rs"));
}

include!(concat!(env!("OUT_DIR"), "/luau_vm.rs"));

mod encryptions;
pub use encryptions::*;

mod update;
pub use update::*;

pub mod shuffles;

pub mod compile;

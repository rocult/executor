use mlua::lua_State;

use crate::rbx::{DataModel, Instance, ScriptContext, TaskJob, TaskScheduler};

use super::import_offsets;

impl Instance {
    pub const PARENT: isize = 0x10;
    pub const CHILDREN: isize = 0x70;
    pub const CLASS_NAME: isize = 0x8;
    pub const CLASS_DESCRIPTOR: isize = 0x18;
}

impl DataModel {
    pub const PADDING: isize = 0x10;
    pub const INSTANCE: isize = 0x10;
}

impl ScriptContext {
    pub const GLOBAL_STATE: isize = 0x10;
    pub const DECRYPT_STATE: isize = 0x10;
}

impl TaskJob {
    pub const JOB_NAME: isize = 0x90;
}

impl TaskScheduler {
    pub const JOBS_START: isize = 0x198;
    pub const JOBS_END: isize = 0x1A0;

    pub const RENDER_VIEW: isize = 0x10;
}

pub type GetGlobalStateForInstanceFn = unsafe extern "fastcall" fn(
    arg0: *const usize, 
    arg1: *const usize, 
    arg2: *const usize
) -> *const usize;

pub type DecryptStateFn = unsafe extern "fastcall" fn(
    arg0: *const usize
) -> *const lua_State;

import_offsets! {
    GET_GLOBAL_STATE_FOR_INSTANCE<GetGlobalStateForInstanceFn> => 0xD72200,
    DECRYPT_STATE<DecryptStateFn> => 0x88,
}
use std::{ffi::{c_int, c_void}, os::raw::c_char};

use mlua::lua_State;

use crate::rbx::{DataModel, Instance, ScriptContext, TaskJob, TaskScheduler};

use super::import_offsets;

pub struct ExtraSpace;
impl ExtraSpace {
    pub const IDENTITY: isize = 0x30;
    pub const CAPABILITIES: isize = 0x48;
    pub const SCRIPT: isize = 0x50;
    pub const ACTOR: isize = 0x58;
}

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
    pub const CONTEXT_OBJECT: isize = 0x10;
}

impl TaskJob {
    pub const JOB_NAME: isize = 0x90;
}

impl TaskScheduler {
    pub const JOBS_START: isize = 0x198;
    pub const JOBS_END: isize = 0x1A0;

    pub const RENDER_VIEW: isize = 0x10;
}

import_offsets! {
    GET_TASK_SCHEDULER<GetTaskSchedulerFn> => 0x0,
    GET_GLOBAL_STATE_FOR_INSTANCE<GetGlobalStateForInstanceFn> => 0xD72200,
    DECRYPT_STATE<DecryptStateFn> => 0x88,
    LUA_VM_LOAD<LuaVmLoadFn> => 0x88,
    GET_CONTEXT_OBJECT<GetContextObjectFn> => 0x88,
    SET_PROTO_CAPABILITIES<SetProtoCapabilitiesFn> => 0x88,
    TASK_DEFER<TaskDeferFn> => 0x88,
}

pub type GetTaskSchedulerFn = unsafe extern "fastcall" fn () -> *const usize;

pub type GetGlobalStateForInstanceFn = unsafe extern "fastcall" fn(
    arg0: *const usize, 
    arg1: *const usize, 
    arg2: *const usize
) -> *const usize;

pub type DecryptStateFn = unsafe extern "fastcall" fn(
    arg0: *const usize
) -> *const lua_State;

pub type LuaVmLoadFn = unsafe extern "fastcall" fn(
    arg0: *const lua_State,
    arg1: *const c_void,
    arg2: *const c_char,
    arg3: c_int
) -> c_int;

pub type GetContextObjectFn = unsafe extern "fastcall" fn(
    arg0: *const usize,
    arg1: *const usize,
) -> *const usize;

pub type SetProtoCapabilitiesFn = unsafe extern "fastcall" fn(
    arg0: *const luau::Proto,
    arg1: *const usize,
) -> c_void;

pub type TaskDeferFn = unsafe extern "fastcall" fn(
    arg0: *const lua_State
) -> c_int;
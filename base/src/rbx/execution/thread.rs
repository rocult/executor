use std::{ffi::{c_void, CString}, time::Duration};

use mlua::{ffi::LUA_TFUNCTION, Error, Lua, ObjectLike, Result, Thread};

use crate::rbx::{ExtraSpace, ScriptContext, TaskScheduler, GET_CONTEXT_OBJECT, LUA_VM_LOAD, SET_PROTO_CAPABILITIES, TASK_DEFER};

use super::compile_script;

pub enum ThreadCapabilities {
    Max,
    Custom(i64)
}
impl ThreadCapabilities {
    const MAX: *const i64 = (0x200000000000003F_i64 | 0x3FFFFFF00_i64) as *const i64;
}
impl From<ThreadCapabilities> for i64 {
    fn from(value: ThreadCapabilities) -> Self {
        match value {
            ThreadCapabilities::Max => Self::MAX as i64,
            ThreadCapabilities::Custom(x) => x,
        }
    }
}

pub trait Execution {
    fn set_thread_identity(&self, identity: u8) -> Result<()>;
    fn set_thread_capabilities(&self, capabilities: ThreadCapabilities) -> Result<()>;
    fn send(&self, source: String, compile: bool, ms_yield_time: u64, task_scheduler: &TaskScheduler) -> Result<Thread>;
}

impl Execution for Lua {
    fn set_thread_identity(&self, identity: u8) -> Result<()> {
        unsafe {
            self.exec_raw((), |raw_state| {
                let userdata_ptr = raw_state.offset(0x78);
                let identity_ptr = userdata_ptr.offset(ExtraSpace::IDENTITY) as *mut i64;
                *identity_ptr = identity as i64;
            })
        }
    }

    fn set_thread_capabilities(&self, capabilities: ThreadCapabilities) -> Result<()> {
        unsafe {
            self.exec_raw((), |raw_state| {
                let userdata_ptr = raw_state.offset(0x78);
                let capabilities_ptr = userdata_ptr.offset(ExtraSpace::CAPABILITIES) as *mut i64;
                *capabilities_ptr = capabilities.into();
            })
        }
    }

    fn send(&self, source: String, compile: bool, ms_yield_time: u64, task_scheduler: &TaskScheduler) -> Result<Thread> {
        // So we don't waste execution time
        if source.is_empty() {
            return Err(Error::runtime("empty source given"))?;
        }

        // Yield execution for some reason?
        if ms_yield_time > 0 {
            std::thread::sleep(Duration::from_millis(ms_yield_time));
        }

        let script_context = task_scheduler.script_context().ok_or(Error::runtime("unable to get script context"))?;
        let f = self.create_function(move |state, ()| {
            // Enable sandbox and set level 8
            state.sandbox(true)?;
            state.set_thread_identity(8)?;
            state.set_thread_capabilities(ThreadCapabilities::Max)?;

            // Create a new LocalScript to hold our code
            let instance: mlua::AnyUserData = state.globals().get("Instance")?;
            let instance_new: mlua::Function = instance.get("new")?;
            instance_new.call::<()>("LocalScript")?;

            // Attempt to compile and load the script
            let script = if compile {
                compile_script(&source)?
            } else {
                source.as_bytes().to_vec()
            };
            let lua_vm_load = unsafe { *LUA_VM_LOAD.get() };

            let mut error: Option<Error> = None;
            unsafe {
                state.exec_raw::<()>((), |raw_state| {
                    // Load the script into the VM
                    let base = CString::new("@Base").unwrap(); // should not error since there aren't any nul bytes
                    let load_result = lua_vm_load(raw_state, script.as_ptr() as *const c_void, base.as_ptr(), 0);
                    if load_result != 0 {
                        error = Some(Error::runtime(format!("unable to load script (error code {load_result}")));
                        return
                    }

                    // Set proto capabilities if there is one
                    let closure = *luau::luaA_toobject(raw_state as *mut luau::lua_State, -1);
                    if closure.tt == LUA_TFUNCTION {
                        let closure = (*(closure.value.gc)).cl;
                        let Some(proto) = (closure.isC == 1).then(|| closure.__bindgen_anon_1.l.p) else {
                            return;
                        };

                        let get_context_object = *GET_CONTEXT_OBJECT.get();
                        let context_object = get_context_object(script_context.offset(ScriptContext::CONTEXT_OBJECT), ThreadCapabilities::MAX as *const usize);

                        let set_proto_capabilities = *SET_PROTO_CAPABILITIES.get();
                        set_proto_capabilities(proto.get(), context_object);
                    }

                    // Continue the thread on the next heartbeat
                    let task_defer = *TASK_DEFER.get();
                    task_defer(raw_state);
                })
            }?;

            if let Some(err) = error {
                return Err(err)?;
            }

            Ok(())
        })?;

        // Finally, create the execution thread
        self.create_thread(f)
    }
}
use std::{borrow::Cow, ffi::c_void, ops::Deref, rc::Rc};

use mlua::{lua_State, Lua};

use super::{RenderView, ScriptContext, DECRYPT_STATE, GET_GLOBAL_STATE_FOR_INSTANCE};

pub struct TaskJob(*const c_void);
impl TaskJob {
    fn name(&self) -> Cow<'_, str> {
        unsafe {
            let ptr = self.0.offset(Self::JOB_NAME) as *mut std::ffi::CString;
            (*ptr).to_string_lossy()
        }
    }
}
impl Deref for TaskJob {
    type Target = *const c_void;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub struct TaskScheduler {
    base: *const c_void,
    lua_state: Option<Rc<Lua>>,
}
impl TaskScheduler {
    fn new() -> Self {
        Self {
            base: std::ptr::null(),
            lua_state: None
        }
    }

    fn iter(&self) -> TaskSchedulerIterator {
        let base = self.base;
        unsafe {
            TaskSchedulerIterator {
                base,
                count: *(self.base.offset(Self::JOBS_START) as *const u64),
                jobs_end: *(self.base.offset(Self::JOBS_END) as *const u64),
            }
        }
    }

    fn job_by_name(&self, name: &str) -> Option<TaskJob> {
        self.iter().find(|x| x.name() == name)
    }

    fn render_view(&self) -> Option<RenderView> {
        self.job_by_name("RenderJob").map(|x| unsafe {
            (*(x.offset(Self::RENDER_VIEW) as *const RenderView)).clone()
        })
    }

    fn lua_state(&mut self) -> Rc<Lua> {
        if let Some(lua_state) = &self.lua_state {
            return lua_state.clone();
        }

        let mut state_index: [usize; 1] = [0];
        let mut actor_index: [usize; 2] = [0, 0];

        // Get a pointer to the global state function.
        let global_state = self.render_view().map(|x| x.data_model()).and_then(|x| x.script_context()).map(|x| x.global_state()).expect("unable to find global state");
        let get_global_state = unsafe { *GET_GLOBAL_STATE_FOR_INSTANCE.get() };

        // Call the function and add the decryption offset.
        let state_addr = unsafe { get_global_state(global_state, state_index.as_mut_ptr(), actor_index.as_mut_ptr()) };
        let full_addr = unsafe { state_addr.offset(ScriptContext::DECRYPT_STATE) };

        // Decrypt the state pointer into a lua_State pointer.
        let decrypt_state = unsafe { *DECRYPT_STATE.get() };
        let lua_state = Rc::new(unsafe { Lua::init_from_ptr(decrypt_state(full_addr) as *mut lua_State) });
        self.lua_state = Some(lua_state.clone());
        lua_state
    }
}

pub struct TaskSchedulerIterator {
    base: *const c_void,
    jobs_end: u64,
    count: u64,
}
impl Iterator for TaskSchedulerIterator {
    type Item = TaskJob;

    fn next(&mut self) -> Option<Self::Item> {
        let result = unsafe {
            match self.count {
                x if x as u64 > self.jobs_end => None,
                x => Some(TaskJob(self.base.offset(x as isize))),
            }
        };

        self.count += 0x10;
        result
    }
}

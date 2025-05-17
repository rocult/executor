use std::{borrow::Cow, ffi::c_void, ops::Deref, rc::Rc, sync::Arc};

use crate::logger::prelude::*;
use mlua::{lua_State, Lua, Result, Error};
use parking_lot::lock_api::ReentrantMutex;

use crate::HB_ORIGINAL_VF;

use super::{RenderView, ScriptContext, CHECK_TASK_SCHEDULER, DECRYPT_STATE, GET_GLOBAL_STATE_FOR_INSTANCE, TASK_SCHEDULER, TASK_SCHEDULER_2};

pub type JobOriginalVFn = unsafe extern "fastcall" fn (
    arg0: *const usize,
    arg1: *const usize,
    arg2: *const usize,
) -> *const usize;

pub struct TaskJob(pub *const usize);
impl TaskJob {
    pub fn name(&self) -> String {
        unsafe {
            // Offset to the job name field
            let name_field = self.0.wrapping_byte_add(Self::JOB_NAME);
            // Offset to the indicator (length/capacity)
            let indicator = *(self.0.wrapping_byte_add(0x30)) as usize;

            let cxx_string_ptr = if indicator >= 0x10 {
                // Long string: name_field points to a pointer to CxxString
                let ptr_to_ptr = name_field as *const *const cxx::CxxString;
                *ptr_to_ptr
            } else {
                // Short string: name_field is a pointer to CxxString
                name_field as *const cxx::CxxString
            };

            info!("job name address: {:#x}", cxx_string_ptr as usize);
            let x = &*cxx_string_ptr;
            x.to_string()
        }
    }
}
impl Deref for TaskJob {
    type Target = *const usize;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub struct TaskScheduler {
    pub base: *const usize,
    lua_state: Option<Rc<Lua>>,
}
impl TaskScheduler {
    pub fn new() -> Self {
        let base_ptr = if unsafe { CHECK_TASK_SCHEDULER() } == 0 {
            info!("task scheduler 2 detected");
            *TASK_SCHEDULER_2
        } else {
            info!("task scheduler 1 detected");
            *TASK_SCHEDULER
        };

        Self {
            base: unsafe { *(base_ptr as *const *const usize) },
            lua_state: None
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

    pub fn script_context(&self) -> Option<ScriptContext> {
        self.render_view().map(|x| x.data_model()).and_then(|x| x.script_context())
    } 

    pub fn hook_job(&self, name: &str, cycle: JobOriginalVFn) -> Result<()> {
        let Some(job) = self.job_by_name(name).map(|x| x.0) else {
            return Ok(())
        };
        if job.is_null() {
            return Ok(());
        }

        unsafe {
            let orig_vtable = job as *mut *mut c_void;
            if orig_vtable.is_null() {
                return Ok(());
            }

            let mut vtable: Vec<*mut c_void> = vec![std::ptr::null_mut(); 25];
            std::ptr::copy_nonoverlapping(orig_vtable, vtable.as_mut_ptr(), 25);

            HB_ORIGINAL_VF
                .set(Arc::new(ReentrantMutex::new(std::mem::transmute(vtable[2]))))
                .map_err(|_| Error::runtime("job vf already set"))?;
            vtable[2] = cycle as *mut c_void;

            *(job as *mut *mut *mut c_void) = vtable.as_mut_ptr(); // TODO: need to check if this is correct

            // Prevent Rust from dropping the vector (which would free our vtable).
            std::mem::forget(vtable);
        }

        Ok(())
    }

    pub fn lua_state(&mut self) -> Result<Rc<Lua>> {
        if let Some(lua_state) = &self.lua_state {
            return Ok(lua_state.clone());
        }

        let mut state_index: [usize; 1] = [0];
        let mut actor_index: [usize; 2] = [0, 0];

        // Get a pointer to the global state function.
        let global_state = self
            .script_context()
            .map(|x| x.global_state())
            .ok_or(Error::runtime("unable to find global state"))?;

        // Call the function and add the decryption offset.
        let state_addr = unsafe { GET_GLOBAL_STATE_FOR_INSTANCE(global_state, state_index.as_mut_ptr(), actor_index.as_mut_ptr()) };
        let full_addr = unsafe { state_addr.offset(ScriptContext::DECRYPT_STATE) };

        // Decrypt the state pointer into a lua_State pointer.
        let lua_state = Rc::new(unsafe { Lua::init_from_ptr(DECRYPT_STATE(full_addr) as *mut lua_State) });
        self.lua_state = Some(lua_state.clone());
        Ok(lua_state)
    }

    pub fn iter(&self) -> TaskSchedulerIterator {
        let ptr_size = std::mem::size_of::<*const ()>();
        let current_ptr = self.base.wrapping_byte_add(Self::JOBS_START) as *const *const *const usize;
        let current = unsafe { *current_ptr };
        let jobs_end = unsafe { *current_ptr.wrapping_byte_add(ptr_size) };
        info!(
            "task scheduler ptr: {}, jobs start: {}, end: {}",
            current_ptr.rebase_display(),
            current.rebase_display(),
            jobs_end.rebase_display()
        );
        TaskSchedulerIterator {
            current,
            jobs_end,
        }
    }
}

pub struct TaskSchedulerIterator {
    current: *const *const usize,
    jobs_end: *const *const usize,
}
impl Iterator for TaskSchedulerIterator {
    type Item = TaskJob;

    fn next(&mut self) -> Option<Self::Item> {
        let result = match self.current {
            x if x >= self.jobs_end => {
                info!("end of jobs at address {}", x.rebase_display());
                None
            },
            x => {
                // info!("got task job: {}", x.rebase_display());
                Some(TaskJob(unsafe { *x }))
            },
        };

        self.current = self.current.wrapping_byte_add(0x10);
        result
    }
}

use std::{collections::VecDeque, sync::{mpsc, Arc, Mutex, MutexGuard}, time::Duration};

use mlua::{lua_State, Function, Lua, Thread};
use once_cell::sync::OnceCell;
use parking_lot::ReentrantMutex;
use rbx::{Execution, JobOriginalVFn, TaskScheduler};

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

mod closure;
mod metatable;
mod extensions;
mod rbx;
mod environment;

pub static JOB_ORIGINAL_VF: OnceCell<Arc<ReentrantMutex<JobOriginalVFn>>> = OnceCell::new();
pub static HEARTBEAT_TX: OnceCell<mpsc::Sender<()>> = OnceCell::new();

extern "fastcall" fn heartbeat(
    arg0: *const usize,
    arg1: *const usize,
    arg2: *const usize,
) -> *const usize {
    if let Some(tx) = HEARTBEAT_TX.get() {
        let _ = tx.send(());
    }

    let vf = JOB_ORIGINAL_VF.get().unwrap().lock();
    unsafe { vf(arg0, arg1, arg2) }
}

fn executor_thread(task_scheduler: &TaskScheduler, state: &Lua) -> mlua::Result<()> {
    // Initialise _G and shared globals to our own
    let g_table = state.create_table()?;
    let shared = state.create_table()?;
    let globals = state.globals();
    globals.set("_G", g_table)?;
    globals.set("shared", shared)?;
    
    // Initialise the environment
    environment::initialise(state)?;
    
    // Sandbox the thread to make stuff read only
    state.sandbox(true)?;
    
    // Ran on heartbeat
    let mut script_queue = VecDeque::<String>::new();
    let (tx, rx) = mpsc::channel::<()>();
    HEARTBEAT_TX.set(tx).expect("unable to set heartbeat tx oncecell");
    while let Ok(_) = rx.recv() {
        if let Some(script) = script_queue.pop_front() {
            if let Err(e) = state.send(script, true, 0, task_scheduler) {
                println!("error: {e}");
            }
        }
    }
    Ok(())
}

pub fn main() -> mlua::Result<Thread> {
    // Initialise the scheduler
    let mut task_scheduler = TaskScheduler::new();
    let lua_state = task_scheduler.lua_state()?;
    task_scheduler.hook_job("Heartbeat", heartbeat)?;
    
    // Handles creating and initialising the executor thread
    let f = lua_state.create_function(move |state, ()| {
        executor_thread(&task_scheduler, state)
    })?;

    // Create the executor thread
    let _thread = lua_state.create_thread(f)?;

    // Infinite loop, keeps thread alive
    loop {
        std::thread::park();
    };
}
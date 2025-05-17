use std::{
    collections::VecDeque,
    io::{BufReader, Read},
    rc::Rc,
    sync::{mpsc, Arc, Mutex},
};

use interprocess::os::windows::named_pipe::RecvPipeStream;
use logger::{setup_logger, prelude::*};
use mlua::{Error, Lua, Result, Thread};
use once_cell::sync::OnceCell;
use parking_lot::ReentrantMutex;
use rbx::{Execution, JobOriginalVFn, TaskScheduler, PRINT};

mod safe;

#[macro_export]
macro_rules! import {
    ($($module:tt,)*) => {
        $(
            mod $module;
            pub use $module::*;
        )*
    };
}

mod environment;
mod logger;
mod rbx;

pub static HB_ORIGINAL_VF: OnceCell<Arc<ReentrantMutex<JobOriginalVFn>>> = OnceCell::new();
pub static HEARTBEAT_TX: OnceCell<mpsc::Sender<()>> = OnceCell::new();

extern "fastcall" fn heartbeat(
    arg0: *const usize,
    arg1: *const usize,
    arg2: *const usize,
) -> *const usize {
    if let Some(tx) = HEARTBEAT_TX.get() {
        let _ = tx.send(());
    }

    let vf = HB_ORIGINAL_VF.get().unwrap().lock();
    unsafe { vf(arg0, arg1, arg2) }
}

fn executor_thread(
    task_scheduler: &TaskScheduler,
    state: &Lua,
    script_queue: Rc<Mutex<VecDeque<String>>>,
) -> Result<()> {
    // Initialise _G and shared globals to our own
    debug!("Initialising globals");
    let g_table = state.create_table()?;
    let shared = state.create_table()?;
    let globals = state.globals();

    debug!("Setting globals");
    globals.set("_G", g_table)?;
    globals.set("shared", shared)?;

    // Initialise the environment
    debug!("Initialising environment");
    environment::initialise(state)?;
    debug!("Environment initialised");

    // Sandbox the thread to make stuff read only
    debug!("Sandboxing thread");
    state.sandbox(true)?;
    debug!("Thread sandboxed");

    // Ran on heartbeat
    let (tx, rx) = mpsc::channel::<()>();
    HEARTBEAT_TX
        .set(tx)
        .map_err(|_| Error::runtime("unable to set heartbeat tx"))?;
    while let Ok(_) = rx.recv() {
        debug!("Heartbeat");
        let mut script_queue = script_queue
            .lock()
            .map_err(|_| Error::runtime("failed to get lock on script queue"))?;
        if let Some(script) = script_queue.pop_front() {
            debug!("Running script");
            if let Err(e) = state.send(script, true, 0, task_scheduler) {
                println!("error: {e}");
            }
        }
    }
    Ok(())
}

pub fn main() -> Result<Thread> {
    setup_logger()
        .map_err(|err| Error::runtime(format!("failed to setup logger: {err}")))?;

    // Initialise the scheduler
    debug!("initialising task scheduler from base {:#x}", *rbx::BASE);
    let mut task_scheduler = TaskScheduler::new();
    debug!(
        "task scheduler initialised at address {}",
        task_scheduler.base.rebase_display()
    );

    std::thread::sleep(std::time::Duration::from_secs(3));

    let lua_state = task_scheduler.lua_state()?;
    debug!("Lua state initialised");
    // task_scheduler.hook_job("Heartbeat", heartbeat)?;
    // debug!("Heartbeat job hooked");

    // // Handles creating and initialising the executor thread
    // let script_queue = Rc::new(Mutex::new(VecDeque::new()));
    // let script_queue_2 = script_queue.clone();
    // let f = lua_state.create_function(move |state, ()| {
    //     executor_thread(&task_scheduler, state, script_queue_2.clone())
    // })?;

    // // Create the executor thread
    // debug!("Creating executor thread");
    // let thread = lua_state.create_thread(f)?;
    // debug!("Executor thread created");

    // // Keep reading from named pipe, for scripts to run
    // debug!("Waiting for scripts");
    // let rx = RecvPipeStream::connect_by_path(r"\\.\pipe\rblx")
    //     .map_err(|err| Error::runtime(format!("unable to create named pipe: {err}")))?;
    // let mut rx = BufReader::new(rx);
    // let mut buffer = String::with_capacity(128);
    // while let Ok(_) = rx.read_to_string(&mut buffer) {
    //     debug!("got script");
    //     let mut script_queue = script_queue
    //         .lock()
    //         .map_err(|_| Error::runtime("failed to get lock on script queue"))?;
    //     script_queue.push_back(buffer.clone());
    //     buffer.clear();
    // }

    // // Keep thread active
    // Ok(thread)

    loop {
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}

use std::{
    collections::VecDeque,
    io::{BufReader, Read},
    rc::Rc,
    sync::{mpsc, Arc, Mutex},
};

use interprocess::os::windows::named_pipe::RecvPipeStream;
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

type MPrint = unsafe extern "fastcall" fn(i32, *const i8) -> usize;
fn rbx_print(base: usize, message: &str) {
    let message = std::ffi::CString::new(message).unwrap();
    let rbx_printf: MPrint = unsafe { std::mem::transmute(base + 0x16D2D00) };
    unsafe { rbx_printf(0, message.as_ptr()) };
}

fn executor_thread(
    task_scheduler: &TaskScheduler,
    state: &Lua,
    script_queue: Rc<Mutex<VecDeque<String>>>,
) -> Result<()> {
    let print = |message_type: i32, message: &str| {
        let c_str = std::ffi::CString::new(message).unwrap();
        unsafe { PRINT(message_type, c_str.as_ptr() as *const i8) }
    };

    // Initialise _G and shared globals to our own
    print(0, "Initialising globals");
    let g_table = state.create_table()?;
    let shared = state.create_table()?;
    let globals = state.globals();

    print(0, "Setting globals");
    globals.set("_G", g_table)?;
    globals.set("shared", shared)?;

    // Initialise the environment
    print(0, "Initialising environment");
    environment::initialise(state)?;
    print(0, "Environment initialised");

    // Sandbox the thread to make stuff read only
    print(0, "Sandboxing thread");
    state.sandbox(true)?;
    print(0, "Thread sandboxed");

    // Ran on heartbeat
    let (tx, rx) = mpsc::channel::<()>();
    HEARTBEAT_TX
        .set(tx)
        .map_err(|_| Error::runtime("unable to set heartbeat tx"))?;
    while let Ok(_) = rx.recv() {
        print(0, "Heartbeat");
        let mut script_queue = script_queue
            .lock()
            .map_err(|_| Error::runtime("failed to get lock on script queue"))?;
        if let Some(script) = script_queue.pop_front() {
            print(0, "Running script");
            if let Err(e) = state.send(script, true, 0, task_scheduler) {
                println!("error: {e}");
            }
        }
    }
    Ok(())
}

pub fn main() -> Result<Thread> {
    let print = |message_type: i32, message: &str| {
        let c_str = std::ffi::CString::new(message).unwrap();
        unsafe { PRINT(message_type, c_str.as_ptr()) }
    };

    // Initialise the scheduler
    print(
        0,
        &format!("Initialising task scheduler from base {:#x}", *rbx::BASE),
    );
    let mut task_scheduler = TaskScheduler::new();
    print(
        0,
        &format!(
            "Task scheduler initialised at address {:#x}",
            task_scheduler.base as usize
        ),
    );

    task_scheduler.iter().for_each(|x| {
        print(0, &format!("Task job: {:?}", x.0));
        // print(0, &format!("Task job name: {:?}", x.name()));
    });
    // let lua_state = task_scheduler.lua_state()?;
    // print(0, "Lua state initialised");
    // task_scheduler.hook_job("Heartbeat", heartbeat)?;
    // print(0, "Heartbeat job hooked");

    // // Handles creating and initialising the executor thread
    // let script_queue = Rc::new(Mutex::new(VecDeque::new()));
    // let script_queue_2 = script_queue.clone();
    // let f = lua_state.create_function(move |state, ()| {
    //     executor_thread(&task_scheduler, state, script_queue_2.clone())
    // })?;

    // // Create the executor thread
    // print(0, "Creating executor thread");
    // let thread = lua_state.create_thread(f)?;
    // print(0, "Executor thread created");

    // // Keep reading from named pipe, for scripts to run
    // print(0, "Waiting for scripts");
    // let rx = RecvPipeStream::connect_by_path(r"\\.\pipe\rblx")
    //     .map_err(|err| Error::runtime(format!("unable to create named pipe: {err}")))?;
    // let mut rx = BufReader::new(rx);
    // let mut buffer = String::with_capacity(128);
    // while let Ok(_) = rx.read_to_string(&mut buffer) {
    //     print(0, "got script");
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

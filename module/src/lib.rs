use windows::{core::{w, PCWSTR}, Win32::{Foundation::*, System::{LibraryLoader::{GetModuleHandleW, LoadLibraryW}, SystemServices::*, Threading::{CreateThread, THREAD_CREATE_RUN_IMMEDIATELY}}, UI::WindowsAndMessaging::{MessageBoxW, MB_OK, MESSAGEBOX_RESULT}}};

/// A stub for the forwarded function.
#[unsafe(no_mangle)]
fn run() {
}

type MPrint = unsafe extern "fastcall" fn(i32, *const i8) -> usize;
fn rbx_print(base: usize, message: &str) {
    let message = std::ffi::CString::new(message).unwrap();
    let rbx_printf: MPrint = unsafe { std::mem::transmute(base + 0x16D2D00) };
    unsafe { rbx_printf(0, message.as_ptr()) };
}

fn show_message(message: PCWSTR) -> MESSAGEBOX_RESULT {
    unsafe {
        MessageBoxW(
            None,
            message,
            w!("Message"),
            MB_OK,
        )
    }
}

fn attach() -> bool {
    // Sideload the actual DLL
    let result = unsafe { LoadLibraryW(w!("bRobloxPlayerBeta.dll")) };
    if result.is_err() {
        show_message(w!("Failed to load bRobloxPlayerBeta.dll"));
        return false;
    }

    // Show a message box to pause execution until the user closes it
    show_message(w!("DLL loaded successfully, press OK to continue."));

    // Call the main function of the base module
    let base = unsafe { GetModuleHandleW(None) }.map(|x| x.0 as usize);
    if base.is_err() {
        show_message(w!("Failed to get module handle."));
        return false;
    }

    std::thread::spawn(move || {
        rbx_print(base.unwrap(), "helloooo from rust");
    }).join().unwrap();
    true
    // base::main().is_ok()
}

unsafe extern "system" fn attach_thread(_lp_param: *mut std::ffi::c_void) -> u32 {
    attach();
    0
}

#[unsafe(no_mangle)]
#[allow(non_snake_case, unused_variables)]
extern "system" fn DllMain(_: HINSTANCE, call_reason: u32, _: *mut ()) -> bool {
    match call_reason {
        DLL_PROCESS_ATTACH => {
            // Avoid hanging the main thread
            // This is needed, and must be done this way.
            //
            // `std::thread::spawn` is not safe to use here, as it will cause the main thread to hang.
            // WinAPI must be used to create a thread.
            unsafe {
                if let Ok(hobject) = CreateThread(
                    None,
                    0,
                    Some(attach_thread),
                    None,
                    THREAD_CREATE_RUN_IMMEDIATELY,
                    None,
                ) {
                    let _ = CloseHandle(hobject);
                }
            }

            true
        },
        DLL_PROCESS_DETACH => true,
        _ => true,
    }
}

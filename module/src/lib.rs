use windows::{Win32::Foundation::*, Win32::System::SystemServices::*};

#[unsafe(no_mangle)]
#[allow(non_snake_case, unused_variables)]
extern "system" fn DllMain(dll_module: HINSTANCE, call_reason: u32, _: *mut ()) -> bool {
    match call_reason {
        DLL_PROCESS_ATTACH => base::main().is_ok(),
        DLL_PROCESS_DETACH => true,
        _ => true,
    }
}

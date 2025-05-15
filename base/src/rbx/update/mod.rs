import!(
    offsets,
);

use once_cell::sync::Lazy;
use windows::Win32::System::LibraryLoader::GetModuleHandleA;

pub static BASE: Lazy<usize> = Lazy::new(|| {
    let base = unsafe { GetModuleHandleA(None) };
    if base.is_err() {
        panic!("Failed to get module handle");
    }
    base.unwrap().0 as usize
});

macro_rules! import_offsets {
    ($($name:ident<$ty:ty> => $offset:expr),* $(,)?) => {
        $(
            pub static $name: ::once_cell::sync::Lazy<$ty> = ::once_cell::sync::Lazy::new(|| {
                unsafe {
                    std::mem::transmute::<usize, $ty>($crate::rbx::BASE.wrapping_add($offset))
                }
            });
        )*
    };
}

pub(crate) use import_offsets;
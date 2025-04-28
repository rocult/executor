import!(
    offsets,
);

use std::marker::PhantomData;

use windows::Win32::System::LibraryLoader::GetModuleHandleA;

pub struct RebaseOffset<T> {
    offset: isize,
    phantom: PhantomData<T>,
}
impl<T> RebaseOffset<T> {
    pub const fn new(offset: isize) -> Self {
        Self { offset, phantom: PhantomData }
    }

    pub unsafe fn get(&self) -> *const T {
        let handle = GetModuleHandleA(None).unwrap();
        handle.0.offset(self.offset) as *const T
    }
}

macro_rules! import_offsets {
    ($($name:ident<$ty:ty> => $offset:expr),* $(,)?) => {
        $(
            pub const $name: $crate::rbx::RebaseOffset<$ty> = $crate::rbx::RebaseOffset::new($offset);
        )*
    };
}

pub(crate) use import_offsets;
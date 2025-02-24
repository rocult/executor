use std::ops::{Deref, DerefMut};

use mlua::Function;

pub struct Closure<'a> {
    _reference: &'a Function,
    inner: &'a mut luau::Closure,
}
impl<'a> Closure<'a> {
    pub fn new(reference: &'a Function) -> Self {
        Closure {
            _reference: &reference,
            // Safety:
            // This should be fine because it's what Luau does under the hood.
            // This reference should remain valid, and it shouldn't be GCd until the `_reference` is dropped.
            inner: unsafe { &mut *(reference.to_pointer() as *mut luau::Closure) },
        }
    }
}
impl<'a> Deref for Closure<'a> {
    type Target = luau::Closure;
    fn deref(&self) -> &Self::Target {
        self.inner
    }
}
impl<'a> DerefMut for Closure<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.inner
    }
}
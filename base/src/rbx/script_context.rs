use std::ops::Deref;

use super::Instance;

pub struct ScriptContext(pub Instance);
impl Deref for ScriptContext {
    type Target = Instance;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl ScriptContext {
    pub fn global_state(&self) -> *const usize {
        unsafe { self.wrapping_byte_add(Self::GLOBAL_STATE) }
    }
}
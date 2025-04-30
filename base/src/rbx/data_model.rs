use std::ops::Deref;

use super::{Instance, ScriptContext};

pub struct DataModel(pub Instance);
impl Deref for DataModel {
    type Target = Instance;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DataModel {
    pub fn script_context(&self) -> Option<ScriptContext> {
        self.find_first_child_of_class("ScriptContext").map(ScriptContext)
    }
}
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
        self
            .children()
            .find(|x| x.class_name() == "ScriptContext")
            .map(ScriptContext)
    }
}
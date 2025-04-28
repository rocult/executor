use std::ops::Deref;

use super::{DataModel, Instance};

#[derive(Clone)]
pub struct RenderView(Instance);
impl Deref for RenderView {
    type Target = Instance;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl RenderView {
    pub fn visual_engine(&self) -> VisualEngine {
        VisualEngine(self.parent())
    }

    pub fn data_model(&self) -> DataModel {
        unsafe {
            DataModel(
                Instance(
                    self.offset(DataModel::PADDING + DataModel::INSTANCE)
                )
            )
        }
    }
}

pub struct VisualEngine(Instance);
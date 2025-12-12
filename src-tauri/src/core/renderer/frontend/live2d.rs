use std::{collections::HashMap, path::PathBuf, sync::Arc};

use live2d_cubism_core_sys::core as live2d;

use super::super::backend::RenderState;

pub struct Live2dFrontend {
    moc: live2d::Moc,
    model: live2d::Model,
    expressions: HashMap<String, ()>
}

impl Live2dFrontend {
    pub fn new(dir: PathBuf) -> Self {
        todo!()
    }
}

impl super::Frontend for Live2dFrontend {
    fn render(&self, state: Arc<RenderState>) {
        // Live2D rendering logic goes here
    }
}
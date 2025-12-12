use std::{path::PathBuf, sync::Arc};

use super::super::backend::RenderState;

pub struct VrmFrontend {

}

impl VrmFrontend {
    pub fn new(model: PathBuf) -> Self {
        Self {

        }
    }
}

impl super::Frontend for VrmFrontend {
    fn render(&self, state: Arc<RenderState>) {
        // VRM rendering logic goes here
    }
}
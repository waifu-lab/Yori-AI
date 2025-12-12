mod live2d;
mod vrm;

pub use live2d::Live2dFrontend;
pub use vrm::VrmFrontend;

use std::sync::Arc;

use super::backend::RenderState;

pub trait Frontend: Send + Sync {
    fn render(&self, state: Arc<RenderState>);
}
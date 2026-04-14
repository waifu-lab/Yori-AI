#[cfg(feature = "live2d")]
mod live2d;
mod vrm;

#[cfg(feature = "live2d")]
pub use live2d::Live2dFrontend;
pub use vrm::VrmFrontend;

use std::sync::Arc;

use super::backend::RenderState;

pub trait Frontend: Send + Sync {
    fn render(&self, state: Arc<RenderState>);
}

#[cfg(not(feature = "live2d"))]
pub struct Live2dFrontend;

#[cfg(not(feature = "live2d"))]
impl Live2dFrontend {
    pub fn new(_: std::path::PathBuf) -> Self {
        panic!("Live2D support is disabled. Enable the `live2d` feature and set LIVE2D_CUBISM_SDK_NATIVE_DIR.")
    }
}

#[cfg(not(feature = "live2d"))]
impl Frontend for Live2dFrontend {
    fn render(&self, _: Arc<RenderState>) {}
}
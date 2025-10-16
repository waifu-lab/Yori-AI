use std::path::PathBuf;

use live2d_cubism_core_sys::core as live2d;

pub struct Live2dModel {
    path: PathBuf,
    model: live2d::Model,
}
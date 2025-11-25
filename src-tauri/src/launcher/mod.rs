use crate::{schema::{ProjectControlMode, ProjectData}};

pub trait Launcher: Send + Sync {
    fn launch(&self) -> anyhow::Result<i32>;
}

pub fn create_launcher(project: ProjectData) -> Box<dyn Launcher> {
    todo!()
}
use crate::{launcher::Launcher, models::ProjectData};

pub struct DesktopLauncher {
    project: ProjectData,
}

impl DesktopLauncher {
    pub fn new(project: ProjectData) -> Self {
        Self { project }
    }
}

impl Launcher for DesktopLauncher {
    fn launch(&self) -> anyhow::Result<i32> {
        // Implement the logic to launch a desktop application here
        println!("Launching desktop application for project: {}", self.project.name());
        Ok(0)
    }
}
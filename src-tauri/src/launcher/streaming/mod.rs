use crate::{launcher::Launcher, domain::ProjectData};

pub struct StreamingLauncher {
    project: ProjectData,
}

impl StreamingLauncher {
    pub fn new(project: ProjectData) -> Self {
        Self { project }
    }
}

impl Launcher for StreamingLauncher {
    fn launch(&self) -> anyhow::Result<i32> {
        // Implement the logic to launch a streaming application here
        println!("Launching streaming application for project: {}", self.project.name());
        Ok(0)
    }
}
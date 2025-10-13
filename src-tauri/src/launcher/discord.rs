use crate::{models::ProjectData, launcher::Launcher};

pub struct DiscordLauncher {
    project: ProjectData,
}

impl DiscordLauncher {
    pub fn new(project: ProjectData) -> Self {
        Self { project }
    }
}

impl Launcher for DiscordLauncher {
    fn launch(&self) -> anyhow::Result<i32> {
        // Implement the logic to launch a Discord application here
        println!("Launching Discord application for project: {}", self.project.name());
        Ok(0)
    }
}

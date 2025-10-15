use crate::{domain::{ProjectControlMode, ProjectData}};
use desktop::DesktopLauncher;
use streaming::StreamingLauncher;
use discord::DiscordLauncher;

mod streaming;
mod desktop;
mod discord;

pub trait Launcher: Send + Sync {
    fn launch(&self) -> anyhow::Result<i32>;
}

pub fn create_launcher(project: ProjectData) -> Box<dyn Launcher> {
    match project.mode() {
        ProjectControlMode::Desktop => Box::new(DesktopLauncher::new(project)),
        ProjectControlMode::Streaming => Box::new(StreamingLauncher::new(project)),
        ProjectControlMode::Discord => Box::new(DiscordLauncher::new(project)),
    }
}
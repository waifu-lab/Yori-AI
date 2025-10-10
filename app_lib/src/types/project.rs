use std::time;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Serialize, Deserialize)]
pub enum ProjectControlMode {
    Streaming,
    Desktop,
    Discord,
    // Add more modes as needed
}

pub trait ProjectManager: Send + Sync {
    fn get_recent_projects(&self) -> Vec<Box<dyn Project>>;
    fn add_recent_project(&self, project: Box<dyn Project>) -> anyhow::Result<()>;
}

pub trait Project: Send + Sync {
    fn name(&self) -> String;
    fn path(&self) -> String;
    fn description(&self) -> String;
    fn mode(&self) -> ProjectControlMode;
    fn plugins(&self) -> Vec<String>;

    fn created_at(&self) -> time::SystemTime;
    fn last_opened(&self) -> time::SystemTime;

    fn update_name(&self, name: &str);
    fn update_description(&self, description: &str);
    fn update_last_opened(&self, time: time::SystemTime);

    fn add_plugin(&self, plugin_id: &str);
    fn remove_plugin(&self, plugin_id: &str);
}
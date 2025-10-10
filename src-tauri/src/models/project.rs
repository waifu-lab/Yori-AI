use std::time;
use serde::{Deserialize, Serialize};
use parking_lot::RwLock;

use crate::utils::serde_transform;
use app_lib::types::{Project, ProjectControlMode};

#[derive(Serialize, Deserialize)]
pub struct ProjectImpl {
    #[serde(
        serialize_with = "serde_transform::remove_lock",
        deserialize_with = "serde_transform::add_lock"
    )]
    name: RwLock<String>,
    path: String,
    #[serde(
        serialize_with = "serde_transform::remove_lock",
        deserialize_with = "serde_transform::add_lock"
    )]
    description: RwLock<String>,
    #[serde(
        serialize_with = "serde_transform::remove_lock",
        deserialize_with = "serde_transform::add_lock"
    )]
    mode: RwLock<ProjectControlMode>,
    #[serde(
        serialize_with = "serde_transform::remove_lock",
        deserialize_with = "serde_transform::add_lock"
    )]
    plugins: RwLock<Vec<String>>,
    created_at: time::SystemTime,
    #[serde(
        serialize_with = "serde_transform::remove_lock",
        deserialize_with = "serde_transform::add_lock"
    )]
    last_opened: RwLock<time::SystemTime>,
}

impl Project for ProjectImpl {
    fn name(&self) -> String {
        self.name.read().clone()
    }

    fn path(&self) -> String {
        self.path.clone()
    }

    fn description(&self) -> String {
        self.description.read().clone()
    }

    fn mode(&self) -> ProjectControlMode {
        *self.mode.read()
    }

    fn plugins(&self) -> Vec<String> {
        self.plugins.read().clone()
    }

    fn created_at(&self) -> time::SystemTime {
        self.created_at.clone()
    }

    fn last_opened(&self) -> time::SystemTime {
        self.last_opened.read().clone()
    }

    fn update_name(&self, name: &str) {
        *self.name.write() = name.to_string();
    }

    fn update_description(&self, description: &str) {
        *self.description.write() = description.to_string();
    }

    fn update_last_opened(&self, time: time::SystemTime) {
        self.last_opened.write().clone_from(&time);
    }

    fn add_plugin(&self, plugin_id: &str) {
        self.plugins.write().push(plugin_id.to_string());
    }

    fn remove_plugin(&self, plugin_id: &str) {
        self.plugins.write().retain(|p| p != plugin_id);
    }
}

impl ProjectImpl {
    pub fn new(name: &str, path: &str, description: &str, mode: ProjectControlMode) -> Self {
        Self {
            name: RwLock::new(name.to_string()),
            path: path.to_string(),
            description: RwLock::new(description.to_string()),
            mode: RwLock::new(mode),
            plugins: RwLock::new(Vec::new()),
            created_at: time::SystemTime::now(),
            last_opened: RwLock::new(time::SystemTime::now()),
        }
    }
}
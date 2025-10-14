use std::sync::Arc;

use parking_lot::RwLock;
use crate::managers::{ProjectManager, PluginManager};

pub struct AppState {
    pub project_manager: Arc<RwLock<ProjectManager>>,
    pub plugin_manager: Arc<RwLock<PluginManager>>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            project_manager: Arc::new(RwLock::new(ProjectManager::new())),
            plugin_manager: Arc::new(RwLock::new(PluginManager::new())),
        }
    }
}
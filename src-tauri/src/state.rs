use parking_lot::RwLock;
use crate::managers::ProjectManager;

pub struct AppState {
    pub project_manager: RwLock<ProjectManager>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            project_manager: RwLock::new(ProjectManager::new()),
        }
    }
}
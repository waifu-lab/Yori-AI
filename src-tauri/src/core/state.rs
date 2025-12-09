use std::sync::Arc;

use parking_lot::RwLock;

use crate::core::project::Project;

pub struct AppConsts {
    app_root: std::path::PathBuf,
}

pub struct AppMemory {
    projects: RwLock<Vec<Arc<Project>>>,
}

impl AppConsts {
    pub fn new<R: tauri::Runtime>(manager: impl tauri::Manager<R>) -> Self {
        let resolver = manager.path();
        let app_root = resolver.app_data_dir().unwrap();
        Self { app_root }
    }

    pub fn app_root(&self) -> std::path::PathBuf {
        self.app_root.clone()
    }
}

impl AppMemory {
    pub fn new() -> Self {
        Self {
            projects: RwLock::new(Vec::new()),
        }
    }

    pub fn add_project(&self, project: Arc<Project>) {
        let mut projects = self.projects.write();
        projects.push(project);
    }
}
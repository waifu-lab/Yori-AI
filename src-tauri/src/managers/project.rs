use crate::{
    domain::ProjectData,
    utils::storage::load_projects
};

pub struct ProjectManager {
    app: Option<tauri::AppHandle>,
    projects: Vec<ProjectData>,
    launching_project: Option<uuid::Uuid>,
}

impl ProjectManager {
    pub fn new() -> Self {
        Self {
            app: None,
            projects: Vec::new(),
            launching_project: None,
        }
    }

    pub fn set_handle(&mut self, app: &tauri::AppHandle) {
        self.app = Some(app.clone());
    }

    pub fn get_handle(&self) -> Option<tauri::AppHandle> {
        self.app.clone()
    }

    pub fn get_recent_projects(&self) -> Vec<ProjectData> {
        self.projects.iter().take(10).cloned().collect()
    }

    pub fn get_project(&self, id: uuid::Uuid) -> Option<ProjectData> {
        self.projects.iter().find(|p| p.id() == id).cloned()
    }

    pub fn add_project(&mut self, project: ProjectData) {
        self.projects.push(project);
    }

    pub fn remove_project(&mut self, id: uuid::Uuid) -> bool {
        if let Some(pos) = self.projects.iter().position(|p| p.id() == id) {
            self.projects.remove(pos);
            true
        } else {
            false
        }
    }

    pub fn toggle_launching(&mut self, id: uuid::Uuid) {
        if let Some(current) = self.launching_project {
            if current == id {
                self.launching_project = None;
            } else {
                self.launching_project = Some(id);
            }
        } else {
            self.launching_project = Some(id);
        }
    }

    pub fn initialize(&mut self) -> anyhow::Result<()> {
        if let Some(app) = &self.app {
            self.projects = load_projects(app)?;
            Ok(())
        } else {
            Err(anyhow::anyhow!("App handle is not set"))
        }
    }
}
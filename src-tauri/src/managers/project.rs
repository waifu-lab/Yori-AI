use crate::{
    models::ProjectData,
    utils::storage::load_projects
};

pub struct ProjectManager {
    app: Option<tauri::AppHandle>,
    projects: Vec<ProjectData>,
}

impl ProjectManager {
    pub fn new() -> Self {
        Self {
            app: None,
            projects: Vec::new(),
        }
    }

    pub fn set_handle(&mut self, app: &tauri::AppHandle) {
        self.app = Some(app.clone());
    }

    pub fn get_recent_projects(&self) -> Vec<&ProjectData> {
        self.projects.iter().take(10).collect()
    }

    pub fn add_project(&mut self, project: ProjectData) {
        self.projects.push(project);
    }

    pub fn remove_project(&mut self, name: &str) -> bool {
        if let Some(pos) = self.projects.iter().position(|p| p.name() == name) {
            self.projects.remove(pos);
            true
        } else {
            false
        }
    }

    pub fn initialize(&mut self) -> anyhow::Result<()> {
        if let Some(app) = &self.app {
            let projects = load_projects(app)?;
            self.projects.clear();
            self.projects.extend(projects);
            Ok(())
        } else {
            Err(anyhow::anyhow!("App handle is not set"))
        }
    }
}
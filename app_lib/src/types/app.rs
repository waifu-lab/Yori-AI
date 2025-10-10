use crate::types::Project;

#[derive(Default)]
pub struct AppState {
    info: AppInfo,
    current_project: Option<Box<dyn Project>>,
}

pub struct AppInfo {
    name: String,
    version: String,
    authors: Vec<String>,
    description: String,

    theme: String,
}

impl Default for AppInfo {
    fn default() -> Self {
        Self {
            name: "AI Vtuber Launcher".to_string(),
            version: "0.1.0".to_string(),
            authors: vec!["Lumina".to_string()],
            description: "A Tauri application for launching AI Vtubers".to_string(),
            theme: "dark".to_string(),
        }
    }
}
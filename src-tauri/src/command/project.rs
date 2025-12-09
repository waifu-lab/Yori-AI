use std::sync::Arc;

use serde::Deserialize;
use tauri::Manager;

use crate::{
    core::{
        project::create_project as create_project_fn,
        state::{AppConsts, AppMemory}
    },
    schema::{ProjectMeta, ProjectMode},
    utils::path
};

#[tauri::command]
pub fn create_project(app: tauri::AppHandle, payload: ProjectCreatePayload) -> Result<ProjectMeta, String> {
    let consts = app.state::<Arc<AppConsts>>();
    let memory = app.state::<Arc<AppMemory>>();
    let app_root = consts.app_root();
    let project_meta = ProjectMeta {
        id: uuid::Uuid::new_v4(),
        name: payload.name.clone(),
        description: payload.description,
        version: payload.version,
        created_at: chrono::Utc::now(),
        data_root: payload.data_root.unwrap_or_else(|| path::project_default_path(app_root, &payload.name)),
        mode: payload.mode,
    };

    let project = Arc::new(create_project_fn(consts.inner().clone(), project_meta.clone())
        .map_err(|e| e.to_string())?);
    project.clone().initialize().map_err(|e| e.to_string())?;
    memory.add_project(project);
    Ok(project_meta)
}

#[derive(Deserialize)]
pub struct ProjectCreatePayload {
    name: String,
    description: String,
    version: String,
    data_root: Option<std::path::PathBuf>,
    mode: ProjectMode,
}
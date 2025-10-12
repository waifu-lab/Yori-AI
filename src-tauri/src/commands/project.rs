use crate::{models::{ProjectControlMode, ProjectData}, state::AppState};

#[tauri::command]
pub fn create_project(
    state: tauri::State<AppState>,
    name: String,
    path: String,
    description: String,
    mode: ProjectControlMode,
) -> Result<uuid::Uuid, String> {
    let mut manager = state.project_manager.write();
    let project = ProjectData::new(&name, &path, &description, mode);
    let id = project.id();
    manager.add_project(project);
    Ok(id)
}
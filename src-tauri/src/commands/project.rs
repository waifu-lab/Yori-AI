use std::thread;

use crate::{
    error::BackendError,
    launcher::create_launcher,
    schema::{ProjectControlMode, ProjectData},
    state::AppState,
    utils::storage::save_project as save_project_fn,
};

#[tauri::command]
pub fn create_project(
    state: tauri::State<AppState>,
    name: String,
    path: String,
    description: String,
    mode: ProjectControlMode,
) -> Result<uuid::Uuid, String> {
    let project = ProjectData::new(&name, &path, &description, mode);
    let id = project.id();

    {
        let manager = state.project_manager.read();
        let app_handle = manager.get_handle().ok_or("App handle not set")?;
        save_project_fn(&app_handle, &project).map_err(|e| e.to_string())?;
    }

    {
        let mut manager = state.project_manager.write();
        manager.add_project(project);
    }

    Ok(id)
}

#[tauri::command]
pub fn delete_project(state: tauri::State<AppState>, id: uuid::Uuid) -> Result<bool, String> {
    let project_name = {
        let manager = state.project_manager.read();
        manager
            .get_project(id)
            .map(|p| p.name())
            .ok_or("Project not found")?
    };

    {
        let manager = state.project_manager.read();
        let app_handle = manager.get_handle().ok_or("App handle not set")?;
        crate::utils::storage::delete_project(&app_handle, &project_name)
            .map_err(|e| e.to_string())?;
    }

    let mut manager = state.project_manager.write();
    Ok(manager.remove_project(id))
}

#[tauri::command]
pub fn save_project(state: tauri::State<AppState>, project: ProjectData) -> Result<(), String> {
    save_project_fn(
        &state.project_manager.read().get_handle().unwrap(),
        &project,
    )
    .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn reload_projects(state: tauri::State<AppState>) -> Result<(), String> {
    let mut manager = state.project_manager.write();
    manager.initialize().map_err(|e| e.to_string())
}

#[tauri::command]
pub fn list_recent_projects(state: tauri::State<AppState>) -> Result<Vec<ProjectData>, String> {
    let manager = state.project_manager.read();
    Ok(manager.get_recent_projects())
}

#[tauri::command]
pub fn launch_project(state: tauri::State<AppState>, id: uuid::Uuid) -> Result<(), String> {
    let manager = state.project_manager.clone();
    let launcher = create_launcher(
        manager
            .read()
            .get_project(id.clone())
            .ok_or(BackendError::ProjectNotFound)
            .map_err(|e| e.to_string())?,
    );
    let (tx, rx) = crossbeam_channel::unbounded::<Result<i32, String>>();
    let launch_thread = move || {
        tx.send(launcher.launch().map_err(|e| e.to_string()))
            .unwrap();
    };

    let id_cloned = id.clone();
    let manager_cloned = state.project_manager.clone();
    let receive_thread = move || {
        match rx.recv() {
            Ok(_exit_code) => {
                manager_cloned.write().toggle_launching(id_cloned);
                // Handle success or failure
            }
            Err(_) => (),
        }
    };

    thread::spawn(launch_thread);
    thread::spawn(receive_thread);
    manager.write().toggle_launching(id);
    Ok(())
}

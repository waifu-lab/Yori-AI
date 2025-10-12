pub use project::*;

mod project;

use crate::state::AppState;

#[tauri::command]
pub fn initialize(state: tauri::State<AppState>) -> Result<(), String> {
    let mut pm = state.project_manager.write();
    pm.initialize().map_err(|e| e.to_string())
}
use crate::{domain::PluginData, state::AppState};

#[tauri::command]
pub fn add_plugin(state: tauri::State<AppState>, file_path: String) -> Result<(), String> {
    let mut pm = state.plugin_manager.write();
    pm.add_plugin(file_path);
    Ok(())
}

#[tauri::command]
pub fn remove_plugin(state: tauri::State<AppState>, plugin_name: String) -> Result<(), String> {
    let mut pm = state.plugin_manager.write();
    pm.remove_plugin(&plugin_name);
    Ok(())
}

#[tauri::command]
pub fn list_plugins(state: tauri::State<AppState>) -> Result<Vec<PluginData>, String> {
    let pm = state.plugin_manager.read();
    Ok(pm.list_plugins())
}
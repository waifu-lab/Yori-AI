use tauri::{AppHandle, Emitter, State};

use crate::waifu_node::{NodeStatus, WaifuNodeManager};

#[tauri::command]
pub fn waifu_node_install(mgr: State<WaifuNodeManager>) -> Result<(), String> {
    mgr.install()
}

#[tauri::command]
pub fn waifu_node_start(mgr: State<WaifuNodeManager>) -> Result<(), String> {
    mgr.start()
}

#[tauri::command]
pub fn waifu_node_stop(mgr: State<WaifuNodeManager>) -> Result<(), String> {
    mgr.stop();
    Ok(())
}

#[tauri::command]
pub fn waifu_node_status(mgr: State<WaifuNodeManager>) -> NodeStatus {
    mgr.status()
}

#[tauri::command]
pub fn waifu_node_is_installed(mgr: State<WaifuNodeManager>) -> bool {
    mgr.is_installed()
}

#[tauri::command]
pub fn waifu_node_install_path(mgr: State<WaifuNodeManager>) -> Option<String> {
    mgr.install_path().map(|p| p.to_string_lossy().into_owned())
}

#[tauri::command]
pub fn waifu_node_set_backend_url(url: String, mgr: State<WaifuNodeManager>) {
    mgr.set_backend_url(url);
}

/// Query the latest release version from GitHub (no download).
#[tauri::command]
pub async fn waifu_node_latest_version() -> Result<String, String> {
    crate::waifu_node::github::latest_version()
        .await
        .map_err(|e| e.to_string())
}

/// Download the latest waifu-node binary from GitHub.
/// Emits `waifu-node-download-progress` events: `{ downloaded: u64, total: u64 }`.
#[tauri::command]
pub async fn waifu_node_download_latest(app: AppHandle) -> Result<String, String> {
    let app_clone = app.clone();
    let path = crate::waifu_node::github::download_latest(move |downloaded, total| {
        app_clone.emit("waifu-node-download-progress", serde_json::json!({
            "downloaded": downloaded,
            "total": total,
        })).ok();
    })
    .await
    .map_err(|e| e.to_string())?;

    Ok(path.to_string_lossy().into_owned())
}

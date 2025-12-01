#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Arc;

use tauri::Manager;

use crate::core::state::AppMemory;

// base modules
mod core;
mod command;

// extra systems
mod extra;

// utilities
mod schema;
mod utils;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_store::Builder::new().build())
        .invoke_handler(tauri::generate_handler![
            command::create_project,
        ])
        .setup(|app| {
            let handle = app.handle();
            app.manage(Arc::new(core::state::AppConsts::from_manager(handle.clone())));
            app.manage(Arc::new(AppMemory::new()));
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

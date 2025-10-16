#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Manager;

use crate::utils::logger;

#[macro_use]
extern crate rust_i18n;

mod commands;
mod event;
mod core;
mod domain;
mod managers;
mod utils;

mod error;
mod state;
mod launcher;

i18n!("locales", fallback = "zh-TW");

#[cfg_attr(mobile, tauri::mobile_entry_point)]
fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_store::Builder::new().build())
        .manage(state::AppState::new())
        .invoke_handler(tauri::generate_handler![
            commands::create_project,
            commands::delete_project,
            commands::save_project,
            commands::reload_projects,
            commands::list_recent_projects,
            commands::launch_project,
            commands::add_plugin,
            commands::remove_plugin,
            commands::list_plugins,
        ])
        .setup(|app| {
            let state = app.state::<state::AppState>();
            {
                let mut pm = state.project_manager.write();
                pm.set_handle(app.handle());
                pm.initialize().expect("Failed to initialize ProjectManager");
            }
            {
                let mut plm = state.plugin_manager.write();
                plm.set_handle(app.handle());
                plm.initialize().expect("Failed to initialize PluginManager");
            }

            logger::init(app.handle(), state.log_level).expect("Failed to initialize logger");
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

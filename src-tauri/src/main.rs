#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Manager;

#[macro_use]
extern crate rust_i18n;

mod commands;
mod models;
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
            commands::initialize,
            commands::create_project,
            commands::delete_project,
            commands::save_project,
            commands::list_recent_projects,
            commands::launch_project,
        ])
        .setup(|app| {
            app.state::<state::AppState>()
                .project_manager.write()
                .set_handle(&app.handle());
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

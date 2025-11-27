#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// base modules
mod core;
mod command;

// sub systems
mod account;
mod plugin;

// utilities
mod utils;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_store::Builder::new().build())
        .invoke_handler(tauri::generate_handler![
            
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Arc;

use tauri::Manager;

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
            app.manage(Arc::new(core::state::AppConsts::new(handle.clone())));
            app.manage(Arc::new(core::state::AppMemory::new()));

            use core::renderer::backend::{RENDER_BACKEND, SHAERD_STATE, RenderBackend};
            let monitor = app.primary_monitor().unwrap().unwrap();
            RENDER_BACKEND.set(Arc::new(pollster::block_on(RenderBackend::new()).unwrap())).ok();
            SHAERD_STATE.set(Arc::new(RENDER_BACKEND.wait().clone().d2_state(*monitor.size()))).ok();

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

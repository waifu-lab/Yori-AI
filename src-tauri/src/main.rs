#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Arc;

use parking_lot::RwLock;
use tauri::Manager;

// base modules
mod app;
mod command;
mod core;

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
        .invoke_handler(tauri::generate_handler![command::create_project,])
        .setup(|app| {
            let handle = app.handle();
            app.manage(Arc::new(core::state::AppConsts::new(handle.clone())));
            app.manage(Arc::new(core::state::AppMemory::new()));

            use core::renderer::backend::{RENDER_BACKEND, RenderBackend, SHAERD_STATE};
            let monitor = app.primary_monitor().unwrap().unwrap();
            RENDER_BACKEND
                .set(Arc::new(pollster::block_on(RenderBackend::new()).unwrap()))
                .ok();
            SHAERD_STATE
                .set(RwLock::new(
                    RENDER_BACKEND.wait().clone().d2_state(*monitor.size()),
                ))
                .ok();
            let _tray = app::build_tray(app)?;

            Ok(())
        })
        .on_window_event(|window, event| {
            if window.label() != "overlay" {
                return;
            }

            match event {
                tauri::WindowEvent::Resized(size)
                | tauri::WindowEvent::ScaleFactorChanged {
                    new_inner_size: size,
                    ..
                } => {
                    use core::renderer::backend::{state_read, state_write};
                    if *size == state_read().size() {
                        return;
                    }

                    let mut state = state_write();
                    *state = state.resize(*size);
                }
                _ => (),
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

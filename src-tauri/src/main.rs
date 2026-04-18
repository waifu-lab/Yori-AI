#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod command;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
fn main() {
    let live2d_server = command::live2d_window::start_live2d_file_server()
        .expect("failed to start Live2D file server");

    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_store::Builder::new().build())
        .manage(live2d_server)
        .invoke_handler(tauri::generate_handler![
            command::live2d_window::open_live2d_window,
            command::live2d_window::close_live2d_window,
            command::live2d_window::read_file,
            command::live2d_window::set_model_path,
            command::live2d_window::get_current_model_url,
        ])
        .setup(|app| {
            let _tray = app::build_tray(app)?;
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

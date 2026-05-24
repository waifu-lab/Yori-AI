#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod audio;
mod backend;
mod command;
mod core;
mod waifu_node;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
fn main() {
    let live2d_server = command::live2d_window::start_live2d_file_server()
        .expect("failed to start Live2D file server");

    let audio_player = audio::AudioPlayer::new().expect("failed to init audio player");
    let vad_manager = audio::VadManager::new();

    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_store::Builder::new().build())
        .manage(live2d_server)
        .manage(backend::BackendWs::new())
        .manage(waifu_node::WaifuNodeManager::new())
        .manage(core::state::AppState::new())
        .manage(audio_player)
        .manage(vad_manager)
        .manage(command::wander::WanderState::new())
        .invoke_handler(tauri::generate_handler![
            // live2d
            command::live2d_window::open_settings_window,
            command::live2d_window::open_live2d_window,
            command::live2d_window::close_live2d_window,
            command::live2d_window::open_float_chat_window,
            command::live2d_window::read_file,
            command::live2d_window::set_model_path,
            command::live2d_window::get_current_model_url,
            command::live2d_window::get_model_path,
            command::live2d_window::list_expressions,
            command::live2d_window::get_live2d_transform,
            command::live2d_window::set_live2d_transform,
            // backend
            command::backend::backend_connect,
            command::backend::backend_disconnect,
            command::backend::backend_status,
            command::backend::backend_get_url,
            command::backend::backend_set_url,
            command::backend::backend_get_conversation_id,
            command::backend::backend_set_conversation_id,
            command::backend::backend_send_chat,
            command::backend::backend_send_voice,
            command::backend::backend_send_live2d_sync,
            command::backend::live2d_set_lipsync_mode,
            // audio
            command::audio::audio_play,
            command::audio::audio_stop,
            command::audio::audio_set_volume,
            // vad
            command::audio::vad_start,
            command::audio::vad_stop,
            command::audio::vad_status,
            command::audio::list_input_devices,
            command::audio::vad_set_device,
            command::audio::vad_get_device,
            // waifu-node
            command::waifu_node::waifu_node_install,
            command::waifu_node::waifu_node_start,
            command::waifu_node::waifu_node_stop,
            command::waifu_node::waifu_node_status,
            command::waifu_node::waifu_node_is_installed,
            command::waifu_node::waifu_node_install_path,
            command::waifu_node::waifu_node_set_backend_url,
            command::waifu_node::waifu_node_latest_version,
            command::waifu_node::waifu_node_download_latest,
            // wander
            command::wander::set_wander_enabled,
            command::wander::get_wander_enabled,
            command::wander::wander_now,
        ])
        .setup(|app| {
            let handle = app.handle();
            if let Ok(store) = tauri_plugin_store::StoreExt::store(handle, "settings.json") {
                if let Some(url) = store
                    .get(core::state::STORE_KEY_BACKEND_URL)
                    .and_then(|v| v.as_str().map(str::to_owned))
                {
                    tauri::Manager::state::<core::state::AppState>(handle).set_backend_url(url);
                    tauri::Manager::state::<waifu_node::WaifuNodeManager>(handle).set_backend_url(
                        tauri::Manager::state::<core::state::AppState>(handle).get_backend_url(),
                    );
                }
                if let Some(model_path) = store
                    .get(core::state::STORE_KEY_LIVE2D_MODEL_PATH)
                    .and_then(|v| v.as_str().map(str::to_owned))
                {
                    tauri::Manager::state::<core::state::AppState>(handle)
                        .set_live2d_model_path(Some(model_path));
                }
                {
                    let scale = store
                        .get(core::state::STORE_KEY_LIVE2D_SCALE)
                        .and_then(|v| v.as_f64())
                        .unwrap_or(1.0);
                    let offset_x = store
                        .get(core::state::STORE_KEY_LIVE2D_OFFSET_X)
                        .and_then(|v| v.as_f64())
                        .unwrap_or(0.0);
                    let offset_y = store
                        .get(core::state::STORE_KEY_LIVE2D_OFFSET_Y)
                        .and_then(|v| v.as_f64())
                        .unwrap_or(0.0);
                    tauri::Manager::state::<core::state::AppState>(handle)
                        .set_live2d_transform(scale, offset_x, offset_y);
                }
            }

            // Auto-connect backend WS
            {
                let url = tauri::Manager::state::<core::state::AppState>(handle).get_backend_url();
                tauri::Manager::state::<backend::BackendWs>(handle).connect(url, handle.clone());
            }

            command::mouse_track::start_mouse_tracker(app.handle().clone());
            {
                let wander = tauri::Manager::state::<command::wander::WanderState>(handle);
                command::wander::start_wander_loop(
                    handle.clone(),
                    wander.enabled.clone(),
                    wander.trigger_now.clone(),
                );
            }
            let _tray = app::build_tray(app)?;
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

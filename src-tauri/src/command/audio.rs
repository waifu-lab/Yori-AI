use tauri::{AppHandle, State};

use crate::audio::{AudioPlayer, VadManager, list_input_devices as cpal_list_devices};

#[tauri::command]
pub fn audio_play(audio_b64: String, player: State<'_, AudioPlayer>) -> Result<(), String> {
    player.play_b64(&audio_b64)
}

#[tauri::command]
pub fn audio_stop(player: State<'_, AudioPlayer>) {
    player.stop();
}

#[tauri::command]
pub fn audio_set_volume(volume: f32, player: State<'_, AudioPlayer>) {
    player.set_volume(volume);
}

#[tauri::command]
pub fn vad_start(
    app: AppHandle,
    vad: State<'_, VadManager>,
    ws: State<'_, crate::backend::BackendWs>,
) -> Result<(), String> {
    if !ws.is_connected() {
        return Err("後端 WebSocket 尚未連線".into());
    }
    vad.start(app)
}

#[tauri::command]
pub fn vad_stop(vad: State<'_, VadManager>) {
    vad.stop();
}

#[tauri::command]
pub fn vad_status(vad: State<'_, VadManager>) -> bool {
    vad.is_running()
}

#[tauri::command]
pub fn list_input_devices() -> Vec<String> {
    cpal_list_devices()
}

#[tauri::command]
pub fn vad_set_device(device: Option<String>, vad: State<'_, VadManager>) {
    vad.set_device(device);
}

#[tauri::command]
pub fn vad_get_device(vad: State<'_, VadManager>) -> Option<String> {
    vad.get_device()
}

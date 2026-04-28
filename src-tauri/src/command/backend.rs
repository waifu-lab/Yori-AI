use tauri::{AppHandle, State};
use tauri_plugin_store::StoreExt;
use uuid::Uuid;

use crate::backend::{
    BackendWs,
    types::{ChatMsg, ClientMsg, VoiceMsg},
};

use crate::core::state::{AppState, STORE_KEY_BACKEND_URL};

#[tauri::command]
pub fn backend_connect(
    app: AppHandle,
    ws: State<BackendWs>,
    state: State<AppState>,
) -> Result<(), String> {
    let url = state.get_backend_url();
    ws.connect(url, app);
    Ok(())
}

#[tauri::command]
pub fn backend_disconnect(ws: State<BackendWs>) -> Result<(), String> {
    ws.disconnect();
    Ok(())
}

#[tauri::command]
pub fn backend_status(ws: State<BackendWs>) -> bool {
    ws.is_connected()
}

#[tauri::command]
pub fn backend_get_url(state: State<AppState>) -> String {
    state.get_backend_url()
}

#[tauri::command]
pub fn backend_set_url(url: String, app: AppHandle, state: State<AppState>) -> Result<(), String> {
    state.set_backend_url(url.clone());
    let store = app.store("settings.json").map_err(|e| e.to_string())?;
    store.set(STORE_KEY_BACKEND_URL, url);
    store.save().map_err(|e| e.to_string())
}

#[tauri::command]
pub fn backend_get_conversation_id(state: State<AppState>) -> Option<String> {
    state.get_conversation_id()
}

#[tauri::command]
pub fn backend_set_conversation_id(id: Option<String>, state: State<AppState>) {
    state.set_conversation_id(id);
}

#[tauri::command]
pub fn backend_send_chat(
    content: String,
    live2d: Option<bool>,
    ws: State<BackendWs>,
    state: State<AppState>,
) -> Result<String, String> {
    let request_id = Uuid::new_v4().to_string();
    let conversation_id = state.get_conversation_id();
    // Only ask the backend to compute lipsync visemes when Live2D is active AND
    // the loaded model has no native LipSync parameter group.
    let need_lipsync = if live2d == Some(true) {
        Some(state.get_need_backend_lipsync())
    } else {
        None
    };
    ws.send(ClientMsg::Chat(ChatMsg {
        conversation_id,
        request_id: request_id.clone(),
        content,
        live2d,
        need_lipsync,
    }))?;
    Ok(request_id)
}

#[tauri::command]
pub fn live2d_set_lipsync_mode(need_backend_lipsync: bool, state: State<AppState>) {
    state.set_need_backend_lipsync(need_backend_lipsync);
}

#[tauri::command]
pub fn backend_send_voice(
    audio_b64: String,
    ws: State<BackendWs>,
    state: State<AppState>,
) -> Result<String, String> {
    let request_id = Uuid::new_v4().to_string();
    let conversation_id = state.get_conversation_id();
    ws.send(ClientMsg::Voice(VoiceMsg {
        conversation_id,
        request_id: request_id.clone(),
        audio: audio_b64,
        format: "webm".to_string(),
        live2d: Some(true),
    }))?;
    Ok(request_id)
}

use tauri::State;

use crate::audio::AudioPlayer;

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

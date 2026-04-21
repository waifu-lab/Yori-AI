use std::io::Cursor;
use std::sync::mpsc;
use std::thread;

use base64::Engine;
use base64::engine::general_purpose::STANDARD as B64;
use rodio::{Decoder, OutputStream, Sink};

enum AudioCmd {
    Play(Vec<u8>),
    Stop,
    Volume(f32),
}

/// Send+Sync handle — all rodio calls happen on a dedicated thread.
pub struct AudioPlayer {
    tx: mpsc::SyncSender<AudioCmd>,
}

impl AudioPlayer {
    pub fn new() -> Result<Self, String> {
        let (tx, rx) = mpsc::sync_channel::<AudioCmd>(32);

        thread::Builder::new()
            .name("audio-player".into())
            .spawn(move || {
                let (_stream, handle) = match OutputStream::try_default() {
                    Ok(v) => v,
                    Err(e) => {
                        log::error!("audio init failed: {e}");
                        return;
                    }
                };
                let sink = Sink::try_new(&handle).expect("sink");

                for cmd in rx {
                    match cmd {
                        AudioCmd::Play(bytes) => {
                            let cursor = Cursor::new(bytes);
                            match Decoder::new(cursor) {
                                Ok(src) => sink.append(src),
                                Err(e) => log::warn!("audio decode: {e}"),
                            }
                        }
                        AudioCmd::Stop => sink.stop(),
                        AudioCmd::Volume(v) => sink.set_volume(v),
                    }
                }
            })
            .map_err(|e| e.to_string())?;

        Ok(Self { tx })
    }

    pub fn play_b64(&self, audio_b64: &str) -> Result<(), String> {
        let bytes = B64.decode(audio_b64).map_err(|e| format!("base64 decode: {e}"))?;
        self.tx.send(AudioCmd::Play(bytes)).map_err(|e| e.to_string())
    }

    pub fn stop(&self) {
        let _ = self.tx.send(AudioCmd::Stop);
    }

    pub fn set_volume(&self, volume: f32) {
        let _ = self.tx.send(AudioCmd::Volume(volume));
    }
}

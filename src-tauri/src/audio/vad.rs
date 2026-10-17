use std::io::Cursor;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;

use base64::Engine;
use base64::engine::general_purpose::STANDARD as B64;
use cpal::SampleFormat;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use earshot::Detector;
use parking_lot::Mutex;
use tauri::{AppHandle, Emitter, Manager};
use uuid::Uuid;

use crate::backend::BackendWs;
use crate::backend::types::{ClientMsg, FEATURE_LIVE2D, VoiceMsg};
use crate::core::state::AppState;

/// earshot requires exactly 256 i16 samples at 16 KHz per predict call.
const EARSHOT_FRAME: usize = 256;
/// Target sample rate for VAD.
const VAD_RATE: u32 = 16_000;
/// Silence threshold: how many silent VAD frames before we treat an utterance as done.
/// 500 ms / 16 ms per frame ??31 frames.
const SILENCE_FRAMES_END: usize = 31;

pub struct VadManager {
    running: Arc<AtomicBool>,
    stop_tx: Mutex<Option<std::sync::mpsc::SyncSender<()>>>,
    selected_device: Mutex<Option<String>>,
}

pub fn list_input_devices() -> Vec<String> {
    let host = cpal::default_host();
    host.input_devices()
        .map(|iter| iter.filter_map(|d| d.name().ok()).collect())
        .unwrap_or_default()
}

impl VadManager {
    pub fn new() -> Self {
        Self {
            running: Arc::new(AtomicBool::new(false)),
            stop_tx: Mutex::new(None),
            selected_device: Mutex::new(None),
        }
    }

    /// Get the currently selected device name (None = use system default).
    pub fn get_device(&self) -> Option<String> {
        self.selected_device.lock().clone()
    }

    /// Set preferred input device by name. Pass None to use system default.
    pub fn set_device(&self, name: Option<String>) {
        *self.selected_device.lock() = name;
    }

    pub fn is_running(&self) -> bool {
        self.running.load(Ordering::Relaxed)
    }

    /// Start VAD on a background thread. Returns an error string if the mic
    /// cannot be opened or if VAD is already running.
    pub fn start(&self, app: AppHandle) -> Result<(), String> {
        if self.running.swap(true, Ordering::SeqCst) {
            return Err("VAD already running".into());
        }

        let (stop_tx, stop_rx) = std::sync::mpsc::sync_channel::<()>(1);
        *self.stop_tx.lock() = Some(stop_tx);

        let running = Arc::clone(&self.running);
        let device_name = self.selected_device.lock().clone();

        std::thread::Builder::new()
            .name("vad-thread".into())
            .spawn(move || {
                if let Err(e) = run_vad(stop_rx, app, device_name) {
                    log::error!("VAD error: {e}");
                }
                running.store(false, Ordering::SeqCst);
                log::info!("VAD stopped");
            })
            .map_err(|e| e.to_string())?;

        Ok(())
    }

    /// Stop a running VAD session.
    pub fn stop(&self) {
        if let Some(tx) = self.stop_tx.lock().take() {
            let _ = tx.send(());
        }
    }
}

fn run_vad(
    stop_rx: std::sync::mpsc::Receiver<()>,
    app: AppHandle,
    device_name: Option<String>,
) -> Result<(), String> {
    let host = cpal::default_host();
    let device = match device_name {
        Some(ref name) => host
            .input_devices()
            .map_err(|e| e.to_string())?
            .find(|d| d.name().as_deref().ok() == Some(name.as_str()))
            .ok_or_else(|| format!("input device '{name}' not found"))?,
        None => host
            .default_input_device()
            .ok_or("no default input device")?,
    };

    log::info!(
        "VAD: using input device: {}",
        device.name().unwrap_or_default()
    );

    let (stream_config, native_rate, sample_fmt) = choose_config(&device)?;

    let (pcm_tx, pcm_rx) = std::sync::mpsc::sync_channel::<Vec<i16>>(256);

    let stream = match sample_fmt {
        SampleFormat::F32 => {
            let pcm_tx = pcm_tx.clone();
            let sc = stream_config.clone();
            device.build_input_stream(
                &stream_config,
                move |data: &[f32], _| {
                    let channels = sc.channels as usize;
                    let mono: Vec<i16> = data
                        .chunks(channels)
                        .map(|frame| {
                            let avg = frame.iter().sum::<f32>() / channels as f32;
                            (avg * 32767.0).clamp(-32768.0, 32767.0) as i16
                        })
                        .collect();
                    let _ = pcm_tx.try_send(mono);
                },
                |e| log::error!("cpal input error: {e}"),
                None,
            )
        }
        _ => {
            let sc = stream_config.clone();
            device.build_input_stream(
                &stream_config,
                move |data: &[i16], _| {
                    let channels = sc.channels as usize;
                    let mono: Vec<i16> = data
                        .chunks(channels)
                        .map(|frame| {
                            let sum: i32 = frame.iter().map(|&s| s as i32).sum();
                            (sum / channels as i32) as i16
                        })
                        .collect();
                    let _ = pcm_tx.try_send(mono);
                },
                |e| log::error!("cpal input error: {e}"),
                None,
            )
        }
    }
    .map_err(|e| e.to_string())?;

    stream.play().map_err(|e| e.to_string())?;

    let mut resample_buf: Vec<f32> = Vec::new();
    let resample_ratio = VAD_RATE as f64 / native_rate as f64;
    let mut resample_pos: f64 = 0.0;

    let mut detector = Detector::default();
    let mut vad_frame_buf: Vec<i16> = Vec::with_capacity(EARSHOT_FRAME);

    enum VadState {
        Silence,
        Speaking,
        EndingSilence(usize),
    }
    let mut vad_state = VadState::Silence;
    let mut utterance_buf: Vec<i16> = Vec::new();

    let mut volume_emit_counter: usize = 0;

    log::info!("VAD: listening (native {native_rate} Hz)");

    'outer: loop {
        match stop_rx.try_recv() {
            Ok(_) | Err(std::sync::mpsc::TryRecvError::Disconnected) => break 'outer,
            Err(std::sync::mpsc::TryRecvError::Empty) => {}
        }

        let chunk = match pcm_rx.recv_timeout(Duration::from_millis(20)) {
            Ok(c) => c,
            Err(std::sync::mpsc::RecvTimeoutError::Timeout) => continue,
            Err(std::sync::mpsc::RecvTimeoutError::Disconnected) => break 'outer,
        };

        match &vad_state {
            VadState::Speaking | VadState::EndingSilence(_) => {
                utterance_buf.extend_from_slice(&chunk);
            }
            VadState::Silence => {}
        }

        let native_f32: Vec<f32> = chunk.iter().map(|&s| s as f32).collect();
        while resample_pos < native_f32.len() as f64 {
            let idx = resample_pos as usize;
            let frac = resample_pos - idx as f64;
            let s = if idx + 1 < native_f32.len() {
                native_f32[idx] * (1.0 - frac as f32) + native_f32[idx + 1] * frac as f32
            } else {
                *native_f32.last().unwrap_or(&0.0)
            };
            resample_buf.push(s);
            resample_pos += 1.0 / resample_ratio;
        }
        resample_pos -= native_f32.len() as f64;

        let resampled_i16: Vec<i16> = resample_buf.drain(..).map(|s| s as i16).collect();

        vad_frame_buf.extend_from_slice(&resampled_i16);
        while vad_frame_buf.len() >= EARSHOT_FRAME {
            let frame: Vec<i16> = vad_frame_buf.drain(..EARSHOT_FRAME).collect();
            let score = detector.predict_i16(&frame);
            let is_voice = score >= 0.6;

            volume_emit_counter += 1;
            if volume_emit_counter >= 3 {
                volume_emit_counter = 0;
                let rms = (chunk.iter().map(|&s| (s as f64).powi(2)).sum::<f64>()
                    / chunk.len() as f64)
                    .sqrt();
                let db = if rms > 1.0 {
                    20.0 * rms.log10()
                } else {
                    -100.0_f64
                };
                app.emit("vad-debug-volume", db).ok();
            }

            vad_state = match vad_state {
                VadState::Silence => {
                    if is_voice {
                        log::debug!("VAD: speech started (score={score:.2})");
                        app.emit("vad-debug-log", format!("[SPEECH START] score={score:.2}"))
                            .ok();
                        utterance_buf.extend_from_slice(&chunk);
                        VadState::Speaking
                    } else {
                        VadState::Silence
                    }
                }
                VadState::Speaking => {
                    if is_voice {
                        VadState::Speaking
                    } else {
                        VadState::EndingSilence(1)
                    }
                }
                VadState::EndingSilence(n) => {
                    if is_voice {
                        VadState::Speaking
                    } else if n >= SILENCE_FRAMES_END {
                        let buf = std::mem::take(&mut utterance_buf);
                        if !buf.is_empty() {
                            let duration_ms = buf.len() as u32 * 1000 / native_rate;
                            app.emit(
                                "vad-debug-log",
                                format!("[UTTERANCE] {duration_ms}ms → sending to WS"),
                            )
                            .ok();
                            send_utterance(&buf, native_rate, &app);
                        }
                        app.emit("vad-debug-log", "[SILENCE]").ok();
                        VadState::Silence
                    } else {
                        VadState::EndingSilence(n + 1)
                    }
                }
            };
        }
    }

    drop(stream);
    Ok(())
}

fn choose_config(device: &cpal::Device) -> Result<(cpal::StreamConfig, u32, SampleFormat), String> {
    use cpal::SampleRate;

    let supported = device
        .supported_input_configs()
        .map_err(|e| e.to_string())?;

    // Priority: f32@16k > i16@16k > f32@any > i16@any
    let mut f32_16k: Option<cpal::SupportedStreamConfigRange> = None;
    let mut i16_16k: Option<cpal::SupportedStreamConfigRange> = None;
    let mut f32_any: Option<(cpal::SupportedStreamConfigRange, u32)> = None;
    let mut i16_any: Option<(cpal::SupportedStreamConfigRange, u32)> = None;

    for range in supported {
        let fmt = range.sample_format();
        if fmt != SampleFormat::F32 && fmt != SampleFormat::I16 {
            continue;
        }
        let min = range.min_sample_rate().0;
        let max = range.max_sample_rate().0;
        let supports_16k = min <= VAD_RATE && VAD_RATE <= max;

        match fmt {
            SampleFormat::F32 => {
                if supports_16k && f32_16k.is_none() {
                    f32_16k = Some(range);
                } else if f32_any.is_none() {
                    f32_any = Some((range, max));
                }
            }
            SampleFormat::I16 => {
                if supports_16k && i16_16k.is_none() {
                    i16_16k = Some(range);
                } else if i16_any.is_none() {
                    i16_any = Some((range, max));
                }
            }
            _ => {}
        }
    }

    if let Some(r) = f32_16k {
        return Ok((
            r.with_sample_rate(SampleRate(VAD_RATE)).config(),
            VAD_RATE,
            SampleFormat::F32,
        ));
    }
    if let Some(r) = i16_16k {
        return Ok((
            r.with_sample_rate(SampleRate(VAD_RATE)).config(),
            VAD_RATE,
            SampleFormat::I16,
        ));
    }
    if let Some((r, rate)) = f32_any {
        return Ok((r.with_max_sample_rate().config(), rate, SampleFormat::F32));
    }
    if let Some((r, rate)) = i16_any {
        return Ok((r.with_max_sample_rate().config(), rate, SampleFormat::I16));
    }

    let default = device.default_input_config().map_err(|e| e.to_string())?;
    let rate = default.sample_rate().0;
    let fmt = default.sample_format();
    Ok((default.config(), rate, fmt))
}

fn send_utterance(pcm: &[i16], sample_rate: u32, app: &AppHandle) {
    let ws = app.state::<BackendWs>();
    let state = app.state::<AppState>();

    let wav = match encode_wav(pcm, sample_rate) {
        Ok(v) => v,
        Err(e) => {
            log::warn!("VAD: WAV encode failed: {e}");
            return;
        }
    };
    let audio_b64 = B64.encode(&wav);
    let request_id = Uuid::new_v4().to_string();
    let conversation_id = state.get_conversation_id();

    if let Err(e) = ws.send(ClientMsg::Voice(VoiceMsg {
        conversation_id,
        request_id: request_id.clone(),
        audio: audio_b64,
        format: "wav".to_string(),
        // Native Live2D LipSync only; see backend_send_voice.
        features: vec![FEATURE_LIVE2D],
    })) {
        log::warn!("VAD: send failed: {e}");
    } else {
        log::debug!("VAD: utterance sent (request_id={request_id})");
    }
}

fn encode_wav(pcm: &[i16], sample_rate: u32) -> Result<Vec<u8>, String> {
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };
    let mut buf = Cursor::new(Vec::new());
    let mut writer = hound::WavWriter::new(&mut buf, spec).map_err(|e| e.to_string())?;
    for &sample in pcm {
        writer.write_sample(sample).map_err(|e| e.to_string())?;
    }
    writer.finalize().map_err(|e| e.to_string())?;
    Ok(buf.into_inner())
}

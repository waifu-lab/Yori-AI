use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;

use futures_util::{SinkExt, StreamExt};
use parking_lot::Mutex;
use tauri::{AppHandle, Emitter, Manager};
use tokio::sync::mpsc;
use tokio_tungstenite::{connect_async, tungstenite::Message};

use super::types::{ClientMsg, ServerMsg};
use crate::waifu_node::WaifuNodeManager;

fn ws_url(base: &str) -> String {
    let trimmed = base.trim_end_matches('/');
    if trimmed.ends_with("/ws") {
        trimmed.to_string()
    } else {
        format!("{trimmed}/ws")
    }
}

const RECONNECT_INITIAL_MS: u64 = 3_000;
const RECONNECT_MAX_MS: u64 = 30_000;

#[derive(Clone)]
pub struct WsHandle {
    pub tx: mpsc::UnboundedSender<ClientMsg>,
}

pub struct WsActor {
    url: String,
    app: AppHandle,
    rx: mpsc::UnboundedReceiver<ClientMsg>,
    connected: Arc<AtomicBool>,
}

impl WsActor {
    pub fn spawn(url: String, app: AppHandle, connected: Arc<AtomicBool>) -> WsHandle {
        let (tx, rx) = mpsc::unbounded_channel();
        let handle = WsHandle { tx: tx.clone() };
        let actor = WsActor { url, app, rx, connected };
        tauri::async_runtime::spawn(actor.run());
        handle
    }

    async fn run(mut self) {
        let url = ws_url(&self.url);
        let mut delay = RECONNECT_INITIAL_MS;

        loop {
            match connect_async(&url).await {
                Ok((ws_stream, _)) => {
                    self.connected.store(true, Ordering::Relaxed);
                    delay = RECONNECT_INITIAL_MS;
                    if let Some(mgr) = self.app.try_state::<WaifuNodeManager>() {
                        if let Err(e) = mgr.start() {
                            log::warn!("waifu-node restart on backend reconnect failed: {e}");
                        }
                    }
                    let (mut sink, mut stream) = ws_stream.split();

                    // Drain any queued outbound messages, then handle incoming.
                    // We use a select loop so both directions work simultaneously.
                    loop {
                        tokio::select! {
                            outbound = self.rx.recv() => {
                                let Some(msg) = outbound else { return; }; // channel closed → shutdown
                                let text = serde_json::to_string(&msg).unwrap_or_default();
                                if sink.send(Message::Text(text.into())).await.is_err() {
                                    break; // connection lost
                                }
                            }
                            inbound = stream.next() => {
                                let Some(Ok(msg)) = inbound else { break; };
                                if let Message::Text(text) = msg {
                                    self.dispatch(text.as_str());
                                }
                            }
                        }
                    }

                    self.app.emit("backend-disconnected", ()).ok();
                    self.connected.store(false, Ordering::Relaxed);
                }
                Err(e) => {
                    log::warn!("backend WS connect failed: {e}, retry in {delay}ms");
                }
            }

            tokio::time::sleep(Duration::from_millis(delay)).await;
            delay = (delay * 2).min(RECONNECT_MAX_MS);
        }
    }

    fn dispatch(&self, text: &str) {
        let Ok(msg) = serde_json::from_str::<ServerMsg>(text) else {
            log::warn!("backend: unknown message: {text}");
            return;
        };
        match msg {
            ServerMsg::Connected(c) => {
                self.app.emit("backend-connected", c).ok();
            }
            ServerMsg::Live2dsyncAck(a) => {
                self.app.emit("backend-live2dsync-ack", a).ok();
            }
            ServerMsg::Transcript(t) => {
                self.app.emit("chat-transcript", t).ok();
            }
            ServerMsg::Chunk(c) => {
                self.app.emit("chat-chunk", c).ok();
            }
            ServerMsg::Done(d) => {
                self.app.emit("chat-done", d).ok();
            }
            ServerMsg::Event(e) => {
                self.app.emit("backend-event", e).ok();
            }
            ServerMsg::Error(e) => {
                self.app.emit("backend-error", e).ok();
            }
            ServerMsg::Stopped(s) => {
                self.app.emit("chat-stopped", s).ok();
            }
        }
    }
}

pub struct BackendWs {
    handle: Mutex<Option<WsHandle>>,
    connected: Arc<AtomicBool>,
}

impl BackendWs {
    pub fn new() -> Self {
        Self {
            handle: Mutex::new(None),
            connected: Arc::new(AtomicBool::new(false)),
        }
    }

    pub fn connect(&self, url: String, app: AppHandle) {
        let h = WsActor::spawn(url, app, Arc::clone(&self.connected));
        *self.handle.lock() = Some(h);
    }

    pub fn disconnect(&self) {
        self.connected.store(false, Ordering::Relaxed);
        *self.handle.lock() = None;
    }

    pub fn send(&self, msg: ClientMsg) -> Result<(), String> {
        let guard = self.handle.lock();
        let h = guard.as_ref().ok_or("not connected")?;
        h.tx.send(msg).map_err(|e| e.to_string())
    }

    /// Returns true only when the WebSocket TCP connection is actually established.
    pub fn is_connected(&self) -> bool {
        self.connected.load(Ordering::Relaxed)
    }
}

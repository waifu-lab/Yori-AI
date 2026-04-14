use std::time::{Duration, SystemTime, UNIX_EPOCH};

use anyhow::Result;
use futures_util::{SinkExt, StreamExt};
use serde_json::Value;
use tokio::sync::mpsc;
use tokio_tungstenite::{connect_async, tungstenite::Message};
use uuid::Uuid;
use waifu_shared::{NodeHello, NodePong, NodeSkillSyncError};

use crate::{dispatch, skill, tools, ConnMode};

pub struct Config {
    pub mode: ConnMode,
    pub backend_url: String,
    pub display_name: String,
    pub token: String,
    pub node_id: Uuid,
    pub reconnect_ms: u64,
}

pub async fn run(cfg: Config) {
    let mut backoff_ms = cfg.reconnect_ms;

    loop {
        match connect_once(&cfg).await {
            Ok(()) => {
                log::info!("connection closed gracefully, reconnecting...");
            }
            Err(e) => {
                log::warn!("connection error: {e:#}, retrying in {backoff_ms}ms");
            }
        }

        tokio::time::sleep(Duration::from_millis(backoff_ms)).await;
        backoff_ms = (backoff_ms * 2).min(60_000);
    }
}

async fn connect_once(cfg: &Config) -> Result<()> {
    match cfg.mode {
        ConnMode::Ws => connect_ws(cfg).await,
        ConnMode::Ipc => connect_ipc(cfg).await,
    }
}

async fn connect_ws(cfg: &Config) -> Result<()> {
    log::info!("connecting to {} (ws mode)", cfg.backend_url);
    let (ws_stream, _) = connect_async(&cfg.backend_url).await?;
    log::info!("connected");

    let (mut sink, mut stream) = ws_stream.split();
    let (tx, mut rx) = mpsc::unbounded_channel::<String>();

    let hello = build_hello(cfg).await;
    sink.send(Message::Text(serde_json::to_string(&hello).unwrap().into()))
        .await?;

    let send_task = tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            if sink.send(Message::Text(msg.into())).await.is_err() {
                break;
            }
        }
    });

    while let Some(item) = stream.next().await {
        let msg = item?;

        match msg {
            Message::Text(text) => {
                handle_text(text.as_str(), cfg, &tx).await;
            }
            Message::Close(_) => {
                log::info!("server closed connection");
                break;
            }
            Message::Ping(data) => {
                let _ = data;
            }
            _ => {}
        }
    }

    send_task.abort();
    Ok(())
}

async fn connect_ipc(cfg: &Config) -> Result<()> {
    #[cfg(target_os = "windows")]
    {
        connect_ipc_windows(cfg).await
    }
    #[cfg(not(target_os = "windows"))]
    {
        connect_ipc_unix(cfg).await
    }
}

#[cfg(target_os = "windows")]
async fn connect_ipc_windows(cfg: &Config) -> Result<()> {
    use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
    use tokio::net::windows::named_pipe::ClientOptions;

    const PIPE_NAME: &str = r"\\.\pipe\waifu-node";
    log::info!("connecting to IPC pipe: {PIPE_NAME}");

    let pipe = ClientOptions::new().open(PIPE_NAME)?;
    log::info!("IPC connected");

    let (reader, mut writer) = tokio::io::split(pipe);
    let mut lines = BufReader::new(reader).lines();

    let (tx, mut rx) = mpsc::unbounded_channel::<String>();

    let hello = build_hello(cfg).await;
    let mut hello_json = serde_json::to_string(&hello).unwrap();
    hello_json.push('\n');
    writer.write_all(hello_json.as_bytes()).await?;

    let send_task = tokio::spawn(async move {
        while let Some(mut msg) = rx.recv().await {
            msg.push('\n');
            if writer.write_all(msg.as_bytes()).await.is_err() {
                break;
            }
        }
    });

    while let Some(line) = lines.next_line().await? {
        if !line.is_empty() {
            handle_text(&line, cfg, &tx).await;
        }
    }

    send_task.abort();
    Ok(())
}

#[cfg(not(target_os = "windows"))]
async fn connect_ipc_unix(cfg: &Config) -> Result<()> {
    use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
    use tokio::net::UnixStream;

    const SOCK_PATH: &str = "/tmp/waifu-node.sock";
    log::info!("connecting to IPC socket: {SOCK_PATH}");

    let stream = UnixStream::connect(SOCK_PATH).await?;
    log::info!("IPC connected");

    let (reader, mut writer) = tokio::io::split(stream);
    let mut lines = BufReader::new(reader).lines();

    let (tx, mut rx) = mpsc::unbounded_channel::<String>();

    let hello = build_hello(cfg).await;
    let mut hello_json = serde_json::to_string(&hello).unwrap();
    hello_json.push('\n');
    writer.write_all(hello_json.as_bytes()).await?;

    let send_task = tokio::spawn(async move {
        while let Some(mut msg) = rx.recv().await {
            msg.push('\n');
            if writer.write_all(msg.as_bytes()).await.is_err() {
                break;
            }
        }
    });

    while let Some(line) = lines.next_line().await? {
        if !line.is_empty() {
            handle_text(&line, cfg, &tx).await;
        }
    }

    send_task.abort();
    Ok(())
}

async fn handle_text(text: &str, _cfg: &Config, tx: &mpsc::UnboundedSender<String>) {
    let raw: Value = match serde_json::from_str(text) {
        Ok(v) => v,
        Err(e) => {
            log::warn!("failed to parse message: {e}");
            return;
        }
    };

    let msg_type = raw
        .get("type")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();

    match msg_type.as_str() {
        "node_welcome" => {
            log::info!("node_welcome received, ready");
        }
        "node_reject" => {
            let code = raw.get("code").and_then(|v| v.as_str()).unwrap_or("?");
            let reason = raw.get("reason").and_then(|v| v.as_str()).unwrap_or("?");
            log::error!("node_reject: [{code}] {reason}");
        }
        "node_invoke" => {
            let invoke: waifu_shared::NodeInvoke = match serde_json::from_value(raw) {
                Ok(v) => v,
                Err(e) => {
                    log::warn!("failed to parse node_invoke: {e}");
                    return;
                }
            };
            let tx = tx.clone();
            tokio::spawn(async move {
                let result = dispatch::handle_invoke(&invoke).await;
                let json = serde_json::to_string(&result).unwrap();
                let _ = tx.send(json);
            });
        }
        "node_skill_sync" => {
            let sync_msg: waifu_shared::NodeSkillSync = match serde_json::from_value(raw) {
                Ok(v) => v,
                Err(e) => {
                    log::warn!("failed to parse node_skill_sync: {e}");
                    return;
                }
            };
            let tx = tx.clone();
            tokio::spawn(async move {
                match skill::sync::perform_sync(&sync_msg).await {
                    Ok(()) => {
                        let ready = waifu_shared::NodeSkillReady {
                            msg_type: "node_skill_ready".into(),
                            skill_id: sync_msg.skill_id.clone(),
                            version: sync_msg.version.clone(),
                        };
                        let _ = tx.send(serde_json::to_string(&ready).unwrap());
                        log::info!(
                            "skill {} v{} synced successfully",
                            sync_msg.skill_id,
                            sync_msg.version
                        );
                    }
                    Err(e) => {
                        let err_code =
                            if e.to_string().contains("disk") || e.to_string().contains("space") {
                                "DISK_FULL"
                            } else if e.to_string().contains("checksum")
                                || e.to_string().contains("sha256")
                            {
                                "CHECKSUM_MISMATCH"
                            } else {
                                "DOWNLOAD_FAILED"
                            };
                        let err_msg = NodeSkillSyncError {
                            msg_type: "node_skill_sync_error".into(),
                            skill_id: sync_msg.skill_id.clone(),
                            code: err_code.into(),
                            message: e.to_string(),
                        };
                        let _ = tx.send(serde_json::to_string(&err_msg).unwrap());
                        log::error!("skill sync failed for {}: {e:#}", sync_msg.skill_id);
                    }
                }
            });
        }
        "node_tool_result" => {
            // waifu 回傳 node_tool_call 的結果；目前不使用雙向呼叫，忽略
            log::debug!("node_tool_result: {text}");
        }
        "node_ping" => {
            let ts = raw
                .get("ts")
                .and_then(|v| v.as_u64())
                .unwrap_or_else(unix_ts_ms);
            let pong = NodePong {
                msg_type: "node_pong".into(),
                ts,
            };
            let _ = tx.send(serde_json::to_string(&pong).unwrap());
        }
        other => {
            log::debug!("unknown message type: {other}");
        }
    }
}

async fn build_hello(cfg: &Config) -> NodeHello {
    let all_caps = vec![
        "screen".to_string(),
        "exec".to_string(),
        "sysinfo".to_string(),
        "audio".to_string(),
        "input".to_string(),
    ];

    let skills_dir = skill::registry::skills_dir();
    let installed_skills = skill::registry::scan_installed(&skills_dir).await;

    NodeHello {
        msg_type: "node_hello".into(),
        protocol: "1".into(),
        version: env!("CARGO_PKG_VERSION").into(),
        token: cfg.token.clone(),
        node_id: cfg.node_id,
        platform: current_platform(),
        display_name: cfg.display_name.clone(),
        caps: all_caps.clone(),
        skills_dir: skills_dir.to_string_lossy().into_owned(),
        installed_skills,
        tools: tools::build_tool_schemas(&all_caps),
    }
}

fn current_platform() -> String {
    if cfg!(target_os = "windows") {
        "windows".into()
    } else if cfg!(target_os = "macos") {
        "macos".into()
    } else {
        "linux".into()
    }
}

fn unix_ts_ms() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis() as u64)
        .unwrap_or(0)
}

use std::sync::Arc;

use futures_util::{SinkExt, StreamExt};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::sync::mpsc;
use tokio_tungstenite::{connect_async, tungstenite::Message};

const NODE_PATH: &str = if cfg!(target_os = "windows") {
    r"\\.\pipe\waifu-node"
} else {
    "/tmp/waifu-node.sock"
};

pub async fn serve(backend_url: Arc<String>, ready_tx: std::sync::mpsc::Sender<()>) {
    #[cfg(target_os = "windows")]
    serve_windows(backend_url, ready_tx).await;

    #[cfg(not(target_os = "windows"))]
    serve_unix(backend_url, ready_tx).await;
}

fn node_ws_url(base: &str) -> String {
    let trimmed = base.trim_end_matches('/');
    if trimmed.ends_with("/node") {
        trimmed.to_string()
    } else {
        format!("{trimmed}/node")
    }
}

#[cfg(target_os = "windows")]
async fn serve_windows(backend_url: Arc<String>, ready_tx: std::sync::mpsc::Sender<()>) {
    use tokio::net::windows::named_pipe::ServerOptions;

    log::info!("IPC proxy listening on {NODE_PATH}");
    let mut signaled = false;
    loop {
        match ServerOptions::new()
            .first_pipe_instance(false)
            .create(NODE_PATH)
        {
            Ok(server) => {
                if !signaled {
                    let _ = ready_tx.send(());
                    signaled = true;
                }
                if server.connect().await.is_ok() {
                    let url = Arc::clone(&backend_url);
                    tokio::spawn(async move {
                        let node_url = node_ws_url(url.as_str());
                        if let Err(e) = proxy_pipe_to_ws(server, &node_url).await {
                            log::warn!("IPC proxy session ended: {e}");
                        }
                    });
                }
            }
            Err(e) => {
                log::error!("IPC pipe create failed: {e}");
                tokio::time::sleep(Duration::from_secs(3)).await;
            }
        }
    }
}

#[cfg(target_os = "windows")]
async fn proxy_pipe_to_ws(
    pipe: tokio::net::windows::named_pipe::NamedPipeServer,
    node_url: &str,
) -> anyhow::Result<()> {
    let (pipe_reader, mut pipe_writer) = tokio::io::split(pipe);
    let mut lines = BufReader::new(pipe_reader).lines();

    let (pipe_tx, mut pipe_rx) = mpsc::unbounded_channel::<String>();
    let read_task = tokio::spawn(async move {
        loop {
            match lines.next_line().await {
                Ok(Some(line)) if !line.is_empty() => {
                    if pipe_tx.send(line).is_err() {
                        break;
                    }
                }
                _ => break,
            }
        }
    });

    // Retry the initial connection until waifu-node's pipe side is alive.
    let (ws_stream, _) = loop {
        match connect_async(node_url).await {
            Ok(v) => break v,
            Err(e) => {
                if read_task.is_finished() {
                    return Ok(());
                }
                log::warn!("IPC proxy failed to connect backend {node_url}: {e}");
                tokio::time::sleep(Duration::from_secs(3)).await;
            }
        }
    };

    let (mut ws_sink, mut ws_stream) = ws_stream.split();
    log::info!("IPC proxy connected backend {node_url}");

    loop {
        tokio::select! {
            line = pipe_rx.recv() => {
                match line {
                    Some(line) => {
                        if ws_sink.send(Message::Text(line.into())).await.is_err() {
                            break;
                        }
                    }
                    None => {
                        read_task.abort();
                        return Ok(());
                    }
                }
            }
            msg = ws_stream.next() => {
                let Some(Ok(msg)) = msg else {
                    log::warn!("IPC proxy backend WS dropped, closing pipe to waifu-node");
                    break;
                };
                if let Message::Text(text) = msg {
                    let mut out = text.to_string();
                    out.push('\n');
                    if pipe_writer.write_all(out.as_bytes()).await.is_err() {
                        read_task.abort();
                        return Ok(());
                    }
                }
            }
        }
    }

    // Drop pipe_writer (closes write end) so waifu-node sees EOF and exits.
    drop(pipe_writer);
    read_task.abort();
    Ok(())
}

#[cfg(not(target_os = "windows"))]
async fn serve_unix(backend_url: Arc<String>, ready_tx: std::sync::mpsc::Sender<()>) {
    use tokio::net::UnixListener;

    let _ = std::fs::remove_file(NODE_PATH);
    let listener = match UnixListener::bind(NODE_PATH) {
        Ok(l) => l,
        Err(e) => {
            log::error!("IPC socket bind failed: {e}");
            return;
        }
    };
    log::info!("IPC proxy listening on {NODE_PATH}");
    let _ = ready_tx.send(());

    loop {
        if let Ok((stream, _)) = listener.accept().await {
            let url = Arc::clone(&backend_url);
            tokio::spawn(async move {
                let node_url = node_ws_url(url.as_str());
                if let Err(e) = proxy_unix_to_ws(stream, &node_url).await {
                    log::warn!("IPC proxy session ended: {e}");
                }
            });
        }
    }
}

#[cfg(not(target_os = "windows"))]
async fn proxy_unix_to_ws(stream: tokio::net::UnixStream, node_url: &str) -> anyhow::Result<()> {
    let (ws_stream, _) = connect_async(node_url).await?;
    let (mut ws_sink, mut ws_stream) = ws_stream.split();
    let (sock_reader, mut sock_writer) = tokio::io::split(stream);
    let mut lines = BufReader::new(sock_reader).lines();

    let (ws_tx, mut ws_rx) = mpsc::unbounded_channel::<String>();

    let send_task = tokio::spawn(async move {
        while let Some(msg) = ws_rx.recv().await {
            if ws_sink.send(Message::Text(msg.into())).await.is_err() {
                break;
            }
        }
    });

    loop {
        tokio::select! {
            line = lines.next_line() => {
                match line {
                    Ok(Some(l)) if !l.is_empty() => { ws_tx.send(l).ok(); }
                    _ => break,
                }
            }
            msg = ws_stream.next() => {
                let Some(Ok(msg)) = msg else { break; };
                if let Message::Text(text) = msg {
                    let mut out = text.to_string();
                    out.push('\n');
                    sock_writer.write_all(out.as_bytes()).await.ok();
                }
            }
        }
    }

    send_task.abort();
    Ok(())
}

pub mod github;
pub mod ipc;

use parking_lot::Mutex;
use serde::Serialize;
use std::path::PathBuf;
use std::process::{Child, Command};
use std::sync::Arc;

#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum NodeStatus {
    Running,
    Stopped,
}

pub struct WaifuNodeManager {
    process: Mutex<Option<Child>>,
    backend_url: Mutex<String>,
    ipc_started: Mutex<bool>,
}

impl WaifuNodeManager {
    pub fn new() -> Self {
        Self {
            process: Mutex::new(None),
            backend_url: Mutex::new("ws://127.0.0.1:8765".to_string()),
            ipc_started: Mutex::new(false),
        }
    }

    pub fn set_backend_url(&self, url: String) {
        *self.backend_url.lock() = url;
    }

    /// Returns true if waifu-node binary is reachable (exe dir or PATH).
    pub fn is_installed(&self) -> bool {
        self.resolve_bin().is_some()
    }

    /// Install: expects the binary to be bundled next to this app's exe.
    /// If it's already there, no-op. If not found anywhere, returns an error.
    pub fn install(&self) -> Result<(), String> {
        if self.resolve_bin().is_some() {
            return Ok(());
        }
        let exe_dir = std::env::current_exe()
            .map_err(|e| e.to_string())?
            .parent()
            .map(PathBuf::from)
            .ok_or("cannot determine exe directory")?;

        Err(format!(
            "waifu-node binary not found. Place '{}' next to the app at {}",
            bin_name(),
            exe_dir.display()
        ))
    }

    /// Returns the resolved path to the binary, if found.
    pub fn install_path(&self) -> Option<PathBuf> {
        self.resolve_bin()
    }

    /// Start IPC proxy (once), then spawn waifu-node in ipc mode.
    pub fn start(&self) -> Result<(), String> {
        let mut guard = self.process.lock();
        if guard.is_some() {
            return Ok(());
        }

        let bin = self.resolve_bin().ok_or("waifu-node is not installed")?;

        {
            let mut started = self.ipc_started.lock();
            if !*started {
                let url = Arc::new(self.backend_url.lock().clone());
                let (ready_tx, ready_rx) = std::sync::mpsc::channel::<()>();
                std::thread::Builder::new()
                    .name("waifu-node-ipc".to_string())
                    .spawn(move || {
                        let rt = tokio::runtime::Builder::new_current_thread()
                            .enable_all()
                            .build()
                            .expect("failed to build waifu-node ipc runtime");
                        rt.block_on(ipc::serve(url, ready_tx));
                    })
                    .map_err(|e| format!("failed to start waifu-node ipc thread: {e}"))?;

                // Wait until the pipe/socket is actually listening before spawning waifu-node.
                let _ = ready_rx.recv_timeout(std::time::Duration::from_secs(5));
                *started = true;
                log::info!("waifu-node IPC proxy started");
            }
        }

        let child = Command::new(&bin)
            .args(["--mode", "ipc"])
            .spawn()
            .map_err(|e| format!("failed to start waifu-node: {e}"))?;

        log::info!("waifu-node started (pid {})", child.id());
        *guard = Some(child);
        Ok(())
    }

    pub fn stop(&self) {
        let mut guard = self.process.lock();
        if let Some(mut child) = guard.take() {
            log::info!("stopping waifu-node (pid {})", child.id());
            let _ = child.kill();
        }
    }

    pub fn status(&self) -> NodeStatus {
        let mut guard = self.process.lock();
        if let Some(child) = guard.as_mut() {
            match child.try_wait() {
                Ok(Some(code)) => {
                    log::info!("waifu-node exited: {code}");
                    *guard = None;
                    NodeStatus::Stopped
                }
                Ok(None) => NodeStatus::Running,
                Err(_) => NodeStatus::Stopped,
            }
        } else {
            NodeStatus::Stopped
        }
    }

    fn resolve_bin(&self) -> Option<PathBuf> {
        if let Ok(exe) = std::env::current_exe() {
            if let Some(dir) = exe.parent() {
                let candidate = dir.join(bin_name());
                if candidate.exists() {
                    return Some(candidate);
                }
            }
        }

        which_bin()
    }
}

fn bin_name() -> &'static str {
    if cfg!(target_os = "windows") {
        "waifu-node.exe"
    } else {
        "waifu-node"
    }
}

fn which_bin() -> Option<PathBuf> {
    std::env::var_os("PATH").and_then(|paths| {
        std::env::split_paths(&paths).find_map(|dir| {
            let p = dir.join(bin_name());
            p.exists().then_some(p)
        })
    })
}

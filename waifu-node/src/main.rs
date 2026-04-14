mod capability;
mod conn;
mod dispatch;
mod skill;
mod tools;

use clap::Parser;
use uuid::Uuid;

#[derive(clap::ValueEnum, Clone, Debug)]
pub enum ConnMode {
    Ws,
    Ipc,
}

#[derive(Parser, Debug)]
#[command(name = "waifu-node", version = env!("CARGO_PKG_VERSION"))]
struct Cli {
    #[arg(long, value_enum, default_value = "ws")]
    mode: ConnMode,

    #[arg(long, default_value = "ws://127.0.0.1:8765/node")]
    backend_url: String,

    #[arg(long, default_value = "")]
    display_name: String,

    #[arg(long, default_value = "")]
    token: String,

    #[arg(long, default_value_t = 3000)]
    reconnect_ms: u64,
}

#[tokio::main]
async fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let mut cli = Cli::parse();

    if cli.display_name.is_empty() {
        cli.display_name = hostname::get()
            .ok()
            .and_then(|h| h.into_string().ok())
            .unwrap_or_else(|| "unknown".into());
    }

    let node_id = Uuid::new_v4();

    log::info!(
        "waifu-node v{} starting, node_id={node_id}, mode={:?}",
        env!("CARGO_PKG_VERSION"),
        cli.mode
    );
    log::info!("backend_url={}", cli.backend_url);
    log::info!("display_name={}", cli.display_name);

    conn::run(conn::Config {
        mode: cli.mode,
        backend_url: cli.backend_url,
        display_name: cli.display_name,
        token: cli.token,
        node_id,
        reconnect_ms: cli.reconnect_ms,
    })
    .await;
}

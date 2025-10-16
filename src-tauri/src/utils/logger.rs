use fern::{colors::{Color, ColoredLevelConfig}, Output};
use log::{debug, error, info, trace, warn};
use tauri::Emitter;

pub struct FrontendOutput(tauri::AppHandle);

pub fn info(id: &str, message: &str) {
    info!("[{id}] {message}");
}

pub fn warn(id: &str, message: &str) {
    warn!("[{id}] {message}");
}

pub fn error(id: &str, message: &str) {
    error!("[{id}] {message}");
}

pub fn debug(id: &str, message: &str) {
    debug!("[{id}] {message}");
}

pub fn trace(id: &str, message: &str) {
    trace!("[{id}] {message}");
}

pub fn init(app: &tauri::AppHandle, level_filter: log::LevelFilter) -> anyhow::Result<()> {
    let colors = ColoredLevelConfig::new()
        .error(Color::Red)
        .warn(Color::Yellow)
        .info(Color::Green)
        .debug(Color::Blue)
        .trace(Color::Magenta);

    fern::Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "[{}][{}]{}",
                chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                colors.color(record.level()),
                message
            ))
        })
        .level(level_filter)
        .chain(std::io::stdout())
        .chain(FrontendOutput(app.clone()))
        .apply()?;

    info("system.logger", &format!("Logger initialized with level: {:?}", level_filter));
    Ok(())
}

impl Into<Output> for FrontendOutput {
    fn into(self) -> Output {
        let (tx, rx) = std::sync::mpsc::channel::<String>();
        let app = self.0;
        std::thread::spawn(move || {
            while let Ok(message) = rx.recv() {
                app.emit("system:log", message).ok();
            }
        });
        Output::sender(tx, "\n")
    }
}
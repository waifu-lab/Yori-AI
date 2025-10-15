use fern::{colors::{Color, ColoredLevelConfig}, Output};
use log::{debug, error, info, trace, warn};
use tauri::Emitter;

pub struct Logger {
    app: tauri::AppHandle,
    level_filter: log::LevelFilter,
}

pub struct FrontendOutput(tauri::AppHandle);

impl Logger {
    pub fn new(app: tauri::AppHandle, level_filter: log::LevelFilter) -> Self {
        Self { app, level_filter }
    }

    pub fn init(&self) -> anyhow::Result<()> {
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
            .level(self.level_filter)
            .chain(std::io::stdout())
            .chain(FrontendOutput(self.app.clone()))
            .apply()?;

        self.info("system.logger", &format!("Logger initialized with level: {:?}", self.level_filter));
        Ok(())
    }

    pub fn info(&self, id: &str, message: &str) {
        info!("[{id}] {message}");
    }

    pub fn warn(&self, id: &str, message: &str) {
        warn!("[{id}] {message}");
    }

    pub fn error(&self, id: &str, message: &str) {
        error!("[{id}] {message}");
    }

    pub fn debug(&self, id: &str, message: &str) {
        debug!("[{id}] {message}");
    }

    pub fn trace(&self, id: &str, message: &str) {
        trace!("[{id}] {message}");
    }
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
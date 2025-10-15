use plugin_lib::types::{LogLevel, AppContext};
use tauri::Manager;

use crate::{state::AppState, utils::logger::Logger};

pub struct GlobalContext {
    app: tauri::AppHandle,
    logger: Logger,
}

impl GlobalContext {
    pub fn new(app: tauri::AppHandle) -> Self {
        let state = app.state::<AppState>();
        let logger = Logger::new(app.clone(), state.log_level);
        logger.init().expect("Failed to initialize logger");
        Self { app, logger }
    }
}

impl AppContext for GlobalContext {
    fn app_version(&self) -> &str {
        "0.1.0"
    }

    fn get_config(&self, key: &str) -> Option<String> {
        let state = self.app.state::<AppState>();
        match key.to_uppercase().as_str() {
            "LOG_LEVEL" => Some(state.log_level.to_string()),
            _ => None,
        }
    }

    fn log(&self, id: &str, level: LogLevel, message: &str) {
        match level {
            LogLevel::Error => self.logger.error(id, message),
            LogLevel::Warn => self.logger.warn(id, message),
            LogLevel::Info => self.logger.info(id, message),
            LogLevel::Debug => self.logger.debug(id, message),
            LogLevel::Trace => self.logger.trace(id, message),
        }
    }
}
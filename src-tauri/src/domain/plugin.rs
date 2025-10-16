use std::path::{self, PathBuf};

use plugin_lib::types::{AppContext, EventBus as Bus, LogLevel, Plugin};
use serde::Serialize;
use tauri::Manager;

use crate::{
    error::BackendError, event::EventBus, state::AppState, utils::logger::{debug, error, info, trace, warn}
};

pub struct GlobalContext {
    app: tauri::AppHandle,
    event_bus: EventBus,
}

pub struct DynamicPlugin {
    inner: Option<Box<dyn Plugin>>,
    path: PathBuf,
    _lib: libloading::Library,
}

#[derive(Serialize)]
pub struct PluginData {
    pub name: String,
    pub version: String,
    pub author: String,
    pub description: String,
    pub is_deprecated: bool,
    pub need_upgrade: bool,
}

impl GlobalContext {
    pub fn new(app: tauri::AppHandle) -> Self {
        let event_bus = EventBus::new();
        event_bus.process_events();
        Self { app, event_bus }
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

    fn event_bus(&mut self) -> &mut dyn Bus {
        &mut self.event_bus
    }

    fn log(&self, id: &str, level: LogLevel, message: &str) {
        match level {
            LogLevel::Error => error(id, message),
            LogLevel::Warn => warn(id, message),
            LogLevel::Info => info(id, message),
            LogLevel::Debug => debug(id, message),
            LogLevel::Trace => trace(id, message),
        }
    }
}

impl DynamicPlugin {
    pub fn new(inner: Box<dyn Plugin>, path: PathBuf, lib: libloading::Library) -> Self {
        Self { inner: Some(inner), path, _lib: lib }
    }
}

impl Plugin for DynamicPlugin {
    fn metadata(&self) -> &plugin_lib::types::PluginMetadata {
        self.inner.as_ref().map(|inner| inner.metadata()).unwrap()
    }

    fn path(&self) -> &str {
        self.path.to_str().unwrap_or("")
    }

    fn is_deprecated(&self) -> bool {
        self.inner.as_ref().map(|inner| inner.is_deprecated()).unwrap_or(false)
    }

    fn need_upgrade(&self) -> bool {
        self.inner.as_ref().map(|inner| inner.need_upgrade()).unwrap_or(false)
    }

    fn initialize(&mut self, context: &mut dyn AppContext) -> anyhow::Result<()> {
        self.inner.as_mut().map(|inner| inner.initialize(context)).unwrap_or(Ok(()))
    }

    fn start(&mut self) -> anyhow::Result<()> {
        self.inner.as_mut().map(|inner| inner.start()).unwrap_or(Ok(()))
    }

    fn stop(&mut self) -> anyhow::Result<()> {
        self.inner.as_mut().map(|inner| inner.stop()).unwrap_or(Ok(()))
    }

    fn reset(&mut self, context: &mut dyn AppContext) -> anyhow::Result<()> {
        self.inner.as_mut().map(|inner| inner.reset(context)).unwrap_or(Ok(()))
    }

    fn get_settings(&self) -> std::collections::HashMap<String, plugin_lib::types::PluginData> {
        self.inner.as_ref().map(|inner| inner.get_settings()).unwrap_or_default()
    }

    fn update_settings(&mut self, settings: std::collections::HashMap<String, plugin_lib::types::PluginData>) -> anyhow::Result<()> {
        self.inner.as_mut().map(|inner| inner.update_settings(settings)).unwrap_or(Ok(()))
    }

    fn destroy(&mut self) -> anyhow::Result<()> {
        self.inner.as_mut().map(|inner| inner.destroy().unwrap()).unwrap();
        Ok(unsafe {
            self._lib
                .get::<unsafe extern "C" fn(*mut dyn Plugin)>(b"_plugin_destroy")?
                (Box::into_raw(self.inner.take().unwrap()))
        })
    }
}

impl From<&Box<dyn Plugin>> for PluginData {
    fn from(plugin: &Box<dyn Plugin>) -> Self {
        let metadata = plugin.metadata();
        Self {
            name: metadata.name.clone(),
            version: metadata.version.clone(),
            author: metadata.author.clone(),
            description: metadata.description.clone(),
            is_deprecated: plugin.is_deprecated(),
            need_upgrade: plugin.need_upgrade(),
        }
    }
}
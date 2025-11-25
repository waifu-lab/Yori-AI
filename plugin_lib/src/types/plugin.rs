use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::types::EventBus;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginMetadata {
    pub id: String,
    pub name: String,
    pub author: String,
    pub version: String,
    pub description: String,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub enum MimeType {
    #[default]
    TextPlain,
    TextHtml,
    ApplicationJson,
    ImagePng,
    ImageJpeg,
    AudioMpeg,
    VideoMp4,
    Other(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum PluginData {
    String {
        value: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        max_length: Option<usize>,
    },
    Integer {
        value: i64,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        min: Option<i64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        max: Option<i64>,
    },
    Float {
        value: f64,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        min: Option<f64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        max: Option<f64>,
    },
    Boolean {
        value: bool,
    },
    Options {
        value: String,
        placeholder: String,
        options: Vec<String>,
    },
    Stream {
        value: String, // endpoint URL
        #[serde(default)]
        mime_type: MimeType,
    },
    Array {
        value: Box<Vec<PluginData>>
    },
    Object {
        value: Box<HashMap<String, PluginData>>
    },
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

pub trait AppContext: Send + Sync {
    fn app_version(&self) -> &str;
    fn get_config(&self, key: &str) -> Option<String>;
    fn event_bus(&mut self) -> &mut dyn EventBus;
    fn log(&self, id: &str, level: LogLevel, message: &str);
}

pub trait Plugin: Send + Sync + AsAny {
    fn metadata(&self) -> &PluginMetadata;

    /// Builds the UI for the plugin in an immediate-mode style.
    ///
    /// This function is called by the host application whenever the UI needs to be
    /// rendered or updated. The plugin should return a `UiSchema` describing
    /// its desired UI layout.
    ///
    /// The default implementation returns `None`, indicating that the plugin does
    /// not have a UI.
    fn ui(&self) -> Option<crate::ui::UiSchema> {
        None
    }
    
    fn is_deprecated(&self) -> bool;
    fn need_upgrade(&self) -> bool;
    
    fn initialize(&mut self, context: &mut dyn AppContext) -> anyhow::Result<()>;
    fn start(&mut self) -> anyhow::Result<()>;
    fn stop(&mut self) -> anyhow::Result<()>;
    fn reset(&mut self, context: &mut dyn AppContext) -> anyhow::Result<()>;
    fn destroy(&mut self) -> anyhow::Result<()> {
        Ok(())
    }
    
    fn get_settings(&self) -> HashMap<String, PluginData> {
        HashMap::new()
    }

    fn update_settings(&mut self, settings: HashMap<String, PluginData>) -> anyhow::Result<()> {
        let _ = settings;
        Ok(())
    }
}

pub trait AsAny {
    fn as_any(&self) -> &dyn std::any::Any;
}

impl<T: 'static> AsAny for T {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl PluginData {
    pub fn string(value: String) -> Self {
        PluginData::String { value, max_length: None }
    }

    pub fn string_with_max(value: String, max_length: usize) -> Self {
        PluginData::String { value, max_length: Some(max_length) }
    }

    pub fn integer(value: i64) -> Self {
        PluginData::Integer { value, min: None, max: None }
    }

    pub fn integer_with_range(value: i64, min: Option<i64>, max: Option<i64>) -> Self {
        PluginData::Integer { value, min, max }
    }

    pub fn float(value: f64) -> Self {
        PluginData::Float { value, min: None, max: None }
    }

    pub fn float_with_range(value: f64, min: Option<f64>, max: Option<f64>) -> Self {
        PluginData::Float { value, min, max }
    }

    pub fn boolean(value: bool) -> Self {
        PluginData::Boolean { value }
    }

    pub fn options(value: String, options: Vec<String>) -> Self {
        PluginData::Options { value, options, placeholder: String::new() }
    }

    pub fn options_with_placeholder(value: String, options: Vec<String>, placeholder: String) -> Self {
        PluginData::Options { value, options, placeholder }
    }

    pub fn stream(value: String) -> Self {
        PluginData::Stream { value, mime_type: MimeType::TextPlain }
    }

    pub fn stream_with_mime(value: String, mime_type: MimeType) -> Self {
        PluginData::Stream { value, mime_type }
    }

    pub fn array(value: Vec<PluginData>) -> Self {
        PluginData::Array { value: Box::new(value) }
    }

    pub fn object(value: HashMap<String, PluginData>) -> Self {
        PluginData::Object { value: Box::new(value) }
    }
    
    pub fn is_string(&self) -> bool {
        matches!(self, PluginData::String { .. })
    }

    pub fn is_integer(&self) -> bool {
        matches!(self, PluginData::Integer { .. })
    }

    pub fn is_float(&self) -> bool {
        matches!(self, PluginData::Float { .. })
    }

    pub fn is_boolean(&self) -> bool {
        matches!(self, PluginData::Boolean { .. })
    }

    pub fn is_options(&self) -> bool {
        matches!(self, PluginData::Options { .. })
    }

    pub fn is_stream(&self) -> bool {
        matches!(self, PluginData::Stream { .. })
    }

    pub fn is_array(&self) -> bool {
        matches!(self, PluginData::Array { .. })
    }

    pub fn is_object(&self) -> bool {
        matches!(self, PluginData::Object { .. })
    }

    pub fn as_string(&self) -> Option<String> {
        if let PluginData::String { value, .. } = self {
            Some(value.clone())
        } else {
            None
        }
    }

    pub fn as_integer(&self) -> Option<i64> {
        if let PluginData::Integer { value, .. } = self {
            Some(*value)
        } else {
            None
        }
    }

    pub fn as_float(&self) -> Option<f64> {
        if let PluginData::Float { value, .. } = self {
            Some(*value)
        } else {
            None
        }
    }

    pub fn as_boolean(&self) -> Option<bool> {
        if let PluginData::Boolean { value } = self {
            Some(*value)
        } else {
            None
        }
    }

    pub fn as_options(&self) -> Option<(String, Vec<String>, String)> {
        if let PluginData::Options { value, options, placeholder } = self {
            Some((value.clone(), options.clone(), placeholder.clone()))
        } else {
            None
        }
    }

    pub fn as_stream(&self) -> Option<(String, MimeType)> {
        if let PluginData::Stream { value, mime_type } = self {
            Some((value.clone(), mime_type.clone()))
        } else {
            None
        }
    }

    pub fn as_array(&self) -> Option<Vec<PluginData>> {
        if let PluginData::Array { value } = self {
            Some(*value.clone())
        } else {
            None
        }
    }

    pub fn as_object(&self) -> Option<HashMap<String, PluginData>> {
        if let PluginData::Object { value } = self {
            Some(*value.clone())
        } else {
            None
        }
    }
}

impl Into<log::Level> for LogLevel {
    fn into(self) -> log::Level {
        match self {
            LogLevel::Trace => log::Level::Trace,
            LogLevel::Debug => log::Level::Debug,
            LogLevel::Info  => log::Level::Info,
            LogLevel::Warn  => log::Level::Warn,
            LogLevel::Error => log::Level::Error,
        }
    }
}

#[macro_export]
macro_rules! declare_plugin {
    ($plugin_type:ty, $constructor:path) => {
        #[no_mangle]
        pub extern "C" fn _plugin_create() -> *mut dyn $crate::types::Plugin {
            let constructor: fn() -> $plugin_type = $constructor;
            let object = constructor();
            let boxed: Box<dyn $crate::types::Plugin> = Box::new(object);
            Box::into_raw(boxed)
        }

        #[no_mangle]
        pub extern "C" fn _plugin_destroy(plugin: *mut dyn $crate::types::Plugin) {
            if plugin.is_null() {
                return;
            }
            
            unsafe { Box::from_raw(plugin); }
        }
    };
}
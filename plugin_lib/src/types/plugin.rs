use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Plugin metadata information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginMetadata {
    pub id: String,
    pub name: String,
    pub author: String,
    pub version: String,
    pub description: String,
}

/// Plugin context provided by the host application
pub trait PluginContext: Send + Sync {
    /// Get application version
    fn app_version(&self) -> &str;
    
    /// Get plugin data directory
    fn data_dir(&self) -> &str;
    
    /// Get configuration value
    fn get_config(&self, key: &str) -> Option<String>;
    
    /// Set configuration value
    fn set_config(&mut self, key: &str, value: String) -> anyhow::Result<()>;
    
    /// Log message
    fn log(&self, level: LogLevel, message: &str);
}

/// Log level for plugin logging
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

/// Main Plugin trait that all plugins must implement
pub trait Plugin: Send + Sync {
    /// Get plugin metadata
    fn metadata(&self) -> &PluginMetadata;
    
    /// Check if plugin is enabled
    fn is_enabled(&self) -> bool;
    
    /// Check if plugin is deprecated
    fn is_deprecated(&self) -> bool;
    
    /// Check if plugin needs upgrade
    fn need_upgrade(&self) -> bool;
    
    /// Initialize plugin with context
    fn initialize(&mut self, context: &mut dyn PluginContext) -> anyhow::Result<()>;
    
    /// Shutdown plugin
    fn shutdown(&mut self) -> anyhow::Result<()>;
    
    /// Handle event from host application
    fn on_event(&mut self, event: &str, data: Option<&str>) -> anyhow::Result<()>;
    
    /// Get plugin settings schema (JSON schema)
    fn settings_schema(&self) -> Option<String> {
        None
    }
    
    /// Get current plugin settings
    fn get_settings(&self) -> HashMap<String, String> {
        HashMap::new()
    }
    
    /// Update plugin settings
    fn set_settings(&mut self, _settings: HashMap<String, String>) -> anyhow::Result<()> {
        Ok(())
    }
}

/// Macro for plugin entry point
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
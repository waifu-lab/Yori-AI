use std::collections::HashMap;

use plugin_lib::types::{Plugin, PluginContext};

pub struct PluginManager {
    plugins: Vec<Box<dyn Plugin>>
}

impl PluginManager {
    pub fn new() -> Self {
        Self {
            plugins: vec![]
        }
    }

    pub fn load_plugins(&self) -> anyhow::Result<()> {
        // Logic to load plugins
        Ok(())
    }
}
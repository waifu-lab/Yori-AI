use plugin_lib::types::Plugin;

use crate::{domain::GlobalContext, utils::storage::{copy_plugin, delete_plugin, load_plugin, load_plugins}};

pub struct PluginManager {
    app: Option<tauri::AppHandle>,
    context: Option<GlobalContext>,
    plugins: Vec<Box<dyn Plugin>>
}

impl PluginManager {
    pub fn new() -> Self {
        Self {
            app: None,
            context: None,
            plugins: vec![]
        }
    }

    pub fn add_plugin<P: AsRef<std::path::Path>>(&mut self, plugin_path: P) {
        let plugin_path = copy_plugin(self.app.as_ref().unwrap(), plugin_path).unwrap();
        let plugin = load_plugin(self.app.as_ref().unwrap(), plugin_path).unwrap();
        self.plugins.push(plugin);
    }

    pub fn remove_plugin(&mut self, plugin_name: &str) {
        let removed_plugin = self.plugins.pop_if(|plugin| &plugin.metadata().name == plugin_name);
        if let Some(mut plugin) = removed_plugin {
            plugin.destroy().unwrap();
            delete_plugin(self.app.as_ref().unwrap(), plugin.path()).unwrap();
        }
    }

    pub fn list_plugins(&self) -> Vec<crate::domain::PluginData> {
        self.plugins.iter().map(|plugin| plugin.into()).collect()
    }

    pub fn set_handle(&mut self, app: &tauri::AppHandle) {
        self.app = Some(app.clone());
    }

    pub fn initialize(&mut self) -> anyhow::Result<()> {
        self.plugins.extend(load_plugins(self.app.as_ref().unwrap())?);
        self.context = Some(GlobalContext::new(self.app.clone().unwrap()));
        for plugin in &mut self.plugins {
            plugin.initialize(self.context.as_mut().unwrap())?;
        }

        Ok(())
    }
}
use plugin_lib::types::Plugin;

use crate::domain::GlobalContext;

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

    pub fn set_handle(&mut self, app: &tauri::AppHandle) {
        self.app = Some(app.clone());
    }

    pub fn initialize(&mut self) -> anyhow::Result<()> {
        self.context = Some(GlobalContext::new(self.app.clone().unwrap()));
        for plugin in &mut self.plugins {
            plugin.initialize(self.context.as_mut().unwrap())?;
        }

        Ok(())
    }
}
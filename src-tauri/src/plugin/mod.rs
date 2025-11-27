use std::sync::OnceLock;

use parking_lot::RwLock;

mod host;

static PLUGIN_SYSTEM_VERSION: (usize, usize, usize) = (0, 0, 1);
static PLUGIN_MANAGER_INSTANCE: OnceLock<PluginManager> = OnceLock::new();

pub struct PluginManager {
    version: (usize, usize, usize),
    loaded_plugins: RwLock<Vec<String>>,
}

impl PluginManager {
    pub fn get_instance() -> &'static PluginManager {
        PLUGIN_MANAGER_INSTANCE.get_or_init(|| PluginManager {
            version: PLUGIN_SYSTEM_VERSION,
            loaded_plugins: RwLock::new(Vec::new()),
        })
    }

    pub fn version(&self) -> (usize, usize, usize) {
        self.version
    }
}
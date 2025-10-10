pub trait PluginManager: Send + Sync {
    fn load_plugins(&self) -> Vec<Box<dyn Plugin>>;
    fn get_plugin_by_id(&self, id: &str) -> Option<Box<dyn Plugin>>;
}

pub trait Plugin: Send + Sync {
    fn get_id(&self) -> &str;
    fn get_name(&self) -> &str;
    fn get_author(&self) -> &str;
    fn get_version(&self) -> &str;
    fn get_description(&self) -> &str;

    fn is_enabled(&self) -> bool;
    fn is_deprecated(&self) -> bool;
    fn need_upgrade(&self) -> bool;

    fn initialize(&self) -> anyhow::Result<()>;
    fn emit_event(&self, event: &str, data: Option<&str>);
}
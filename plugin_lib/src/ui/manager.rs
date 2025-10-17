use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use crate::types::{Event, Plugin};
use crate::ui::UiSchema;
use crate::EVENT_UI_RENDER_UPDATE;

/// Manages the UI lifecycle for all registered plugins.
///
/// This manager is responsible for:
/// - Keeping track of registered plugins.
/// - Caching the last rendered UI schema for each plugin to perform diffing.
/// - Handling UI rebuild requests by executing the plugin's `ui()` callback.
/// - Returning an update event only when the UI schema has actually changed.
///
/// # Example Usage in a Host Application
///
/// ```rust,ignore
/// // 1. In your application setup
/// let event_bus = ...; // Your EventBus implementation
/// let ui_manager = Arc::new(UiManager::new());
///
/// // 2. Register plugins
/// for plugin in all_plugins {
///     ui_manager.register_plugin(plugin);
/// }
///
/// // 3. Set up the event listener
/// let manager_clone = Arc::clone(&ui_manager);
/// event_bus.on(EVENT_UI_REBUILD_REQUEST, Box::new(move |event| {
///     if let Some(plugin_id) = event.get::<String>("plugin_id") {
///         // Handle the request and get a potential update event
///         if let Some(update_event) = manager_clone.handle_rebuild_request(&plugin_id) {
///             // Emit the new event from your event bus
///             event_bus.emit(update_event);
///         }
///     }
/// }));
/// ```
pub struct UiManager {
    plugins: Mutex<HashMap<String, Arc<Mutex<dyn Plugin>>>>,
    schema_cache: Mutex<HashMap<String, UiSchema>>,
}

impl UiManager {
    /// Creates a new, empty `UiManager`.
    pub fn new() -> Self {
        Self {
            plugins: Mutex::new(HashMap::new()),
            schema_cache: Mutex::new(HashMap::new()),
        }
    }

    /// Registers a plugin with the UI manager.
    ///
    *   Once registered, the manager will be able to handle UI rebuild requests for this plugin.
    pub fn register_plugin(&self, plugin: Arc<Mutex<dyn Plugin>>) {
        let plugin_id = plugin.lock().unwrap().metadata().id.clone();
        self.plugins.lock().unwrap().insert(plugin_id, plugin);
    }

    /// Handles a UI rebuild request for a specific plugin.
    ///
    /// This is the core callback logic. It fetches the new UI schema from the plugin,
    /// compares it with the cached version, and if they differ, returns an `Event` to be emitted.
    pub fn handle_rebuild_request(&self, plugin_id: &str) -> Option<Event> {
        let plugins = self.plugins.lock().unwrap();

        if let Some(plugin_mutex) = plugins.get(plugin_id) {
            let plugin = plugin_mutex.lock().unwrap();

            // Execute the plugin's ui() callback to get the new schema
            if let Some(new_schema) = plugin.ui() {
                let mut cache = self.schema_cache.lock().unwrap();

                // Compare with the cached schema
                let should_update = match cache.get(plugin_id) {
                    Some(cached_schema) if cached_schema == &new_schema => false,
                    _ => true,
                };

                if should_update {
                    // Update the cache
                    cache.insert(plugin_id.to_string(), new_schema.clone());

                    // Return the render update event to be emitted by the caller
                    let update_event = Event::new(EVENT_UI_RENDER_UPDATE)
                        .with("plugin_id", plugin_id.to_string())
                        .with("schema", new_schema)
                        .freeze();
                    return Some(update_event);
                }
            }
        }
        None
    }
}
pub mod types;
pub mod ui;

// Re-export the core UI structures and the manager for easier access by plugin developers.
pub use ui::{
    Button, Card, Component, Content, HorizontalStack, Text, TextInput, UiManager, UiSchema,
    VerticalStack,
};

/// The standard event name used to request a UI rebuild for a plugin.
/// Plugins should emit this event via the `EventBus` when their state changes
/// in a way that requires a UI update.
///
/// The event payload should contain a "plugin_id" field with the ID of the plugin
/// requesting the rebuild.
pub const EVENT_UI_REBUILD_REQUEST: &str = "system:ui.rebuild_request";

/// The event name used by the `UiManager` to notify the frontend that a UI
/// needs to be re-rendered.
///
/// The event payload will contain the new `UiSchema` to be rendered.
pub const EVENT_UI_RENDER_UPDATE: &str = "system:ui.render_update";
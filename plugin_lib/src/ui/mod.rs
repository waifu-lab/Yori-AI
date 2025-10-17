//! This module contains all the definitions and logic for the immediate-mode UI description system.
//! It allows plugins to define their UI structure dynamically, which is then rendered by the frontend.

// Declare the modules within the `ui` module.
pub mod schema;
pub mod manager;

// Re-export the core data structures for easier access from other parts of the plugin_lib
// and by plugin developers. This allows them to use `plugin_lib::ui::UiSchema` instead of
// `plugin_lib::ui::schema::UiSchema`.
pub use schema::{
    Button, Card, Component, Content, HorizontalStack, Text, TextInput, UiSchema, VerticalStack,
};
pub use manager::UiManager;
// data schemas
pub mod live2d;

// project related schemas
mod project;
mod profile;
mod plugin;

pub use project::{ProjectMeta, ProjectMode};
pub use profile::Profile;
pub use plugin::PluginConfig;
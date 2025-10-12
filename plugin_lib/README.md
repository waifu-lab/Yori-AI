# Plugin Library for AI VTuber Launcher

This library provides the public API for developing plugins for the AI VTuber Launcher application.

## Features

- **Plugin Trait**: Define your plugin behavior by implementing the `Plugin` trait
- **Plugin Context**: Access host application services through `PluginContext`
- **Metadata**: Structured plugin information with `PluginMetadata`
- **Settings**: Optional plugin settings with JSON schema support
- **Event System**: Handle events from the host application
- **Logging**: Built-in logging support with different log levels

## Usage

Add this to your plugin's `Cargo.toml`:

```toml
[dependencies]
plugin_lib = { path = "../plugin_lib" }
anyhow = "1.0"
serde = { version = "1.0", features = ["derive"] }

[lib]
crate-type = ["cdylib"]
```

## Example Plugin

```rust
use plugin_lib::types::{Plugin, PluginContext, PluginMetadata, LogLevel};
use plugin_lib::declare_plugin;
use std::collections::HashMap;

pub struct MyPlugin {
    metadata: PluginMetadata,
    enabled: bool,
}

impl MyPlugin {
    pub fn new() -> Self {
        Self {
            metadata: PluginMetadata {
                id: "com.example.myplugin".to_string(),
                name: "My Plugin".to_string(),
                author: "Your Name".to_string(),
                version: "1.0.0".to_string(),
                description: "A sample plugin".to_string(),
            },
            enabled: true,
        }
    }
}

impl Plugin for MyPlugin {
    fn metadata(&self) -> &PluginMetadata {
        &self.metadata
    }

    fn is_enabled(&self) -> bool {
        self.enabled
    }

    fn is_deprecated(&self) -> bool {
        false
    }

    fn need_upgrade(&self) -> bool {
        false
    }

    fn initialize(&mut self, context: &mut dyn PluginContext) -> anyhow::Result<()> {
        context.log(LogLevel::Info, "MyPlugin initialized!");
        Ok(())
    }

    fn shutdown(&mut self) -> anyhow::Result<()> {
        Ok(())
    }

    fn on_event(&mut self, event: &str, data: Option<&str>) -> anyhow::Result<()> {
        match event {
            "start" => {
                // Handle start event
            }
            "stop" => {
                // Handle stop event
            }
            _ => {}
        }
        Ok(())
    }
}

// Declare plugin entry point
declare_plugin!(MyPlugin, MyPlugin::new);
```

## Plugin Lifecycle

1. **Load**: Plugin is loaded from DLL/dylib/so file
2. **Initialize**: `initialize()` is called with `PluginContext`
3. **Run**: Plugin receives events through `on_event()`
4. **Shutdown**: `shutdown()` is called before unloading

## Plugin Context

The `PluginContext` trait provides access to host application services:

- `app_version()`: Get the host application version
- `data_dir()`: Get plugin-specific data directory
- `get_config()` / `set_config()`: Access plugin configuration
- `log()`: Write log messages

## Settings

Plugins can optionally provide settings:

- `settings_schema()`: Return JSON schema for settings UI
- `get_settings()`: Return current settings as key-value pairs
- `set_settings()`: Update settings

## Events

Common events your plugin might receive:

- `"start"`: VTuber session started
- `"stop"`: VTuber session stopped
- `"config_changed"`: Configuration updated
- Custom events defined by the host application

## Building

Build your plugin with:

```bash
cargo build --release
```

The compiled plugin will be in `target/release/` with a platform-specific extension:
- Windows: `.dll`
- macOS: `.dylib`
- Linux: `.so`

## License

See the main project LICENSE file.

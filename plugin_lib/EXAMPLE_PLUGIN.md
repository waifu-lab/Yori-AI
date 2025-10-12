# Example Plugin Project Structure

This document shows how to structure a plugin project.

## Directory Structure

```
my-awesome-plugin/
├── Cargo.toml
├── src/
│   └── lib.rs
└── README.md
```

## Cargo.toml

```toml
[package]
name = "my-awesome-plugin"
version = "0.1.0"
edition = "2021"

[lib]
name = "my_awesome_plugin"
crate-type = ["cdylib"]

[dependencies]
plugin_lib = { path = "../plugin_lib" }
anyhow = "1.0"
serde = { version = "1.0", features = ["derive"] }

# Add your plugin-specific dependencies here
# tokio = { version = "1", features = ["full"] }
# reqwest = "0.11"
```

## src/lib.rs

```rust
use plugin_lib::types::{Plugin, PluginContext, PluginMetadata, LogLevel};
use plugin_lib::declare_plugin;
use std::collections::HashMap;

pub struct MyAwesomePlugin {
    metadata: PluginMetadata,
    enabled: bool,
    // Add your plugin state here
}

impl MyAwesomePlugin {
    pub fn new() -> Self {
        Self {
            metadata: PluginMetadata {
                id: "com.yourname.myawesomeplugin".to_string(),
                name: "My Awesome Plugin".to_string(),
                author: "Your Name".to_string(),
                version: "0.1.0".to_string(),
                description: "Description of what your plugin does".to_string(),
            },
            enabled: true,
        }
    }
}

impl Plugin for MyAwesomePlugin {
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
        context.log(LogLevel::Info, &format!("{} v{} initialized", 
            self.metadata.name, self.metadata.version));
        
        // Initialize your plugin here
        // Load configurations, setup connections, etc.
        
        Ok(())
    }

    fn shutdown(&mut self) -> anyhow::Result<()> {
        // Cleanup resources here
        Ok(())
    }

    fn on_event(&mut self, event: &str, data: Option<&str>) -> anyhow::Result<()> {
        match event {
            "start" => {
                // Handle VTuber session start
            }
            "stop" => {
                // Handle VTuber session stop
            }
            "message" => {
                // Handle chat messages
                if let Some(msg) = data {
                    // Process message
                }
            }
            _ => {
                // Handle unknown events or ignore
            }
        }
        Ok(())
    }

    fn settings_schema(&self) -> Option<String> {
        // Return JSON schema for settings UI (optional)
        Some(r#"{
            "type": "object",
            "properties": {
                "api_key": {
                    "type": "string",
                    "title": "API Key",
                    "description": "Your API key"
                },
                "enabled_features": {
                    "type": "array",
                    "items": {
                        "type": "string"
                    },
                    "title": "Enabled Features"
                }
            }
        }"#.to_string())
    }

    fn get_settings(&self) -> HashMap<String, String> {
        // Return current settings
        let mut settings = HashMap::new();
        settings.insert("api_key".to_string(), "your-api-key".to_string());
        settings
    }

    fn set_settings(&mut self, settings: HashMap<String, String>) -> anyhow::Result<()> {
        // Update plugin settings
        if let Some(api_key) = settings.get("api_key") {
            // Store and use the API key
        }
        Ok(())
    }
}

// Required: Declare the plugin entry point
declare_plugin!(MyAwesomePlugin, MyAwesomePlugin::new);
```

## Building and Installing

1. Build your plugin:
   ```bash
   cargo build --release
   ```

2. The compiled plugin will be in:
   - Windows: `target/release/my_awesome_plugin.dll`
   - macOS: `target/release/libmy_awesome_plugin.dylib`
   - Linux: `target/release/libmy_awesome_plugin.so`

3. Copy the compiled plugin to your AI VTuber Launcher plugins directory

## Tips

- Use `context.log()` for debugging instead of `println!`
- Store plugin data in `context.data_dir()`
- Use `context.get_config()` / `set_config()` for persistent configuration
- Handle all events gracefully - unknown events should be ignored
- Always implement proper cleanup in `shutdown()`
- Test your plugin with different project modes (Streaming, Desktop, Discord)

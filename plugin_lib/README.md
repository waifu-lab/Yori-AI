# Plugin Library

AI VTuber Launcher 插件開發函式庫

---

## 快速開始

### Cargo.toml 設定

```toml
[package]
name = "my-plugin"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib"]

[dependencies]
plugin_lib = { path = "../plugin_lib" }
anyhow = "1.0"
serde = { version = "1.0", features = ["derive"] }
```

### 基本插件實作

```rust
use plugin_lib::types::{Plugin, AppContext, PluginMetadata, LogLevel};
use plugin_lib::declare_plugin;

pub struct MyPlugin {
    metadata: PluginMetadata,
}

impl MyPlugin {
    pub fn new() -> Self {
        Self {
            metadata: PluginMetadata {
                id: "com.example.myplugin".to_string(),
                name: "我的插件".to_string(),
                author: "作者".to_string(),
                version: "1.0.0".to_string(),
                description: "描述".to_string(),
            },
        }
    }
}

impl Plugin for MyPlugin {
    fn metadata(&self) -> &PluginMetadata { &self.metadata }
    fn is_deprecated(&self) -> bool { false }
    fn need_upgrade(&self) -> bool { false }
    fn initialize(&mut self, ctx: &mut dyn AppContext) -> anyhow::Result<()> {
        ctx.log(&self.metadata.id, LogLevel::Info, "初始化完成");
        Ok(())
    }
    fn start(&mut self) -> anyhow::Result<()> { Ok(()) }
    fn stop(&mut self) -> anyhow::Result<()> { Ok(()) }
    fn reset(&mut self, _: &mut dyn AppContext) -> anyhow::Result<()> { Ok(()) }
}

declare_plugin!(MyPlugin, MyPlugin::new);
```

---

## 核心 API

### AppContext
- `app_version()` - 取得應用程式版本
- `log(id, level, msg)` - 記錄日誌
- `get_config(key)` - 取得配置
- `event_bus()` - 存取事件總線

### Event 事件系統
```rust
// 建立並發送事件
let evt = Event::new("name").with("key", "value").freeze();
event_bus.emit(evt);

// 監聽事件
event_bus.on("name", Box::new(|evt| { /* ... */ }));
```

### PluginData 資料型別
```rust
PluginData::string("text".to_string())
PluginData::integer(42)
PluginData::float(3.14)
PluginData::boolean(true)
```

---

## 建置

```bash
cargo build --release
```

輸出: `target/release/your_plugin.dll` (Windows) / `.dylib` (macOS) / `.so` (Linux)

---

## 重要提醒

 必須使用 `declare_plugin!` 巨集  
 必須設定 `crate-type = ["cdylib"]`  
 使用 `context.log()` 而非 `println!`

---

## 詳細文檔

- [API Reference](./API_REFERENCE.md) - 完整 API 參考
- [Example Plugin](./EXAMPLE_PLUGIN.md) - 完整範例專案

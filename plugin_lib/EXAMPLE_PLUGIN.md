# 範例插件專案完整指南

本文檔提供了一個完整的插件開發範例，包含專案結構、程式碼實作和最佳實踐。

## 📁 專案結構

建議的插件專案結構如下：

```
my-awesome-plugin/
├── Cargo.toml              # 專案設定檔
├── README.md               # 插件說明文件
├── LICENSE                 # 授權檔案（可選）
├── src/
│   ├── lib.rs             # 主要插件實作
│   ├── config.rs          # 配置管理（可選）
│   ├── handlers/          # 事件處理器（可選）
│   │   ├── mod.rs
│   │   ├── message.rs
│   │   └── stream.rs
│   └── utils/             # 工具函式（可選）
│       └── mod.rs
└── examples/              # 使用範例（可選）
    └── basic.rs
```

## 📝 Cargo.toml 配置

```toml
[package]
name = "my-awesome-plugin"
version = "0.1.0"
edition = "2021"
authors = ["Your Name <your.email@example.com>"]
description = "一個 AI VTuber Launcher 的範例插件"
license = "MIT"

[lib]
name = "my_awesome_plugin"
# 重要：必須使用 cdylib 才能被動態載入
crate-type = ["cdylib"]

[dependencies]
# 核心依賴
plugin_lib = { path = "../plugin_lib" }
anyhow = "1.0"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
uuid = { version = "1.0", features = ["v4"] }
log = "0.4"

# 可選依賴（根據需求添加）
tokio = { version = "1", features = ["full"], optional = true }
reqwest = { version = "0.11", features = ["json"], optional = true }
chrono = { version = "0.4", features = ["serde"] }

[features]
default = []
async-support = ["tokio"]
http-client = ["reqwest"]

[dev-dependencies]
# 測試用依賴
```

## 🚀 基礎插件實作 (src/lib.rs)

### 最小化範例

```rust
use plugin_lib::types::{Plugin, AppContext, PluginMetadata, PluginData, LogLevel};
use plugin_lib::declare_plugin;
use std::collections::HashMap;

/// 基礎插件結構
pub struct MyAwesomePlugin {
    metadata: PluginMetadata,
    settings: HashMap<String, PluginData>,
}

impl MyAwesomePlugin {
    /// 建構函式
    pub fn new() -> Self {
        Self {
            metadata: PluginMetadata {
                id: "com.yourname.myawesomeplugin".to_string(),
                name: "我的超棒插件".to_string(),
                author: "你的名字".to_string(),
                version: "0.1.0".to_string(),
                description: "這是一個範例插件，展示如何開發 AI VTuber Launcher 插件".to_string(),
            },
            settings: HashMap::new(),
        }
    }
    
    /// 初始化預設設定
    fn init_default_settings(&mut self) {
        self.settings.insert(
            "enabled".to_string(),
            PluginData::boolean(true),
        );
        self.settings.insert(
            "api_key".to_string(),
            PluginData::string_with_max("".to_string(), 256),
        );
        self.settings.insert(
            "timeout_seconds".to_string(),
            PluginData::integer_with_range(30, Some(1), Some(300)),
        );
    }
}

impl Plugin for MyAwesomePlugin {
    fn metadata(&self) -> &PluginMetadata {
        &self.metadata
    }

    fn is_deprecated(&self) -> bool {
        false
    }

    fn need_upgrade(&self) -> bool {
        false
    }

    fn initialize(&mut self, context: &mut dyn AppContext) -> anyhow::Result<()> {
        context.log(
            &self.metadata.id,
            LogLevel::Info,
            &format!("{} v{} 正在初始化...", self.metadata.name, self.metadata.version)
        );
        
        // 初始化預設設定
        self.init_default_settings();
        
        // 註冊事件處理器
        self.register_event_handlers(context)?;
        
        context.log(&self.metadata.id, LogLevel::Info, "初始化完成");
        Ok(())
    }

    fn start(&mut self) -> anyhow::Result<()> {
        // 插件啟動邏輯
        Ok(())
    }

    fn stop(&mut self) -> anyhow::Result<()> {
        // 插件停止邏輯
        Ok(())
    }

    fn reset(&mut self, context: &mut dyn AppContext) -> anyhow::Result<()> {
        context.log(&self.metadata.id, LogLevel::Info, "重置插件狀態");
        self.settings.clear();
        self.init_default_settings();
        Ok(())
    }

    fn get_settings(&self) -> HashMap<String, PluginData> {
        self.settings.clone()
    }

    fn update_settings(&mut self, settings: HashMap<String, PluginData>) -> anyhow::Result<()> {
        // 驗證設定
        for (key, value) in &settings {
            match key.as_str() {
                "api_key" => {
                    if let Some(s) = value.as_string() {
                        if s.is_empty() {
                            anyhow::bail!("API Key 不能為空");
                        }
                    }
                }
                "timeout_seconds" => {
                    if let Some(t) = value.as_integer() {
                        if t < 1 || t > 300 {
                            anyhow::bail!("超時時間必須在 1-300 秒之間");
                        }
                    }
                }
                _ => {}
            }
        }
        
        // 更新設定
        self.settings = settings;
        Ok(())
    }
}

impl MyAwesomePlugin {
    /// 註冊事件處理器
    fn register_event_handlers(&mut self, context: &mut dyn AppContext) -> anyhow::Result<()> {
        let event_bus = context.event_bus();
        
        // 監聽訊息事件
        event_bus.on("message_received", Box::new(|event| {
            if let Some(content) = event.get::<String>("content") {
                println!("收到訊息: {}", content);
            }
        }));
        
        // 監聽狀態變更事件
        event_bus.on("state_changed", Box::new(|event| {
            if let Some(state) = event.get::<String>("state") {
                println!("狀態變更為: {}", state);
            }
        }));
        
        Ok(())
    }
}

// 必須：宣告插件進入點
declare_plugin!(MyAwesomePlugin, MyAwesomePlugin::new);
```

## 🎯 進階範例：帶有事件處理的插件

### src/lib.rs (進階版)

```rust
use plugin_lib::types::{Plugin, AppContext, PluginMetadata, PluginData, LogLevel, Event};
use plugin_lib::declare_plugin;
use std::collections::HashMap;
use std::sync::Arc;
use parking_lot::RwLock;

mod handlers;
use handlers::MessageHandler;

/// 進階插件結構
pub struct AdvancedPlugin {
    metadata: PluginMetadata,
    settings: HashMap<String, PluginData>,
    state: Arc<RwLock<PluginState>>,
}

/// 插件內部狀態
#[derive(Default)]
struct PluginState {
    message_count: usize,
    is_running: bool,
    last_error: Option<String>,
}

impl AdvancedPlugin {
    pub fn new() -> Self {
        Self {
            metadata: PluginMetadata {
                id: "com.yourname.advancedplugin".to_string(),
                name: "進階插件".to_string(),
                author: "你的名字".to_string(),
                version: "0.1.0".to_string(),
                description: "展示進階功能的插件範例".to_string(),
            },
            settings: Self::default_settings(),
            state: Arc::new(RwLock::new(PluginState::default())),
        }
    }
    
    fn default_settings() -> HashMap<String, PluginData> {
        let mut settings = HashMap::new();
        
        // 基本設定
        settings.insert("enabled".to_string(), PluginData::boolean(true));
        settings.insert("auto_reply".to_string(), PluginData::boolean(false));
        
        // API 設定
        settings.insert("api_endpoint".to_string(), 
            PluginData::string("https://api.example.com".to_string()));
        settings.insert("api_key".to_string(), 
            PluginData::string_with_max("".to_string(), 256));
        
        // 進階設定
        settings.insert("mode".to_string(),
            PluginData::options_with_placeholder(
                "normal".to_string(),
                vec!["normal".to_string(), "advanced".to_string(), "expert".to_string()],
                "請選擇模式".to_string()
            ));
        
        // 數值設定
        settings.insert("max_retries".to_string(),
            PluginData::integer_with_range(3, Some(0), Some(10)));
        settings.insert("volume".to_string(),
            PluginData::float_with_range(0.8, Some(0.0), Some(1.0)));
        
        // 串流設定
        settings.insert("websocket_url".to_string(),
            PluginData::stream_with_mime(
                "ws://localhost:8080".to_string(),
                plugin_lib::types::MimeType::ApplicationJson
            ));
        
        // 陣列設定
        settings.insert("allowed_users".to_string(),
            PluginData::array(vec![
                PluginData::string("user1".to_string()),
                PluginData::string("user2".to_string()),
            ]));
        
        // 物件設定
        let mut server_config = HashMap::new();
        server_config.insert("host".to_string(), PluginData::string("localhost".to_string()));
        server_config.insert("port".to_string(), PluginData::integer(8080));
        server_config.insert("ssl".to_string(), PluginData::boolean(false));
        settings.insert("server".to_string(), PluginData::object(server_config));
        
        settings
    }
}

impl Plugin for AdvancedPlugin {
    fn metadata(&self) -> &PluginMetadata {
        &self.metadata
    }

    fn is_deprecated(&self) -> bool {
        false
    }

    fn need_upgrade(&self) -> bool {
        // 檢查是否需要升級
        self.metadata.version.starts_with("0.")
    }

    fn initialize(&mut self, context: &mut dyn AppContext) -> anyhow::Result<()> {
        let id = &self.metadata.id;
        context.log(id, LogLevel::Info, "開始初始化進階插件");
        
        // 檢查應用程式版本
        let app_ver = context.app_version();
        context.log(id, LogLevel::Debug, &format!("主應用程式版本: {}", app_ver));
        
        // 載入儲存的配置
        if let Some(saved_config) = context.get_config("plugin_enabled") {
            context.log(id, LogLevel::Debug, &format!("載入配置: {}", saved_config));
        }
        
        // 註冊事件處理器
        self.register_handlers(context)?;
        
        context.log(id, LogLevel::Info, "初始化完成");
        Ok(())
    }

    fn start(&mut self) -> anyhow::Result<()> {
        let mut state = self.state.write();
        state.is_running = true;
        state.message_count = 0;
        Ok(())
    }

    fn stop(&mut self) -> anyhow::Result<()> {
        let mut state = self.state.write();
        state.is_running = false;
        Ok(())
    }

    fn reset(&mut self, context: &mut dyn AppContext) -> anyhow::Result<()> {
        context.log(&self.metadata.id, LogLevel::Warn, "重置插件");
        
        // 重置狀態
        {
            let mut state = self.state.write();
            *state = PluginState::default();
        }
        
        // 重置設定
        self.settings = Self::default_settings();
        
        Ok(())
    }

    fn get_settings(&self) -> HashMap<String, PluginData> {
        self.settings.clone()
    }

    fn update_settings(&mut self, settings: HashMap<String, PluginData>) -> anyhow::Result<()> {
        // 詳細的設定驗證
        if let Some(api_key) = settings.get("api_key") {
            if let Some(key_str) = api_key.as_string() {
                if key_str.len() > 256 {
                    anyhow::bail!("API Key 超過最大長度限制");
                }
            }
        }
        
        if let Some(max_retries) = settings.get("max_retries") {
            if let Some(retries) = max_retries.as_integer() {
                if retries < 0 || retries > 10 {
                    anyhow::bail!("重試次數必須在 0-10 之間");
                }
            }
        }
        
        if let Some(volume) = settings.get("volume") {
            if let Some(vol) = volume.as_float() {
                if vol < 0.0 || vol > 1.0 {
                    anyhow::bail!("音量必須在 0.0-1.0 之間");
                }
            }
        }
        
        self.settings = settings;
        Ok(())
    }
}

impl AdvancedPlugin {
    /// 註冊所有事件處理器
    fn register_handlers(&mut self, context: &mut dyn AppContext) -> anyhow::Result<()> {
        let event_bus = context.event_bus();
        let state = Arc::clone(&self.state);
        let id = self.metadata.id.clone();
        
        // 訊息處理器
        event_bus.on("message_received", Box::new(move |event| {
            let mut state = state.write();
            state.message_count += 1;
            
            if let Some(content) = event.get::<String>("content") {
                println!("[{}] 收到第 {} 則訊息: {}", id, state.message_count, content);
            }
        }));
        
        // 錯誤處理器
        let state = Arc::clone(&self.state);
        event_bus.on("error_occurred", Box::new(move |event| {
            if let Some(error) = event.get::<String>("error") {
                let mut state = state.write();
                state.last_error = Some(error.clone());
                eprintln!("錯誤: {}", error);
            }
        }));
        
        Ok(())
    }
}

// 宣告插件進入點
declare_plugin!(AdvancedPlugin, AdvancedPlugin::new);
```

### src/handlers/mod.rs

```rust
pub mod message;

pub use message::MessageHandler;
```

### src/handlers/message.rs

```rust
use plugin_lib::types::Event;

pub struct MessageHandler;

impl MessageHandler {
    pub fn handle_message(event: &Event) -> anyhow::Result<()> {
        if let Some(content) = event.get::<String>("content") {
            println!("處理訊息: {}", content);
            // 訊息處理邏輯
        }
        Ok(())
    }
}
```

## 🏗️ 建置與測試

### 開發建置

```bash
# 建置插件
cargo build

# 建置並檢查警告
cargo build --all-features

# 執行格式檢查
cargo fmt --check

# 執行 clippy 檢查
cargo clippy -- -D warnings
```

### 發布建置

```bash
# 建置 release 版本
cargo build --release

# 建置所有功能
cargo build --release --all-features
```

### 測試

```bash
# 執行測試
cargo test

# 執行測試並顯示輸出
cargo test -- --nocapture

# 執行特定測試
cargo test test_name
```

## 📦 安裝插件

### Windows

```powershell
# 複製 DLL 到插件目錄
Copy-Item "target\release\my_awesome_plugin.dll" "$env:APPDATA\AI-Vtuber-Launcher\plugins\"
```

### macOS

```bash
# 複製 dylib 到插件目錄
cp target/release/libmy_awesome_plugin.dylib ~/Library/Application\ Support/AI-Vtuber-Launcher/plugins/
```

### Linux

```bash
# 複製 so 到插件目錄
cp target/release/libmy_awesome_plugin.so ~/.local/share/AI-Vtuber-Launcher/plugins/
```

## 🎓 最佳實踐

### 1. 錯誤處理

```rust
use anyhow::{Result, Context, bail};

fn risky_operation(&self) -> Result<()> {
    self.connect()
        .context("連接失敗")?;
    
    if !self.is_valid() {
        bail!("無效的狀態");
    }
    
    Ok(())
}
```

### 2. 日誌記錄

```rust
// 使用不同等級的日誌
context.log(&self.metadata.id, LogLevel::Trace, "詳細追蹤資訊");
context.log(&self.metadata.id, LogLevel::Debug, "偵錯資訊");
context.log(&self.metadata.id, LogLevel::Info, "一般資訊");
context.log(&self.metadata.id, LogLevel::Warn, "警告訊息");
context.log(&self.metadata.id, LogLevel::Error, "錯誤訊息");
```

### 3. 執行緒安全

```rust
use std::sync::Arc;
use parking_lot::RwLock;

struct MyPlugin {
    // 使用 Arc<RwLock<>> 進行共享狀態
    shared_state: Arc<RwLock<State>>,
}
```

### 4. 資源清理

```rust
impl Plugin for MyPlugin {
    fn stop(&mut self) -> anyhow::Result<()> {
        // 確保清理所有資源
        self.close_connections()?;
        self.flush_buffers()?;
        self.save_state()?;
        Ok(())
    }
}
```

## 📚 常見問題

### Q: 插件無法載入？
**A:** 檢查以下項目：
- 確認 `crate-type = ["cdylib"]` 設定正確
- 確認使用了 `declare_plugin!` 巨集
- 檢查檔案副檔名是否正確
- 查看主應用程式的日誌

### Q: 如何偵錯插件？
**A:** 使用以下方法：
- 使用 `context.log()` 而非 `println!`
- 在主應用程式的偵錯模式下執行
- 使用 `RUST_BACKTRACE=1` 環境變數

### Q: 事件沒有被觸發？
**A:** 確認：
- 事件名稱完全匹配（區分大小寫）
- 事件處理器已正確註冊
- 檢查事件資料格式

## 📖 更多資源

- [Plugin Library README](./README.md)
- [主專案文檔](../.github/copilot-instructions.md)
- [Rust 官方文檔](https://doc.rust-lang.org/)

## 📄 授權

請參閱主專案的 LICENSE 檔案。

# AI VTuber Launcher 插件系統

本文檔詳細說明 AI VTuber Launcher 的插件系統架構、開發指南與 API 參考。

## 1. 系統架構 (Architecture)

插件系統由以下幾個核心組件構成：

*   **Plugin Library (`plugin_lib`)**: 提供插件開發所需的 Traits、資料結構與巨集。所有插件都必須依賴此函式庫。
*   **Plugin Manager (`src-tauri/src/managers/plugin.rs`)**: 負責插件的載入、生命週期管理與卸載。
    *   **載入機制**: 支援動態載入 `.dll` (Windows), `.dylib` (macOS), `.so` (Linux) 檔案。
    *   **生命週期**: 管理插件的初始化 (`initialize`)、啟動 (`start`)、停止 (`stop`) 與重置 (`reset`)。
*   **Global Context (`GlobalContext`)**: 實作了 `AppContext` Trait，作為插件與主程式溝通的橋樑，提供日誌記錄、配置存取與事件總線功能。

## 2. 插件開發指南 (Development Guide)

### 2.1 專案設定 (Cargo.toml)

插件必須是一個 `cdylib` 類型的 Rust 專案。

```toml
[package]
name = "my-plugin"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib"]

[dependencies]
plugin_lib = { path = "../plugin_lib" } # 或指定版本
anyhow = "1.0"
serde = { version = "1.0", features = ["derive"] }
```

### 2.2 核心 Trait: `Plugin`

所有插件必須實作 `Plugin` Trait。

```rust
pub trait Plugin: Send + Sync {
    fn metadata(&self) -> &PluginMetadata;
    fn is_deprecated(&self) -> bool;
    fn need_upgrade(&self) -> bool;
    fn initialize(&mut self, context: &mut dyn AppContext) -> anyhow::Result<()>;
    fn start(&mut self) -> anyhow::Result<()>;
    fn stop(&mut self) -> anyhow::Result<()>;
    fn reset(&mut self, context: &mut dyn AppContext) -> anyhow::Result<()>;
    fn get_settings(&self) -> HashMap<String, PluginData>;
    fn update_settings(&mut self, settings: HashMap<String, PluginData>) -> anyhow::Result<()>;
}
```

### 2.3 插件生命週期 (Lifecycle)

1.  **載入 (Load)**: `PluginManager` 讀取動態連結庫。
2.  **初始化 (Initialize)**: 呼叫 `initialize`，傳入 `AppContext`。此時應進行配置讀取、事件監聽註冊等一次性設定。
3.  **啟動 (Start)**: 呼叫 `start`。插件開始執行主要邏輯或背景任務。
4.  **停止 (Stop)**: 呼叫 `stop`。插件應清理資源、停止背景任務。
5.  **重置 (Reset)**: 呼叫 `reset`。將插件狀態回復到預設值。
6.  **卸載 (Unload)**: 從記憶體中移除插件。

### 2.4 插件入口點 (Entry Point)

必須使用 `declare_plugin!` 巨集來宣告插件的入口點。

```rust
declare_plugin!(MyPlugin, MyPlugin::new);
```

## 3. 核心 API (Core API)

### 3.1 AppContext

提供插件與主程式互動的介面。

*   `app_version()`: 取得主程式版本。
*   `get_config(key)`: 取得全域配置。
*   `event_bus()`: 取得事件總線，用於發送或監聽事件。
*   `log(id, level, message)`: 記錄日誌。

### 3.2 EventBus & Event

事件驅動通訊機制。

*   **發送事件**:
    ```rust
    let event = Event::new("my_event").with("data", "value").freeze();
    context.event_bus().emit(event);
    ```
*   **監聽事件**:
    ```rust
    context.event_bus().on("my_event", Box::new(|event| {
        // 處理事件
    }));
    ```

### 3.3 PluginData

用於插件設定的強型別資料結構，支援：
*   `String`, `Integer`, `Float`, `Boolean`
*   `Options` (下拉選單)
*   `Stream` (串流資料)
*   `Array`, `Object` (複雜結構)

## 4. 範例插件 (Example)

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
                description: "這是一個範例插件".to_string(),
            },
        }
    }
}

impl Plugin for MyPlugin {
    fn metadata(&self) -> &PluginMetadata { &self.metadata }
    fn is_deprecated(&self) -> bool { false }
    fn need_upgrade(&self) -> bool { false }
    
    fn initialize(&mut self, ctx: &mut dyn AppContext) -> anyhow::Result<()> {
        ctx.log(&self.metadata.id, LogLevel::Info, "插件初始化完成");
        Ok(())
    }
    
    fn start(&mut self) -> anyhow::Result<()> { Ok(()) }
    fn stop(&mut self) -> anyhow::Result<()> { Ok(()) }
    fn reset(&mut self, _: &mut dyn AppContext) -> anyhow::Result<()> { Ok(()) }
    
    fn get_settings(&self) -> std::collections::HashMap<String, plugin_lib::types::PluginData> {
        std::collections::HashMap::new()
    }
    
    fn update_settings(&mut self, _: std::collections::HashMap<String, plugin_lib::types::PluginData>) -> anyhow::Result<()> {
        Ok(())
    }
}

declare_plugin!(MyPlugin, MyPlugin::new);
```

## 5. 更多資源

*   [API Reference](plugin_lib/API_REFERENCE.md): 完整的 API 說明。
*   [Example Plugin](plugin_lib/EXAMPLE_PLUGIN.md): 完整的範例專案。

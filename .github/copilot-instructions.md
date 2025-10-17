# AI VTuber Launcher - Copilot 指引

## 專案架構

這是一個**基於 Tauri 的桌面應用程式**，採用可插拔架構來管理 AI VTuber 專案。系統遵循前端（SvelteKit + TypeScript）與後端（Rust + Tauri）的清晰分離原則，並透過動態插件系統實現可擴展性。

### 核心元件

1. **前端 (`src/`)**: SvelteKit 5 + TypeScript + Tailwind CSS 4
2. **後端 (`src-tauri/`)**: Rust + Tauri 2.x，使用 `parking_lot::RwLock` 進行狀態管理
3. **插件系統 (`plugin_lib/`)**: 動態函式庫載入（`.dll`/`.so`/`.dylib`），基於 trait 介面

### 資料流

```
前端 (Svelte) → Tauri 命令 → 管理器 (Project/Plugin) → 領域模型 → 儲存 (bincode)
                                  ↓
                             EventBus (crossbeam) → 插件事件處理器
```

## 關鍵開發模式

### 插件開發（最重要）

插件在**執行時期動態載入**。每個插件必須：

1. 在 `Cargo.toml` 中使用 `crate-type = ["cdylib"]`
2. 實作來自 `plugin_lib::types` 的 `Plugin` trait
3. 使用 `declare_plugin!` 巨集匯出：`declare_plugin!(MyPlugin, MyPlugin::new);`
4. 放置於 `{APP_DATA}/plugins/` 目錄（由 `PluginManager::add_plugin` 自動複製）

**範例結構**（參見 `plugin_lib/EXAMPLE_PLUGIN.md`）：
```rust
use plugin_lib::types::{Plugin, AppContext, PluginMetadata, LogLevel};
use plugin_lib::declare_plugin;

pub struct MyPlugin {
    metadata: PluginMetadata,
}

impl Plugin for MyPlugin {
    fn initialize(&mut self, ctx: &mut dyn AppContext) -> anyhow::Result<()> {
        ctx.log(&self.metadata.id, LogLevel::Info, "初始化完成");
        Ok(())
    }
    // ... 其他必要方法
}

declare_plugin!(MyPlugin, MyPlugin::new);
```

### 狀態管理

應用程式使用 `parking_lot::RwLock` 的**共享狀態模式**：

- `AppState`（`src-tauri/src/state.rs`）持有 `Arc<RwLock<ProjectManager>>` 和 `Arc<RwLock<PluginManager>>`
- 始終使用 `.read()` 或 `.write()` 存取管理器
- 管理器持有 `tauri::AppHandle` 的參考，用於發送事件和存取路徑

### 事件系統

`EventBus`（`src-tauri/src/event/bus.rs`）是一個**基於佇列的事件分派器**：

- 使用 `Arc<Mutex<Vec<Event>>>` 作為事件佇列
- 背景執行緒透過 `process_events()` 處理事件
- 插件透過 `AppContext::event_bus()` 接收事件
- 事件會被複製到所有匹配的處理器（依事件名稱匹配）

### 儲存模式

使用 `bincode` 在 `{APP_DATA}` 目錄進行二進位序列化：

- 專案：`{APP_DATA}/projects/*.proj`
- 插件：`{APP_DATA}/plugins/*.dll|.so|.dylib`
- 透過 `storage::load_projects()` 和 `storage::load_plugins()` 載入

## 建置與開發命令

```powershell
# 開發模式（熱重載）
pnpm dev                    # 前端 + 後端一起啟動

# 建置
pnpm tauri build           # 發布版本建置
cargo build --release      # 僅建置後端

# 插件開發
cd plugin_lib
cargo build --release      # 在 target/release/ 中建立 .dll
```

**重要**：插件必須單獨建置並放置於 plugins 目錄中。

## 檔案命名慣例

- **Rust 模組**：Snake_case 搭配描述性名稱（`project_manager.rs`、`event_bus.rs`）
- **前端路由**：SvelteKit 慣例（`+page.svelte`、`+layout.svelte`）
- **專案檔案**：`{name}.proj`（bincode 序列化）
- **語系檔**：`{lang}.yml` 格式（使用 `rust_i18n` 巨集）

## 技術特定說明

### Tauri 整合

- 命令定義於 `src-tauri/src/commands/*.rs` 並在 `main.rs` 中註冊
- 使用 `#[tauri::command]` 屬性巨集（標記函式為 Tauri 命令）
- 前端透過 `@tauri-apps/api/core` 的 `invoke()` 呼叫
- 透過 `app.state::<AppState>()` 存取狀態

### Live2D 整合

專案包含 Live2D Cubism SDK（`src-tauri/libs/Live2D/`）：
- 使用 `live2d-cubism-core-sys` 進行 FFI 綁定
- 需要 `wgpu` 進行渲染
- SDK 檔案位於 `libs/Live2D/Core/`

### 國際化

使用 `rust_i18n` 巨集搭配 YAML 語系檔：
- 在 `main.rs` 中的巨集呼叫：`i18n!("locales", fallback = "zh-TW");`
- 語系檔：`src-tauri/locales/{lang}.yml`
- 在 Rust 程式碼中使用 `t!()` 巨集進行翻譯

## 常見陷阱

1. **插件載入失敗**：確保已加入 `declare_plugin!` 且設定 `crate-type = ["cdylib"]`
2. **狀態死鎖**：永遠不要同時持有多個 `RwLock` 守衛；`parking_lot` 已啟用死鎖檢測
3. **路徑處理**：使用 `tauri::Manager::path()` 方法，不要使用硬編碼路徑
4. **事件匯流排初始化**：建立 `EventBus` 後呼叫 `process_events()` 以啟動背景執行緒

## 關鍵整合點

- **前端 ↔ 後端**：`src/lib/commands.ts` 中的 Tauri 命令（目前為存根檔案）
- **插件 ↔ 核心**：`AppContext` trait 提供 API 介面（`event_bus()`、`log()`、`get_config()`）
- **管理器 ↔ 儲存**：`utils/storage.rs` 處理 bincode 序列化的檔案 I/O

## 工作區結構

這是一個包含兩個成員的 **Cargo 工作區**：
- `plugin_lib`：插件開發用的共享函式庫
- `src-tauri`：主應用程式後端

在根目錄的 `Cargo.toml` 中使用工作區依賴以保持版本一致性。

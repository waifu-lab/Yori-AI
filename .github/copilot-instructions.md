# AI VTuber 啟動器 - Copilot 開發指南

## 專案概述

這是一個 **Tauri 桌面應用程式**，用於管理和啟動 AI VTuber 專案，具有可插拔架構。專案由三個核心部分組成：

1. **前端**：SvelteKit + TypeScript（Web UI）
2. **後端**：Rust + Tauri（桌面整合、狀態管理）
3. **插件系統**：共享 Rust 函式庫 (`plugin_lib`) 用於可擴充的插件

應用程式使用領域驅動設計模式管理 **專案**（VTuber 配置）和 **插件**（可擴充模組），透過管理器類別處理生命週期和持久化。

## 架構與資料流

### 三層架構

```
前端 (SvelteKit/Svelte)
    ↓ [invoke() 命令]
後端 (Tauri + Rust)
    ├─ AppState (管理器 + 事件總線)
    ├─ ProjectManager (CRUD + 啟動)
    ├─ PluginManager (動態載入 + 初始化)
    └─ EventBus (插件間通訊)
    ↓ [透過 libloading 動態載入]
插件系統 (編譯的 .dll/.so/.dylib 檔案)
```

### 關鍵元件

- **`src-tauri/src/state.rs`**：`AppState` 包含 `ProjectManager` 和 `PluginManager`，使用 `Arc<RwLock<>>` 包裝（執行緒安全狀態共享）
- **`src-tauri/src/domain/`**：領域模型（`ProjectData`、`ProjectControlMode`）
- **`src-tauri/src/managers/`**：有狀態管理器（`ProjectManager`、`PluginManager`），在 `main.rs` 中初始化
- **`src-tauri/src/event/bus.rs`**：插件與主機間的事件驅動通訊
- **`plugin_lib/src/types/`**：共享特質定義（`Plugin`、`PluginContext`、`AppContext`）
- **儲存層**：使用 `bincode` 序列化專案（`.proj` 檔案）和動態 `.dll`/`.so`/`.dylib` 插件載入

### 專案 vs 插件生命週期

**專案**（儲存為 `.proj` 檔案）：
1. 透過 `create_project` 命令建立 → 儲存到 `app_data/projects/`
2. 啟動時透過 `ProjectManager::initialize()` 載入
3. 可與關聯插件啟動
4. 儲存包括插件引用的中繼資料

**插件**（編譯的二進位插件）：
1. 啟動時從 `app_data/plugins/` 目錄發現
2. 透過 `libloading::Library::new()` 動態載入（每個檔案一個）
3. 必須透過 `declare_plugin!(struct, constructor)` 巨集宣告進入點
4. 透過 `Plugin::initialize()` 使用 `PluginContext` 初始化
5. 當事件名稱匹配時透過 `EventBus` 通訊

## 關鍵模式與約定

### 管理器模式
- **必須在 `main.rs` 中的 `initialize()` 之前呼叫 `set_handle()`**
- 管理器儲存 `Option<tauri::AppHandle>` 用於檔案系統/事件存取
- 透過 `app.state::<AppState>()` 然後 `.project_manager.write()` 或 `.plugin_manager.write()` 存取

### 複製與序列化
- `ProjectData` 使用 `derive(Clone, Serialize, Deserialize)` – 所有資料都可複製
- 使用 `bincode::serialize/deserialize()` 進行持久化（非 JSON）
- 所有領域模型實作 `Clone`（在熱路徑中使用前檢查）

### 事件總線
- `EventBus` 透過 `process_events()` 產生背景執行緒
- 事件透過 **名稱字串** 匹配（非型別安全）
- 透過 `on()` 註冊處理器，回傳 `Uuid` 用於透過 `off(id)` 移除
- 事件名稱區分大小寫

### 插件架構
- 插件是特質物件（`Box<dyn Plugin>`）
- 必須實作：`metadata()`、`is_enabled()`、`is_deprecated()`、`need_upgrade()`、`initialize()`、`shutdown()`、`on_event()`
- 透過 `&mut dyn PluginContext` 進行上下文注入 → 透過它存取應用程式服務
- 插件可以發出事件、記錄日誌和儲存型別化資料（帶驗證的 PluginData 列舉）

### 工作區結構
- 工作區根目錄包含 `plugin_lib/` 和 `src-tauri/` 作為成員
- 前端在根目錄 `src/`，後端在 `src-tauri/src/`
- **前端必須使用 `pnpm`**（見 `package.json` 中的 `pnpm` 設定）
- 透過根目錄 `Cargo.toml` 中的 `[workspace.dependencies]` 共享相依性

## 建置與開發命令

```bash
# 前端開發 (SvelteKit)
pnpm install
pnpm run dev              # 執行於 localhost:8080

# 後端/完整建置 (Tauri)
cargo build --release    # 建置桌面應用程式
cargo run                # 偵錯模式

# 程式碼品質檢查
pnpm run lint:fix        # Biome 格式化/檢查
pnpm run check           # Svelte 型別檢查
```

**重要**：執行 `pnpm dev` 時如果後端檔案改變，Tauri 會自動重建後端。

## 約定與注意事項

1. **執行緒安全**：使用 `parking_lot::RwLock`（非 `std::sync::Mutex`）– 管理器間一致使用
2. **錯誤處理**：整個後端使用 `anyhow::Result<T>`（標準化錯誤型別）
3. **國際化**：透過 `rust_i18n` 巨集進行本地化 – 資源檔在 `src-tauri/locales/{en.yml,zh-TW.yml}`
4. **插件載入**：平台特定副檔名（Windows 上 `.dll`、Linux 上 `.so`、macOS 上 `.dylib`）– 見 `storage.rs`
5. **啟動器特質**：每個 `ProjectControlMode`（Desktop、Streaming、Discord）在 `src-tauri/src/launcher/` 中有自己的 `Launcher` 實作
6. **沒有測試**– 測試基礎設施尚未建立

## 檔案參考指南

| 路徑 | 用途 |
|------|------|
| `src-tauri/src/main.rs` | Tauri 應用程式初始化、管理器設定、命令註冊 |
| `src-tauri/src/state.rs` | `AppState` 結構（管理器容器） |
| `src-tauri/src/managers/*.rs` | `ProjectManager`、`PluginManager` 生命週期 |
| `src-tauri/src/domain/*.rs` | `ProjectData`、`PluginMetadata`、領域列舉 |
| `src-tauri/src/commands/*.rs` | 暴露給前端的 Tauri 命令處理器 |
| `src-tauri/src/event/bus.rs` | 事件驅動的插件通訊 |
| `src-tauri/src/utils/storage.rs` | 專案/插件持久化、動態載入 |
| `plugin_lib/src/types/*.rs` | `Plugin` 特質、`PluginContext`、事件型別 |
| `src/lib/commands.ts` | 前端命令包裝器（WIP） |
| `src/lib/stores/project.ts` | 前端狀態（WIP） |

## 立即提高生產力的技巧

1. **添加新的 Tauri 命令**：在 `src-tauri/src/commands/mod.rs` 中定義，在 `main.rs` 中的 `invoke_handler!` 中註冊，從前端透過 `invoke()` 呼叫
2. **添加插件功能**：在插件邏輯中實作新的 `on_event()` 處理器；透過 `PluginContext` 發出
3. **前後端同步**：使用 SvelteKit stores + Tauri `invoke()` – 透過 `ProjectManager` 確保狀態一致性
4. **偵錯插件**：檢查 `app_data/plugins/` 目錄是否存在；確認插件建置中呼叫了 `declare_plugin!` 巨集
5. **儲存路徑**：始終使用 `app.path().data_dir()`（非硬編碼路徑）– 遵守 OS 慣例

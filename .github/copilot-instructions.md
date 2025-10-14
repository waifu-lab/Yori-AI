# AI VTuber Launcher - AI 編碼助手指引

## 專案概覽

**AI VTuber Launcher** 是一個基於 Tauri v2 的桌面應用程式，用於管理和啟動 AI VTuber 專案。架構組成：
- **後端**: Rust (Tauri) 與插件系統 (`src-tauri/`)
- **前端**: SvelteKit 5 搭配 TypeScript 和 Tailwind CSS 4 (`src/`)
- **插件函式庫**: 獨立的 Rust crate 用於擴充功能 (`plugin_lib/`)

這是一個使用 Cargo workspace 的**工作區專案**，包含三個成員：`plugin_lib`、`src-tauri` 和根目錄。

## 架構模式

### 1. 插件系統（CDylib 動態載入）
應用程式使用**基於 trait 的插件架構**搭配動態函式庫載入：

- 插件實作 `plugin_lib/src/types/plugin.rs` 中的 `Plugin` trait
- 每個插件是獨立的 cdylib crate，暴露 `_plugin_create()` 和 `_plugin_destroy()` FFI 函式
- 使用 `declare_plugin!` 巨集來匯出插件進入點：
  ```rust
  declare_plugin!(MyPlugin, MyPlugin::new);
  ```
- 插件生命週期：`initialize()` → `on_event()` → `shutdown()`
- 完整插件結構請參考 `plugin_lib/EXAMPLE_PLUGIN.md`

### 2. 狀態管理（後端）
使用 **parking_lot RwLock** 進行執行緒安全的共享狀態管理：

```rust
pub struct AppState {
    pub project_manager: Arc<RwLock<ProjectManager>>,
}
```

- 透過 `state.project_manager.write()`（獨佔）或 `.read()`（共享）存取
- Manager 模式：`ProjectManager` 處理所有專案 CRUD 操作
- 務必在 Tauri setup 後設定 app handle：`manager.set_handle(&app.handle())`

### 3. 專案持久化
專案以 **bincode 序列化**的 `.proj` 檔案儲存在應用程式設定目錄：

- 路徑：`<config_dir>/projects/<project_name>.proj`
- 儲存工具位於 `src-tauri/src/utils/storage.rs`
- 使用 `save_project()`、`load_project()`、`load_projects()` 函式
- 專案包含：UUID、名稱、路徑、模式、插件清單、時間戳記、中繼資料

### 4. 啟動器模式
針對不同啟動模式使用**策略模式**：

```rust
pub trait Launcher: Send + Sync {
    fn launch(&self) -> anyhow::Result<i32>;
}
```

- 模式：`Streaming`、`Desktop`、`Discord`（參見 `ProjectControlMode` 列舉）
- 工廠函式：`create_launcher(project)` 回傳對應的 launcher
- 每個 launcher 在獨立模組：`src-tauri/src/launcher/{streaming,desktop,discord}.rs`

### 5. 命令查詢分離
Tauri 命令遵循類 CQRS 模式：

- 命令依領域分組於 `src-tauri/src/commands/`
- 每個命令以 `tauri::State<AppState>` 作為第一個參數
- 回傳 `Result<T, String>` 供前端使用
- 在 `main.rs` 透過 `generate_handler!` 巨集註冊

## 開發流程

### 建置與執行
```powershell
# 前端開發（SvelteKit 開發伺服器於 :8080）
pnpm dev

# Tauri 開發（開啟桌面應用程式）
pnpm tauri dev

# 正式版建置
pnpm build
pnpm tauri build

# 執行測試（如果有的話）
cargo test --workspace
```

### 程式碼品質
```powershell
# 前端檢查（Biome）
pnpm lint        # 檢查
pnpm lint:fix    # 修正

# 前端格式化（Prettier）
pnpm format      # 檢查
pnpm format:fix  # 修正

# 型別檢查
pnpm check
```

### 插件開發
1. 建立新 crate，在 `Cargo.toml` 設定 `crate-type = ["cdylib"]`
2. 新增相依性：`plugin_lib = { path = "../plugin_lib" }`
3. 實作 `Plugin` trait 並使用 `declare_plugin!` 巨集
4. 建置：`cargo build --release` → 產生 `.dll`（Windows）/ `.dylib`（macOS）/ `.so`（Linux）
5. 將插件放置於應用程式插件目錄以供載入

## 專案慣例

### Rust 程式碼風格
- **錯誤處理**：內部程式碼使用 `anyhow::Result<T>`，在命令邊界轉換為 `Result<T, String>`
- **序列化**：持久化儲存使用 `bincode`，API 通訊使用 `serde_json`
- **並行處理**：優先使用 `parking_lot` 而非 `std::sync`（RwLock、Mutex）
- **命名**：函式/變數使用 snake_case，型別使用 PascalCase

### 前端慣例
- **Svelte 5 runes**：適當時使用 `$state`、`$derived`、`$effect` 而非舊版 stores
- **Tauri 命令**：從 `@tauri-apps/api/core` 匯入，透過 `invoke()` 呼叫
- **樣式**：優先使用 Tailwind 工具類別而非自訂 CSS
- **結構**：路由於 `src/routes/`，可重用元件於 `src/lib/`

### 國際化
- 使用 `rust-i18n` crate，YAML 語系檔案位於 `src-tauri/locales/`
- 預設語系：`zh-TW`（繁體中文）
- 可用語系：`en.yml`、`zh-TW.yml`
- 在 Rust 程式碼中透過 `i18n!()` 巨集存取

## 關鍵檔案參考

### 後端進入點
- `src-tauri/src/main.rs` - 應用程式初始化、插件註冊、setup hook
- `src-tauri/src/state.rs` - 全域應用程式狀態定義
- `src-tauri/src/commands/mod.rs` - 命令註冊中心

### 模型與型別
- `src-tauri/src/models/project.rs` - `ProjectData` 和 `ProjectControlMode` 定義
- `plugin_lib/src/types/plugin.rs` - Plugin trait 和 context 介面

### 儲存與持久化
- `src-tauri/src/utils/storage.rs` - 專案序列化/反序列化
- 使用 Tauri 的 `path().config_dir()` 取得應用程式資料位置

### 前端整合
- `src/lib/commands.ts` - Tauri 命令的 TypeScript 包裝（目前最小化）
- `src/lib/stores/project.ts` - 專案狀態的 Svelte stores（目前最小化）

## 常見陷阱

1. **插件 ABI 相容性**：插件必須使用與主程式相同的 Rust 版本編譯
2. **狀態死鎖**：切勿同時持有多個寫入鎖；以一致順序取得鎖
3. **Tauri handle 時機**：務必在 setup hook 中呼叫 `set_handle()` 後才使用 managers
4. **路徑分隔符號**：設定中使用正斜線；Tauri 會跨平台正規化
5. **前後端型別**：手動保持 TypeScript 介面與 Rust serde 型別同步

## 外部相依性

- **Tauri**：視窗管理、IPC、檔案系統 API、應用程式路徑
- **SvelteKit**：前端的 SSG 模式（建議正式版使用 static adapter）
- **pnpm**：套件管理器（前端側不使用 workspaces）
- **Biome**：快速 linter/formatter，取代 ESLint+Prettier 組合

## 貢獻者後續步驟

- 完成 i18n 翻譯檔案（`src-tauri/locales/en.yml` 為空）
- 在 `src/lib/commands.ts` 實作前端命令包裝
- 在 `src/lib/stores/project.ts` 新增響應式專案狀態的 Svelte stores
- 記錄插件事件型別和資料格式
- 為每個 launcher 模式新增範例（`Streaming`、`Desktop`、`Discord`）

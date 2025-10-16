# AI VTuber Launcher

基於 Tauri 的 AI VTuber 專案管理與啟動工具，採用可插拔架構設計。

---

## 專案架構

```
前端 (SvelteKit + TypeScript)
    
後端 (Tauri + Rust)
     ProjectManager (專案管理)
     PluginManager (插件管理)
     EventBus (事件通訊)
    
插件系統 (動態載入 .dll/.so/.dylib)
```

### 技術棧

- **前端**: SvelteKit + TypeScript + Tailwind CSS
- **後端**: Rust + Tauri
- **插件**: Rust 動態函式庫
- **狀態管理**: parking_lot::RwLock
- **序列化**: bincode
- **國際化**: rust_i18n

---

## 快速開始

### 環境需求

- Node.js 18+
- pnpm 8+
- Rust 1.70+
- Tauri CLI

### 安裝依賴

```bash
# 安裝前端依賴
pnpm install

# 安裝 Tauri CLI (如果尚未安裝)
cargo install tauri-cli
```

### 開發

```bash
# 啟動開發伺服器 (前端 + 後端)
pnpm dev

# 僅前端開發
pnpm run dev:fe

# 僅後端開發
cd src-tauri
cargo run
```

### 建置

```bash
# 建置桌面應用程式
pnpm tauri build

# 或使用 cargo
cargo build --release
```

---

## 專案結構

```
AI-Vtuber-Launcher-Tauri/
 src/                    # 前端程式碼 (SvelteKit)
    lib/
       commands.ts    # Tauri 命令包裝器
       stores/        # Svelte stores
    routes/            # 頁面路由
 src-tauri/             # 後端程式碼 (Rust)
    src/
       commands/      # Tauri 命令處理器
       domain/        # 領域模型
       managers/      # 管理器 (Project, Plugin)
       event/         # 事件總線
       launcher/      # 啟動器實作
       main.rs
    locales/           # 國際化資源
 plugin_lib/            # 插件開發函式庫
     src/
        types/         # Plugin trait 定義
     README.md          # 插件開發指南
```

---

## 核心功能

### 專案管理
-  建立、儲存、刪除 VTuber 專案
-  專案配置管理
-  三種控制模式: Desktop / Streaming / Discord
-  專案持久化 (.proj 檔案)

### 插件系統
-  動態載入插件
-  插件生命週期管理
-  事件驅動通訊
-  型別安全的設定系統

### 啟動器
-  Desktop 模式啟動器
-  Streaming 模式啟動器
-  Discord 模式啟動器

---

## 開發插件

查看 [插件開發指南](./plugin_lib/README.md) 了解如何建立自訂插件。

### 快速範例

```rust
use plugin_lib::types::{Plugin, AppContext, PluginMetadata};
use plugin_lib::declare_plugin;

pub struct MyPlugin {
    metadata: PluginMetadata,
}

impl Plugin for MyPlugin {
    // 實作插件邏輯...
}

declare_plugin!(MyPlugin, MyPlugin::new);
```

---

## 開發指南

### 程式碼品質

```bash
# 格式化與檢查 (前端)
pnpm run lint:fix
pnpm run check

# Rust clippy 檢查
cargo clippy --all-targets
```

### 重要約定

- 前端**必須使用 pnpm**
- 管理器必須在 `initialize()` 前呼叫 `set_handle()`
- 使用 `parking_lot::RwLock` 而非 `std::sync::Mutex`
- 專案使用 `bincode` 序列化，非 JSON
- 事件名稱區分大小寫

---

## 文檔

- [開發指南](./.github/copilot-instructions.md) - 完整架構說明
- [插件開發](./plugin_lib/README.md) - 插件 API 文檔
- [TODO 清單](./TODO.md) - 開發任務

---

## 授權

待定

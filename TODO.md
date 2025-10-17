# AI VTuber Launcher - TODO 清單

> 根據現有程式碼分析生成的開發任務清單
> 最後更新: 2025-10-17

---

## 🚀 高優先級 (High Priority)

### 前端開發

- [ ] **完成 Tauri 命令包裝器** (`src/lib/commands.ts`)
  - [ ] 實作專案相關命令
    - `createProject(name, path, description, mode)`
    - `deleteProject(id)`
    - `saveProject(project)`
    - `reloadProjects()`
    - `listRecentProjects()`
    - `launchProject(id)`
  - [ ] 實作插件相關命令
    - `addPlugin(filePath)`
    - `removePlugin(pluginName)`
    - `listPlugins()`
  - [ ] 添加 TypeScript 型別定義

- [ ] **實作專案 Store** (`src/lib/stores/project.ts`)
  - [ ] 定義 ProjectData 介面
  - [ ] 定義 ProjectControlMode 列舉
  - [ ] 建立專案列表 store
  - [ ] 建立當前專案 store
  - [ ] 實作專案 CRUD 操作

- [ ] **完成首頁** (`src/routes/+page.svelte`)
  - [ ] 顯示最近專案列表
  - [ ] 新增快速啟動按鈕
  - [ ] 新增專案搜尋功能
  - [ ] 顯示專案統計資訊

- [ ] **完成新建專案頁面** (`src/routes/new/+page.svelte`)
  - [ ] 專案名稱輸入
  - [ ] 專案路徑選擇器 (使用 tauri-plugin-dialog)
  - [ ] 專案描述輸入
  - [ ] 控制模式選擇 (Desktop/Streaming/Discord)
  - [ ] 表單驗證
  - [ ] 建立專案後導航

- [ ] **完成專案管理頁面** (`src/routes/project/+page.svelte`)
  - [ ] 專案列表視圖
  - [ ] 專案卡片元件
  - [ ] 編輯/刪除操作
  - [ ] 專案狀態顯示 (運行中/已停止)

- [ ] **完成插件管理頁面** (`src/routes/plugin/+page.svelte`)
  - [ ] 插件列表視圖
  - [ ] 新增插件按鈕 (檔案選擇器)
  - [ ] 插件詳情顯示
  - [ ] 啟用/停用插件功能
  - [ ] 插件升級提示

### 後端開發

- [ ] **完善 Launcher 實作**
  - [ ] 實作 `DesktopLauncher` (`src-tauri/src/launcher/desktop/`)
  - [ ] 實作 `StreamingLauncher` (`src-tauri/src/launcher/streaming/`)
  - [ ] 實作 `DiscordLauncher` (`src-tauri/src/launcher/discord/`)
  - [ ] 添加進程管理
  - [ ] 添加日誌輸出重導向

- [ ] **改進錯誤處理**
  - [ ] 在所有 Tauri 命令中添加適當的錯誤處理
  - [ ] 實作前端錯誤類型 (`FrontendError`)
  - [ ] 統一錯誤訊息格式
  - [ ] 添加錯誤日誌記錄

- [ ] **完善插件系統**
  - [ ] 實作插件版本檢查
  - [ ] 實作插件相依性管理
  - [ ] 添加插件沙箱機制
  - [ ] 實作插件熱重載
  - [ ] 添加插件設定持久化

---

## 🔧 中優先級 (Medium Priority)

### 核心功能

- [ ] **專案配置管理**
  - [ ] 實作專案配置檔案讀寫
  - [ ] 添加專案元數據編輯功能
  - [ ] 實作專案範本系統
  - [ ] 添加專案匯入/匯出功能

- [ ] **插件 UI Schema 渲染**
  - [ ] 實作 UI Schema 解析器
  - [ ] 建立動態表單元件
  - [ ] 支援所有 PluginData 類型
  - [ ] 實作表單驗證

- [ ] **事件系統增強**
  - [ ] 添加更多系統事件
  - [ ] 實作事件過濾器
  - [ ] 添加事件優先級
  - [ ] 實作事件持久化日誌

- [ ] **Live2D 整合**
  - [ ] 完成 Live2D Cubism SDK 綁定
  - [ ] 實作模型載入器
  - [ ] 實作渲染管線 (使用 wgpu)
  - [ ] 添加動作控制 API
  - [ ] 整合到插件系統

### 開發體驗

- [ ] **插件開發工具**
  - [ ] 建立插件專案範本
  - [ ] 撰寫更多插件範例
  - [ ] 添加插件除錯工具
  - [ ] 實作插件測試框架

- [ ] **文件完善**
  - [ ] 補充 API 文件
  - [ ] 撰寫使用者手冊
  - [ ] 添加架構圖表
  - [ ] 建立貢獻指南

---

## 🎨 低優先級 (Low Priority)

### UI/UX 改進

- [ ] **介面美化**
  - [ ] 設計應用圖示
  - [ ] 建立主題系統 (亮/暗模式)
  - [ ] 添加動畫效果
  - [ ] 改進載入狀態顯示

- [ ] **國際化**
  - [ ] 補充英文翻譯 (`src-tauri/locales/en.yml`)
  - [ ] 添加日文支援
  - [ ] 實作前端國際化
  - [ ] 建立語言切換介面

- [ ] **效能優化**
  - [ ] 專案列表虛擬滾動
  - [ ] 插件載入快取
  - [ ] 減少不必要的重新渲染
  - [ ] 最佳化建置大小

### 進階功能

- [ ] **專案協作**
  - [ ] 實作專案分享功能
  - [ ] 添加雲端同步
  - [ ] 支援多人協作
  - [ ] 版本控制整合

- [ ] **插件商店**
  - [ ] 建立插件倉庫
  - [ ] 實作線上安裝
  - [ ] 添加插件評分系統
  - [ ] 實作自動更新

- [ ] **進階啟動器功能**
  - [ ] 添加排程啟動
  - [ ] 實作自動重啟
  - [ ] 監控系統資源使用
  - [ ] 效能分析工具

---

## 🐛 已知問題 (Known Issues)

### 待修復

- [ ] `PluginManager::remove_plugin` - 使用 `pop_if` 可能導致移除錯誤的插件
  - 應改用 `retain` 或確保正確的移除邏輯
  
- [ ] `storage::load_project` - 目前被註解掉
  - 決定是否需要此功能或完全移除

- [ ] 插件銷毀邏輯 - `DynamicPlugin::destroy` 中的 unsafe 區塊
  - 需要更仔細的記憶體管理審查

- [ ] 專案啟動執行緒管理 - `launch_project` 命令
  - 需要追蹤執行緒生命週期
  - 考慮使用執行緒池

### 程式碼品質

- [ ] 移除所有 unwrap() 調用，改用適當的錯誤處理
- [ ] 為所有公開 API 添加文件註解
- [ ] 統一命名慣例 (特別是 `AppHandle` 參數名稱)
- [ ] 添加單元測試
- [ ] 添加整合測試

---

## 📋 模組狀態

### 已實作 ✅
- 專案管理器基本功能
- 插件管理器基本功能
- 事件匯流排系統
- 專案/插件序列化儲存
- 日誌系統
- 基本 Tauri 命令

### 進行中 🚧
- Launcher 實作 (三種模式)
- 前端介面
- 插件 UI Schema 系統

### 未開始 ❌
- Live2D 整合
- 插件商店
- 雲端同步
- 協作功能

---

## 🎯 里程碑 (Milestones)

### v0.1.0 - MVP (最小可行產品)
- [x] 基本專案架構
- [ ] 完整的專案 CRUD 操作
- [ ] 至少一種 Launcher 實作
- [ ] 插件載入與執行
- [ ] 基本前端介面

### v0.2.0 - 功能完善
- [ ] 三種 Launcher 全部實作
- [ ] 完整的插件系統
- [ ] UI Schema 渲染
- [ ] 完整的前端介面
- [ ] 錯誤處理與日誌

### v0.3.0 - 體驗優化
- [ ] Live2D 整合
- [ ] 主題系統
- [ ] 國際化支援
- [ ] 效能優化
- [ ] 文件完善

### v1.0.0 - 正式版本
- [ ] 所有核心功能穩定
- [ ] 完整測試覆蓋
- [ ] 使用者文件
- [ ] 插件開發指南
- [ ] 發布到各平台

---

## 📝 開發注意事項

### 設計原則
1. **保持前後端分離**: 前端只透過 Tauri 命令與後端通訊
2. **插件隔離**: 插件應透過定義良好的 API 與核心互動
3. **錯誤優先**: 所有錯誤都應被妥善處理並記錄
4. **型別安全**: 充分利用 TypeScript 和 Rust 的型別系統

### 程式碼規範
- Rust: 遵循標準 Rust 風格指南
- TypeScript: 使用 Biome 進行 lint
- 提交前執行 `pnpm lint` 和 `cargo clippy`
- 遵循 Conventional Commits 規範

### 測試策略
- 單元測試: 測試個別函數和模組
- 整合測試: 測試模組間互動
- E2E 測試: 測試完整使用者流程
- 插件測試: 建立測試插件範例

---

## 🔗 相關資源

- [Tauri 文件](https://tauri.app/zh-cn/)
- [SvelteKit 文件](https://kit.svelte.dev/)
- [Plugin Development Guide](./plugin_lib/EXAMPLE_PLUGIN.md)
- [API Reference](./plugin_lib/API_REFERENCE.md)

---

## 📞 需要協助？

如有問題或建議，請：
1. 查閱專案文件
2. 搜尋現有 Issues
3. 建立新 Issue 並詳細描述問題
4. 參與討論區交流

---

*此 TODO 清單會隨專案進展持續更新*

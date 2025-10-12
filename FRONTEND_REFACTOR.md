# 前端重構說明

## 📋 重構概覽

根據後端的實際結構，重新設計了前端的數據流和組件。

## 🔄 主要變更

### 1. TypeScript 類型定義 (`src/lib/commands.ts`)

**之前：**
- 使用簡化的 `Project` 接口
- 命令函數不完整

**現在：**
- `ProjectData` 接口完全匹配後端的 `ProjectData` 結構
- `ProjectControlMode` 枚舉（Streaming, Desktop, Discord）
- 完整的命令函數集合：
  - `createProject()` - 創建新專案
  - `getRecentProjects()` - 獲取最近專案列表
  - `getProjectByName()` - 按名稱獲取專案
  - `removeProject()` - 刪除專案
  - `updateProjectName()` - 更新專案名稱
  - `updateProjectDescription()` - 更新專案描述
  - `addPluginToProject()` - 添加插件
  - `removePluginFromProject()` - 移除插件
  - `launchProject()` - 啟動專案
  - `stopProject()` - 停止專案
  - `getProjectStatus()` - 獲取運行狀態

### 2. Store 重構 (`src/lib/stores/project.ts`)

**之前：**
- Store 包含業務邏輯
- 直接調用 Tauri 命令

**現在：**
- Store 只負責狀態管理
- 簡單的 `setCurrent()` 和 `clearCurrent()` 方法
- 所有業務邏輯通過 `commands.ts` 處理

### 3. 首頁 (`src/routes/+page.svelte`)

**新增功能：**
- ✅ 顯示專案模式徽章（直播/桌面/Discord）
- ✅ 顯示上次開啟時間
- ✅ 顯示插件數量
- ✅ 更豐富的專案卡片設計
- ✅ 載入狀態處理
- ✅ 空狀態提示

**UI 改進：**
- 模式圖標：📹 直播、🖥️ 桌面、💬 Discord
- 日期格式化
- 更好的錯誤處理

### 4. 創建專案頁 (`src/routes/new/+page.svelte`)

**新增功能：**
- ✅ 模式選擇器（卡片式）
- ✅ 專案路徑只讀，必須通過瀏覽按鈕選擇
- ✅ 創建中的載入狀態
- ✅ 完整的表單驗證
- ✅ 調用後端 `createProject` 命令

**UI 改進：**
- 三個模式選項卡，帶描述
- 更好的視覺反饋
- 禁用狀態處理

### 5. 專案控制頁 (`src/routes/project/+page.svelte`)

**新增功能：**
- ✅ 返回首頁按鈕
- ✅ 專案詳細信息顯示
- ✅ 插件列表展示
- ✅ 實際的啟動/停止功能
- ✅ 運行狀態檢查
- ✅ 載入狀態處理

**UI 改進：**
- 專案標題區域，顯示圖標和模式
- 分區布局（插件管理、專案資訊）
- 統計數據卡片
- 更好的空狀態提示

### 6. 佈局 (`src/routes/+layout.svelte`)

**變更：**
- ✅ 使用新的 `projectStore`
- ✅ 顯示專案模式徽章
- ✅ 調用 `updateProjectName` 命令更新名稱
- ✅ 移除不必要的重定向邏輯

## 🎨 設計系統

### 配色方案
- 主色：`#8b5cf6` (紫色)
- 次色：`#ec4899` (粉色)
- 背景：`#0f0f0f` (深黑)
- 卡片：`#18181b` (灰黑)
- 邊框：`#27272a` / `#3f3f46`
- 文字：`#e4e4e7` / `#a1a1aa` / `#71717a`

### 圖標映射
```typescript
Streaming → 📹
Desktop   → 🖥️
Discord   → 💬
Plugin    → 🧩
```

## 🔗 數據流

```
用戶操作
   ↓
Svelte 組件
   ↓
commands.ts (調用 invoke)
   ↓
Tauri Commands (Rust 後端)
   ↓
ProjectManager / State
   ↓
返回結果
   ↓
更新 UI
```

## 📦 需要後端實現的命令

以下命令需要在 `src-tauri/src/commands/project.rs` 中實現：

```rust
#[tauri::command]
pub fn get_recent_projects(state: State<AppState>) -> Result<Vec<ProjectData>, String>

#[tauri::command]
pub fn get_project_by_name(state: State<AppState>, name: String) -> Result<Option<ProjectData>, String>

#[tauri::command]
pub fn remove_project(state: State<AppState>, name: String) -> Result<bool, String>

#[tauri::command]
pub fn update_project_name(state: State<AppState>, old_name: String, new_name: String) -> Result<(), String>

#[tauri::command]
pub fn update_project_description(state: State<AppState>, name: String, description: String) -> Result<(), String>

#[tauri::command]
pub fn add_plugin_to_project(state: State<AppState>, project_name: String, plugin_id: String) -> Result<(), String>

#[tauri::command]
pub fn remove_plugin_from_project(state: State<AppState>, project_name: String, plugin_id: String) -> Result<(), String>

#[tauri::command]
pub fn launch_project(state: State<AppState>, name: String) -> Result<(), String>

#[tauri::command]
pub fn stop_project(state: State<AppState>, name: String) -> Result<(), String>

#[tauri::command]
pub fn get_project_status(state: State<AppState>, name: String) -> Result<String, String>
```

並在 `main.rs` 中註冊：

```rust
.invoke_handler(tauri::generate_handler![
    create_project,
    get_recent_projects,
    get_project_by_name,
    remove_project,
    update_project_name,
    update_project_description,
    add_plugin_to_project,
    remove_plugin_from_project,
    launch_project,
    stop_project,
    get_project_status,
])
```

## 🚀 下一步

1. 實現後端命令
2. 添加插件頁面 (`/plugin`)
3. 添加專案設置頁面
4. 實現實際的啟動邏輯
5. 添加日誌查看功能
6. 添加更多統計數據

## 📝 注意事項

- 所有日期使用 `SystemTime` 轉換為毫秒時間戳
- 錯誤處理使用 `try-catch` 並顯示用戶友好的錯誤訊息
- 載入狀態防止重複操作
- Store 只負責前端狀態，不持久化數據
- 數據持久化完全由後端處理

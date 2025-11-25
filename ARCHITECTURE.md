# AI VTuber Launcher - 系統架構文件

> **專案版本**: 0.1.0  
> **最後更新**: 2025-11-12  
> **技術棧**: Tauri 2.x + SvelteKit 5 + Rust + wgpu

---

## 📋 目錄

- [系統概覽](#系統概覽)
- [核心架構](#核心架構)
- [資料流](#資料流)
- [模組詳解](#模組詳解)
- [視窗管理](#視窗管理)
- [插件系統](#插件系統)
- [渲染管線](#渲染管線)
- [開發指南](#開發指南)

---

## 🎯 系統概覽

AI VTuber Launcher 是一個**可插拔式桌面應用程式**，用於管理和啟動 AI VTuber 專案。採用前後端分離架構，透過 Tauri 橋接，並支援動態插件載入。

### 技術選型

| 層級 | 技術 | 用途 |
|------|------|------|
| **前端** | SvelteKit 5 + TypeScript | UI 介面與路由 |
| **樣式** | Tailwind CSS 4 | 樣式系統 |
| **後端** | Rust + Tauri 2.x | 原生後端邏輯 |
| **渲染** | wgpu + WGSL | Live2D 圖形渲染 |
| **狀態管理** | parking_lot::RwLock | 線程安全的共享狀態 |
| **事件系統** | crossbeam::channel | 跨線程事件通訊 |
| **序列化** | bincode | 二進位資料持久化 |
| **國際化** | rust_i18n | 多語言支援 |

---

## 🏗️ 核心架構

### 整體架構圖

```
┌─────────────────────────────────────────────────────────────────┐
│                         前端層 (SvelteKit)                       │
│  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌──────────┐       │
│  │  主頁面  │  │ 專案管理 │  │ 插件管理 │  │ 設定頁面 │       │
│  └────┬─────┘  └────┬─────┘  └────┬─────┘  └────┬─────┘       │
│       │             │              │             │              │
│       └─────────────┴──────────────┴─────────────┘              │
│                          │                                       │
│                    Tauri Commands                                │
│                          │                                       │
└──────────────────────────┼───────────────────────────────────────┘
                           │
┌──────────────────────────┼───────────────────────────────────────┐
│                    Tauri 橋接層                                   │
│                          │                                       │
│  ┌───────────────────────▼────────────────────────┐             │
│  │          命令處理器 (commands/)                 │             │
│  │  ├─ project.rs    (專案操作)                   │             │
│  │  └─ plugin.rs     (插件操作)                   │             │
│  └───────────────────────┬────────────────────────┘             │
└──────────────────────────┼───────────────────────────────────────┘
                           │
┌──────────────────────────┼───────────────────────────────────────┐
│                     後端層 (Rust)                                 │
│                          │                                       │
│  ┌───────────────────────▼────────────────────────┐             │
│  │         狀態管理 (state.rs)                     │             │
│  │  ┌─────────────────────────────────────────┐  │             │
│  │  │  AppState                               │  │             │
│  │  │  ├─ Arc<RwLock<ProjectManager>>        │  │             │
│  │  │  └─ Arc<RwLock<PluginManager>>         │  │             │
│  │  └─────────────────────────────────────────┘  │             │
│  └────────────┬────────────────┬──────────────────┘             │
│               │                │                                │
│       ┌───────▼─────┐   ┌──────▼───────┐                       │
│       │ 專案管理器   │   │  插件管理器   │                       │
│       │ (managers/  │   │  (managers/  │                       │
│       │  project.rs)│   │   plugin.rs) │                       │
│       └───────┬─────┘   └──────┬───────┘                       │
│               │                │                                │
│       ┌───────▼─────────────────▼───────┐                       │
│       │      事件總線 (event/bus.rs)    │                       │
│       │   ┌─────────────────────────┐  │                       │
│       │   │  crossbeam::channel     │  │                       │
│       │   │  背景執行緒處理事件      │  │                       │
│       │   └─────────────────────────┘  │                       │
│       └─────────────┬───────────────────┘                       │
│                     │                                           │
│       ┌─────────────▼───────────────┐                           │
│       │   核心模組 (core/)           │                           │
│       │   ├─ modules/                │                           │
│       │   │   ├─ models/             │                           │
│       │   │   │   ├─ backend/        │                           │
│       │   │   │   └─ frontend/       │                           │
│       │   │   └─ plugins/            │                           │
│       │   │       └─ obs.rs          │                           │
│       │   └─ renderer/               │                           │
│       │       └─ mod.rs (wgpu)       │                           │
│       └─────────────────────────────┘                           │
│                                                                  │
│  ┌──────────────────────────────────────────────────┐          │
│  │          啟動器 (launcher/)                       │          │
│  │  ├─ desktop/     (桌面應用啟動)                   │          │
│  │  ├─ discord/     (Discord 整合)                   │          │
│  │  └─ streaming/   (串流平台整合)                   │          │
│  └──────────────────────────────────────────────────┘          │
│                                                                  │
│  ┌──────────────────────────────────────────────────┐          │
│  │          工具模組 (utils/)                        │          │
│  │  ├─ storage.rs   (bincode 序列化/反序列化)        │          │
│  │  └─ logger.rs    (日誌系統)                       │          │
│  └──────────────────────────────────────────────────┘          │
└──────────────────────────────────────────────────────────────────┘
                           │
┌──────────────────────────┼───────────────────────────────────────┐
│                   外部插件層                                      │
│  ┌────────────────────────────────────────────────┐             │
│  │   動態載入插件 (.dll/.so/.dylib)                │             │
│  │   ┌──────────────────────────────────────┐    │             │
│  │   │  plugin_lib (共享介面)               │    │             │
│  │   │  ├─ types::Plugin trait              │    │             │
│  │   │  ├─ types::AppContext                │    │             │
│  │   │  └─ declare_plugin! macro            │    │             │
│  │   └──────────────────────────────────────┘    │             │
│  └────────────────────────────────────────────────┘             │
└──────────────────────────────────────────────────────────────────┘
```

---

## 🔄 資料流

### 1. 使用者操作專案

```
使用者點擊「創建專案」
    │
    ▼
前端 Svelte 元件觸發
    │
    ▼
invoke('create_project', { name, path, ... })
    │
    ▼
Tauri 命令處理器 (commands/project.rs)
    │
    ▼
AppState.project_manager.write()
    │
    ▼
ProjectManager.add_project()
    │
    ▼
儲存到 {APP_DATA}/projects/{uuid}.proj (bincode)
    │
    ▼
回傳結果到前端
```

### 2. 插件事件處理

```
專案狀態變更
    │
    ▼
EventBus.emit(Event)
    │
    ▼
crossbeam::channel 佇列
    │
    ▼
背景執行緒 process_events()
    │
    ▼
遍歷所有已載入插件
    │
    ▼
Plugin.handle_event(event)
    │
    ▼
插件執行自訂邏輯 (OBS 控制、Discord 狀態等)
```

### 3. 渲染管線

```
Tauri 主視窗啟動
    │
    ▼
Renderer::new(Arc<WebviewWindow>)
    │
    ▼
wgpu Instance 創建
    │
    ▼
Surface<'static> 綁定到視窗
    │
    ▼
Device + Queue 初始化
    │
    ▼
載入 WGSL Shader (shader.wgsl)
    │
    ▼
建立 RenderPipeline
    │
    ▼
Renderer.render() 每幀執行
    │
    ├─ 獲取 Surface 紋理
    ├─ 創建 CommandEncoder
    ├─ RenderPass (清除 + 繪製)
    ├─ 提交 Queue
    └─ present() 輸出到螢幕
```

---

## 📦 模組詳解

### src-tauri/ (後端)

```
src-tauri/
├── src/
│   ├── main.rs                 # 應用程式入口，Tauri Builder 設定
│   ├── state.rs                # 全域狀態管理 (AppState)
│   ├── error.rs                # 錯誤類型定義
│   │
│   ├── commands/               # Tauri 命令處理器
│   │   ├── mod.rs
│   │   ├── project.rs          # 專案相關命令
│   │   │   ├─ create_project()
│   │   │   ├─ delete_project()
│   │   │   ├─ save_project()
│   │   │   ├─ launch_project()
│   │   │   └─ list_recent_projects()
│   │   └── plugin.rs           # 插件相關命令
│   │       ├─ add_plugin()
│   │       ├─ remove_plugin()
│   │       └─ list_plugins()
│   │
│   ├── managers/               # 業務邏輯管理器
│   │   ├── mod.rs
│   │   ├── project.rs          # ProjectManager
│   │   │   └─ 使用 RwLock<Vec<ProjectData>>
│   │   └── plugin.rs           # PluginManager
│   │       └─ 動態載入 .dll/.so/.dylib
│   │
│   ├── core/                   # 核心功能模組
│   │   ├── mod.rs
│   │   ├── modules/            # 可擴展模組系統
│   │   │   ├── mod.rs
│   │   │   ├── models/         # 模型抽象
│   │   │   │   ├── mod.rs
│   │   │   │   ├── backend/    # Backend trait
│   │   │   │   └── frontend/   # Frontend trait
│   │   │   └── plugins/        # 內建插件
│   │   │       └── obs.rs      # OBS 整合
│   │   └── renderer/           # 渲染器
│   │       └── mod.rs          # wgpu 渲染管線
│   │
│   ├── event/                  # 事件系統
│   │   ├── mod.rs
│   │   └── bus.rs              # EventBus (crossbeam channel)
│   │
│   ├── launcher/               # 專案啟動器
│   │   ├── mod.rs
│   │   ├── desktop/            # 桌面應用啟動
│   │   ├── discord/            # Discord Rich Presence
│   │   └── streaming/          # 串流平台整合
│   │
│   ├── schema/                 # 資料模型
│   │   ├── mod.rs
│   │   ├── project.rs          # ProjectData, ProjectControlMode
│   │   └── plugin.rs           # DynamicPlugin
│   │
│   └── utils/                  # 工具函式
│       ├── mod.rs
│       ├── storage.rs          # bincode 序列化儲存
│       └── logger.rs           # fern 日誌系統
│
├── libs/                       # 外部函式庫
│   └── Live2D/                 # Live2D Cubism SDK
│       ├── Core/               # C++ 核心庫
│       ├── Framework/          # C++ 框架
│       └── Samples/            # 範例與 Shader
│           └── OpenGL/Shaders/ # GLSL Shader (需轉換為 WGSL)
│
├── locales/                    # 國際化語系檔
│   ├── en.yml
│   └── zh-TW.yml
│
├── shader.wgsl                 # WGSL 頂點/片段著色器
├── Cargo.toml                  # Rust 專案設定
└── tauri.conf.json             # Tauri 應用程式設定
```

### src/ (前端)

```
src/
├── lib/
│   ├── commands.ts             # Tauri 命令包裝 (目前為存根)
│   └── stores/                 # Svelte stores (狀態管理)
│
└── routes/                     # SvelteKit 路由
    ├── +layout.svelte          # 全域佈局
    ├── +page.svelte            # 首頁
    ├── new/                    # 新建專案頁面
    ├── plugin/                 # 插件管理頁面
    └── project/                # 專案詳情頁面
```

### plugin_lib/ (插件開發庫)

```
plugin_lib/
├── src/
│   ├── lib.rs                  # 庫入口
│   ├── types/                  # 核心類型定義
│   │   ├── Plugin trait        # 插件必須實作的介面
│   │   ├── AppContext trait    # 提供給插件的 API
│   │   ├── PluginMetadata      # 插件元資料
│   │   └── Event, LogLevel     # 事件與日誌類型
│   └── ui/                     # UI 元件 (可選)
│
├── API_REFERENCE.md            # API 參考文件
├── EXAMPLE_PLUGIN.md           # 插件開發範例
└── Cargo.toml                  # crate-type = ["cdylib", "rlib"]
```

---

## 🪟 視窗管理

### 主視窗配置

**檔案**: `src-tauri/tauri.conf.json`

```json
{
  "app": {
    "windows": [
      {
        "title": "AI Vtuber Launcher",
        "width": 800,
        "height": 600,
        "resizable": true,
        "fullscreen": false
      }
    ]
  }
}
```

### 視窗生命週期

```rust
// main.rs
fn main() {
    tauri::Builder::default()
        .setup(|app| {
            // 主視窗在這裡由 Tauri 自動創建
            // 可以在這裡存取視窗進行初始化
            let window = app.get_webview_window("main").unwrap();
            
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

### 動態創建視窗 (未實作)

```rust
// 範例：創建渲染視窗
use tauri::{Manager, WebviewWindowBuilder};

#[tauri::command]
async fn create_render_window(app: tauri::AppHandle) -> Result<(), String> {
    let window = WebviewWindowBuilder::new(
        &app,
        "render-window",
        tauri::WebviewUrl::App("/render".into())
    )
    .title("Live2D Renderer")
    .inner_size(800.0, 600.0)
    .build()
    .map_err(|e| e.to_string())?;

    Ok(())
}
```

---

## 🔌 插件系統

### 插件載入流程

```
1. 應用程式啟動
    │
    ▼
2. PluginManager.initialize()
    │
    ▼
3. 掃描 {APP_DATA}/plugins/*.dll
    │
    ▼
4. 使用 libloading 動態載入
    │
    ▼
5. 呼叫 _plugin_create() 函式
    │
    ▼
6. 獲取 Box<dyn Plugin>
    │
    ▼
7. plugin.initialize(AppContext)
    │
    ▼
8. 插件註冊事件處理器
    │
    ▼
9. 插件準備就緒
```

### 插件開發範例

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
                id: "com.example.my-plugin".to_string(),
                name: "我的插件".to_string(),
                version: "1.0.0".to_string(),
                author: "作者名稱".to_string(),
            },
        }
    }
}

impl Plugin for MyPlugin {
    fn initialize(&mut self, ctx: &mut dyn AppContext) -> anyhow::Result<()> {
        ctx.log(&self.metadata.id, LogLevel::Info, "插件初始化完成");
        
        // 註冊事件處理器
        ctx.event_bus().subscribe("project_launched", |event| {
            println!("專案已啟動: {:?}", event);
        });
        
        Ok(())
    }

    fn metadata(&self) -> &PluginMetadata {
        &self.metadata
    }

    fn shutdown(&mut self) -> anyhow::Result<()> {
        Ok(())
    }
}

// 匯出插件 (必須)
declare_plugin!(MyPlugin, MyPlugin::new);
```

### Cargo.toml 設定

```toml
[package]
name = "my_plugin"
version = "1.0.0"

[lib]
crate-type = ["cdylib"]  # 動態函式庫

[dependencies]
plugin_lib = { path = "../plugin_lib" }
anyhow = "1.0"
```

### 建置與安裝

```powershell
# 建置插件
cd plugin_lib
cargo build --release

# 複製到插件目錄
Copy-Item target/release/my_plugin.dll {APP_DATA}/plugins/
```

---

## 🎨 渲染管線

### wgpu 架構

```
Renderer
├── State
│   ├── surface: Surface<'static>      # 綁定到 Tauri 視窗
│   ├── device: Device                 # GPU 裝置
│   ├── queue: Queue                   # 命令佇列
│   ├── config: SurfaceConfiguration   # 表面設定
│   └── size: PhysicalSize<u32>        # 視窗大小
│
├── _window: Arc<WebviewWindow>        # 持有視窗以確保生命週期
│
└── render_pipeline (待實作)
    ├── shader_module (shader.wgsl)
    ├── bind_group_layout
    │   ├── @binding(0) texture_sampler
    │   ├── @binding(1) texture_view
    │   └── @binding(2) base_color
    └── vertex_buffer (頂點資料)
```

### WGSL Shader 結構

**檔案**: `src-tauri/src/shader.wgsl`

```wgsl
// 頂點輸入 (來自頂點緩衝區)
struct VertexInput {
    @location(0) position: vec3<f32>,  // 每個頂點不同
    @location(1) uv: vec2<f32>,        // 每個頂點不同
}

// 頂點輸出 / 片段輸入
struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) uv: vec2<f32>,
}

// Uniforms (所有頂點共享)
@group(0) @binding(0)
var texture_sampler: sampler;

@group(0) @binding(1)
var texture_view: texture_2d<f32>;

@group(0) @binding(2)
var<uniform> base_color: vec4<f32>;

// 頂點著色器
@vertex
fn vs_main(in: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    out.clip_position = vec4<f32>(in.position, 1.0);
    out.uv = in.uv;
    return out;
}

// 片段著色器
@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let sampled_color = textureSample(texture_view, texture_sampler, in.uv);
    return sampled_color * base_color;
}
```

### 參數傳遞方式

| 類型 | 語法 | 用途 | 更新頻率 |
|------|------|------|----------|
| **頂點屬性** | `@location(N)` | 每個頂點不同的資料 (位置、UV、顏色) | 固定 (模型載入時) |
| **Uniform** | `@group(G) @binding(B)` | 所有頂點共享 (變換矩陣、全域參數) | 每幀或每次繪製 |
| **Texture** | `@group(G) @binding(B)` | 紋理資源 | 材質切換時 |
| **Sampler** | `@group(G) @binding(B)` | 紋理採樣器 | 固定 |

### 渲染循環

```rust
impl Renderer {
    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        // 1. 獲取 Surface 的下一幀紋理
        let output = self.state.surface.get_current_texture()?;
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());

        // 2. 創建命令編碼器
        let mut encoder = self.state.device.create_command_encoder(
            &wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            }
        );

        // 3. 開始渲染通道
        {
            let _render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.1, g: 0.2, b: 0.3, a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
            });
            
            // TODO: 在這裡繪製 Live2D 模型
            // render_pass.set_pipeline(&self.render_pipeline);
            // render_pass.set_bind_group(0, &self.bind_group, &[]);
            // render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            // render_pass.draw(0..vertex_count, 0..1);
        }

        // 4. 提交命令到 GPU
        self.state.queue.submit(std::iter::once(encoder.finish()));
        
        // 5. 呈現到螢幕
        output.present();

        Ok(())
    }
}
```

---

## 💾 資料儲存

### 儲存位置

```
{APP_DATA}/ai-vtuber-launcher/
├── projects/                   # 專案檔案
│   ├── {uuid-1}.proj          # bincode 序列化
│   ├── {uuid-2}.proj
│   └── ...
│
└── plugins/                    # 插件動態庫
    ├── my_plugin.dll          # Windows
    ├── my_plugin.so           # Linux
    └── my_plugin.dylib        # macOS
```

### 序列化格式 (bincode)

```rust
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct ProjectData {
    id: uuid::Uuid,
    name: String,
    path: String,
    description: String,
    mode: ProjectControlMode,
    created_at: chrono::DateTime<chrono::Utc>,
    updated_at: chrono::DateTime<chrono::Utc>,
}

// 儲存
pub fn save_project(app: &AppHandle, project: &ProjectData) -> Result<(), BackendError> {
    let path = app.path().app_data_dir()?.join("projects").join(format!("{}.proj", project.id));
    let data = bincode::serialize(project).map_err(BackendError::SerializationFailed)?;
    fs::write(path, data).map_err(BackendError::IoError)?;
    Ok(())
}

// 載入
pub fn load_project(path: &Path) -> Result<ProjectData, BackendError> {
    let data = fs::read(path).map_err(BackendError::IoError)?;
    bincode::deserialize(&data).map_err(BackendError::DeserializationFailed)
}
```

---

## 🔐 狀態管理

### AppState 結構

```rust
use parking_lot::RwLock;
use std::sync::Arc;

pub struct AppState {
    pub project_manager: Arc<RwLock<ProjectManager>>,
    pub plugin_manager: Arc<RwLock<PluginManager>>,
    pub log_level: fern::LevelFilter,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            project_manager: Arc::new(RwLock::new(ProjectManager::new())),
            plugin_manager: Arc::new(RwLock::new(PluginManager::new())),
            log_level: fern::LevelFilter::Info,
        }
    }
}
```

### 線程安全存取

```rust
#[tauri::command]
fn create_project(state: tauri::State<AppState>, name: String) -> Result<uuid::Uuid, String> {
    // 寫入鎖定
    let mut manager = state.project_manager.write();
    let project = ProjectData::new(&name, "", "", ProjectControlMode::Desktop);
    let id = project.id();
    manager.add_project(project);
    Ok(id)
    // 自動釋放鎖定
}

#[tauri::command]
fn list_projects(state: tauri::State<AppState>) -> Result<Vec<ProjectData>, String> {
    // 讀取鎖定
    let manager = state.project_manager.read();
    Ok(manager.get_recent_projects())
    // 自動釋放鎖定
}
```

---

## 🌍 國際化

### rust_i18n 配置

```rust
// main.rs
#[macro_use]
extern crate rust_i18n;

i18n!("locales", fallback = "zh-TW");

fn main() {
    // 在程式碼中使用
    let msg = t!("common.error");
    println!("{}", msg);
}
```

### 語系檔範例

**locales/zh-TW.yml**
```yaml
common:
  error: "錯誤"
  success: "成功"

project:
  created: "專案已建立"
  not_found: "找不到專案"
```

**locales/en.yml**
```yaml
common:
  error: "Error"
  success: "Success"

project:
  created: "Project created"
  not_found: "Project not found"
```

---

## 🚀 開發指南

### 環境設置

```powershell
# 安裝 Rust
winget install Rustlang.Rustup

# 安裝 Node.js + pnpm
winget install OpenJS.NodeJS
npm install -g pnpm

# 安裝依賴
pnpm install

# 安裝 Tauri CLI
cargo install tauri-cli
```

### 開發命令

```powershell
# 開發模式（熱重載）
pnpm dev

# 建置應用程式
pnpm tauri build

# 僅建置後端
cargo build --release

# 執行測試
cargo test

# 格式化程式碼
cargo fmt
pnpm biome format --write .

# 檢查錯誤
cargo clippy
pnpm biome check .
```

### 插件開發流程

```powershell
# 1. 創建插件專案
cargo new --lib my_plugin
cd my_plugin

# 2. 修改 Cargo.toml
[lib]
crate-type = ["cdylib"]

[dependencies]
plugin_lib = { path = "../plugin_lib" }

# 3. 實作 Plugin trait (參考 EXAMPLE_PLUGIN.md)

# 4. 建置
cargo build --release

# 5. 複製到插件目錄
Copy-Item target/release/my_plugin.dll "$env:APPDATA\ai-vtuber-launcher\plugins\"

# 6. 重啟應用程式以載入插件
```

### 渲染器開發

```rust
// 1. 在 setup 中初始化渲染器
.setup(|app| {
    let window = app.get_webview_window("main").unwrap();
    
    tauri::async_runtime::spawn(async move {
        let renderer = Renderer::new(Arc::new(window)).await.unwrap();
        // 儲存 renderer 到全域狀態
    });
    
    Ok(())
})

// 2. 實作渲染循環（在事件循環中）
// TODO: 整合到 Tauri 的事件系統
```

---

## 📊 效能考量

### 狀態管理

- ✅ 使用 `parking_lot::RwLock` 而非 `std::sync::RwLock`（更快）
- ✅ 讀多寫少的場景使用 `Arc<RwLock<T>>`
- ⚠️ 避免持有鎖定時間過長
- ⚠️ 避免在持有鎖定時進行 I/O 操作

### 事件系統

- ✅ 使用 `crossbeam::channel` 進行跨線程通訊
- ✅ 背景執行緒處理事件避免阻塞主線程
- ⚠️ 事件處理器應快速執行，長時間操作應開新執行緒

### 渲染

- ✅ wgpu 使用 GPU 硬體加速
- ✅ 頂點資料使用緩衝區快取
- ⚠️ 避免每幀重新建立管線和綁定組
- ⚠️ 紋理應預先載入並快取

---

## 🐛 常見問題

### 1. 插件載入失敗

**原因**:
- 缺少 `declare_plugin!` 巨集
- `Cargo.toml` 中未設定 `crate-type = ["cdylib"]`
- 插件與主程式的 ABI 不相容

**解決方案**:
```rust
// 確保插件匯出正確
declare_plugin!(MyPlugin, MyPlugin::new);

// Cargo.toml
[lib]
crate-type = ["cdylib"]
```

### 2. Surface 生命週期錯誤

**原因**:
- `Surface<'static>` 要求 window 存活整個程式週期
- window 提前被釋放

**解決方案**:
```rust
pub struct Renderer {
    state: State,
    _window: Arc<tauri::WebviewWindow>,  // 持有 window
}
```

### 3. 狀態死鎖

**原因**:
- 同時持有多個 `RwLock` 守衛
- 在持有鎖定時呼叫其他需要鎖定的函式

**解決方案**:
```rust
// ❌ 錯誤
let pm = state.project_manager.read();
let plm = state.plugin_manager.write();  // 可能死鎖

// ✓ 正確
{
    let pm = state.project_manager.read();
    // 使用 pm
}  // 釋放鎖定
{
    let plm = state.plugin_manager.write();
    // 使用 plm
}
```

---

## 📚 參考資源

### 官方文件

- [Tauri Documentation](https://tauri.app/v2/)
- [wgpu Tutorial](https://sotrh.github.io/learn-wgpu/)
- [SvelteKit Documentation](https://kit.svelte.dev/)
- [Live2D Cubism SDK](https://www.live2d.com/en/download/cubism-sdk/)

### 專案內部文件

- [plugin_lib/API_REFERENCE.md](plugin_lib/API_REFERENCE.md) - 插件 API 參考
- [plugin_lib/EXAMPLE_PLUGIN.md](plugin_lib/EXAMPLE_PLUGIN.md) - 插件開發範例
- [.github/copilot-instructions.md](.github/copilot-instructions.md) - Copilot 開發指引

---

## 🗺️ 開發路線圖

### 已完成 ✅

- [x] 基礎專案架構
- [x] 專案管理器 (CRUD)
- [x] 插件系統框架
- [x] 事件總線
- [x] 資料持久化 (bincode)
- [x] wgpu 渲染器初始化
- [x] WGSL Shader 基礎架構

### 進行中 🚧

- [ ] Live2D 模型載入與渲染
- [ ] 完整的渲染管線實作
- [ ] 前端 UI 介面開發

### 待開發 📋

- [ ] OBS 插件整合
- [ ] Discord Rich Presence
- [ ] 串流平台整合
- [ ] 專案設定頁面
- [ ] 插件市場
- [ ] 自動更新系統
- [ ] 效能監控面板

---

**版本**: 0.1.0  
**維護者**: AI VTuber Launcher Team  
**授權**: Live2D Open Software License

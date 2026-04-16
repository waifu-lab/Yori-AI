# Yori AI (Project Name Tentative)

## 核心特色

- **強大的渲染能力**
  - 原生支援 **Live2D Cubism** (.model3.json) 與 **VRM/GLTF** 3D 模型。

## 技術堆疊

本專案採用 **Rust Native 優先** 的混合架構，強調圖形效能與資源隔離。

| 層級           | 技術選型                  | 用途                                      |
| :------------- | :------------------------ | :---------------------------------------- |
| **Frontend**   | Svelte + TailwindCSS + TS | 使用者介面、儀表板、對話氣泡              |
| **App Shell**  | Tauri v2                  | 視窗管理、系統托盤、IPC                   |
| **Core Logic** | **Rust**                  | 主程式邏輯、串接後端                      |
| **Renderer**   | **wgpu** (WebGPU Native)  | 獨立於 WebView 的原生渲染環境 (Offscreen) |
| **Sharing**    | Spout2 (Windows API)      | 直播零延遲紋理共享                        |

## 系統架構

### 資料流與渲染管線

系統嚴格禁止 UI (JS) 直接操作渲染資源。渲染採用 **Offscreen Rendering** 機制：

1.  **Update Phase (Rust)**: 計算物理、Live2D 參數、VRM 骨架。
2.  **Draw Phase (wgpu)**: 繪製到一張 **Shared Texture (RGBA8)**。
3.  **Composite Phase**:
    - **Desktop**: 繪製到 Tauri 視窗綁定的 Surface。
    - **OBS**: 透過 Interop 寫入 Spout Sender。

### 專案目錄結構

```text
src-tauri/
├── src/
│   ├── commands/       # Tauri IPC (Layout, Project CRUD)
│   ├── renderer/       # wgpu 核心 (Engine, Spout2, Texture)
│   ├── models/         # Live2D & VRM 解析與處理
│   ├── audio/          # TTS 引擎與裝置路由
│   └── main.rs         # 程式入口
├── wit/                # Wasm Interface Type 定義 (.wit)
├── capabilities/       # Windows App Manifest
└── tauri.conf.json     # Tauri 配置
```

Spout2 為輸出obs用

### 語音辨識

Voice Activity Detection 前端處理
將語音資料傳給後端處理

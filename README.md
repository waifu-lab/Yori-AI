# Yori AI (Project Name Tentative)

> **打造具備「長期記憶」與「跨場景一致性」的 AI 伴侶載體。** 

Yori AI 不僅是一個聊天機器人，而是一個整合了渲染、記憶與多模態交互的 **OS 層級 AI 啟動器**。它旨在建立一個「單一存在 (One Entity)」的系統，無論是在桌面工作、直播互動或是 Discord 交流，使用者面對的都是同一個擁有連續記憶與統一視覺形象（Live2D/3D）的數位靈魂。

## 🌟 核心特色 (Key Features)

  * **單一後端，多重介面 (One Core, Multi-Interfaces)**
      * **桌面寵物模式**：背景透明、可穿透滑鼠的 OS 上層視窗，用於日常陪伴。
      * **直播助手模式**：透過 Spout2/NDI 輸出零延遲、無背景的共享紋理給 OBS，支援 VAD (語音活動偵測) 實現持續聆聽。
      * **Discord 協作模式**：Headless 運行，連結伺服器並同步桌面狀態（如：閒置、睡覺）。
  * **長期記憶系統 (Long-term Memory)**
      * 使用 **ChromaDB** 作為本地向量資料庫，具備分區儲存（Desktop/Stream/Discord）與動態總結機制。
  * **本地優先與隱私 (Local First)**
      * 對話紀錄與記憶預設儲存於本機，絕不主動上傳雲端。
  * **強大的渲染能力**
      * 原生支援 **Live2D Cubism** (.model3.json) 與 **VRM/GLTF** 3D 模型。
  * **Wasm 插件系統**
      * 基於 **Wasmtime** 的沙盒環境，允許開發者撰寫 Rust/Wasm 插件來擴充 AI 功能。

## 🛠️ 技術堆疊 (Tech Stack)

本專案採用 **Rust Native 優先** 的混合架構，強調圖形效能與資源隔離。

| 層級 | 技術選型 | 用途 |
| :--- | :--- | :--- |
| **Frontend** | Svelte + TailwindCSS + TS | 使用者介面、儀表板、對話氣泡  |
| **App Shell** | Tauri v2 | 視窗管理、系統托盤、IPC  |
| **Core Logic** | **Rust** | 主程式邏輯、LLM 串接、物理運算  |
| **Renderer** | **wgpu** (WebGPU Native) | 獨立於 WebView 的原生渲染環境 (Offscreen)  |
| **Memory** | ChromaDB (Python Sidecar) | 向量資料庫服務，透過 PyInstaller 打包  |
| **Sharing** | Spout2 (Windows API) | 直播零延遲紋理共享  |
| **Plugins** | Wasmtime | 安全的第三方插件執行環境  |

## 🏗️ 系統架構 (Architecture)

### 資料流與渲染管線

系統嚴格禁止 UI (JS) 直接操作渲染資源。渲染採用 **Offscreen Rendering** 機制：

1.  **Update Phase (Rust)**: 計算物理、Live2D 參數、VRM 骨架。
2.  **Draw Phase (wgpu)**: 繪製到一張 **Shared Texture (RGBA8)**。
3.  **Composite Phase**:
      * **Desktop**: 繪製到 Tauri 視窗綁定的 Surface。
      * **OBS**: 透過 Interop 寫入 Spout Sender。

### 專案目錄結構 (Project Structure)

```text
src-tauri/
├── src/
│   ├── commands/       # Tauri IPC (Layout, Project CRUD)
│   ├── renderer/       # wgpu 核心 (Engine, Spout2, Texture)
│   ├── models/         # Live2D & VRM 解析與處理
│   ├── memory/         # ChromaDB Client & Sidecar 管理
│   ├── plugin/         # Wasmtime Host & WIT Bindings
│   ├── audio/          # TTS 引擎與裝置路由
│   └── main.rs         # 程式入口
├── wit/                # Wasm Interface Type 定義 (.wit)
├── capabilities/       # Windows App Manifest
└── tauri.conf.json     # Tauri 配置
```



## 🚀 開發路線圖 (Roadmap)

  * **Phase 1: 基礎建設與渲染核心** 
      * [ ] 建立 Tauri v2 + wgpu 專案骨架
      * [ ] 實作 Windows 透明視窗與穿透邏輯
      * [ ] 載入靜態 Live2D/VRM 模型
  * **Phase 2: 核心邏輯與記憶體** 
      * [ ] 整合 ChromaDB Sidecar
      * [ ] 串接 LLM API (OpenAI/Anthropic)
      * [ ] 實作 TTS 與 LipSync (唇形同步)
  * **Phase 3: 互動與介面** 
      * [ ] 完善 Dashboard UI (專案管理)
      * [ ] 實作版面同步 (Layout Sync) 機制
  * **Phase 4: 直播功能** 
      * [ ] 實作 Spout2 紋理共享
      * [ ] 整合 VAD 實現持續聆聽
  * **Phase 5: 插件系統** 
      * [ ] 整合 Wasmtime 與 WIT
      * [ ] 實作權限管控 (plugin.toml)

## 📦 需求環境 (Prerequisites)

  * **OS**: Windows 10 (1903+) / Windows 11 
  * **Runtime**: WebView2 Runtime 
  * **Dev Tools**:
      * Rust (Latest Stable)
      * Node.js (for Svelte/Tauri CLI)
      * Python (for building ChromaDB Sidecar)
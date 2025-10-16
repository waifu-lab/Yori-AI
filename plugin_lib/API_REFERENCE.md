# Plugin Library API 參考文檔

本文檔提供 AI VTuber Launcher Plugin Library 的完整 API 參考。

## 📚 目錄

- [核心特質 (Traits)](#核心特質-traits)
  - [Plugin](#plugin)
  - [AppContext](#appcontext)
  - [EventBus](#eventbus)
- [資料型別 (Data Types)](#資料型別-data-types)
  - [PluginMetadata](#pluginmetadata)
  - [PluginData](#plugindata)
  - [Event](#event)
  - [LogLevel](#loglevel)
  - [MimeType](#mimetype)
- [巨集 (Macros)](#巨集-macros)
  - [declare_plugin!](#declare_plugin)

---

## 核心特質 (Traits)

### Plugin

插件的主要介面，所有插件必須實作此特質。

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

#### 方法說明

##### `metadata(&self) -> &PluginMetadata`

回傳插件的中繼資料。

**回傳值：** 包含插件 ID、名稱、作者、版本和描述的 `PluginMetadata` 引用。

**範例：**
```rust
fn metadata(&self) -> &PluginMetadata {
    &self.metadata
}
```

---

##### `is_deprecated(&self) -> bool`

指示此插件是否已被棄用。

**回傳值：** 
- `true`：插件已棄用，可能在未來版本中移除
- `false`：插件仍受支援

**範例：**
```rust
fn is_deprecated(&self) -> bool {
    false // 插件仍在維護中
}
```

---

##### `need_upgrade(&self) -> bool`

指示此插件是否需要升級。

**回傳值：**
- `true`：需要升級到新版本
- `false`：當前版本正常

**範例：**
```rust
fn need_upgrade(&self) -> bool {
    // 版本低於 1.0.0 需要升級
    self.metadata.version.starts_with("0.")
}
```

---

##### `initialize(&mut self, context: &mut dyn AppContext) -> anyhow::Result<()>`

初始化插件。在插件載入後立即呼叫。

**參數：**
- `context`：應用程式上下文，提供對主應用程式服務的存取

**回傳值：** `Result<()>`

**範例：**
```rust
fn initialize(&mut self, context: &mut dyn AppContext) -> anyhow::Result<()> {
    context.log(&self.metadata.id, LogLevel::Info, "初始化插件");
    
    // 載入配置
    if let Some(config) = context.get_config("my_setting") {
        self.apply_config(config)?;
    }
    
    // 註冊事件處理器
    let event_bus = context.event_bus();
    event_bus.on("my_event", Box::new(|event| {
        // 處理事件
    }));
    
    Ok(())
}
```

---

##### `start(&mut self) -> anyhow::Result<()>`

啟動插件。當專案啟動時呼叫。

**回傳值：** `Result<()>`

**範例：**
```rust
fn start(&mut self) -> anyhow::Result<()> {
    self.is_running = true;
    self.start_background_tasks()?;
    Ok(())
}
```

---

##### `stop(&mut self) -> anyhow::Result<()>`

停止插件。當專案停止時呼叫。

**回傳值：** `Result<()>`

**範例：**
```rust
fn stop(&mut self) -> anyhow::Result<()> {
    self.is_running = false;
    self.cleanup_resources()?;
    Ok(())
}
```

---

##### `reset(&mut self, context: &mut dyn AppContext) -> anyhow::Result<()>`

重置插件到初始狀態。

**參數：**
- `context`：應用程式上下文

**回傳值：** `Result<()>`

**範例：**
```rust
fn reset(&mut self, context: &mut dyn AppContext) -> anyhow::Result<()> {
    context.log(&self.metadata.id, LogLevel::Info, "重置插件");
    self.state = Default::default();
    self.settings = Self::default_settings();
    Ok(())
}
```

---

##### `get_settings(&self) -> HashMap<String, PluginData>`

取得當前插件設定。預設實作回傳空 HashMap。

**回傳值：** 包含設定鍵值對的 HashMap

**範例：**
```rust
fn get_settings(&self) -> HashMap<String, PluginData> {
    let mut settings = HashMap::new();
    settings.insert("enabled".to_string(), PluginData::boolean(true));
    settings.insert("api_key".to_string(), PluginData::string("".to_string()));
    settings
}
```

---

##### `update_settings(&mut self, settings: HashMap<String, PluginData>) -> anyhow::Result<()>`

更新插件設定。預設實作不做任何事。

**參數：**
- `settings`：新的設定值

**回傳值：** `Result<()>`

**範例：**
```rust
fn update_settings(&mut self, settings: HashMap<String, PluginData>) -> anyhow::Result<()> {
    // 驗證設定
    if let Some(api_key) = settings.get("api_key") {
        if let Some(key) = api_key.as_string() {
            if key.is_empty() {
                anyhow::bail!("API Key 不能為空");
            }
        }
    }
    
    self.settings = settings;
    Ok(())
}
```

---

### AppContext

提供對主應用程式服務的存取。

```rust
pub trait AppContext: Send + Sync {
    fn app_version(&self) -> &str;
    fn get_config(&self, key: &str) -> Option<String>;
    fn event_bus(&mut self) -> &mut dyn EventBus;
    fn log(&self, id: &str, level: LogLevel, message: &str);
}
```

#### 方法說明

##### `app_version(&self) -> &str`

取得主應用程式版本。

**範例：**
```rust
let version = context.app_version();
println!("應用程式版本: {}", version);
```

---

##### `get_config(&self, key: &str) -> Option<String>`

取得全域配置值。

**參數：**
- `key`：配置鍵

**回傳值：** `Option<String>`

**範例：**
```rust
if let Some(value) = context.get_config("app.language") {
    println!("語言設定: {}", value);
}
```

---

##### `event_bus(&mut self) -> &mut dyn EventBus`

存取事件總線。

**範例：**
```rust
let event_bus = context.event_bus();
event_bus.emit(Event::new("custom_event"));
```

---

##### `log(&self, id: &str, level: LogLevel, message: &str)`

記錄日誌訊息。

**參數：**
- `id`：插件 ID
- `level`：日誌等級
- `message`：日誌訊息

**範例：**
```rust
context.log(&self.metadata.id, LogLevel::Info, "操作完成");
context.log(&self.metadata.id, LogLevel::Error, "發生錯誤");
```

---

### EventBus

事件總線，用於插件與主應用程式之間的事件驅動通訊。

```rust
pub trait EventBus: Send + Sync + 'static {
    fn emit(&mut self, event: Event);
    fn off(&mut self, id: uuid::Uuid);
    fn on(&mut self, name: &str, handler: Box<dyn FnMut(Event) + Send + Sync>) -> uuid::Uuid;
}
```

#### 方法說明

##### `emit(&mut self, event: Event)`

發送事件。

**參數：**
- `event`：要發送的事件

**範例：**
```rust
let event = Event::new("user_login")
    .with("username", "alice")
    .with("timestamp", chrono::Utc::now().to_rfc3339())
    .freeze();
    
event_bus.emit(event);
```

---

##### `on(&mut self, name: &str, handler: Box<dyn FnMut(Event) + Send + Sync>) -> uuid::Uuid`

註冊事件處理器。

**參數：**
- `name`：事件名稱
- `handler`：事件處理器閉包

**回傳值：** 處理器 UUID（用於後續移除）

**範例：**
```rust
let handler_id = event_bus.on("message_received", Box::new(|event| {
    if let Some(msg) = event.get::<String>("content") {
        println!("收到訊息: {}", msg);
    }
}));
```

---

##### `off(&mut self, id: uuid::Uuid)`

移除事件處理器。

**參數：**
- `id`：處理器 UUID（由 `on` 方法回傳）

**範例：**
```rust
event_bus.off(handler_id);
```

---

## 資料型別 (Data Types)

### PluginMetadata

插件中繼資料。

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginMetadata {
    pub id: String,
    pub name: String,
    pub author: String,
    pub version: String,
    pub description: String,
}
```

#### 欄位說明

- `id`：插件唯一識別碼（建議使用反向網域名稱，如 `com.example.myplugin`）
- `name`：插件顯示名稱
- `author`：作者名稱
- `version`：版本號（建議使用語意化版本）
- `description`：插件描述

**範例：**
```rust
PluginMetadata {
    id: "com.example.chatbot".to_string(),
    name: "智慧聊天機器人".to_string(),
    author: "Alice Smith".to_string(),
    version: "1.0.0".to_string(),
    description: "提供智慧對話功能的聊天機器人插件".to_string(),
}
```

---

### PluginData

型態化的插件設定資料。

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum PluginData {
    String { value: String, max_length: Option<usize> },
    Integer { value: i64, min: Option<i64>, max: Option<i64> },
    Float { value: f64, min: Option<f64>, max: Option<f64> },
    Boolean { value: bool },
    Options { value: String, placeholder: String, options: Vec<String> },
    Stream { value: String, mime_type: MimeType },
    Array { value: Box<Vec<PluginData>> },
    Object { value: Box<HashMap<String, PluginData>> },
}
```

#### 建構方法

##### String 型別

```rust
// 基本字串
PluginData::string("Hello".to_string())

// 帶最大長度限制
PluginData::string_with_max("Hello".to_string(), 100)
```

##### Integer 型別

```rust
// 基本整數
PluginData::integer(42)

// 帶範圍限制
PluginData::integer_with_range(50, Some(0), Some(100))
```

##### Float 型別

```rust
// 基本浮點數
PluginData::float(3.14)

// 帶範圍限制
PluginData::float_with_range(0.5, Some(0.0), Some(1.0))
```

##### Boolean 型別

```rust
PluginData::boolean(true)
```

##### Options 型別

```rust
// 基本選項列表
PluginData::options(
    "option1".to_string(),
    vec!["option1".to_string(), "option2".to_string()]
)

// 帶佔位符
PluginData::options_with_placeholder(
    "".to_string(),
    vec!["small".to_string(), "medium".to_string(), "large".to_string()],
    "選擇大小".to_string()
)
```

##### Stream 型別

```rust
// 基本串流
PluginData::stream("ws://localhost:8080".to_string())

// 帶 MIME 類型
PluginData::stream_with_mime(
    "ws://localhost:8080".to_string(),
    MimeType::ApplicationJson
)
```

##### Array 型別

```rust
PluginData::array(vec![
    PluginData::string("item1".to_string()),
    PluginData::string("item2".to_string()),
])
```

##### Object 型別

```rust
let mut obj = HashMap::new();
obj.insert("key1".to_string(), PluginData::string("value1".to_string()));
obj.insert("key2".to_string(), PluginData::integer(42));
PluginData::object(obj)
```

#### 檢查方法

```rust
data.is_string()   // -> bool
data.is_integer()  // -> bool
data.is_float()    // -> bool
data.is_boolean()  // -> bool
data.is_options()  // -> bool
data.is_stream()   // -> bool
data.is_array()    // -> bool
data.is_object()   // -> bool
```

#### 提取方法

```rust
data.as_string()   // -> Option<String>
data.as_integer()  // -> Option<i64>
data.as_float()    // -> Option<f64>
data.as_boolean()  // -> Option<bool>
data.as_options()  // -> Option<(String, Vec<String>, String)>
data.as_stream()   // -> Option<(String, MimeType)>
data.as_array()    // -> Option<Vec<PluginData>>
data.as_object()   // -> Option<HashMap<String, PluginData>>
```

---

### Event

事件資料結構。

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    name: String,
    data: HashMap<String, Value>,
    modifiable: bool,
}
```

#### 方法

##### `new(name: impl Into<String>) -> Self`

建立新事件。

```rust
let event = Event::new("user_login");
```

---

##### `name(&self) -> &str`

取得事件名稱。

```rust
let name = event.name();
```

---

##### `with(self, key: impl Into<String>, value: impl Serialize) -> Self`

添加資料到事件（可鏈式呼叫）。

```rust
let event = Event::new("message")
    .with("content", "Hello")
    .with("sender", "Alice")
    .with("timestamp", 1234567890);
```

---

##### `get<T: for<'de> Deserialize<'de>>(&self, key: &str) -> Option<T>`

從事件取得資料。

```rust
if let Some(content) = event.get::<String>("content") {
    println!("內容: {}", content);
}

if let Some(timestamp) = event.get::<i64>("timestamp") {
    println!("時間戳: {}", timestamp);
}
```

---

##### `contains_key(&self, key: &str) -> bool`

檢查鍵是否存在。

```rust
if event.contains_key("sender") {
    // 處理
}
```

---

##### `freeze(self) -> Self`

凍結事件，防止後續修改。

```rust
let event = Event::new("final_event")
    .with("data", "value")
    .freeze();
```

---

##### `is_frozen(&self) -> bool`

檢查事件是否已凍結。

```rust
if event.is_frozen() {
    println!("事件已凍結");
}
```

---

### LogLevel

日誌等級。

```rust
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}
```

#### 使用範例

```rust
context.log(id, LogLevel::Trace, "詳細追蹤資訊");
context.log(id, LogLevel::Debug, "偵錯資訊");
context.log(id, LogLevel::Info, "一般資訊");
context.log(id, LogLevel::Warn, "警告訊息");
context.log(id, LogLevel::Error, "錯誤訊息");
```

---

### MimeType

MIME 類型列舉。

```rust
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub enum MimeType {
    #[default]
    TextPlain,
    TextHtml,
    ApplicationJson,
    ImagePng,
    ImageJpeg,
    AudioMpeg,
    VideoMp4,
    Other(String),
}
```

#### 使用範例

```rust
// 預定義類型
MimeType::ApplicationJson
MimeType::TextHtml
MimeType::ImagePng

// 自訂類型
MimeType::Other("application/xml".to_string())
```

---

## 巨集 (Macros)

### declare_plugin!

宣告插件進入點。**這是必須的**，否則插件無法被載入。

#### 語法

```rust
declare_plugin!(PluginType, constructor_function);
```

#### 參數

- `PluginType`：實作 `Plugin` 特質的型別
- `constructor_function`：建構函式（`fn() -> PluginType`）

#### 範例

```rust
pub struct MyPlugin {
    // ...
}

impl MyPlugin {
    pub fn new() -> Self {
        Self { /* ... */ }
    }
}

impl Plugin for MyPlugin {
    // 實作...
}

// 宣告進入點
declare_plugin!(MyPlugin, MyPlugin::new);
```

#### 注意事項

- 必須在 `lib.rs` 的最後呼叫
- 每個插件只能呼叫一次
- 確保 `Cargo.toml` 中設定了 `crate-type = ["cdylib"]`

---

## 📝 完整範例

```rust
use plugin_lib::types::{Plugin, AppContext, PluginMetadata, PluginData, LogLevel, Event};
use plugin_lib::declare_plugin;
use std::collections::HashMap;

pub struct ExamplePlugin {
    metadata: PluginMetadata,
    settings: HashMap<String, PluginData>,
}

impl ExamplePlugin {
    pub fn new() -> Self {
        let mut settings = HashMap::new();
        settings.insert("enabled".to_string(), PluginData::boolean(true));
        
        Self {
            metadata: PluginMetadata {
                id: "com.example.demo".to_string(),
                name: "示範插件".to_string(),
                author: "開發者".to_string(),
                version: "1.0.0".to_string(),
                description: "API 示範插件".to_string(),
            },
            settings,
        }
    }
}

impl Plugin for ExamplePlugin {
    fn metadata(&self) -> &PluginMetadata {
        &self.metadata
    }

    fn is_deprecated(&self) -> bool {
        false
    }

    fn need_upgrade(&self) -> bool {
        false
    }

    fn initialize(&mut self, context: &mut dyn AppContext) -> anyhow::Result<()> {
        context.log(&self.metadata.id, LogLevel::Info, "初始化中...");
        
        let event_bus = context.event_bus();
        event_bus.on("test_event", Box::new(|event| {
            if let Some(data) = event.get::<String>("data") {
                println!("收到: {}", data);
            }
        }));
        
        Ok(())
    }

    fn start(&mut self) -> anyhow::Result<()> {
        Ok(())
    }

    fn stop(&mut self) -> anyhow::Result<()> {
        Ok(())
    }

    fn reset(&mut self, _context: &mut dyn AppContext) -> anyhow::Result<()> {
        Ok(())
    }

    fn get_settings(&self) -> HashMap<String, PluginData> {
        self.settings.clone()
    }

    fn update_settings(&mut self, settings: HashMap<String, PluginData>) -> anyhow::Result<()> {
        self.settings = settings;
        Ok(())
    }
}

declare_plugin!(ExamplePlugin, ExamplePlugin::new);
```

---

## 📚 更多資源

- [README](./README.md) - 插件開發概覽
- [EXAMPLE_PLUGIN](./EXAMPLE_PLUGIN.md) - 完整專案範例
- [主專案文檔](../.github/copilot-instructions.md)

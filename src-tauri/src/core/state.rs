use parking_lot::Mutex;

pub const STORE_KEY_BACKEND_URL: &str = "backend_url";
pub const STORE_KEY_LIVE2D_MODEL_PATH: &str = "live2d_model_path";
pub const STORE_KEY_LIVE2D_SCALE: &str = "live2d_scale";
pub const STORE_KEY_LIVE2D_OFFSET_X: &str = "live2d_offset_x";
pub const STORE_KEY_LIVE2D_OFFSET_Y: &str = "live2d_offset_y";
const DEFAULT_BACKEND_URL: &str = "ws://127.0.0.1:8765";

/// In-memory runtime state shared across Tauri commands.
pub struct AppState {
    /// Active conversation id assigned by the backend (None until first response).
    pub conversation_id: Mutex<Option<String>>,
    /// Cached backend base URL (ws://host:port). Persisted via tauri-plugin-store.
    pub backend_url: Mutex<String>,
    /// Whether the currently loaded Live2D model lacks a native LipSync parameter
    /// group, so the backend should compute and return viseme data.
    pub need_backend_lipsync: Mutex<bool>,
    /// Last used Live2D model path. Persisted via tauri-plugin-store.
    pub live2d_model_path: Mutex<Option<String>>,
    /// Live2D display transform. Persisted via tauri-plugin-store.
    pub live2d_scale: Mutex<f64>,
    pub live2d_offset_x: Mutex<f64>,
    pub live2d_offset_y: Mutex<f64>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            conversation_id: Mutex::new(None),
            backend_url: Mutex::new(DEFAULT_BACKEND_URL.to_string()),
            need_backend_lipsync: Mutex::new(false),
            live2d_model_path: Mutex::new(None),
            live2d_scale: Mutex::new(1.0),
            live2d_offset_x: Mutex::new(0.0),
            live2d_offset_y: Mutex::new(0.0),
        }
    }

    pub fn set_conversation_id(&self, id: Option<String>) {
        *self.conversation_id.lock() = id;
    }

    pub fn get_conversation_id(&self) -> Option<String> {
        self.conversation_id.lock().clone()
    }

    pub fn get_backend_url(&self) -> String {
        self.backend_url.lock().clone()
    }

    pub fn set_backend_url(&self, url: String) {
        *self.backend_url.lock() = url;
    }

    pub fn set_need_backend_lipsync(&self, need: bool) {
        *self.need_backend_lipsync.lock() = need;
    }

    pub fn get_need_backend_lipsync(&self) -> bool {
        *self.need_backend_lipsync.lock()
    }

    pub fn get_live2d_model_path(&self) -> Option<String> {
        self.live2d_model_path.lock().clone()
    }

    pub fn set_live2d_model_path(&self, path: Option<String>) {
        *self.live2d_model_path.lock() = path;
    }

    pub fn get_live2d_transform(&self) -> (f64, f64, f64) {
        (
            *self.live2d_scale.lock(),
            *self.live2d_offset_x.lock(),
            *self.live2d_offset_y.lock(),
        )
    }

    pub fn set_live2d_transform(&self, scale: f64, offset_x: f64, offset_y: f64) {
        *self.live2d_scale.lock() = scale;
        *self.live2d_offset_x.lock() = offset_x;
        *self.live2d_offset_y.lock() = offset_y;
    }
}


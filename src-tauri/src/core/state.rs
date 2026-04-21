use parking_lot::Mutex;

pub const STORE_KEY_BACKEND_URL: &str = "backend_url";
const DEFAULT_BACKEND_URL: &str = "ws://127.0.0.1:8765";

/// In-memory runtime state shared across Tauri commands.
pub struct AppState {
    /// Active conversation id assigned by the backend (None until first response).
    pub conversation_id: Mutex<Option<String>>,
    /// Cached backend base URL (ws://host:port). Persisted via tauri-plugin-store.
    pub backend_url: Mutex<String>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            conversation_id: Mutex::new(None),
            backend_url: Mutex::new(DEFAULT_BACKEND_URL.to_string()),
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
}


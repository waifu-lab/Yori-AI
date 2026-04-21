use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ClientMsg {
    Chat(ChatMsg),
    Voice(VoiceMsg),
}

#[derive(Debug, Clone, Serialize)]
pub struct ChatMsg {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversation_id: Option<String>,
    pub request_id: String,
    pub content: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub live2d: Option<bool>,
}

#[derive(Debug, Clone, Serialize)]
pub struct VoiceMsg {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversation_id: Option<String>,
    pub request_id: String,
    pub audio: String,  // base64
    pub format: String, // "webm"
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ServerMsg {
    Chunk(ChunkMsg),
    Done(DoneMsg),
    Event(EventMsg),
    Error(BackendError),
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ChunkMsg {
    pub request_id: String,
    pub content: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DoneMsg {
    pub request_id: String,
    pub content: String,
}

/// Live2D / expression event; carries arbitrary extra fields from the backend hook.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EventMsg {
    #[serde(flatten)]
    pub data: serde_json::Map<String, serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BackendError {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    pub code: String,
    pub message: String,
}

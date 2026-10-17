use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ClientMsg {
    Chat(ChatMsg),
    Voice(VoiceMsg),
    Live2dsync(Live2dSyncMsg),
    #[serde(rename = "event")]
    UserEvent(UserEventMsg),
    Stop(StopMsg),
}

#[derive(Debug, Clone, Serialize)]
pub struct StopMsg {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversation_id: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct UserEventMsg {
    pub name: String,
    pub data: serde_json::Value,
}

#[derive(Debug, Clone, Serialize)]
pub struct Live2dSyncMsg {
    pub expressions: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Attachment {
    pub filename: String,
    pub content_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ChatMsg {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversation_id: Option<String>,
    pub request_id: String,
    pub content: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Vec<Attachment>>,
    /// Frontend capability flags, see [`FEATURE_LIVE2D`] / [`FEATURE_LIPSYNC`].
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub features: Vec<&'static str>,
}

/// Frontend can show emotions / play audio: backend emits emotion / audio events.
pub const FEATURE_LIVE2D: &str = "live2d";
/// Backend should compute lipsync visemes (the model has no native LipSync).
/// Only relevant together with `live2d` and TTS enabled.
pub const FEATURE_LIPSYNC: &str = "lipsync";

#[derive(Debug, Clone, Serialize)]
pub struct VoiceMsg {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversation_id: Option<String>,
    pub request_id: String,
    pub audio: String,  // base64
    pub format: String, // e.g. "wav", "webm"
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub features: Vec<&'static str>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ServerMsg {
    Connected(ConnectedMsg),
    Live2dsyncAck(Live2dSyncAckMsg),
    Transcript(TranscriptMsg),
    Chunk(ChunkMsg),
    Done(DoneMsg),
    Event(EventMsg),
    Error(BackendError),
    Stopped(StoppedMsg),
}

/// Reply to `stop`; everything the interrupted request sent arrives before this.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct StoppedMsg {
    pub request_id: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TranscriptMsg {
    pub request_id: String,
    pub content: String,
}

/// First message sent by the server after WebSocket connection is established.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ConnectedMsg {
    pub session_token: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Live2dSyncAckMsg {
    pub count: usize,
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

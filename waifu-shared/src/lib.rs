use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeHello {
    #[serde(rename = "type")]
    pub msg_type: String, // "node_hello"
    pub protocol: String,
    pub version: String,
    pub token: String,
    pub node_id: Uuid,
    pub platform: String,
    pub display_name: String,
    pub caps: Vec<String>,

    // skill 相關
    pub skills_dir: String,
    pub installed_skills: Vec<InstalledSkill>,

    pub tools: Vec<ToolSchema>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstalledSkill {
    pub skill_id: String,
    pub version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeSkillReady {
    #[serde(rename = "type")]
    pub msg_type: String, // "node_skill_ready"
    pub skill_id: String,
    pub version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeSkillSyncError {
    #[serde(rename = "type")]
    pub msg_type: String, // "node_skill_sync_error"
    pub skill_id: String,
    pub code: String,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeResult {
    #[serde(rename = "type")]
    pub msg_type: String, // "node_result"
    pub request_id: Uuid,
    pub ok: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<NodeError>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeToolCall {
    #[serde(rename = "type")]
    pub msg_type: String, // "node_tool_call"
    pub request_id: Uuid,
    pub tool: String,
    pub params: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodePong {
    #[serde(rename = "type")]
    pub msg_type: String, // "node_pong"
    pub ts: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeBye {
    #[serde(rename = "type")]
    pub msg_type: String, // "node_bye"
    pub reason: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeWelcome {
    #[serde(rename = "type")]
    pub msg_type: String, // "node_welcome"
    pub session_id: Uuid,
    pub accepted_caps: Vec<String>,
    pub accepted_tools: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeReject {
    #[serde(rename = "type")]
    pub msg_type: String, // "node_reject"
    pub code: String,
    pub reason: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeInvoke {
    #[serde(rename = "type")]
    pub msg_type: String, // "node_invoke"
    pub request_id: Uuid,
    pub action: String,
    pub params: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeSkillSync {
    #[serde(rename = "type")]
    pub msg_type: String, // "node_skill_sync"
    pub skill_id: String,
    pub version: String,
    pub base_url: String,
    pub files: Vec<SkillFile>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillFile {
    pub path: String,
    pub sha256: String,
    pub size: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeToolResult {
    #[serde(rename = "type")]
    pub msg_type: String, // "node_tool_result"
    pub request_id: Uuid,
    pub ok: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<NodeError>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodePing {
    #[serde(rename = "type")]
    pub msg_type: String, // "node_ping"
    pub ts: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeError {
    pub code: String,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolSchema {
    pub name: String,
    pub description: String,
    pub parameters: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum InboundMsg {
    NodeWelcome(NodeWelcome),
    NodeReject(NodeReject),
    NodeInvoke(NodeInvoke),
    NodeSkillSync(NodeSkillSync),
    NodeToolResult(NodeToolResult),
    NodePing(NodePing),
}

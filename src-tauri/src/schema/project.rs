use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ProjectMeta {
    pub id: uuid::Uuid,
    pub name: String,
    pub description: String,
    pub version: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub data_root: std::path::PathBuf,
    pub mode: ProjectMode,
}

bitflags::bitflags! {
    #[derive(Serialize, Deserialize, Clone, Copy, Debug)]
    pub struct ProjectMode: u32 {
        const Desktop   = 0b001;
        const Streaming = 0b010;
        const Discord   = 0b100;
    }
}
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ProjectMeta {
    pub id: uuid::Uuid,
    pub name: String,
    pub description: String,
    pub version: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub data_root: std::path::PathBuf,
}
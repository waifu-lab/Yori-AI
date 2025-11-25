use std::{collections::HashMap, time};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
pub struct ProjectData {
    id: uuid::Uuid,
    name: String,
    path: String,
    description: String,
    mode: ProjectControlMode,
    plugins: Vec<String>,
    created_at: time::SystemTime,
    last_opened: time::SystemTime,
    metadata: ProjectMetadata,
}

#[derive(Deserialize, Serialize, Clone, Copy)]
pub enum ProjectControlMode {
    Streaming,
    Desktop,
    Discord,
    Union(u32),
    // Add more modes as needed
}

#[derive(Deserialize, Serialize, Clone)]
pub struct ProjectMetadata {
    plugin_data: HashMap<String, (String, String)>, // (type, value)
    // Add metadata fields as needed
}

impl ProjectData {
    pub fn id(&self) -> uuid::Uuid {
        self.id
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn path(&self) -> String {
        self.path.clone()
    }

    pub fn description(&self) -> String {
        self.description.clone()
    }

    pub fn mode(&self) -> ProjectControlMode {
        self.mode.clone()
    }

    pub fn plugins(&self) -> Vec<String> {
        self.plugins.clone()
    }

    pub fn created_at(&self) -> time::SystemTime {
        self.created_at.clone()
    }

    pub fn last_opened(&self) -> time::SystemTime {
        self.last_opened.clone()
    }

    pub fn update_name(&mut self, name: &str) {
        self.name = name.to_string();
    }

    pub fn update_description(&mut self, description: &str) {
        self.description = description.to_string();
    }

    pub fn update_last_opened(&mut self, time: time::SystemTime) {
        self.last_opened = time;
    }

    pub fn add_plugin(&mut self, plugin_id: &str) {
        self.plugins.push(plugin_id.to_string());
    }

    pub fn remove_plugin(&mut self, plugin_id: &str) {
        self.plugins.retain(|p| p != plugin_id);
    }
}

impl ProjectData {
    pub fn new(name: &str, path: &str, description: &str, mode: ProjectControlMode) -> Self {
        Self {
            id: uuid::Uuid::new_v4(),
            name: name.to_string(),
            path: path.to_string(),
            description: description.to_string(),
            mode: mode,
            plugins: Vec::new(),
            created_at: time::SystemTime::now(),
            last_opened: time::SystemTime::now(),
            metadata: ProjectMetadata {
                plugin_data: HashMap::new(),
            },
        }
    }
}

impl std::ops::BitOr for ProjectControlMode {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (ProjectControlMode::Union(a), ProjectControlMode::Union(b)) => {
                ProjectControlMode::Union(a | b)
            }
            (ProjectControlMode::Union(a), other) | (other, ProjectControlMode::Union(a)) => {
                let b = match other {
                    ProjectControlMode::Streaming => 0b0001,
                    ProjectControlMode::Desktop => 0b0010,
                    ProjectControlMode::Discord => 0b0100,
                    _ => 0,
                };
                ProjectControlMode::Union(a | b)
            }
            (a, b) => {
                let a_bit = match a {
                    ProjectControlMode::Streaming => 0b0001,
                    ProjectControlMode::Desktop => 0b0010,
                    ProjectControlMode::Discord => 0b0100,
                    _ => 0,
                };
                let b_bit = match b {
                    ProjectControlMode::Streaming => 0b0001,
                    ProjectControlMode::Desktop => 0b0010,
                    ProjectControlMode::Discord => 0b0100,
                    _ => 0,
                };
                ProjectControlMode::Union(a_bit | b_bit)
            }
        }
    }
}
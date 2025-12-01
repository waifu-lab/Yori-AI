use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PluginConfig {
    pub package: PackageInfo,
    pub permissions: PluginPermissionRequest,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PackageInfo {
    pub id: String,
    pub name: String,
    pub version: String,
    pub description: String,
    pub author: String,
    #[serde(default)]
    pub repository: String,
    #[serde(default)]
    pub website: String,
    pub entry: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PluginPermissionRequest {
    filesystem: Vec<String>,  // e.g. read:data/, write:config.json
    network: Vec<String>,     // e.g. api.example.com, *.example.org
}
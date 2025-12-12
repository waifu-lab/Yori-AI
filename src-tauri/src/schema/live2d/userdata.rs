// userdata3.json schema

use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Config {
    pub version: u8,
    pub meta: Meta,
    pub user_data: Vec<UserData>,
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Meta {
    pub user_data_count: u32,
    pub total_user_data_size: u32,
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct UserData {
    pub target: String,
    pub id: String,
    pub value: String,
}

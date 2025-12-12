// motion3.json schema

use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Config {
    pub version: u8,
    pub meta: Meta,
    pub curves: Vec<Curve>,
    pub user_data: Option<Vec<UserData>>,
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Meta {
    pub duration: f32,
    pub fps: f32,
    pub r#loop: Option<bool>,
    pub are_beziers_restricted: Option<bool>,
    pub fade_in_time: Option<f32>,
    pub fade_out_time: Option<f32>,
    pub curve_count: u32,
    pub total_segment_count: u32,
    pub total_point_count: u32,
    pub user_data_count: Option<u32>,
    pub total_user_data_size: Option<u32>,
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Curve {
    pub target: String,
    pub id: String,
    pub fade_in_time: Option<f32>,
    pub fade_out_time: Option<f32>,
    pub segments: Vec<f32>,
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct UserData {
    pub time: f32,
    pub value: String,
}
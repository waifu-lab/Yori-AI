// model3.json schema

use std::collections::HashMap;
use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Config {
    pub version: u8,
    pub file_references: FileReferences,
    pub groups: Option<Vec<Group>>,
    pub hit_areas: Option<Vec<HitArea>>,
    pub layout: Option<Layout>,
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct FileReferences {
    pub moc: String,
    pub textures: Vec<String>,
    pub physics: Option<String>,
    pub user_data: Option<String>,
    pub pose: Option<String>,
    pub display_info: Option<String>,
    pub motion_sync: Option<String>,
    pub expressions: Option<Vec<ExpressionFile>>,
    pub motions: Option<HashMap<String, Vec<MotionFile>>>,
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ExpressionFile {
    pub name: String,
    pub file: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MotionFile {
    pub file: String,
    pub fade_out_time: Option<f32>,
    pub fade_in_time: Option<f32>,
    pub sound: Option<String>,
    pub motion_sync: Option<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Group {
    pub target: String,
    pub name: String,
    pub ids: Vec<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct HitArea {
    pub name: String,
    pub id: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Layout {
    pub width: f32,
    pub height: f32,
    pub x: f32,
    pub y: f32,
    pub center_x: f32,
    pub center_y: f32,
    pub top: f32,
    pub bottom: f32,
    pub left: f32,
    pub right: f32,
}
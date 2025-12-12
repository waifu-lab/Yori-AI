// physics3.json schema

use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Config {
    pub physics_settings: Vec<Setting>,
    pub version: u8,
    pub meta: Meta,
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Setting {
    pub id: String,
    pub input: Vec<Input>,
    pub output: Vec<Output>,
    pub vertices: Vec<Vertex>,
    pub normalization: Normalization,
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Meta {
    pub physics_setting_count: u32,
    pub total_input_count: u32,
    pub total_output_count: u32,
    pub vertex_count: u32,
    pub fps: Option<u32>,
    pub effective_forces: EffectiveForce,
    pub physics_dictionary: Vec<PhysicsDictionary>,
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct EffectiveForce {
    pub gravity: Vector2,
    pub wind: Vector2,
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PhysicsDictionary {
    pub id: String,
    pub name: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Input {
    pub source: Parameter,
    pub weight: f32,
    pub r#type: Type,
    pub reflect: bool,
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Output {
    pub destination: Parameter,
    pub vertex_index: u32,
    pub scale: f32,
    pub weight: f32,
    pub r#type: Type,
    pub reflect: bool,
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum Type {
    X,
    Angle,
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Vertex {
    pub position: Vector2,
    pub mobility: f32,
    pub delay: f32,
    pub acceleration: f32,
    pub radius: f32,
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Normalization {
    pub position: NormalizationValue,
    pub angle: NormalizationValue,
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct NormalizationValue {
    pub minimum: f32,
    pub default: f32,
    pub maximum: f32,
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Parameter {
    pub target: String,
    pub id: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}
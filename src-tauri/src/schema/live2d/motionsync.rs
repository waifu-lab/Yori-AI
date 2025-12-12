// motionsync3.json schema

use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Config {
    pub version: u8,
    pub meta: Meta,
    pub settings: Vec<Setting>,
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Meta {
    pub setting_count: u32,
    pub dictionary: Vec<Dictionary>,
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Dictionary {
    pub id: String,
    pub name: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Setting {
    pub id: String,
    pub analysis_type: String,
    pub use_case: String,
    pub cubism_parameters: Vec<CubismParameter>,
    pub audio_parameters: Vec<AudioParameter>,
    pub mappings: Vec<MappingTarget>,
    pub post_processing: PostProcessing,
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CubismParameter {
    pub name: String,
    pub id: String,
    pub min: f32,
    pub max: f32,
    pub damper: f32,
    pub smooth: f32,
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AudioParameter {
    pub name: String,
    pub id: String,
    pub min: f32,
    pub max: f32,
    pub scale: f32,
    pub enabled: bool,
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MappingTarget {
    pub r#type: String,
    pub id: String,
    pub targets: Vec<MappingParameter>,
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MappingParameter {
    pub id: String,
    pub value: f32,
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PostProcessing {
    pub blend_ratio: f32,
    pub smoothing: f32,
    pub sample_rate: f32,
}

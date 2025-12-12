// exp3.json schema

use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Config {
    pub r#type: ExpressionType,
    pub fade_in_time: Option<f32>,
    pub fade_out_time: Option<f32>,
    pub parameters: Vec<Parameter>,
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Parameter {
    pub id: String,
    pub value: f32,
    pub blend: Option<Blend>,
}

#[derive(Deserialize)]
pub enum ExpressionType {
    #[serde(rename = "Live2D Expression")]
    Live2DExpression,
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum Blend {
    Add,
    Multiply,
    Overwrite,
}

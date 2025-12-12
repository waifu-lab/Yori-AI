// cdi3.json schema

use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Config {
    pub version: u8,
    pub parameters: Option<Vec<Parameter>>,
    pub parameter_groups: Vec<ParameterGroup>,
    pub parts: Vec<Part>,
    pub combined_parameters: Option<Vec<Vec<String>>>,
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Parameter {
    pub id: String,
    pub group_id: String,
    pub name: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ParameterGroup {
    pub id: String,
    pub group_id: String,
    pub name: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Part {
    pub id: String,
    pub name: String,
}
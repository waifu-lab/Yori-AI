// pose3.json schema

use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Config {
    pub r#type: PoseType,
    pub fade_in_time: Option<f32>,
    pub groups: Vec<Vec<Node>>,
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Node {
    pub id: String,
    pub link: Option<Vec<String>>,
}

#[derive(Deserialize)]
pub enum PoseType {
    #[serde(rename = "Live2D Pose")]
    Live2DPose,
}

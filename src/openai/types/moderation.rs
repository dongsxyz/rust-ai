use super::model::Model;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Result {
    pub flagged: bool,
    pub categories: CategoryFlags,
    pub category_scores: CategoryScores,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CategoryFlags {
    #[serde(rename = "hate")]
    hate: bool,
    #[serde(rename = "hate/threatening")]
    hate_or_threatening: bool,
    #[serde(rename = "self-harm")]
    self_harm: bool,
    #[serde(rename = "sexual")]
    sexual: bool,
    #[serde(rename = "sexual/minors")]
    sexual_or_minors: bool,
    #[serde(rename = "violence")]
    violence: bool,
    #[serde(rename = "violence/graphic")]
    violence_or_graphic: bool,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CategoryScores {
    #[serde(rename = "hate")]
    hate: f32,
    #[serde(rename = "hate/threatening")]
    hate_or_threatening: f32,
    #[serde(rename = "self-harm")]
    self_harm: f32,
    #[serde(rename = "sexual")]
    sexual: f32,
    #[serde(rename = "sexual/minors")]
    sexual_or_minors: f32,
    #[serde(rename = "violence")]
    violence: f32,
    #[serde(rename = "violence/graphic")]
    violence_or_graphic: f32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ModerationResponse {
    pub id: String,
    pub model: Model,
    pub results: Vec<Result>,
}

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Image {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub b64_json: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ImageResponse {
    pub created: f32,
    pub data: Vec<Image>,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Size {
    #[serde(rename = "256x256")]
    SIZE_256,
    #[serde(rename = "512x512")]
    SIZE_512,
    #[serde(rename = "1024x1024")]
    SIZE_1024,
}
#[allow(non_camel_case_types)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Format {
    #[serde(rename = "url")]
    Url,
    #[serde(rename = "b64_json")]
    B64_JSON,
}

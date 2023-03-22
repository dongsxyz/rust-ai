use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum FinishReason {
    #[serde(rename = "stop")]
    Stop,
    #[serde(rename = "length")]
    Length,
    #[serde(rename = "content_filter")]
    ContentFilter,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Usage {
    pub prompt_tokens: usize,
    pub completion_tokens: usize,
    pub total_tokens: usize,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Error {
    pub error: ErrorInfo,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ErrorInfo {
    pub message: String,
    #[serde(rename = "type")]
    pub error_type: String,
    pub code: Option<u32>,
}

use serde::{Deserialize, Serialize};
use super::{model::Model, common::{FinishReason, Usage}};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChatMessage {
    pub role: MessageRole,
    pub content: String,
}

impl ChatMessage {
    pub fn new(role: MessageRole, content: &str) -> Self {
        Self {
            role,
            content: String::from(content),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum MessageRole {
    #[serde(rename = "user")]
    User,
    #[serde(rename = "system")]
    System,
    #[serde(rename = "assistant")]
    Assistant,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChoiceChunked {
    pub delta: Delta,
    pub index: usize,
    pub finish_reason: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Delta {
    pub content: Option<String>,
    pub role: Option<MessageRole>,
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Chunk {
    pub id: String,
    pub object: String,
    pub created: f32,
    pub model: Model,
    pub choices: Vec<ChoiceChunked>,
}

type MessageResult = Delta;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Choice {
    pub message: MessageResult,
    pub index: usize,
    pub finish_reason: Option<FinishReason>,
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChatCompletionResponse {
    pub id: String,
    pub object: String,
    pub created: f32,
    pub model: Model,
    pub usage: Usage,
    pub choices: Vec<Choice>,
}

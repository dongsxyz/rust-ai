use serde::{Deserialize, Serialize};
use super::{model::Model, common::{FinishReason, Usage}};


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Chunk {
    pub id: String,
    pub object: String,
    pub created: f32,
    pub model: Model,
    pub choices: Vec<Choice>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Choice {
    pub text: String,
    pub index: usize,
    pub logprobs: Option<u32>,
    pub finish_reason: Option<FinishReason>,
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CompletionResponse {
    pub id: String,
    pub object: String,
    pub created: f32,
    pub model: Model,
    pub usage: Usage,
    pub choices: Vec<Choice>,
}
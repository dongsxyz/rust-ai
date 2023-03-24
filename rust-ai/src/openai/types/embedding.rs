use super::{common::Usage, model::Model};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EmbeddingData {
    pub object: String,
    pub index: usize,
    pub embedding: Vec<f32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EmbeddingResponse {
    pub object: String,
    pub model: Model,
    pub usage: Usage,
    pub data: Vec<EmbeddingData>,
}

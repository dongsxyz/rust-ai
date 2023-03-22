use super::common::Usage;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Choice {
    pub text: String,
    pub index: usize,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EditResponse {
    pub object: String,
    pub created: f32,
    pub usage: Usage,
    pub choices: Vec<Choice>,
}

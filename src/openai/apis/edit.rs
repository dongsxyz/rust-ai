//!
//! Given a prompt and an instruction, the model will return an edited version
//! of the prompt.
//!
//! Source: OpenAI documentation

////////////////////////////////////////////////////////////////////////////////

use crate::openai::{
    endpoint::{endpoint_filter, request_endpoint, Endpoint, EndpointVariant},
    types::{common::Error, edit::EditResponse, model::Model},
};
use log::{debug, warn};
use serde::{Deserialize, Serialize};
use serde_with::serde_as;

/// Given a prompt and an instruction, the model will return an edited version
/// of the prompt.
#[serde_as]
#[derive(Serialize, Deserialize, Debug)]
pub struct Edit {
    /// ID of the model to use. You can use the `text-davinci-edit-001` or
    /// `code-davinci-edit-001` model with this endpoint.
    pub model: Model,

    /// The input text to use as a starting point for the edit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<String>,

    /// The instruction that tells the model how to edit the prompt.
    pub instruction: String,

    /// What sampling temperature to use, between 0 and 2. Higher values like
    /// 0.8 will make the output more random, while lower values like 0.2
    /// will make it more focused and deterministic.
    ///
    /// We generally recommend altering this or `top_p` but not both.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,

    /// An alternative to sampling with temperature, called nucleus sampling,
    ///  where the model considers the results of the tokens with `top_p`
    /// probability mass. So 0.1 means only the tokens comprising the top 10%
    /// probability mass are considered.
    ///
    /// We generally recommend altering this or `temperature` but not both.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f32>,

    /// How many edits to generate for the input and instruction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n: Option<u32>,
}

impl Default for Edit {
    fn default() -> Self {
        Self {
            model: Model::TEXT_DAVINCI_EDIT_001,
            input: Some(String::from("")),
            instruction: String::new(),
            temperature: None,
            top_p: None,
            n: None,
        }
    }
}

impl Edit {
    /// Set target input.
    ///
    /// # Arguments
    /// - `content` - Target input
    pub fn set_input(&mut self, content: &str) {
        self.input = Some(content.into());
    }

    /// Set target instruction.
    ///
    /// # Arguments
    /// - `content` - Target instruction
    pub fn set_instruction(&mut self, content: &str) {
        self.instruction = content.into();
    }

    /// Send edit request to OpenAI.
    pub async fn edit(&self) -> Result<EditResponse, Box<dyn std::error::Error>> {
        if !endpoint_filter(&self.model, &Endpoint::Edit_v1) {
            return Err("Model not compatible with this endpoint".into());
        }

        let mut edit_response: Option<EditResponse> = None;

        request_endpoint(&self, &Endpoint::Edit_v1, EndpointVariant::None, |res| {
            if let Ok(text) = res {
                if let Ok(response_data) = serde_json::from_str::<EditResponse>(&text) {
                    debug!(target: "openai", "Response parsed, edit response deserialized.");
                    edit_response = Some(response_data);
                } else {
                    if let Ok(response_error) = serde_json::from_str::<Error>(&text) {
                        warn!(target: "openai",
                            "OpenAI error code {}: `{:?}`",
                            response_error.error.code.unwrap_or(0),
                            text
                        );
                    } else {
                        warn!(target: "openai", "Edit response not deserializable.");
                    }
                }
            }
        })
        .await?;

        if let Some(response_data) = edit_response {
            Ok(response_data)
        } else {
            Err("No response or error parsing response".into())
        }
    }
}

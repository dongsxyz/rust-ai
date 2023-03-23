//!
//! Given a input text, outputs if the model classifies it as violating
//! OpenAI's content policy.
//!
//! Related guide: [Moderations](https://platform.openai.com/docs/guides/moderation)
//!
//! Source: OpenAI documentation

////////////////////////////////////////////////////////////////////////////////

use crate::openai::{
    endpoint::{endpoint_filter, request_endpoint, Endpoint, EndpointVariant},
    types::{common::Error, model::Model, moderation::ModerationResponse},
};
use log::{debug, warn};
use serde::{Deserialize, Serialize};
use serde_with::serde_as;

/// Given a input text, outputs if the model classifies it as violating
/// OpenAI's content policy.
#[serde_as]
#[derive(Serialize, Deserialize, Debug)]
pub struct Moderation {
    /// Two content moderations models are available: `text-moderation-stable`
    /// and `text-moderation-latest`.
    ///
    /// The default is `text-moderation-latest` which will be automatically
    /// upgraded over time. This ensures you are always using our most accurate
    /// model. If you use `text-moderation-stable`, we will provide advanced
    /// notice before updating the model. Accuracy of `text-moderation-stable`
    /// may be slightly lower than for `text-moderation-latest`.
    pub model: Model,

    /// The input text to classify
    pub input: Vec<String>,
}

impl Default for Moderation {
    fn default() -> Self {
        Self {
            model: Model::TEXT_MODERATION_LATEST,
            input: vec![],
        }
    }
}

impl Moderation {
    /// Set target input.
    ///
    /// # Arguments
    /// - `content` - Target input
    pub fn add_input(&mut self, content: &str) {
        self.input.push(content.into());
    }

    /// Send moderation request to OpenAI.
    pub async fn moderate(&self) -> Result<ModerationResponse, Box<dyn std::error::Error>> {
        if !endpoint_filter(&self.model, &Endpoint::Moderation_v1) {
            return Err("Model not compatible with this endpoint".into());
        }

        let mut moderation_response: Option<ModerationResponse> = None;

        request_endpoint(&self, &Endpoint::Moderation_v1, EndpointVariant::None, |res| {
            if let Ok(text) = res {
                if let Ok(response_data) = serde_json::from_str::<ModerationResponse>(&text) {
                    debug!(target: "openai", "Response parsed, moderation response deserialized.");
                    moderation_response = Some(response_data);
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

        if let Some(response_data) = moderation_response {
            Ok(response_data)
        } else {
            Err("No response or error parsing response".into())
        }
    }
}

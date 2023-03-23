//! 
//! Get a vector representation of a given input that can be easily consumed by 
//! machine learning models and algorithms.
//! 
//! Source: OpenAI documentation

////////////////////////////////////////////////////////////////////////////////

use crate::openai::{
  endpoint::{endpoint_filter, request_endpoint, Endpoint, EndpointVariant},
  types::{common::Error, embedding::EmbeddingResponse, model::Model},
};
use log::{debug, warn};
use serde::{Deserialize, Serialize};
use serde_with::serde_as;

/// Given a prompt and an instruction, the model will return an edited version
/// of the prompt.
#[serde_as]
#[derive(Serialize, Deserialize, Debug)]
pub struct Embedding {
  /// ID of the model to use. You can use the [List models](https://platform.openai.com/docs/api-reference/models/list) API to see all of 
  /// your available models, or see our [Model overview](https://platform.openai.com/docs/models/overview) for descriptions of them.
  pub model: Model,

  /// Input text to get embeddings for, encoded as a string or array of tokens. 
  /// To get embeddings for multiple inputs in a single request, pass an array 
  /// of strings or array of token arrays. Each input must not exceed 8192 
  /// tokens in length.
  pub input: String,


  /// A unique identifier representing your end-user, which can help OpenAI to 
  /// monitor and detect abuse. [Learn more](https://platform.openai.com/docs/guides/safety-best-practices/end-user-ids).
  #[serde(skip_serializing_if = "Option::is_none")]
  pub user: Option<String>,
}

impl Default for Embedding {
  fn default() -> Self {
      Self {
          model: Model::TEXT_EMBEDDING_ADA_002,
          input: String::from(""),
          user: None,
      }
  }
}

impl Embedding {
  /// Set target input.
  ///
  /// # Arguments
  /// - `content` - Target input
  pub fn set_input(&mut self, content: &str) {
      self.input = content.into();
  }
  
  /// Send embedding request to OpenAI.
  pub async fn embedding(&self) -> Result<EmbeddingResponse, Box<dyn std::error::Error>> {
      if !endpoint_filter(&self.model, &Endpoint::Embedding_v1) {
          return Err("Model not compatible with this endpoint".into());
      }

      let mut embedding_response: Option<EmbeddingResponse> = None;

      request_endpoint(&self, &Endpoint::Embedding_v1, EndpointVariant::None, |res| {
          if let Ok(text) = res {
              if let Ok(response_data) = serde_json::from_str::<EmbeddingResponse>(&text) {
                  debug!(target: "openai", "Response parsed, embedding response deserialized.");
                  embedding_response = Some(response_data);
              } else {
                  if let Ok(response_error) = serde_json::from_str::<Error>(&text) {
                      warn!(
                          "OpenAI error code {}: {:?}",
                          response_error.error.code.unwrap_or(0),
                          text
                      );
                  } else {
                      warn!(target: "openai", "Embedding response not deserializable.");
                  }
              }
          }
      })
      .await?;

      if let Some(response_data) = embedding_response {
          Ok(response_data)
      } else {
          Err("No response or error parsing response".into())
      }
  }
}

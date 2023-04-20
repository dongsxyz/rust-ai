//!
//! Given a chat conversation, the model will return a chat completion response.
//!
//! Source: OpenAI documentation

////////////////////////////////////////////////////////////////////////////////

use std::collections::HashMap;

use crate::openai::{
    endpoint::{
        endpoint_filter, request_endpoint, request_endpoint_stream, Endpoint, EndpointVariant,
    },
    types::{
        chat_completion::{ChatCompletionResponse, ChatMessage, Chunk, MessageRole},
        common::Error,
        Model,
    },
};
use log::{debug, warn};
use serde::{Deserialize, Serialize};
use serde_with::serde_as;

/// Given a chat conversation, the model will return a chat completion response.
#[serde_as]
#[derive(Serialize, Deserialize, Debug)]
pub struct ChatCompletion {
    pub model: Model,

    pub messages: Vec<ChatMessage>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub n: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub presence_penalty: Option<f32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency_penalty: Option<f32>,

    #[serde_as(as = "Option<Vec<(_,_)>>")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logit_bias: Option<HashMap<String, f32>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}

impl Default for ChatCompletion {
    fn default() -> Self {
        Self {
            model: Model::GPT_3_5_TURBO,
            messages: vec![],
            stream: Some(false),
            temperature: None,
            top_p: None,
            n: None,
            stop: None,
            max_tokens: None,
            presence_penalty: None,
            frequency_penalty: None,
            logit_bias: None,
            user: None,
        }
    }
}

impl ChatCompletion {
    /// ID of the model to use. See the [model endpoint compatibility](https://platform.openai.com/docs/models/model-endpoint-compatibility) table
    /// for details on which models work with the Chat API.
    ///
    /// # Argument
    /// - `model` - Target model to make use of
    pub fn model(self, model: Model) -> Self {
        Self { model, ..self }
    }

    /// Add message to prompt by role and content.
    ///
    /// The messages to generate chat completions for, in the [chat format](https://platform.openai.com/docs/guides/chat/introduction).
    ///
    /// # Arguments
    /// - `role` - Message role enum variant
    /// - `content` - Message content
    pub fn message(self, role: MessageRole, content: &str) -> Self {
        let mut messages = if self.messages.len() == 0 {
            vec![]
        } else {
            self.messages
        };
        messages.push(ChatMessage::new(role, content));

        Self {
            messages: messages,
            ..self
        }
    }

    /// Add message to prompt by message instance.
    ///
    /// The messages to generate chat completions for, in the [chat format](https://platform.openai.com/docs/guides/chat/introduction).
    ///
    /// # Argument
    /// - `messages` - Message instance vector, will replace all existing
    ///     messages
    pub fn messages(self, messages: Vec<ChatMessage>) -> Self {
        Self { messages, ..self }
    }

    /// What sampling temperature to use, between 0 and 2. Higher values like 0.
    /// 8 will make the output more random, while lower values like 0.2 will
    /// make it more focused and deterministic.
    ///
    /// We generally recommend altering this or `top_p` but not both.
    pub fn temperature(self, temperature: f32) -> Self {
        Self {
            temperature: Some(temperature),
            ..self
        }
    }

    /// An alternative to sampling with temperature, called nucleus sampling,
    /// where the model considers the results of the tokens with top_p
    /// probability mass. So 0.1 means only the tokens comprising the top 10%
    /// probability mass are considered.
    ///
    /// We generally recommend altering this or `temperature` but not both.
    pub fn top_p(self, top_p: f32) -> Self {
        Self {
            top_p: Some(top_p),
            ..self
        }
    }

    /// How many chat completion choices to generate for each input message.
    pub fn n(self, n: u32) -> Self {
        Self { n: Some(n), ..self }
    }

    /// Up to 4 sequences where the API will stop generating further tokens.
    pub fn stop(self, stop: Vec<String>) -> Self {
        Self {
            stop: Some(stop),
            ..self
        }
    }

    // The maximum number of [tokens](https://platform.openai.com/tokenizer) to generate in the chat completion.
    ///
    /// The total length of input tokens and generated tokens is limited by the
    /// model's context length.
    pub fn max_tokens(self, max_tokens: u32) -> Self {
        Self {
            max_tokens: Some(max_tokens),
            ..self
        }
    }

    /// Number between -2.0 and 2.0. Positive values penalize new tokens based
    /// on whether they appear in the text so far, increasing the model's
    /// likelihood to talk about new topics.
    ///
    /// [See more information about frequency and presence penalties.](https://platform.openai.com/docs/api-reference/parameter-details)
    pub fn presence_penalty(self, presence_penalty: f32) -> Self {
        Self {
            presence_penalty: Some(presence_penalty),
            ..self
        }
    }

    /// Number between -2.0 and 2.0. Positive values penalize new tokens based
    /// on their existing frequency in the text so far, decreasing the model's
    /// likelihood to repeat the same line verbatim.
    ///
    /// [See more information about frequency and presence penalties.](https://platform.openai.com/docs/api-reference/parameter-details)
    pub fn frequency_penalty(self, frequency_penalty: f32) -> Self {
        Self {
            frequency_penalty: Some(frequency_penalty),
            ..self
        }
    }

    /// Modify the likelihood of specified tokens appearing in the completion.
    ///
    /// Accepts a json object that maps tokens (specified by their token ID in
    /// the tokenizer) to an associated bias value from -100 to 100.
    /// Mathematically, the bias is added to the logits generated by the model
    /// prior to sampling. The exact effect will vary per model, but values
    /// between -1 and 1 should decrease or increase likelihood of selection;
    /// values like -100 or 100 should result in a ban or exclusive selection
    /// of the relevant token.
    pub fn logit_bias(self, logit_bias: HashMap<String, f32>) -> Self {
        Self {
            logit_bias: Some(logit_bias),
            ..self
        }
    }
    
    /// A unique identifier representing your end-user, which can help OpenAI
    /// to monitor and detect abuse. [Learn more](https://platform.openai.com/docs/guides/safety-best-practices/end-user-ids).
    pub fn user(self, user: &str) -> Self {
        Self {
            user: Some(user.into()),
            ..self
        }
    }

    /// Send chat completion request to OpenAI using streamed method.
    ///
    /// Partial message deltas will be sent, like in ChatGPT. Tokens
    /// will be sent as data-only [server-sent events](https://developer.mozilla.org/en-US/docs/Web/API/Server-sent_events/Using_server-sent_events#Event_stream_format) as they become available,
    /// with the stream terminated by a `data: [DONE]` message. See the OpenAI
    /// Cookbook for [example code](https://github.com/openai/openai-cookbook/blob/main/examples/How_to_stream_completions.ipynb).
    pub async fn streamed_completion(
        self,
        mut cb: Option<impl FnMut(Chunk)>,
    ) -> Result<Vec<Chunk>, Box<dyn std::error::Error>> {
        let data = Self {
            stream: Some(true),
            ..self
        };

        if !endpoint_filter(&data.model, &Endpoint::ChatCompletion_v1) {
            return Err("Model not compatible with this endpoint".into());
        }

        let mut ret_val: Vec<Chunk> = vec![];
        let ret_val_ref = &mut ret_val;

        request_endpoint_stream(
            &data,
            &Endpoint::ChatCompletion_v1,
            EndpointVariant::None,
            |res| {
                if let Ok(chunk_data_raw) = res {
                    for chunk_data in chunk_data_raw.split("\n") {
                    let chunk_data = chunk_data.trim().to_string();
                    if &chunk_data == "data: [DONE]" {
                        debug!(target: "openai", "Last chunk received.");
                        return;
                    }
                    if chunk_data.starts_with("data: ") {
                        // Strip response content:
                        let stripped_chunk = &chunk_data.trim()[6..];
                        if let Ok(message_chunk) = serde_json::from_str::<Chunk>(stripped_chunk) {
                            ret_val_ref.push(message_chunk.clone());
                            if let Some(cb) = &mut cb {
                                cb(message_chunk);
                            }
                        } else {
                            if let Ok(response_error) =
                                serde_json::from_str::<Error>(&stripped_chunk)
                            {
                                warn!(target: "openai",
                                    "OpenAI error code {}: `{:?}`",
                                    response_error.error.code.unwrap_or(0),
                                    stripped_chunk
                                );
                            } else {
                                warn!(target: "openai", "Completion response not deserializable.");
                            }
                        }
                    }
                };
                }
            },
        )
        .await?;

        Ok(ret_val)
    }

    /// Send chat completion request to OpenAI.
    pub async fn completion(self) -> Result<ChatCompletionResponse, Box<dyn std::error::Error>> {
        let data = Self {
            stream: None,
            ..self
        };

        if !endpoint_filter(&data.model, &Endpoint::ChatCompletion_v1) {
            return Err("Model not compatible with this endpoint".into());
        }

        let mut completion_response: Option<ChatCompletionResponse> = None;

        request_endpoint(&data, &Endpoint::ChatCompletion_v1, EndpointVariant::None, |res| {
            if let Ok(text) = res {
                if let Ok(response_data) = serde_json::from_str::<ChatCompletionResponse>(&text) {
                    debug!(target: "openai", "Response parsed, completion response deserialized.");
                    completion_response = Some(response_data);
                } else {
                    if let Ok(response_error) = serde_json::from_str::<Error>(&text) {
                        warn!(target: "openai",
                            "OpenAI error code {}: `{:?}`",
                            response_error.error.code.unwrap_or(0),
                            text
                        );
                    } else {
                        warn!(target: "openai", "Completion response not deserializable.");
                    }
                }
            }
        })
        .await?;

        if let Some(response_data) = completion_response {
            Ok(response_data)
        } else {
            Err("No response".into())
        }
    }
}

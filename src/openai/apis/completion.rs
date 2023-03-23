//! 
//! Given a prompt, the model will return one or more predicted completions, 
//! and can also return the probabilities of alternative tokens at each position.
//! 
//! Source: OpenAI documentation

////////////////////////////////////////////////////////////////////////////////

use std::collections::HashMap;

use crate::openai::{
    endpoint::{
        endpoint_filter, request_endpoint, request_endpoint_stream, Endpoint, EndpointVariant,
    },
    types::{
        common::Error,
        completion::{Chunk, CompletionResponse},
        model::Model,
    },
};
use log::{debug, warn};
use serde::{Deserialize, Serialize};
use serde_with::serde_as;

/// Given a prompt, the model will return one or more predicted completions,
/// and can also return the probabilities of alternative tokens at each
/// position.
#[serde_as]
#[derive(Serialize, Deserialize, Debug)]
pub struct Completion {
    /// ID of the model to use. You can use the [List models API](https://platform.openai.com/docs/api-reference/models/list) to see all of
    /// your available models, or see our [Model overview](https://platform.openai.com/docs/models/overview) for descriptions of
    /// them.
    pub model: Model,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt: Option<Vec<String>>,

    /// Whether to stream back partial progress. If set, tokens will be sent as
    /// data-only [server-sent events](https://developer.mozilla.org/en-US/docs/Web/API/Server-sent_events/Using_server-sent_events#Event_stream_format) as they become available, with the stream
    /// terminated by a `data: [DONE]` message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,

    /// The suffix that comes after a completion of inserted text.
    pub suffix: Option<String>,

    /// What sampling temperature to use, between 0 and 2. Higher values like 0.
    /// 8 will make the output more random, while lower values like 0.2 will
    /// make it more focused and deterministic.
    ///
    /// We generally recommend altering this or `top_p` but not both.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,

    /// An alternative to sampling with temperature, called nucleus sampling,
    /// where the model considers the results of the tokens with top_p
    /// probability mass. So 0.1 means only the tokens comprising the top 10%
    /// probability mass are considered.
    ///
    /// We generally recommend altering this or `temperature` but not both.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f32>,

    /// How many completions to generate for each prompt.
    ///
    /// **Note**: Because this parameter generates many completions, it can quickly
    /// consume your token quota. Use carefully and ensure that you have
    /// reasonable settings for `max_tokens` and `stop`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n: Option<u32>,

    /// Include the log probabilities on the `logprobs` most likely tokens, as
    /// well the chosen tokens. For example, if `logprobs` is 5, the API will
    /// return a list of the 5 most likely tokens. The API will always return
    /// the `logprob` of the sampled token, so there may be up to `logprobs+1`
    /// elements in the response.
    ///
    /// The maximum value for `logprobs` is 5. If you need more than this,
    /// please contact us through our **Help center** and describe your use
    /// case.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logprobs: Option<u32>,

    /// Echo back the prompt in addition to the completion
    #[serde(skip_serializing_if = "Option::is_none")]
    pub echo: Option<Vec<bool>>,

    /// Up to 4 sequences where the API will stop generating further tokens.
    /// The returned text will not contain the stop sequence.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop: Option<Vec<String>>,

    /// The maximum number of [tokens](https://platform.openai.com/tokenizer) to generate in the completion.
    ///
    /// The token count of your prompt plus `max_tokens` cannot exceed the
    /// model's context length. Most models have a context length of 2048
    /// tokens (except for the newest models, which support 4096).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<u32>,

    /// Number between -2.0 and 2.0. Positive values penalize new tokens based
    /// on whether they appear in the text so far, increasing the model's
    /// likelihood to talk about new topics.
    ///
    /// [See more information about frequency and presence penalties.](https://platform.openai.com/docs/api-reference/parameter-details)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presence_penalty: Option<f32>,

    /// Number between -2.0 and 2.0. Positive values penalize new tokens based
    /// on their existing frequency in the text so far, decreasing the model's
    /// likelihood to repeat the same line verbatim.
    ///
    /// [See more information about frequency and presence penalties.](https://platform.openai.com/docs/api-reference/parameter-details)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency_penalty: Option<f32>,

    /// Generates `best_of` completions server-side and returns the "best" (the
    /// one with the highest log probability per token). Results cannot be
    /// streamed.
    ///
    /// When used with `n`, `best_of` controls the number of candidate
    /// completions and `n` specifies how many to return – `best_of` must be
    /// greater than n.
    ///
    /// **Note**: Because this parameter generates many completions, it can
    /// quickly consume your token quota. Use carefully and ensure that you
    /// have reasonable settings for `max_tokens` and `stop`.
    #[serde_as(as = "Option<Vec<(_,_)>>")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub best_of: Option<HashMap<String, u32>>,

    /// Modify the likelihood of specified tokens appearing in the completion.
    ///
    /// Accepts a json object that maps tokens (specified by their token ID in
    /// the GPT tokenizer) to an associated bias value from -100 to 100. You
    /// can use this [tokenizer tool](https://platform.openai.com/tokenizer?view=bpe) (which works for both GPT-2 and GPT-3) to
    ///  convert text to token IDs. Mathematically, the bias is added to the
    /// logits generated by the model prior to sampling. The exact effect will
    /// vary per model, but values between -1 and 1 should decrease or increase
    ///  likelihood of selection; values like -100 or 100 should result in a
    /// ban or exclusive selection of the relevant token.
    ///
    /// As an example, you can pass `{"50256": -100}` to prevent the
    /// <|endoftext|> token from being generated.
    #[serde_as(as = "Option<Vec<(_,_)>>")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logit_bias: Option<HashMap<String, f32>>,

    /// A unique identifier representing your end-user, which can help OpenAI
    /// to monitor and detect abuse. [Learn more](https://platform.openai.com/docs/guides/safety-best-practices/end-user-ids).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}

impl Default for Completion {
    fn default() -> Self {
        Self {
            model: Model::TEXT_DAVINCI_003,
            prompt: None,
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
            suffix: None,
            logprobs: None,
            echo: None,
            best_of: None,
        }
    }
}

impl Completion {
    /// Add message to prompt.
    ///
    /// # Arguments
    /// - `content` - Message content
    pub fn add_prompt(&mut self, content: &str) {
        if let None = self.prompt {
            self.prompt = Some(vec![]);
        }
        self.prompt.as_mut().unwrap().push(String::from(content));
    }

    /// Send completion request to OpenAI using streamed method.
    pub async fn completion_streamed<F>(
        &self,
        mut cb: Option<F>,
    ) -> Result<Vec<Chunk>, Box<dyn std::error::Error>>
    where
        F: FnMut(Chunk),
    {
        if let Some(false) = self.stream {
            return Err("Cannot call streaming API when `stream` disabled.".into());
        }

        if !endpoint_filter(&self.model, &Endpoint::Completion_v1) {
            return Err("Model not compatible with this endpoint".into());
        }

        let mut ret_val: Vec<Chunk> = vec![];

        request_endpoint_stream(&self, &Endpoint::Completion_v1, EndpointVariant::None,|res| {
            if let Ok(chunk_data_raw) = res {
                chunk_data_raw.split("\n").for_each(|chunk_data| {
                    let chunk_data = chunk_data.trim().to_string();
                    if &chunk_data == "data: [DONE]" {
                        debug!(target: "openai", "Last chunk received.");
                        return;
                    }
                    if chunk_data.starts_with("data: ") {
                        // Strip response content:
                        let stripped_chunk = &chunk_data.trim()[6..];
                        if let Ok(message_chunk) = serde_json::from_str::<Chunk>(stripped_chunk) {
                            ret_val.push(message_chunk.clone());
                            if let Some(cb) = &mut cb {
                                cb(message_chunk);
                            }
                        } else {
                            if let Ok(response_error) = serde_json::from_str::<Error>(&stripped_chunk) {
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
                });
            }
        })
        .await?;

        Ok(ret_val)
    }

    /// Send completion request to OpenAI.
    pub async fn completion(&self) -> Result<CompletionResponse, Box<dyn std::error::Error>> {
        if let Some(true) = self.stream {
            return Err("Cannot call non-streaming API when `stream` enabled.".into());
        }

        if !endpoint_filter(&self.model, &Endpoint::Completion_v1) {
            return Err("Model not compatible with this endpoint".into());
        }

        let mut completion_response: Option<CompletionResponse> = None;

        request_endpoint(&self, &Endpoint::Completion_v1, EndpointVariant::None, |res| {
            if let Ok(text) = res {
                if let Ok(response_data) = serde_json::from_str::<CompletionResponse>(&text) {
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
            Err("No response or error parsing response".into())
        }
    }
}

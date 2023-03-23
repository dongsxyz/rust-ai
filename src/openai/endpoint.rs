//! 
//! # OpenAI's API endpoints
//! 
//! Note that for Audios and Images, an extended endpoint variant will be 
//! needed.

////////////////////////////////////////////////////////////////////////////////

use std::fmt::Display;

use log::{debug, error, warn};
use reqwest::multipart::Form;
use serde::Serialize;

use crate::utils::config::Config;

use super::types::model::Model;

/// Check if selected model is available to certain API endpoint
///
/// # Arguments
/// - `model` - A provided model enum variant
/// - `endpoint` - API endpoint name, e.g. `/v1/completions`
pub fn endpoint_filter(model: &Model, endpoint: &Endpoint) -> bool {
    match endpoint {
        Endpoint::ChatCompletion_v1 => [
            Model::GPT_3_5_TURBO,
            Model::GPT_3_5_TURBO_0301,
            Model::GPT_4,
            Model::GPT_4_0314,
            Model::GPT_4_32K,
            Model::GPT_4_32K_0314,
        ]
        .contains(&model),
        Endpoint::Completion_v1 => [
            Model::TEXT_DAVINCI_003,
            Model::TEXT_DAVINCI_002,
            Model::TEXT_CURIE_001,
            Model::TEXT_BABBAGE_001,
            Model::TEXT_ADA_001,
            Model::DAVINCI,
            Model::CURIE,
            Model::BABBAGE,
            Model::ADA,
        ]
        .contains(&model),
        Endpoint::Edit_v1 => {
            [Model::TEXT_DAVINCI_EDIT_001, Model::CODE_DAVINCI_EDIT_001].contains(&model)
        }
        Endpoint::Audio_v1 => [Model::WHISPER_1].contains(&model),
        Endpoint::FineTune_v1 => {
            [Model::DAVINCI, Model::CURIE, Model::BABBAGE, Model::ADA].contains(&model)
        }
        Endpoint::Embedding_v1 => [
            Model::TEXT_EMBEDDING_ADA_002,
            Model::TEXT_SEARCH_ADA_DOC_001,
        ]
        .contains(&model),
        Endpoint::Moderation_v1 => [
            Model::TEXT_MODERATION_LATEST,
            Model::TEXT_MODERATION_STABLE,
            Model::TEXT_MODERATION_004,
        ]
        .contains(&model),
        _ => false,
    }
}

/// Enum for endpoints that have several variants.
pub enum EndpointVariant {
    /// No sub variants.
    None,
    /// Denotes a variant of some endpoint.
    Extended(String),
}

impl From<String> for EndpointVariant {
    fn from(value: String) -> Self {
        Self::Extended(value)
    }
}

/// API endpoint definition enum
#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Endpoint {
    ChatCompletion_v1,
    Completion_v1,
    Edit_v1,
    Image_v1,
    Audio_v1,
    FineTune_v1,
    Embedding_v1,
    Moderation_v1,
}

impl Display for Endpoint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", <Self as Into<&str>>::into(self.clone()))
    }
}

impl Into<&'static str> for Endpoint {
    fn into(self) -> &'static str {
        match self {
            Self::Audio_v1 => "/v1/audio",
            Self::ChatCompletion_v1 => "/v1/chat/completions",
            Self::Completion_v1 => "/v1/completions",
            Self::Edit_v1 => "/v1/edits",
            Self::Image_v1 => "/v1/images",
            Self::Embedding_v1 => "/v1/embeddings",
            Self::FineTune_v1 => "/v1/fine-tunes",
            Self::Moderation_v1 => "/v1/moderations",
        }
    }
}

/// Endpoint variants for Images
#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ImageEndpointVariant {
    Generation,
    Editing,
    Variation,
}

impl Into<String> for ImageEndpointVariant {
    fn into(self) -> String {
        String::from(match self {
            Self::Editing => "/edits",
            Self::Variation => "/variations",
            Self::Generation => "/generations",
        })
    }
}

/// Endpoint variants for Audios
#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum AudioEndpointVariant {
    Transcription,
    Translation,
}

impl Into<String> for AudioEndpointVariant {
    fn into(self) -> String {
        String::from(match self {
            Self::Transcription => "/transcriptions",
            Self::Translation => "/translations",
        })
    }
}

/// Send request to remote endpoint using JSON.
///  
/// # Arguments 
/// - `json` - the serialized contents to send 
/// - `endpoint` - Endpoint enum variant
/// - `variant` - Endpoint variant enum
/// - `cb` - callback function that will be called when message received.
pub async fn request_endpoint<'a, T, F>(
    json: &'a T,
    endpoint: &'a Endpoint,
    variant: EndpointVariant,
    mut cb: F,
) -> Result<(), Box<dyn std::error::Error>>
where
    T: Serialize,
    F: FnMut(Result<String, Box<dyn std::error::Error>>),
{
    let client = reqwest::Client::new();
    let config = Config::load();
    let url = if let EndpointVariant::Extended(var) = variant {
        format!("https://api.openai.com{}{}", endpoint, var.to_owned())
    } else {
        format!("https://api.openai.com{}", endpoint)
    };

    let mut req = client.post(url);
    req = req.header("Authorization", format!("Bearer {}", config.openai.api_key));

    let res = req.json(&json).send().await?;

    if let Ok(text) = res.text().await {
        debug!(target: "openai", "Received response from OpenAI: `{:?}`", text);
        cb(Ok(text.clone()));
    } else {
        error!(target: "openai", "Error receiving response from OpenAI");
        cb(Err("Error receiving response from OpenAI".into()))
    }

    Ok(())
}


/// Send request to remote endpoint using JSON but response will be streamed.
///  
/// # Arguments 
/// - `json` - the serialized contents to send 
/// - `endpoint` - Endpoint enum variant
/// - `variant` - Endpoint variant enum
/// - `cb` - callback function that will be called when message received. Note 
/// the differences of the function parameters.
pub async fn request_endpoint_stream<'a, T, F>(
    json: &'a T,
    endpoint: &'a Endpoint,
    variant: EndpointVariant,
    mut cb: F,
) -> Result<(), Box<dyn std::error::Error>>
where
    T: Serialize,
    F: FnMut(Result<String, Box<dyn std::error::Error>>),
{
    let client = reqwest::Client::new();
    let config = Config::load();
    let url = if let EndpointVariant::Extended(var) = variant {
        format!("https://api.openai.com{}{}", endpoint, var.to_owned())
    } else {
        format!("https://api.openai.com{}", endpoint)
    };

    let mut req = client.post(url);
    req = req.header("Authorization", format!("Bearer {}", config.openai.api_key));

    let mut res = req.json(&json).send().await?;

    while let Some(chunk) = res.chunk().await? {
        if let Ok(chunk_data_raw) = String::from_utf8(chunk.to_vec()) {
            debug!(target: "openai", "Received response chunk from OpenAI: `{:?}`", chunk_data_raw);
            cb(Ok(chunk_data_raw));
        } else {
            warn!(target: "openai", "Response chunk empty");
        }
    }

    Ok(())
}


/// Send request to remote endpoint using Form data.
///  
/// # Arguments 
/// - `form` - the constructed HTTP form to send 
/// - `endpoint` - Endpoint enum variant
/// - `variant` - Endpoint variant enum
/// - `cb` - callback function that will be called when message received.
pub async fn request_endpoint_form_data<'a, F>(
    form: Form,
    endpoint: &'a Endpoint,
    variant: EndpointVariant,
    mut cb: F,
) -> Result<(), Box<dyn std::error::Error>>
where
    F: FnMut(Result<String, Box<dyn std::error::Error>>),
{
    let client = reqwest::Client::new();
    let config = Config::load();
    let url = if let EndpointVariant::Extended(var) = variant {
        format!("https://api.openai.com{}{}", endpoint, var.to_owned())
    } else {
        format!("https://api.openai.com{}", endpoint)
    };

    let mut req = client.post(url);
    req = req.header("Authorization", format!("Bearer {}", config.openai.api_key));
    let res = req.multipart(form).send().await?;

    if let Ok(text) = res.text().await {
        debug!(target: "openai", "Received response from OpenAI: `{:?}`", text);
        cb(Ok(text.clone()));
    } else {
        error!(target: "openai", "Error receiving response from OpenAI");
        cb(Err("Error receiving response from OpenAI".into()))
    }

    Ok(())
}

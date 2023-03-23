use std::fmt::Display;

use serde::Serialize;
use reqwest::multipart::Form;
use log::{debug, error, warn};

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
        Endpoint::AudioTranscription_v1 => [Model::WHISPER_1].contains(&model),
        Endpoint::AudioTranslation_v1 => [Model::WHISPER_1].contains(&model),
        Endpoint::FineTune_v1 => {
            [Model::DAVINCI, Model::CURIE, Model::BABBAGE, Model::ADA].contains(&model)
        }
        Endpoint::Embedding_v1 => [
            Model::TEXT_EMBEDDING_ADA_002,
            Model::TEXT_SEARCH_ADA_DOC_001,
        ]
        .contains(&model),
        _ => false,
    }
}

pub enum EndpointVariant {
    None,
    Extended(String),
}

impl From<String> for EndpointVariant {
    fn from(value: String) -> Self {
        Self::Extended(value)
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Endpoint {
    ChatCompletion_v1,
    Completion_v1,
    Edit_v1,
    Image_v1,
    AudioTranscription_v1,
    AudioTranslation_v1,
    FineTune_v1,
    Embedding_v1,
}

impl Display for Endpoint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", <Self as Into<&str>>::into(self.clone()))
    }
}

impl Into<&'static str> for Endpoint {
    fn into(self) -> &'static str {
        match self {
            Self::AudioTranscription_v1 => "/v1/audio/transcriptions",
            Self::AudioTranslation_v1 => "/v1/audio/translations",
            Self::ChatCompletion_v1 => "/v1/chat/completions",
            Self::Completion_v1 => "/v1/completions",
            Self::Edit_v1 => "/v1/edits",
            Self::Image_v1 => "/v1/images",
            Self::Embedding_v1 => "/v1/embeddings",
            Self::FineTune_v1 => "/v1/fine-tunes",
        }
    }
}

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
use log::{debug, error};
use reqwest::{header::HeaderMap, Client};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use urlencoding::encode;

use crate::utils::config::Config;

use super::{
    types::common::{ResponseExpectation, ResponseType},
    SSML,
};

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum SpeechServiceEndpoint {
    Get_List_of_Voices,
    Post_Text_to_Speech_v1,
    Get_Speech_to_Text_Health_Status_v3_1,
    Get_List_of_Models_v3_1,
    Post_Create_Transcription_v3_1,
    Get_Transcription_v3_1,
    Get_Transcription_Files_v3_1,
    Get_Transcription_File_v3_1,
    None,
}

impl SpeechServiceEndpoint {
    pub fn build(&self, region: &str) -> String {
        match self {
            Self::Get_List_of_Voices => format!(
                "https://{}.tts.speech.microsoft.com/cognitiveservices/voices/list",
                region
            ),

            Self::Post_Text_to_Speech_v1 => format!(
                "https://{}.tts.speech.microsoft.com/cognitiveservices/v1",
                region
            ),

            Self::Get_Speech_to_Text_Health_Status_v3_1 => format!(
                "https://{}.cognitiveservices.azure.com/speechtotext/v3.1/healthstatus",
                region
            ),

            Self::Get_List_of_Models_v3_1 => format!(
                "https://{}.cognitiveservices.azure.com/speechtotext/v3.1/models",
                region
            ),

            Self::Post_Create_Transcription_v3_1 => format!(
                "https://{}.api.cognitive.microsoft.com/speechtotext/v3.1/transcriptions",
                region
            ),

            Self::Get_Transcription_v3_1 => format!(
                "https://{}.api.cognitive.microsoft.com/speechtotext/v3.1/transcriptions/",
                region
            ),

            Self::Get_Transcription_Files_v3_1 => format!(
                "https://{}.api.cognitive.microsoft.com/speechtotext/v3.1/transcriptions/",
                region
            ),

            Self::Get_Transcription_File_v3_1 => format!(
                "https://{}.api.cognitive.microsoft.com/speechtotext/v3.1/transcriptions/",
                region
            ),
            Self::None => String::new(),
        }
    }
}

/// If you would like to use a plain request URI, pass 
/// [`SpeechServiceEndpoint::None`] as the endpoint, and then provide the 
/// complete URI through `url_suffix` parameter.
pub async fn request_get_endpoint(
    endpoint: &SpeechServiceEndpoint,
    params: Option<HashMap<String, String>>,
    url_suffix: Option<String>,
) -> Result<String, Box<dyn std::error::Error>> {
    let config = Config::load().unwrap();
    let region = config.azure.speech.region;

    let mut url = match endpoint {
        SpeechServiceEndpoint::None => String::new(),
        _ => endpoint.build(&region),
    };

    if let Some(url_suffix) = url_suffix {
        url.push_str(&url_suffix);
    }

    if let Some(params) = params {
        let combined = params
            .iter()
            .map(|(k, v)| format!("{}={}", encode(k), encode(v)))
            .collect::<Vec<String>>()
            .join("&");
        url.push_str(&format!("?{}", combined));
    }

    let client = Client::new();
    let mut req = client.get(url);
    req = req.header("Ocp-Apim-Subscription-Key", config.azure.speech.key);

    let res = req.send().await?;

    match res.text().await {
        Ok(text) => Ok(text),
        Err(e) => {
            error!(target: "azure", "Error requesting Azure endpoint (GET): {:?}", e);
            Err(Box::new(e))
        }
    }
}

pub async fn request_post_endpoint_ssml(
    endpoint: &SpeechServiceEndpoint,
    ssml: SSML,
    expect: ResponseExpectation,
    extra_headers: Option<HeaderMap>,
) -> Result<ResponseType, Box<dyn std::error::Error>> {
    let config = Config::load().unwrap();
    let region = config.azure.speech.region;

    let url = endpoint.build(&region);

    let client = Client::new();
    let mut req = client
        .post(url)
        .header("Ocp-Apim-Subscription-Key", config.azure.speech.key)
        .header("User-Agent", "rust-ai/example")
        .header("Content-Type", "application/ssml+xml");
    if let Some(extra_headers) = extra_headers {
        req = req.headers(extra_headers);
    }

    let body = ssml.to_string();
    req = req.body(body.clone());
    debug!(target: "azure", "Request body: {:?}", body);

    let res = req.send().await?;

    match expect {
        ResponseExpectation::Text => match res.text().await {
            Ok(text) => Ok(ResponseType::Text(text)),
            Err(e) => {
                error!(target: "azure", "Error requesting Azure endpoint (GET): {:?}", e);
                Err(Box::new(e))
            }
        },
        ResponseExpectation::Bytes => match res.bytes().await {
            Ok(bytes) => Ok(ResponseType::Bytes(bytes.to_vec())),
            Err(e) => {
                error!(target: "azure", "Error requesting Azure endpoint (GET): {:?}", e);
                Err(Box::new(e))
            }
        },
    }
}

pub async fn request_post_endpoint(
    endpoint: &SpeechServiceEndpoint,
    json: impl Serialize + Into<String>,
    expect: ResponseExpectation,
    extra_headers: Option<HeaderMap>,
) -> Result<ResponseType, Box<dyn std::error::Error>> {
    let config = Config::load().unwrap();
    let region = config.azure.speech.region;

    let url = endpoint.build(&region);

    let client = Client::new();
    let mut req = client
        .post(url)
        .header("Ocp-Apim-Subscription-Key", config.azure.speech.key)
        .header("User-Agent", "rust-ai/example")
        .header("Content-Type", "application/ssml+xml");
    if let Some(extra_headers) = extra_headers {
        req = req.headers(extra_headers);
    }

    req = req.json(&json);
    debug!(target: "azure", "Request body: {:?}", Into::<String>::into(json));

    let res = req.send().await?;

    match expect {
        ResponseExpectation::Text => match res.text().await {
            Ok(text) => Ok(ResponseType::Text(text)),
            Err(e) => {
                error!(target: "azure", "Error requesting Azure endpoint (GET): {:?}", e);
                Err(Box::new(e))
            }
        },
        ResponseExpectation::Bytes => match res.bytes().await {
            Ok(bytes) => Ok(ResponseType::Bytes(bytes.to_vec())),
            Err(e) => {
                error!(target: "azure", "Error requesting Azure endpoint (GET): {:?}", e);
                Err(Box::new(e))
            }
        },
    }
}

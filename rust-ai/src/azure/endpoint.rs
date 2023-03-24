use log::{debug, error};
use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::utils::config::Config;

use super::types::common::{ResponseExpectation, ResponseType, SSML};

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum SpeechServiceEndpoint {
    Get_List_of_Voices,
    Convert_Text_to_Speech,
}

impl SpeechServiceEndpoint {
    pub fn build(&self, region: &str) -> String {
        match self {
            Self::Get_List_of_Voices => format!(
                "https://{}.tts.speech.microsoft.com/cognitiveservices/voices/list",
                region
            ),

            Self::Convert_Text_to_Speech => format!(
                "https://{}.tts.speech.microsoft.com/cognitiveservices/v1",
                region
            ),
        }
    }
}

pub async fn request_get_endpoint(
    endpoint: &SpeechServiceEndpoint,
) -> Result<String, Box<dyn std::error::Error>> {
    let config = Config::load().unwrap();
    let region = config.azure.speech.region;

    let url = endpoint.build(&region);

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

pub async fn request_post_endpoint_ssml<T>(
    endpoint: &SpeechServiceEndpoint,
    ssml: T,
    expect: ResponseExpectation,
) -> Result<ResponseType, Box<dyn std::error::Error>>
where
    T: SSML,
{
    let config = Config::load().unwrap();
    let region = config.azure.speech.region;

    let url = endpoint.build(&region);

    let client = Client::new();
    let mut req = client.post(url);
    req = req.header("Ocp-Apim-Subscription-Key", config.azure.speech.key);
    req = req.header("User-Agent", "rust-ai/example");
    req = req.header("Content-Type", "application/ssml+xml");
    req = req.header("X-Microsoft-OutputFormat", "ogg-24khz-16bit-mono-opus");
    let body = Into::<String>::into(ssml);
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

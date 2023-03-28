//!
//! Selection of AI Voice Names:
//!
//! (This is not a comprehensive list. You may find all available endpoints fetched from WestUS region in [`VoiceName`][crate::azure::VoiceName])
//!
//! - [`VoiceName::yue_CN_YunSongNeural`][crate::azure::VoiceName::yue_CN_YunSongNeural]: A 25-year-old young man's voice (deeper than Yunfeng).
//! - [`VoiceName::yue_CN_XiaoMinNeural`][crate::azure::VoiceName::yue_CN_XiaoMinNeural]: Sounds like a 20-year-old young lady's voice.
//! - [`VoiceName::zh_CN_YunfengNeural`][crate::azure::VoiceName::zh_CN_YunfengNeural]: A 22-25 years' young man's voice.
//! - [`VoiceName::zh_CN_YunhaoNeural`][crate::azure::VoiceName::zh_CN_YunhaoNeural]: A 25-year-old young man's voice (deeper than Yunfeng).
//! - [`VoiceName::zh_CN_YunjianNeural`][crate::azure::VoiceName::zh_CN_YunjianNeural]: A 30 to 35-year-old man's voice (deeper and powerful).
//! - [`VoiceName::zh_CN_YunxiNeural`][crate::azure::VoiceName::zh_CN_YunxiNeural]: A 14 to 17-year-old boy student's voice.
//! - [`VoiceName::zh_CN_YunxiaNeural`][crate::azure::VoiceName::zh_CN_YunxiaNeural]: A 10-year-old boy's voice.
//! - [`VoiceName::zh_CN_YunyangNeural`][crate::azure::VoiceName::zh_CN_YunyangNeural]: A 35-year-old man's voice (announcer like).
//! - [`VoiceName::zh_CN_YunyeNeural`][crate::azure::VoiceName::zh_CN_YunyeNeural]: A 35 to 40-year-old man's voice (documentary voice actor like).
//! - [`VoiceName::zh_CN_YunzeNeural`][crate::azure::VoiceName::zh_CN_YunzeNeural]: A 45 to 50-year-old man's voice (kind and deep voice).
//! - [`VoiceName::zh_CN_XiaochenNeural`][crate::azure::VoiceName::zh_CN_XiaochenNeural]: Sounds like a 19-year-old college girl.
//! - [`VoiceName::zh_CN_XiaohanNeural`][crate::azure::VoiceName::zh_CN_XiaohanNeural]: Sounds like a 27-year-old young woman.
//! - [`VoiceName::zh_CN_XiaomengNeural`][crate::azure::VoiceName::zh_CN_XiaomengNeural]: Sounds like a 23-year-old young lady.
//! - [`VoiceName::zh_CN_XiaomoNeural`][crate::azure::VoiceName::zh_CN_XiaomoNeural]: Sounds like a 25-year-old young lady with intellectual voice.
//! - [`VoiceName::zh_CN_XiaoqiuNeural`][crate::azure::VoiceName::zh_CN_XiaoqiuNeural]: Sounds like a 35 to 40-year-old female's voice (announcer like).
//! - [`VoiceName::zh_CN_XiaoruiNeural`][crate::azure::VoiceName::zh_CN_XiaoruiNeural]: Sounds like a 50-year-old female's voice (kind and slow).
//! - [`VoiceName::zh_CN_XiaoshuangNeural`][crate::azure::VoiceName::zh_CN_XiaoshuangNeural]: Sounds like a 10-year-old little girl.
//! - [`VoiceName::zh_CN_XiaoxiaoNeural`][crate::azure::VoiceName::zh_CN_XiaoxiaoNeural]: Sounds like a 20-year-old young lady with sweet voice.
//! - [`VoiceName::zh_CN_XiaoxuanNeural`][crate::azure::VoiceName::zh_CN_XiaoxuanNeural]: Sounds like a 27-year-old young woman with cold voice.
//! - [`VoiceName::zh_CN_XiaoyanNeural`][crate::azure::VoiceName::zh_CN_XiaoyanNeural]: Sounds like a 30-year-old woman's voice (normal).
//! - [`VoiceName::zh_CN_XiaoyiNeural`][crate::azure::VoiceName::zh_CN_XiaoyiNeural]: Sounds like a 19-year-old college girl's voice.
//! - [`VoiceName::zh_CN_XiaoyouNeural`][crate::azure::VoiceName::zh_CN_XiaoyouNeural]: Sounds like a 7-year-old little girl's voice (cute and sweet).
//! - [`VoiceName::zh_CN_XiaozhenNeural`][crate::azure::VoiceName::zh_CN_XiaozhenNeural]: Sounds like a 22-year-old young lady's voice (full of careness).
//!
//! For use of styles and roles, see [docs/azure-voices-n-roles.md](https://github.com/dongsxyz/rust-ai/blob/master/docs/azure-voices-n-roles.md).

use std::collections::HashMap;

use log::{error, warn};
use reqwest::header::HeaderMap;

use crate::azure::{
    endpoint::{
        request_get_endpoint, request_post_endpoint, request_post_endpoint_ssml,
        SpeechServiceEndpoint,
    },
    types::{
        common::{MicrosoftOutputFormat, ResponseExpectation, ResponseType},
        speech::{
            entity::EntityReference,
            file::PaginatedFiles,
            filter::FilterOperator,
            health::ServiceHealth,
            transcription::{Status, Transcription},
            ErrorResponse,
        },
        tts::Voice,
        SSML,
    },
    Locale,
};

/// The Speech service allows you to convert text into synthesized speech and
/// get a list of supported voices for a region by using a REST API.
///
/// Source: <https://learn.microsoft.com/en-us/azure/cognitive-services/speech-service/rest-text-to-speech>
pub struct Speech {
    ssml: SSML,
    output_format: MicrosoftOutputFormat,
}

impl Default for Speech {
    fn default() -> Self {
        Self {
            ssml: SSML::default(),
            output_format: MicrosoftOutputFormat::Audio_24khz_48kbitrate_Mono_Mp3,
        }
    }
}

impl From<SSML> for Speech {
    /// Convert SSML document into Speech endpoint instance directly.
    fn from(value: SSML) -> Self {
        Self {
            ssml: value,
            output_format: MicrosoftOutputFormat::Audio_24khz_48kbitrate_Mono_Mp3,
        }
    }
}

impl Speech {
    pub fn new_transcription(display_name: String) -> Transcription {
        Transcription::default().display_name(display_name)
    }

    pub fn format(self, f: MicrosoftOutputFormat) -> Self {
        Self {
            output_format: f,
            ..self
        }
    }

    pub fn ssml(self, ssml: SSML) -> Self {
        Self { ssml, ..self }
    }

    /// Get a full list of voices for a specific region or endpoint. Prefix the
    /// voices list endpoint with a region to get a list of voices for that
    /// region. This is preconfigured in your `config.yml`.
    ///
    /// Voices and styles in preview are only available in three service
    /// regions: East US, West Europe, and Southeast Asia.
    ///
    /// Source: <https://learn.microsoft.com/en-us/azure/cognitive-services/speech-service/rest-text-to-speech>
    pub async fn voice_list() -> Result<Vec<Voice>, Box<dyn std::error::Error>> {
        let text =
            request_get_endpoint(&SpeechServiceEndpoint::Get_List_of_Voices, None, None).await?;
        match serde_json::from_str::<Vec<Voice>>(&text) {
            Ok(voices) => Ok(voices),
            Err(e) => {
                warn!(target: "azure", "Error parsing response: {:?}", e);
                Err("Unable to parse voice list, check log for details".into())
            }
        }
    }

    /// Health status provides insights about the overall health of the service
    /// and sub-components.
    ///
    /// V3.1 API supported only.
    pub async fn health_check() -> Result<ServiceHealth, Box<dyn std::error::Error>> {
        let text = request_get_endpoint(
            &SpeechServiceEndpoint::Get_Speech_to_Text_Health_Status_v3_1,
            None,
            None,
        )
        .await?;

        match serde_json::from_str::<ServiceHealth>(&text) {
            Ok(status) => Ok(status),
            Err(e) => {
                warn!(target: "azure", "Error parsing response: {:?}", e);
                Err("Unable to parse health status of speech cognitive services, check log for details".into())
            }
        }
    }

    /// The text-to-speech REST API supports neural text-to-speech voices, which
    /// support specific languages and dialects that are identified by locale. Each
    /// available endpoint is associated with a region. A Speech resource key for
    /// the endpoint or region that you plan to use is required.
    ///
    /// Source: <https://learn.microsoft.com/en-us/azure/cognitive-services/speech-service/rest-text-to-speech>
    pub async fn text_to_speech(self) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let mut headers = HeaderMap::new();
        headers.insert("X-Microsoft-OutputFormat", self.output_format.into());
        match request_post_endpoint_ssml(
            &SpeechServiceEndpoint::Post_Text_to_Speech_v1,
            self.ssml,
            ResponseExpectation::Bytes,
            Some(headers),
        )
        .await
        {
            Ok(ResponseType::Bytes(bytes)) => Ok(bytes),
            Err(e) => Err(e),
            _ => Err("Incorrect response type".into()),
        }
    }

    /// Same as `text_to_speech`.
    pub async fn tts(self) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        Ok(self.text_to_speech().await?)
    }
}

impl Transcription {
    pub fn skip(self, skip: usize) -> Self {
        Self {
            skip: Some(skip),
            ..self
        }
    }
    pub fn top(self, top: usize) -> Self {
        Self {
            top: Some(top),
            ..self
        }
    }
    pub fn filter(self, filter: FilterOperator) -> Self {
        Self {
            filter: Some(filter),
            ..self
        }
    }

    pub fn sas_validity_in_seconds(self, sec: u32) -> Self {
        Self {
            sas_validity_in_seconds: Some(sec),
            ..self
        }
    }

    pub fn model(self, model: String) -> Self {
        Self {
            model: Some(EntityReference::from(model)),
            ..self
        }
    }

    pub fn content_container_url(self, url: String) -> Self {
        Self {
            content_container_url: Some(url),
            ..self
        }
    }
    pub fn content_urls(self, urls: Vec<String>) -> Self {
        Self {
            content_urls: Some(urls),
            ..self
        }
    }

    pub fn project(self, project: String) -> Self {
        Self {
            project: Some(EntityReference::from(project)),
            ..self
        }
    }

    pub fn set_self(self, _self: String) -> Self {
        Self {
            _self: Some(_self),
            ..self
        }
    }

    /// Change display name of current transcription job.
    ///
    /// No effect after transcription submitted.
    pub fn display_name(self, display_name: String) -> Self {
        Self {
            display_name,
            ..self
        }
    }

    /// Change default locale of this transcription job.
    ///
    /// No effect after transcription submitted.
    pub fn locale(self, locale: Locale) -> Self {
        Self { locale, ..self }
    }

    /// [Custom Speech]
    /// Gets the list of custom models for the authenticated subscription.
    ///
    /// TODO: implement this.
    pub async fn models(self) -> Result<(), Box<dyn std::error::Error>> {
        todo!("Test with custom models");
        // let mut params = HashMap::<String, String>::new();

        // if let Some(skip) = self.skip {
        //     params.insert("skip".into(), skip.to_string());
        // }
        // if let Some(top) = self.top {
        //     params.insert("top".into(), top.to_string());
        // }
        // if let Some(filter) = self.filter {
        //     params.insert("filter".into(), filter.to_string());
        // }

        // let text = request_get_endpoint(
        //     &SpeechServiceEndpoint::Get_List_of_Models_v3_1,
        //     Some(params),
        // )
        // .await?;

        // println!("{}", text);

        // match serde_json::from_str::<ServiceHealthResponse>(&text) {
        //     Ok(status) => Ok(status),
        //     Err(e) => {
        //         warn!(target: "azure", "Error parsing response: {:?}", e);
        //         Err("Unable to parse health status of speech cognitive services, check log for details".into())
        //     }
        // }

        // Ok(())
    }

    /// Create a new audio transcription job.
    pub async fn create(self) -> Result<Transcription, Box<dyn std::error::Error>> {
        return if let ResponseType::Text(text) = request_post_endpoint(
            &SpeechServiceEndpoint::Post_Create_Transcription_v3_1,
            self,
            ResponseExpectation::Text,
            None,
        )
        .await?
        {
            return match serde_json::from_str::<Transcription>(&text) {
                Ok(trans) => Ok(trans),
                Err(e) => {
                    warn!(target: "azure", "Unable to parse transcription creation result: `{:#?}`", e);
                    match serde_json::from_str::<ErrorResponse>(&text) {
                        Ok(error) => {
                            println!("{:#?}", error);
                            error!(target: "azure", "Error from Azure: `{:?}`", e);
                            Err(Box::new(e))
                        }
                        Err(e) => {
                            error!(target: "azure", "Unable to parse error response: `{:?}`", e);
                            Err(Box::new(e))
                        }
                    }
                }
            };
        } else {
            Err("Unable to load output from Azure speech service endpoint".into())
        };
    }

    /// Check transcription status
    ///
    /// This will only succeed when you've submitted the initial batch create
    /// request to Azure endpoint.
    pub async fn status(self) -> Result<Transcription, Box<dyn std::error::Error>> {
        let text = request_get_endpoint(
            &SpeechServiceEndpoint::Get_Transcription_v3_1,
            None,
            Some(self.transcription_id().unwrap()),
        )
        .await?;

        return match serde_json::from_str::<Transcription>(&text) {
            Ok(trans) => Ok(trans),
            Err(e) => {
                warn!(target: "azure", "Unable to parse transcription status result: `{:#?}`", e);
                match serde_json::from_str::<ErrorResponse>(&text) {
                    Ok(error) => {
                        println!("{:#?}", error);
                        error!(target: "azure", "Error from Azure: `{:?}`", e);
                        Err(Box::new(e))
                    }
                    Err(e) => {
                        error!(target: "azure", "Unable to parse error response: `{:?}`", e);
                        Err(Box::new(e))
                    }
                }
            }
        };
    }

    /// Get batch transcription result from Azure endpoint
    pub async fn files(self) -> Result<PaginatedFiles, Box<dyn std::error::Error>> {
        if let None = self.status.clone() {
            return Err("You should submit the create request first.".into());
        } else {
            match self.status.clone().unwrap() {
                Status::Succeeded => (),
                Status::Failed => {
                    return Err("The transcription failed, thus no results available.".into())
                }
                Status::NotStarted => return Err("Please wait for transcription to start.".into()),
                Status::Running => return Err("Please wait until results available.".into()),
            }
        }

        let mut params = HashMap::<String, String>::new();
        if let Some(sas) = self.sas_validity_in_seconds.clone() {
            params.insert("sasValidityInSeconds".into(), sas.to_string());
        }
        if let Some(skip) = self.skip.clone() {
            params.insert("skip".into(), skip.to_string());
        }
        if let Some(top) = self.top.clone() {
            params.insert("top".into(), top.to_string());
        }
        if let Some(filter) = self.filter.clone() {
            params.insert("filter".into(), filter.to_string());
        }

        let text = request_get_endpoint(
            &SpeechServiceEndpoint::Get_Transcription_Files_v3_1,
            Some(params),
            Some(format!("{}/files", self.transcription_id().unwrap())),
        )
        .await?;

        return match serde_json::from_str::<PaginatedFiles>(&text) {
            Ok(files) => Ok(files),
            Err(e) => {
                warn!(target: "azure", "Unable to parse transcription status result: `{:#?}`", e);
                match serde_json::from_str::<ErrorResponse>(&text) {
                    Ok(error) => {
                        println!("{:#?}", error);
                        error!(target: "azure", "Error from Azure: `{:?}`", e);
                        Err(Box::new(e))
                    }
                    Err(e) => {
                        error!(target: "azure", "Unable to parse error response: `{:?}`", e);
                        Err(Box::new(e))
                    }
                }
            }
        };
    }
}

impl PaginatedFiles {
    /// Load more results from Azure endpoint.
    ///
    /// TODO: Note, this method is not implemented yet!
    pub async fn more(self) -> Result<Option<PaginatedFiles>, Box<dyn std::error::Error>> {
        if let Some(_next_page_url) = self.next_link {
            todo!("Unimplemented method");
        } else {
            Ok(None)
        }
    }
}

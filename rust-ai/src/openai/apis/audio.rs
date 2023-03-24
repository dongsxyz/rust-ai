//!
//! Turn speech to text
//!
//! Source: OpenAI documentation

////////////////////////////////////////////////////////////////////////////////

use crate::openai::{
    endpoint::{
        request_endpoint_form_data, Endpoint, EndpointVariant, AudioEndpointVariant,
    },
    types::{
        audio::{AudioResponse, Format},
        common::Error,
        model::Model,
    },
};
use isolang::Language;
use log::{debug, warn};
use reqwest::multipart::{Form, Part};

/// Given a prompt and an instruction, the model will return an edited version
/// of the prompt.
#[derive(Debug)]
pub struct Audio {
    model: Model,
    prompt: Option<String>,
    response_format: Option<Format>,
    temperature: Option<f32>,
    language: Option<Language>,
}

impl Default for Audio {
    fn default() -> Self {
        Self {
            model: Model::WHISPER_1,
            prompt: None,
            response_format: None,
            temperature: None,
            language: None,
        }
    }
}

impl Audio {
    /// Determine and verify MIME type of input file.
    ///
    /// # Arguments
    /// - `file_name` - Name of the input file
    fn mime(
        &self,
        file_name: &str,
    ) -> Result<&'static str, Box<dyn std::error::Error>> {
        Ok(match *file_name.split('.').collect::<Vec<&str>>().last().unwrap() {
            "mp3" => "audio/mpeg",
            "mp4" => "audio/mp4",
            "mpeg" => "audio/mpeg",
            "mpga" => "audio/mpeg",
            "m4a" => "audio/mp4a-latm",
            "wav" => "audio/wav",
            "webm" => "audio/webm",
            _ => return Err("Unsupported format!".into()),
        })
    }

    /// ID of the model to use. Only `whisper-1` is currently available.
    pub fn model(self, model: Model)->Self {
        Self {
            model,
            ..self
        }
    }

    /// Set target prompt for image generations.
    ///
    /// An optional text to guide the model's style or continue a previous
    /// audio segment. The prompt should match the audio language.
    pub fn prompt(self, content: &str)->Self {
        Self {
            prompt: Some(content.into()),
            ..self
        }
    }

    /// The format of the transcript output, in one of these options: `json`,
    /// `text`, `srt`, `verbose_json`, or `vtt`.
    pub fn response_format(self, response_format: Format)->Self {
        Self {
            response_format: Some(response_format.into()),
            ..self
        }
    }

    /// The sampling temperature, between 0 and 1. Higher values like 0.8 will
    /// make the output more random, while lower values like 0.2 will make it
    /// more focused and deterministic. If set to 0, the model will use
    /// [log probability](https://en.wikipedia.org/wiki/Log_probability) to automatically increase the temperature until certain
    /// thresholds are hit.
    pub fn temperature(self, temperature: f32)->Self {
        Self {
            temperature: Some(temperature.into()),
            ..self
        }
    }

    /// The language of the input audio. Supplying the input language in [ISO-639-1](https://en.wikipedia.org/wiki/List_of_ISO_639-1_codes) format will improve accuracy and latency.
    pub fn language(self, language: Language)->Self {
        Self {
            language: Some(language.into()),
            ..self
        }
    }

    /// Send transcription request to OpenAI.
    /// 
    /// The audio file to transcribe, in one of these formats: mp3, mp4, mpeg,
    /// mpga, m4a, wav, or webm.
    /// 
    /// # Arguments
    /// - `file_name` - Audio file name
    /// - `bytes` - Bytes vector of the file
    pub async fn transcription(&self, file_name: String, bytes: Vec<u8>) -> Result<AudioResponse, Box<dyn std::error::Error>> {

        let mut audio_response: Option<AudioResponse> = None;

        let mut form = Form::new();
        let file = Part::bytes(bytes)
        .file_name(file_name.clone())
        .mime_str(self.mime(&file_name).unwrap())
        .unwrap();
        form = form.part("file", file);
        
        let m: &str = self.model.clone().into();
        form = form.part("model", Part::text(m));

        if let Some(prompt) = self.prompt.clone() {
            form = form.part("prompt", Part::text(prompt));
        }

        if let Some(lang) = self.language {
            form = form.part("language", Part::text(lang.to_639_1().unwrap()));
        }

        if let Some(fmt) = self.response_format.clone() {
            let fmt: &str = fmt.into();
            form = form.part("response_format", Part::text(fmt));
        }

        if let Some(temp) = self.temperature {
            let temp = temp.to_string();
            form = form.part("temperature", Part::text(temp));
        }

        let variant: String = AudioEndpointVariant::Transcription.into();

        request_endpoint_form_data(
            form, 
            &Endpoint::Audio_v1, EndpointVariant::Extended(variant),
             |res| {
            if let Ok(text) = res {
                if let Ok(response_data) = serde_json::from_str::<AudioResponse>(&text) {
                    debug!(target: "openai", "Response parsed, audio transcription response deserialized.");
                    audio_response = Some(response_data);
                } else {
                    if let Ok(response_error) = serde_json::from_str::<Error>(&text) {
                        warn!(target: "openai",
                            "OpenAI error code {}: `{:?}`",
                            response_error.error.code.unwrap_or(0),
                            text
                        );
                    } else {
                        warn!(target: "openai", "Audio response not deserializable.");
                    }
                }
            }
        })
        .await?;

        if let Some(response_data) = audio_response {
            Ok(response_data)
        } else {
            Err("No response or error parsing response".into())
        }
    }

    
    /// Translates audio into into English.
    /// 
    /// The audio file to transcribe, in one of these formats: mp3, mp4, mpeg,
    /// mpga, m4a, wav, or webm.
    /// 
    /// # Arguments
    /// - `file_name` - Audio file name
    /// - `bytes` - Bytes vector of the file
    pub async fn translation(&self, file_name: String, bytes: Vec<u8>) -> Result<AudioResponse, Box<dyn std::error::Error>> {

        let mut audio_response: Option<AudioResponse> = None;

        let mut form = Form::new();
        let file = Part::bytes(bytes)
        .file_name(file_name.clone())
        .mime_str(self.mime(&file_name).unwrap())
        .unwrap();
        form = form.part("file", file);
        
        let m: &str = self.model.clone().into();
        form = form.part("model", Part::text(m));

        if let Some(prompt) = self.prompt.clone() {
            form = form.part("prompt", Part::text(prompt));
        }

        if let Some(fmt) = self.response_format.clone() {
            let fmt: &str = fmt.into();
            form = form.part("response_format", Part::text(fmt));
        }

        if let Some(temp) = self.temperature {
            let temp = temp.to_string();
            form = form.part("temperature", Part::text(temp));
        }

        let variant: String = AudioEndpointVariant::Transcription.into();

        request_endpoint_form_data(
            form, 
            &Endpoint::Audio_v1, EndpointVariant::Extended(variant),
             |res| {
            if let Ok(text) = res {
                if let Ok(response_data) = serde_json::from_str::<AudioResponse>(&text) {
                    debug!(target: "openai", "Response parsed, audio translation response deserialized.");
                    audio_response = Some(response_data);
                } else {
                    if let Ok(response_error) = serde_json::from_str::<Error>(&text) {
                        warn!(target: "openai",
                            "OpenAI error code {}: `{:?}`",
                            response_error.error.code.unwrap_or(0),
                            text
                        );
                    } else {
                        warn!(target: "openai", "Audio response not deserializable.");
                    }
                }
            }
        })
        .await?;

        if let Some(response_data) = audio_response {
            Ok(response_data)
        } else {
            Err("No response or error parsing response".into())
        }
    }
}

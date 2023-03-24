use log::warn;
use reqwest::header::HeaderMap;

use crate::azure::{
    endpoint::{request_get_endpoint, request_post_endpoint_ssml, SpeechServiceEndpoint},
    types::{
        common::{MicrosoftOutputFormat, ResponseExpectation, ResponseType},
        tts::{Voice, TTS_SSML},
        Gender, Locale, VoiceName,
    },
};

/// The Speech service allows you to convert text into synthesized speech and
/// get a list of supported voices for a region by using a REST API.
///
/// Source: <https://learn.microsoft.com/en-us/azure/cognitive-services/speech-service/rest-text-to-speech>
pub struct Speech {
    voice: VoiceName,
    gender: Gender,
    lang: Locale,
    output_format: MicrosoftOutputFormat,
}

impl Default for Speech {
    fn default() -> Self {
        Self {
            voice: VoiceName::en_US_ChristopherNeural,
            gender: Gender::Male,
            lang: Locale::en_US,
            output_format: MicrosoftOutputFormat::Ogg_24khz_16bit_Mono_Opus,
        }
    }
}

impl Speech {
    pub fn voice(self, v: VoiceName) -> Self {
        Self { voice: v, ..self }
    }

    pub fn gender(self, g: Gender) -> Self {
        Self { gender: g, ..self }
    }

    pub fn lang(self, l: Locale) -> Self {
        Self { lang: l, ..self }
    }

    pub fn format(self, f: MicrosoftOutputFormat) -> Self {
        Self {
            output_format: f,
            ..self
        }
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
        let text = request_get_endpoint(&SpeechServiceEndpoint::Get_List_of_Voices).await?;
        match serde_json::from_str::<Vec<Voice>>(&text) {
            Ok(voices) => Ok(voices),
            Err(e) => {
                warn!(target: "azure", "Error parsing response: {:?}", e);
                Err("Unable to parse voice list, check log for details".into())
            }
        }
    }

    /// The text-to-speech REST API supports neural text-to-speech voices, which
    /// support specific languages and dialects that are identified by locale. Each
    /// available endpoint is associated with a region. A Speech resource key for
    /// the endpoint or region that you plan to use is required.
    ///
    /// Source: <https://learn.microsoft.com/en-us/azure/cognitive-services/speech-service/rest-text-to-speech>
    pub async fn text_to_speech(self, text: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let ssml = TTS_SSML::default()
            .set_content(text)
            .set_gender(self.gender.clone())
            .set_lang(self.lang.clone())
            .set_voice(self.voice.clone());
        let mut headers = HeaderMap::new();
        headers.insert("X-Microsoft-OutputFormat", self.output_format.into());
        match request_post_endpoint_ssml(
            &SpeechServiceEndpoint::Convert_Text_to_Speech,
            ssml,
            ResponseExpectation::Bytes,
            headers,
        )
        .await
        {
            Ok(ResponseType::Bytes(bytes)) => Ok(bytes),
            Err(e) => Err(e),
            _ => Err("Incorrect response type".into()),
        }
    }

    /// Same as `text_to_speech`.
    pub async fn tts(self, text: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        Ok(self.text_to_speech(text).await?)
    }
}

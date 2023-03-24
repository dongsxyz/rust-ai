use log::warn;

use crate::azure::{
    endpoint::{request_get_endpoint, request_post_endpoint_ssml, SpeechServiceEndpoint},
    types::{
        common::{ResponseExpectation, ResponseType},
        tts::{Voice, TTS_SSML},
        Gender, Locale, VoiceName,
    },
};

pub struct Speech {
    voice: VoiceName,
    gender: Gender,
    lang: Locale,
}

impl Default for Speech {
    fn default() -> Self {
        Self {
            voice: VoiceName::en_US_ChristopherNeural,
            gender: Gender::Male,
            lang: Locale::en_US,
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

    pub async fn text_to_speech(self, text: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let ssml = TTS_SSML::default()
            .set_content(text)
            .set_gender(self.gender.clone())
            .set_lang(self.lang.clone())
            .set_voice(self.voice.clone());

        match request_post_endpoint_ssml(
            &SpeechServiceEndpoint::Convert_Text_to_Speech,
            ssml,
            ResponseExpectation::Bytes,
        )
        .await
        {
            Ok(ResponseType::Bytes(bytes)) => Ok(bytes),
            Err(e) => Err(e),
            _ => Err("Incorrect response type".into()),
        }
    }

    pub async fn tts(self, text: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        Ok(self.text_to_speech(text).await?)
    }
}

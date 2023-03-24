use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::{
    common::{Gender, SSML},
    locale::Locale,
    voice_name::VoiceName,
};

#[derive(Serialize, Deserialize, Debug)]
pub struct Voice {
    #[serde(rename = "Name")]
    pub name: String,

    #[serde(rename = "DisplayName")]
    pub display_name: String,

    #[serde(rename = "LocalName")]
    pub local_name: String,

    #[serde(rename = "ShortName")]
    pub short_name: String,

    #[serde(rename = "Gender")]
    pub gender: String,

    #[serde(rename = "Locale")]
    pub locale: String,

    #[serde(rename = "LocaleName")]
    pub locale_name: String,

    #[serde(rename = "SampleRateHertz")]
    pub sample_rate_hertz: String,

    #[serde(rename = "VoiceType")]
    pub voice_type: String,

    #[serde(rename = "Status")]
    pub status: String,

    #[serde(rename = "WordsPerMinute")]
    pub words_per_minute: Option<String>,

    #[serde(rename = "StyleList")]
    pub style_list: Option<Vec<String>>,

    #[serde(rename = "RolePlayList")]
    pub role_play_list: Option<Vec<String>>,

    #[serde(rename = "SecondaryLocaleList")]
    pub secondary_locale_list: Option<Vec<String>>,

    #[serde(rename = "ExtendedPropertyMap")]
    pub extended_property_map: Option<HashMap<String, String>>,
}

#[allow(non_camel_case_types)]
#[derive(Debug)]
pub struct TTS_SSML {
    speak_lang: String,
    voice_lang: String,
    voice_gender: String,
    voice_name: String,
    content: String,
}
impl SSML for TTS_SSML {}

impl Default for TTS_SSML {
    fn default() -> Self {
        Self {
            speak_lang: Locale::en_US.into(),
            voice_lang: Locale::en_US.into(),
            voice_gender: Gender::Male.into(),
            voice_name: VoiceName::en_US_ChristopherNeural.into(),
            content: String::new(),
        }
    }
}

impl TTS_SSML {
    pub fn set_lang(self, locale: Locale) -> Self {
        Self {
            speak_lang: locale.clone().into(),
            voice_lang: locale.clone().into(),
            ..self
        }
    }

    pub fn set_gender(self, gender: Gender) -> Self {
        Self {
            voice_gender: gender.into(),
            ..self
        }
    }

    pub fn set_voice(self, voice: VoiceName) -> Self {
        Self {
            voice_name: voice.into(),
            ..self
        }
    }

    pub fn set_content(self, content: &str) -> Self {
        Self {
            content: String::from(content),
            ..self
        }
    }
}

impl Into<String> for TTS_SSML {
    fn into(self) -> String {
        format!(
            r#"<speak version='1.0' xml:lang='{}'><voice xml:lang='{}' xml:gender='{}'
        name='{}'>{}</voice></speak>"#,
            &self.speak_lang, &self.voice_lang, &self.voice_gender, &self.voice_name, &self.content
        )
    }
}

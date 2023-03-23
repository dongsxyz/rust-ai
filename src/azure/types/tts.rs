use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::{common::SSML, voice_name::VoiceName};

#[derive(Serialize, Deserialize, Debug)]
pub struct Voice {
    #[serde(rename = "Name")]
    name: String,

    #[serde(rename = "DisplayName")]
    display_name: String,

    #[serde(rename = "LocalName")]
    local_name: String,

    #[serde(rename = "ShortName")]
    short_name: String,

    #[serde(rename = "Gender")]
    gender: String,

    #[serde(rename = "Locale")]
    locale: String,

    #[serde(rename = "LocaleName")]
    locale_name: String,

    #[serde(rename = "SampleRateHertz")]
    sample_rate_hertz: String,

    #[serde(rename = "VoiceType")]
    voice_type: String,

    #[serde(rename = "Status")]
    status: String,

    #[serde(rename = "WordsPerMinute")]
    words_per_minute: Option<String>,

    #[serde(rename = "StyleList")]
    style_list: Option<Vec<String>>,

    #[serde(rename = "RolePlayList")]
    role_play_list: Option<Vec<String>>,

    #[serde(rename = "SecondaryLocaleList")]
    secondary_locale_list: Option<Vec<String>>,

    #[serde(rename = "ExtendedPropertyMap")]
    extended_property_map: Option<HashMap<String, String>>,
}

#[allow(non_camel_case_types)]
#[derive(Debug)]
pub struct TTS_SSML {
    pub speak_lang: String,
    pub voice_lang: String,
    pub voice_gender: String,
    pub voice_name: String,
    pub content: String,
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

impl Into<String> for TTS_SSML {
    fn into(self) -> String {
        format!(
            r#"<speak version='1.0' xml:lang='{}'><voice xml:lang='{}' xml:gender='{}'
        name='{}'>{}</voice></speak>"#,
            &self.speak_lang, &self.voice_lang, &self.voice_gender, &self.voice_name, &self.content
        )
    }
}

pub enum Gender {
    Male,
    Female,
}

impl Into<String> for Gender {
    fn into(self) -> String {
        (match self {
            Self::Male => "Male",
            Self::Female => "Female",
        })
        .into()
    }
}

#[allow(non_camel_case_types)]
pub enum Locale {
    en_US,
    zh_CN,
}

impl Into<String> for Locale {
    fn into(self) -> String {
        (match self {
            Self::en_US => "en-US",
            Self::zh_CN => "zh-CN",
        })
        .into()
    }
}

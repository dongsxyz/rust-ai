
use std::collections::HashMap;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
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

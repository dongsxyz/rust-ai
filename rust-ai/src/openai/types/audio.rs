use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AudioResponse {
    pub text: Option<String>,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Format {
    #[serde(rename = "json")]
    JSON,
    #[serde(rename = "text")]
    Text,
    #[serde(rename = "srt")]
    SRT,
    #[serde(rename = "verbose_json")]
    Verbose_JSON,
    #[serde(rename = "vtt")]
    VTT,
}

impl Into<&'static str> for Format {
    fn into(self) -> &'static str {
        match self {
            Self::JSON => "json",
            Self::Text => "text",
            Self::SRT => "srt",
            Self::Verbose_JSON => "verbose_json",
            Self::VTT => "vtt"
        }
    }
}
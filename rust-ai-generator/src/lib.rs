use std::path::PathBuf;

use chrono::Utc;
use reqwest::Client;
use types::Voice;
use utils::config::Config;

/// Support functions
pub mod utils;

pub mod types;

pub async fn get_azure_voice_list() -> Result<Vec<Voice>, Box<dyn std::error::Error>> {
    let config = Config::load().unwrap();

    let url = format!(
        "https://{}.tts.speech.microsoft.com/cognitiveservices/voices/list",
        config.azure.speech.region
    );

    let text = request_get(&url).await?;
    match serde_json::from_str::<Vec<Voice>>(&text) {
        Ok(voices) => Ok(voices),
        Err(e) => Err(format!(
            "Unable to parse voice list, check log for details: {:#?}",
            e
        )
        .into()),
    }
}

pub async fn request_get(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    let config = Config::load().unwrap();

    let client = Client::new();
    let mut req = client.get(url);
    req = req.header("Ocp-Apim-Subscription-Key", config.azure.speech.key);

    let res = req.send().await?;

    match res.text().await {
        Ok(text) => Ok(text),
        Err(e) => Err(Box::new(e)),
    }
}

pub fn generate_voice_names(
    path: PathBuf,
    voices: Vec<Voice>,
) -> Result<bool, Box<dyn std::error::Error>> {
    if voices.len() > 0 {
        // Generate file
        let mut voice_names = vec![];
        let mut voice_name_intos = vec![];

        voices.iter().for_each(|vn| {
            let enum_variant_name = vn.short_name.replace("-", "_");

            voice_names.push(format!(
                "\n{}/// Voice name variant for `{}`\n{}{},",
                " ".repeat(4),
                vn.short_name,
                " ".repeat(4),
                enum_variant_name
            ));
            voice_name_intos.push(format!(
                "{}Self::{} => \"{}\",",
                " ".repeat(12),
                enum_variant_name,
                vn.short_name
            ));
        });

        // Generate `voice_name.rs`
        let voice_name_file_content = format!(
            r#"//!
//! *Auto-generated file, you should NOT update its contents directly*
//! 
//! Voice names fetched from Microsoft Cognitive Services API.
//! 
//! Updated on {}.

////////////////////////////////////////////////////////////////////////////////

/// VoiceNames generated from API call
#[allow(non_camel_case_types)]
#[derive(Debug, Clone)]
pub enum VoiceName {{
{}
}}

impl Into<String> for VoiceName {{
    fn into(self) -> String {{
        (match self {{
{}
        }})
        .into()
    }}
}}"#,
            format!("{}", Utc::now().format("%Y-%m-%d")),
            voice_names.join("\n"),
            voice_name_intos.join("\n")
        );

        return if path.exists() {
            std::fs::write(path, &voice_name_file_content).unwrap();
            Ok(true)
        } else {
            Ok(false)
        };
    }
    Ok(true)
}

pub fn generate_locale_names(
    path: PathBuf,
    voices: Vec<Voice>,
) -> Result<bool, Box<dyn std::error::Error>> {
    if voices.len() > 0 {
        // Generate file
        let mut locale_names = vec![];
        let mut locale_name_intos = vec![];
        let mut locale_name_froms = vec![];

        voices.iter().for_each(|vn| {
            let locale_name = vn
                .short_name
                .split("-")
                .take(2)
                .collect::<Vec<&str>>()
                .join("-");
            let locale_variant_name = locale_name.replace("-", "_");

            let temp = format!(
                "\n{}/// Locale variant for `{}`\n{}{},",
                " ".repeat(4),
                locale_name,
                " ".repeat(4),
                locale_variant_name
            );
            if !locale_names.contains(&temp) {
                locale_names.push(temp);
            }

            let temp = format!(
                "{}Self::{} => \"{}\",",
                " ".repeat(12),
                locale_variant_name,
                locale_name
            );
            if !locale_name_intos.contains(&temp) {
                locale_name_intos.push(temp);
            }

            let temp = format!(
                "{}\"{}\" => Self::{},",
                " ".repeat(12),
                locale_name,
                locale_variant_name,
            );
            if !locale_name_froms.contains(&temp) {
                locale_name_froms.push(temp);
            }
        });

        // Generate `locale.rs`
        let locale_file_content = format!(
            r#"//!
//! *Auto-generated file, you should NOT update its contents directly*
//! 
//! Locale names fetched from Microsoft Cognitive Services API.
//! 
//! Updated on {}.

////////////////////////////////////////////////////////////////////////////////

/// Locales generated from API call
#[allow(non_camel_case_types)]
#[derive(Debug, Clone)]
pub enum Locale {{
{}
}}

impl Into<String> for Locale {{
    fn into(self) -> String {{
      (match self {{
{}
      }})
      .into()
  }}
}}


impl From<&str> for Locale {{
    fn from(value: &str) -> Self {{
        match value {{
{}
        _ => {{
          log::warn!( target: "rust-ai", "Unrecognized locale `{{}}`", value);
          todo!("The local file should be updated and regenerated")
        }}
      }}
  }}
}}

{}
"#,
            format!("{}", Utc::now().format("%Y-%m-%d")),
            locale_names.join("\n"),
            locale_name_intos.join("\n"),
            locale_name_froms.join("\n"),
            r#"impl serde::Serialize for Locale {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let content = Into::<String>::into(self.clone());
        serializer.serialize_str(&content)
    }
}
struct LocaleVisitor;

impl<'de> serde::de::Visitor<'de> for LocaleVisitor {
    type Value = Locale;
    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(Into::<Self::Value>::into(v.as_str()))
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(Into::<Self::Value>::into(v))
    }

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("Unrecognizable locale string.")
    }
}

impl<'de> serde::Deserialize<'de> for Locale {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_string(LocaleVisitor)
    }
}"#
        );

        if path.exists() {
            std::fs::write(path, &locale_file_content).unwrap();
        }
    }
    Ok(true)
}

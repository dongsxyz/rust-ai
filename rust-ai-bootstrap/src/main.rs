use std::path::PathBuf;

use chrono::Utc;
use rust_ai::azure::Speech;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let voices = Speech::voice_list().await?;

    if voices.len() > 0 {
        // Generate file
        let mut voice_names = vec![];
        let mut voice_name_intos = vec![];
        let mut locale_names = vec![];
        let mut locale_name_intos = vec![];

        voices.iter().for_each(|vn| {
            let enum_variant_name = vn.short_name.replace("-", "_");
            let locale_name = vn
                .short_name
                .split("-")
                .take(2)
                .collect::<Vec<&str>>()
                .join("-");
            let locale_variant_name = locale_name.replace("-", "_");

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

        let mut voice_name_path = PathBuf::from("rust-ai");
        "src/azure/types/voice_name.rs".split("/").for_each(|seg| {
            voice_name_path = voice_name_path.join(seg);
        });

        if voice_name_path.exists() {
            std::fs::write(voice_name_path, &voice_name_file_content).unwrap();
        }

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
}}"#,
            format!("{}", Utc::now().format("%Y-%m-%d")),
            locale_names.join("\n"),
            locale_name_intos.join("\n")
        );

        let mut locale_path = PathBuf::from("rust-ai");
        "src/azure/types/locale.rs".split("/").for_each(|seg| {
            locale_path = locale_path.join(seg);
        });

        if locale_path.exists() {
            std::fs::write(locale_path, &locale_file_content).unwrap();
        }
    }

    Ok(())
}

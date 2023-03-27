use std::path::PathBuf;

use rust_ai_generator::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let voices = get_azure_voice_list().await?;

    let mut voice_name_path = PathBuf::from("rust-ai");
    "src/azure/types/ssml/voice_name.rs"
        .split("/")
        .for_each(|seg| {
            voice_name_path = voice_name_path.join(seg);
        });
    generate_voice_names(voice_name_path, voices.clone()).unwrap();

    let mut locale_path = PathBuf::from("rust-ai");
    "src/azure/types/locale.rs".split("/").for_each(|seg| {
        locale_path = locale_path.join(seg);
    });
    generate_locale_names(locale_path, voices.clone()).unwrap();

    Ok(())
}

use std::{error::Error, path::PathBuf};

use rust_ai::azure::{ssml, MicrosoftOutputFormat, Speech, VoiceName, SSML};
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    log4rs::init_file("log4rs.yml", Default::default()).unwrap();

    let ssml = SSML::default().speak(ssml::Speak::new(
        ssml::Voice::new(VoiceName::zh_CN_YunhaoNeural
        ).content("亲爱的朋友，美丽中国欢迎你！".into()),
    ));

    println!("{}", ssml.to_string());

    let result = Speech::default()
        .ssml(ssml)
        .format(MicrosoftOutputFormat::Audio_24khz_48kbitrate_Mono_Mp3)
        .tts()
        .await?;

    std::fs::write(
        PathBuf::from(r"C:\Users\zhongdongy\Downloads\output.mp3"),
        result,
    )
    .unwrap();

    Ok(())
}

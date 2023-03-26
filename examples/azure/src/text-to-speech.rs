use std::{error::Error, path::PathBuf};

use rust_ai::azure::{ssml::Speak, Locale, Speech, VoiceName, SSML};
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    log4rs::init_file("log4rs.yml", Default::default()).unwrap();

    let ssml = SSML::from(
        Speak::voice_content(
            VoiceName::zh_CN_YunhaoNeural,
            "亲爱的朋友，美丽中国欢迎你！",
        )
        .lang(Locale::zh_CN),
    );

    println!("{}", ssml.to_string());

    let result = Speech::from(ssml).tts().await?;

    std::fs::write(
        PathBuf::from(r"C:\Users\zhongdongy\Downloads\output.mp3"),
        result,
    )
    .unwrap();

    Ok(())
}

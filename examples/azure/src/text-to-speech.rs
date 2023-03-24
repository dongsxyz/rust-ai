use std::{error::Error, path::PathBuf};

use rust_ai::azure::{MicrosoftOutputFormat, Speech, VoiceName};
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    log4rs::init_file("log4rs.yml", Default::default()).unwrap();

    let result = Speech::default()
        .voice(VoiceName::zh_CN_YunfengNeural)
        .format(MicrosoftOutputFormat::Audio_24khz_48kbitrate_Mono_Mp3)
        .tts(r#"美丽中国，大好河山欢迎您！"#.into())
        .await?;

    std::fs::write(PathBuf::from(r"D:\Contents\Downloads\output.mp3"), result).unwrap();

    Ok(())
}

use std::error::Error;

use rust_ai::azure::endpoint::{request_post_endpoint_ssml, SpeechServiceEndpoint};
use rust_ai::azure::types::tts::TTS_SSML;
use rust_ai::azure::Speech;
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    log4rs::init_file("log4rs.yml", Default::default()).unwrap();

    let mut ssml = TTS_SSML::default();
    ssml.content = "Hello World! This is little David".into();
    let result =
        request_post_endpoint_ssml(&SpeechServiceEndpoint::Convert_Text_to_Speech, ssml).await?;

    // let result = Speech::get_voice_list().await?;
    println!("{:?}", result);
    Ok(())
}

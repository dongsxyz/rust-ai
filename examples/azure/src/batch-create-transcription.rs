use std::{error::Error, time::Duration};

use rust_ai::azure::{
    types::speech::transcription::{Status, Transcription},
    SpeechModel,
};
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    log4rs::init_file("log4rs.yml", Default::default()).unwrap();

    let mut trans = Transcription::default();
    trans.display_name = "Test".into();
    trans.content_urls = Some(vec![String::from(
        "https://crbn.us/whatstheweatherlike.wav",
    )]);
    let trans = SpeechModel::default()
        .transcription(trans)
        .create_transcription()
        .await?;

    std::thread::sleep(Duration::from_secs(5));
    let trans = trans.status().await?;
    if let Some(Status::Succeeded) = trans.status {
        let results = trans.results().await?;
        println!("{:?}", results.values);
    }

    Ok(())
}

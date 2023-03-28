use std::{error::Error, time::Duration};

use rust_ai::azure::{types::speech::Status, Speech};
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    log4rs::init_file("log4rs.yml", Default::default()).unwrap();

    let trans = Speech::new_transcription("Test".into())
        .content_urls(vec![String::from(
            "https://crbn.us/whatstheweatherlike.wav",
        )])
        .create()
        .await?;

    std::thread::sleep(Duration::from_secs(5));
    let trans = trans.status().await?;
    if let Some(Status::Succeeded) = trans.status {
        let results = trans.files().await?;
        println!("{:#?}", results.values);
    }

    Ok(())
}

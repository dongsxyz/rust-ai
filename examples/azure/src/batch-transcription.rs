use std::{error::Error, time::Duration};

use rust_ai::azure::{
    types::speech::{file::FileType, transcription::TranscriptionProperties, Status},
    Speech,
};
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    log4rs::init_file("log4rs.yml", Default::default()).unwrap();

    let mut trans = Speech::new_transcription("Test".into())
        .content_urls(vec![String::from(
            "https://crbn.us/whatstheweatherlike.wav",
        )])
        .properties(|props| {
            if let Some(props) = props {
                props
            } else {
                TranscriptionProperties::default()
            }
            .display_form_word_level_timestamps_enabled(true)
        })
        .create()
        .await?;

    // Check transcription job status.
    while [Status::Running, Status::NotStarted]
        .contains(&trans.status().await?.status.clone().unwrap())
    {
        std::thread::sleep(Duration::from_secs(2));
    }

    // Get transcription result files.
    let results = trans.files().await?;
    let files = results.values.clone();

    if files.len() > 0 {
        // Get transcription report.
        let _report = results.report().await?;
        // println!("{:#?}", _report);

        // Get transcription result file via individual API endpoint.
        for file in files.iter() {
            match file.details().await? {
                FileType::TranscriptionReport(report) => {
                    println!(
                        "Completed transcription: {} of {}",
                        report.successful_transcriptions_count,
                        report.successful_transcriptions_count + report.failed_transcriptions_count
                    );
                }
                FileType::Transcription(transcription) => {
                    println!(
                        "Transcription output: {}",
                        transcription
                            .combined_recognized_phrases
                            .get(0)
                            .unwrap()
                            .display
                    );
                }
            };
        }
    }

    Ok(())
}

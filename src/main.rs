use std::{error::Error, path::PathBuf};

use rust_ai::openai::Audio;
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    log4rs::init_file("log4rs.yml", Default::default()).unwrap();

    let audio = Audio::default();
    let mut file_path = PathBuf::from("D:\\Contents\\Downloads");
    file_path = file_path.join("20220918_000937.m4a");
    let result = audio
        .translation(String::from("input.m4a"), std::fs::read(file_path).unwrap())
        .await?;
    println!("{:?}", result.text);
    Ok(())
}

use std::{error::Error, path::PathBuf};

use rust_ai::azure::Speech;
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    log4rs::init_file("log4rs.yml", Default::default()).unwrap();

    let result = Speech::default().tts("I'm a fat boy, I don't like myself.").await?;

    
    std::fs::write(PathBuf::from(r"D:\Contents\Downloads\output.ogg"), result).unwrap();

    Ok(())
}
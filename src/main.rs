use std::error::Error;

use rust_ai::openai::Moderation;
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    log4rs::init_file("log4rs.yml", Default::default()).unwrap();

    let mut moderation = Moderation::default();
    moderation.add_input("I'm so stupid. I wanna kill myself.");
    moderation.model = rust_ai::openai::types::model::Model::TEXT_MODERATION_004;
    let result = moderation.moderate().await?;
    println!("{:?}", result);
    Ok(())
}

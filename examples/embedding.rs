use std::error::Error;

use rust_ai::openai::Embedding;
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    log4rs::init_file("log4rs.yml", Default::default()).unwrap();

    let mut embedding = Embedding::default();
    embedding.set_input("Bard vs ChatGPT");
    let result = embedding.embedding().await?;
    println!("{:?}", result.data.get(0).unwrap().embedding.clone());
    Ok(())
}

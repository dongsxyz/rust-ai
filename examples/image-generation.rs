use std::error::Error;

use rust_ai::openai::Image;
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    log4rs::init_file("log4rs.yml", Default::default()).unwrap();

    let mut completion = Image::default();
    completion.set_prompt("A giant swan.");
    let result = completion.generation().await?;
    println!("{}", result.data.get(0).unwrap().url.clone().unwrap());
    Ok(())
}

use std::error::Error;

use rust_ai::openai::Completion;
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    std::env::set_var("RUST_LOG", "info");
    std::env::set_var("RUST_BACKTRACE", "1");
    log4rs::init_file("log4rs.yml", Default::default()).unwrap();

    let result = Completion::default()
        .prompt("写一首类似《雨霖霖》的词")
        .max_tokens(1024)
        .stream_completion(Some(|chunk| {
            println!("{:?}", chunk);
        }))
        .await?;
    println!(
        "{}",
        result
            .iter()
            .map(|chunk| chunk.choices.get(0).unwrap().text.clone())
            .collect::<Vec<String>>()
            .join("")
    );
    Ok(())
}

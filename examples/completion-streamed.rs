use std::error::Error;

use rust_ai::openai::apis::completion::Completion;
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    std::env::set_var("RUST_LOG", "info");
    std::env::set_var("RUST_BACKTRACE", "1");
    log4rs::init_file("log4rs.yml", Default::default()).unwrap();

    let mut completion = Completion::default();
    completion.stream = Some(true);
    completion.add_prompt("写一首类似《雨霖霖》的词");
    completion.max_tokens = Some(1024);
    let result = completion
        .completion_streamed(Some(|chunk| {
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

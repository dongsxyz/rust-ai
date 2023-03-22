use std::error::Error;

use rust_ai::openai::{ChatCompletion, types::chat_completion::MessageRole};
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    std::env::set_var("RUST_LOG", "info");
    std::env::set_var("RUST_BACKTRACE", "1");
    log4rs::init_file("log4rs.yml", Default::default()).unwrap();

    let mut completion = ChatCompletion::default();
    completion.stream = Some(true);
    completion.add_message(
        MessageRole::System,
        "你是一个混迹糗事百科多年的网友，你需要根据用户给出的内容，给出下一句话，请保证句式工整。",
    );
    completion.add_message(MessageRole::User, "“天王盖地虎”");
    let result = completion
        .chat_completion_streamed(Some(|chunk| {
            println!("{:?}", chunk);
        }))
        .await?;
    println!(
        "{}",
        result
            .iter()
            .map(|chunk| chunk
                .choices
                .get(0)
                .unwrap()
                .delta
                .content
                .clone()
                .unwrap_or("".into()))
            .collect::<Vec<String>>()
            .join("")
    );
    Ok(())
}

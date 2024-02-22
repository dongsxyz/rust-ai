use std::error::Error;

use rust_ai::openai::{types::chat_completion::MessageRole, ChatCompletion, Model};
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    std::env::set_var("RUST_LOG", "info");
    std::env::set_var("RUST_BACKTRACE", "1");
    log4rs::init_file("log4rs.yml", Default::default()).unwrap();

    let completion = ChatCompletion::default().model(Model::GPT_4_0125_PREVIEW).message(
        MessageRole::System,
        "你是一个混迹糗事百科多年的网友，你需要根据用户给出的内容，给出下一句话，请保证句式工整。",
    ).message(MessageRole::User, "“天王盖地虎”");
    let result = completion.completion().await?;
    println!("{:?}", result);
    println!(
        "{}",
        result
            .choices
            .get(0)
            .unwrap()
            .message
            .content
            .clone()
            .unwrap()
    );
    Ok(())
}

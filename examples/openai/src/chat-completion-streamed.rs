use std::error::Error;

use rust_ai::{
    openai::{types::chat_completion::MessageRole, ChatCompletion},
    utils::header::AdditionalHeaders,
};
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    std::env::set_var(
        "RUST_AI_CONFIG",
        String::from(
            r#"
openai:
  api_key: sk-
  base_endpoint: https://api.openai.com
azure:
  speech:
    key: 
    region: westus"#,
        ),
    );
    log4rs::init_file("log4rs.yml", Default::default()).unwrap();

    // Set additional headers
    let mut headers = AdditionalHeaders::default();
    headers.set_header(("rust", "test"));
    headers.to_var();

    let result = ChatCompletion::default()
    .message(
        MessageRole::System,
        "你是一个混迹糗事百科多年的网友，你需要根据用户给出的内容，给出下一句话，请保证句式工整。",
    )
    .message(MessageRole::User, "“天王盖地虎”")
    .streamed_completion(Some(|chunk| {
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

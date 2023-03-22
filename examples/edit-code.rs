use std::error::Error;

use rust_ai::openai::Edit;
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    log4rs::init_file("log4rs.yml", Default::default()).unwrap();

    let mut completion = Edit::default();
    completion.temperature = Some(0.0);
    completion.set_input("println(\"Hello Rust\")");
    completion.set_instruction("Rectify this line of code");
    let result = completion.edit().await?;
    println!(
        "{}",
        result
            .choices
            .get(0)
            .unwrap()
            .text
            .clone()
    );
    Ok(())
}

use std::error::Error;

use rust_ai::openai::Edit;
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    log4rs::init_file("log4rs.yml", Default::default()).unwrap();

    let result = Edit::default()
        .temperature(0.0)
        .input("println(\"Hello Rust\")")
        .instruction("Rectify this line of code")
        .edit()
        .await?;
    println!("{}", result.choices.get(0).unwrap().text.clone());
    Ok(())
}

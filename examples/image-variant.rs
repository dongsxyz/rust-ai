use std::error::Error;

use rust_ai::openai::Image;
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    log4rs::init_file("log4rs.yml", Default::default()).unwrap();

    let mut image = Image::default();
    image.set_prompt("Make it cartoon-like.");
    image.set_image(
        "a.png",
        std::fs::read(std::path::PathBuf::from(
            "D:\\Contents\\Downloads\\WeChat Image_20230320174259.png",
        ))
        .unwrap(),
    );
    let result = image.variation().await?;
    println!("{}", result.data.get(0).unwrap().url.clone().unwrap());
    Ok(())
}

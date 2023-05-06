use std::error::Error;

use rust_ai::openai::Image;
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    log4rs::init_file("log4rs.yml", Default::default()).unwrap();

    let result = Image::default()
        .prompt(
            r#"
A cozy living room with a fireplace burning bright. A family gathering with smiles all around. Children playing with their furry friends. Soft blankets and pillows strewn about. Cups of hot cocoa steaming on the table. Candles flickering in the warm glow. A peaceful silence, broken only by happy laughter. A contented cat curled up on a lap. A cute puppy wagging its tail and chasing a toy. A sense of love and togetherness that lifts the soul.
        "#
            .trim(),
        )
        .generation()
        .await?;
    println!("{}", result.data.get(0).unwrap().url.clone().unwrap());
    Ok(())
}

use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    log4rs::init_file("log4rs.yml", Default::default()).unwrap();

    Ok(())
}

use morse_messenger::server::execute;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    execute().await
}

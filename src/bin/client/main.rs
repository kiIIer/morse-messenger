use morse_messenger::client::execute;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    execute()
}

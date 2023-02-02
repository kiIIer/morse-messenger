use morser::messenger_client::MessengerClient;

pub mod morser {
    tonic::include_proto!("morser");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = MessengerClient::connect("http://[::1]:50051").await?;

    let request = tonic::Request::new(morser::Signal { state: false });
    let response = client.chat(request).await?;

    let state = response.into_inner().state;

    println!("{}", state);

    Ok(())
}

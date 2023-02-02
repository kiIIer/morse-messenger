use morser::messenger_client::MessengerClient;
use morser::Signal;
use std::thread::sleep;
use std::time::Duration;
use tokio_stream::wrappers::ReceiverStream;

pub mod morser {
    tonic::include_proto!("morser");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let channel = tonic::transport::channel::Channel::from_static("http://[::1]:50051")
        .connect()
        .await?;

    let mut client = MessengerClient::new(channel);

    let (tx, rx) = tokio::sync::mpsc::channel(5);

    tokio::spawn(async move {
        for i in 0..10 {
            tx.send(Signal { state: i % 2 == 0 })
                .await
                .expect("Sending to server failed");
        }
    });

    let request = tonic::Request::new(ReceiverStream::new(rx));

    let mut in_stream = client.chat(request).await?.into_inner();

    while let Some(sig) = in_stream.message().await? {
        println!("{}", sig.state)
    }

    Ok(())
}

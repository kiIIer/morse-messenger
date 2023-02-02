use morser::messenger_client::MessengerClient;
use morser::Signal;
use std::io::stdin;
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

    let (out_stream, rx) = tokio::sync::mpsc::channel(1);

    let request = tonic::Request::new(ReceiverStream::new(rx));

    let mut in_stream = client.chat(request).await?.into_inner();

    tokio::spawn(async move {
        while let Ok(Some(sig)) = in_stream.message().await {
            println!("{}", sig.state)
        }
    });

    loop {
        let mut buffer = String::new();
        stdin().read_line(&mut buffer);

        out_stream.send(Signal { state: true }).await?;
    }

    Ok(())
}

use morser::messenger_client::MessengerClient;
use morser::Signal;
use std::io::stdin;
use tokio_stream::{wrappers, StreamExt};

pub mod morser {
    tonic::include_proto!("morser");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = MessengerClient::connect("http://[::1]:50051")
        .await
        .expect("Couldn't connect");

    let (tx, rx) = tokio::sync::mpsc::channel(8);

    let out_stream = wrappers::ReceiverStream::new(rx);

    let response = client.chat(out_stream).await.expect("couldn't chat");

    let mut in_stream = response.into_inner();

    tokio::spawn(async move {
        while let Some(result) = in_stream.next().await {
            let signal = result.expect("Got an error from server");
            println!("\treceived message: `{}`", signal.state);
        }
    });

    let mut input = String::new();

    loop {
        stdin().read_line(&mut input).expect("Couldn't read line");

        let state: bool = input.trim().parse().expect("That is not bool");
        input.clear();

        tx.send(Signal { state }).await.expect("couldn't send");
    }
}

use morser::messenger_client::MessengerClient;
use morser::Signal;
use tokio::io;
use tokio::io::AsyncBufReadExt;
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

    let writer = tokio::spawn(async move {
        while let Some(result) = in_stream.next().await {
            let signal = result.expect("Got an error from server");
            println!("\treceived message: `{}`", signal.state);
        }
    });

    let reader = tokio::spawn(async move {
        loop {
            let stdin = io::stdin();

            let mut reader = io::BufReader::new(stdin);

            let mut buffer = String::new();

            reader.read_line(&mut buffer).await.expect("Couldn't read");

            let input = buffer.trim().parse::<bool>();

            match input {
                Ok(state) => {
                    tx.send(Signal { state }).await.expect("Couldn't send");
                }
                Err(_) => println!("That ain't bool"),
            }
        }
    });

    writer.await.unwrap();
    reader.await.unwrap();

    Ok(())
}

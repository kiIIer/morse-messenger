use crate::morser::messenger_client::MessengerClient;
use crate::morser::Signal;
use std::fmt::Display;
use tokio::io;
use tokio::io::{AsyncBufReadExt, AsyncRead};
use tokio::join;
use tokio::sync::mpsc::Sender;
use tokio_stream::{wrappers, StreamExt};
use tonic::Streaming;

async fn printer<T: Display>(mut stream: Streaming<T>) {
    while let Some(result) = stream.next().await {
        let value = result.expect("Couldn't read from provided stream");
        println!("\treceived message: `{}`", value);
    }
}

async fn reader(tx: Sender<Signal>) {
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
}

pub async fn execute() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = MessengerClient::connect("http://[::1]:50051")
        .await
        .expect("Couldn't connect");

    let (tx, rx) = tokio::sync::mpsc::channel(8);

    let out_stream = wrappers::ReceiverStream::new(rx);
    let response = client.chat(out_stream).await.expect("couldn't chat");

    let in_stream = response.into_inner();

    let printer = tokio::spawn(printer(in_stream));
    let reader = tokio::spawn(reader(tx));

    join!(reader, printer);

    Ok(())
}

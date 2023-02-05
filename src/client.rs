use crate::morser::messenger_client::MessengerClient;
use crate::morser::Signal;
use rdev::{grab, Event, EventType, Key};
use rodio::source::SineWave;
use rodio::{OutputDevices, OutputStream, Sink};
use std::fmt::Display;
use std::sync::mpsc as smpsc;
use std::thread;
use tokio::io;
use tokio::io::{AsyncBufReadExt, AsyncRead};
use tokio::join;
use tokio::sync::mpsc;
use tokio::sync::mpsc::{Receiver, Sender, UnboundedReceiver, UnboundedSender};
use tokio_stream::{wrappers, StreamExt};
use tonic::Streaming;

async fn singer(mut stream: Streaming<Signal>, sink: Sink) {
    while let Some(result) = stream.next().await {
        let value = result.expect("Couldn't read from provided stream");
        if value.state {
            sink.play();
        } else {
            sink.pause();
        }
    }

    ()
}

fn event_listener(tx: UnboundedSender<bool>) {
    let callback = move |event: Event| -> Option<Event> {
        match event.event_type {
            EventType::KeyPress(Key::Space) => {
                tx.send(true).unwrap();
            }
            EventType::KeyRelease(Key::Space) => {
                tx.send(false).unwrap();
            }
            _ => {}
        }

        Some(event)
    };

    if let Err(error) = grab(callback) {
        println!("Error: {:?}", error)
    }
}

async fn change_manager(mut rx: UnboundedReceiver<bool>, tx: Sender<Signal>) {
    let mut state = false;

    while let Some(value) = rx.recv().await {
        if value != state {
            state = !state;
            tx.send(Signal { state }).await.unwrap();
        }
    }
}

pub async fn execute() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = MessengerClient::connect("http://192.168.1.41:50051")
        .await
        .expect("Couldn't connect");

    let (to_server, rx) = mpsc::channel(8);

    let out_stream = wrappers::ReceiverStream::new(rx);
    let response = client.chat(out_stream).await.expect("couldn't chat");

    let in_stream = response.into_inner();

    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();
    sink.pause();
    let source = SineWave::new(440.0);
    sink.append(source);

    let (tx, rx) = mpsc::unbounded_channel();

    let change = tokio::spawn(change_manager(rx, to_server));

    let singer = tokio::spawn(singer(in_stream, sink));
    let event_listener = thread::spawn(move || event_listener(tx));

    change.await?;
    singer.await?;
    event_listener.join().unwrap();

    Ok(())
}
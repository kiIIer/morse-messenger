use crate::morser::Signal;
use futures_core::Stream;
use morser::messenger_server;
use std::error::Error;
use std::io::ErrorKind;
use std::net::ToSocketAddrs;
use std::pin::Pin;
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tokio_stream::StreamExt;
use tonic::transport::Server;
use tonic::{Request, Response, Status, Streaming};

pub mod morser {
    tonic::include_proto!("morser");
}

fn match_for_io_error(err_status: &Status) -> Option<&std::io::Error> {
    let mut err: &(dyn Error + 'static) = err_status;

    loop {
        if let Some(io_err) = err.downcast_ref::<std::io::Error>() {
            return Some(io_err);
        }

        // h2::Error do not expose std::io::Error with `source()`
        // https://github.com/hyperium/h2/pull/462
        if let Some(h2_err) = err.downcast_ref::<h2::Error>() {
            if let Some(io_err) = h2_err.get_io() {
                return Some(io_err);
            }
        }

        err = match err.source() {
            Some(err) => err,
            None => return None,
        };
    }
}

#[derive(Default)]
struct MessengerService {}

#[tonic::async_trait]
impl messenger_server::Messenger for MessengerService {
    type ChatStream = Pin<Box<dyn Stream<Item = Result<Signal, Status>> + Send>>;

    async fn chat(
        &self,
        request: Request<Streaming<Signal>>,
    ) -> Result<Response<Self::ChatStream>, Status> {
        let mut in_stream = request.into_inner();

        let (tx, rx) = mpsc::channel(128);

        tokio::spawn(async move {
            while let Some(result) = in_stream.next().await {
                match result {
                    Ok(v) => {
                        println!("Got {}", v.state);
                        tx.send(Ok(Signal { state: v.state }))
                            .await
                            .expect("working rx")
                    }
                    Err(err) => {
                        if let Some(io_err) = match_for_io_error(&err) {
                            if io_err.kind() == ErrorKind::BrokenPipe {
                                // here you can handle special case when client
                                // disconnected in unexpected way
                                eprintln!("\tclient disconnected: broken pipe");
                                break;
                            }
                        }

                        match tx.send(Err(err)).await {
                            Ok(_) => (),
                            Err(_err) => break,
                        }
                    }
                }
            }
            println!("\tstream ended");
        });

        let out_stream = ReceiverStream::new(rx);

        Ok(Response::new(Box::pin(out_stream) as Self::ChatStream))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let service = MessengerService::default();
    Server::builder()
        .add_service(morser::messenger_server::MessengerServer::new(service))
        .serve("[::1]:50051".to_socket_addrs().unwrap().next().unwrap())
        .await
        .unwrap();

    Ok(())
}

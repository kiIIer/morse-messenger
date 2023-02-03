use crate::morser::Signal;
use futures_core::Stream;
use morser::messenger_server;
use std::error::Error;
use std::net::ToSocketAddrs;
use std::pin::Pin;
use tokio::sync::broadcast;
use tokio_stream::StreamExt;
use tonic::transport::Server;
use tonic::{Request, Response, Status, Streaming};

type Message = Result<Signal, Status>;

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

pub struct MessengerServer {
    tx: broadcast::Sender<Message>,
}

impl Default for MessengerServer {
    fn default() -> Self {
        let (tx, _) = broadcast::channel(16);
        MessengerServer { tx }
    }
}

#[tonic::async_trait]
impl messenger_server::Messenger for MessengerServer {
    type ChatStream = Pin<Box<dyn Stream<Item = Result<Signal, Status>> + Send>>;

    async fn chat(
        &self,
        request: Request<Streaming<Signal>>,
    ) -> Result<Response<Self::ChatStream>, Status> {
        let mut in_stream = request.into_inner();

        let tx = self.tx.clone();
        let rx = tx.subscribe();
        tokio::spawn(async move {
            while let Some(result) = in_stream.next().await {
                match result {
                    Ok(v) => {
                        tx.send(Ok(v)).expect("Couldn't send");
                    }
                    Err(err) => {
                        if let Some(io_err) = match_for_io_error(&err) {
                            eprintln!("{:?}", io_err);
                            eprintln!("\tclient disconnected: broken pipe");
                        }

                        match tx.send(Err(err)) {
                            Ok(_) => (),
                            Err(_) => break,
                        }
                    }
                }
            }
            println!("\tstream ended");
        });
        let out_stream = tokio_stream::wrappers::BroadcastStream::from(rx).map(|x| x.unwrap());
        Ok(Response::new(Box::pin(out_stream) as Self::ChatStream))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let server = MessengerServer::default();
    Server::builder()
        .add_service(morser::messenger_server::MessengerServer::new(server))
        .serve("[::1]:50051".to_socket_addrs().unwrap().next().unwrap())
        .await
        .unwrap();
    Ok(())
}

use crate::morser::{messenger_server, Signal};
use crate::server::utils::match_for_io_error;
use futures_core::Stream;
use std::pin::Pin;
use tokio::sync::broadcast;
use tokio_stream::StreamExt;
use tonic::{Request, Response, Status, Streaming};

type Message = Result<Signal, Status>;

pub struct MyMessengerServer {
    tx: broadcast::Sender<Message>,
}

impl Default for MyMessengerServer {
    fn default() -> Self {
        let (tx, _) = broadcast::channel(16);
        MyMessengerServer { tx }
    }
}

#[tonic::async_trait]
impl messenger_server::Messenger for MyMessengerServer {
    type ChatStream = Pin<Box<dyn Stream<Item = Message> + Send>>;

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
                        println!("{:?}", v);
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

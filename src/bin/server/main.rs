use morser::{
    messenger_server::{Messenger, MessengerServer},
    Signal,
};
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tonic::{transport::Server, Request, Response, Status, Streaming};

pub mod morser {
    tonic::include_proto!("morser");
}

#[derive(Debug, Default)]
pub struct MorserService {}

#[tonic::async_trait]
impl Messenger for MorserService {
    type ChatStream = ReceiverStream<Result<Signal, Status>>;

    async fn chat(
        &self,
        request: Request<Streaming<Signal>>,
    ) -> Result<Response<Self::ChatStream>, Status> {
        let mut in_stream = request.into_inner();

        let (tx, rx) = mpsc::channel(5);

        tokio::spawn(async move {
            while let Some(sig) = in_stream.message().await.unwrap() {
                println!("Got {}", sig.state);
                tx.send(Ok(sig))
                    .await
                    .expect("The sending to client failed");
            }
        });

        Ok(Response::new(ReceiverStream::new(rx)))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let address = "[::1]:50051".parse().unwrap();
    let morser_service = MorserService::default();

    Server::builder()
        .add_service(MessengerServer::new(morser_service))
        .serve(address)
        .await?;

    Ok(())
}

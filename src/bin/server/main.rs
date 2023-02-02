use morser::{
    messenger_server::{Messenger, MessengerServer},
    Signal,
};
use tonic::{transport::Server, Request, Response, Status};

pub mod morser {
    tonic::include_proto!("morser");
}

#[derive(Debug, Default)]
pub struct MorserService {}

#[tonic::async_trait]
impl Messenger for MorserService {
    async fn chat(&self, request: Request<Signal>) -> Result<Response<Signal>, Status> {
        let r = request.into_inner();
        let state = r.state;
        println!("{}", state);
        Ok(Response::new(morser::Signal { state }))
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

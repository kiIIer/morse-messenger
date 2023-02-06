use crate::morser::messenger_server::MessengerServer;
use crate::server::my_messenger_server::MyMessengerServer;
use std::net::ToSocketAddrs;
use tonic::transport::Server;

mod my_messenger_server;
mod utils;

pub async fn execute() -> Result<(), Box<dyn std::error::Error>> {
    let server = MyMessengerServer::default();
    Server::builder()
        .add_service(MessengerServer::new(server))
        .serve("192.168.1.41:50051".to_socket_addrs().unwrap().next().unwrap())
        .await
        .unwrap();
    Ok(())
}

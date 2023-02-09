use crate::morser::messenger_server::MessengerServer;
use crate::server::my_messenger_server::MyMessengerServer;
use std::env;
use std::net::ToSocketAddrs;
use tonic::transport::Server;

mod my_messenger_server;
mod utils;

pub async fn execute() -> Result<(), Box<dyn std::error::Error>> {
    let mut args: Vec<String> = env::args().collect();
    if args.is_empty() {
        args.push("192.168.1.41:50051".to_string());
    }
    let server = MyMessengerServer::default();
    Server::builder()
        .add_service(MessengerServer::new(server))
        .serve(args[0].to_socket_addrs().unwrap().next().unwrap())
        .await
        .unwrap();
    Ok(())
}

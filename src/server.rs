use crate::{
    morser::messenger_server::MessengerServer, server::my_messenger_server::MyMessengerServer,
};
use std::{env, net::ToSocketAddrs};
use tonic::transport::Server;

mod my_messenger_server;
mod utils;

pub async fn execute() -> Result<(), Box<dyn std::error::Error>> {
    let mut args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        args.push("192.168.1.41:50051".to_string());
    }
    let server = MyMessengerServer::default();
    Server::builder()
        .add_service(MessengerServer::new(server))
        .serve(args[1].to_socket_addrs().unwrap().next().unwrap())
        .await
        .unwrap();
    Ok(())
}

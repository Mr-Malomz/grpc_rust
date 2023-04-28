use std::net::SocketAddr;

use service::{user::user_service_server::UserServiceServer, User};
use tonic::transport::Server;

mod mongo_connection;
mod service;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let address: SocketAddr = "[::1]:8080".parse().unwrap();
    let user = User::default();

    Server::builder()
        .add_service(UserServiceServer::new(user))
        .serve(address)
        .await?;

    Ok(())
}

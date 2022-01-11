use std::env;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use tonic::transport::Server;

pub mod server;

use server::tfplugin5_proto::provider_server::ProviderServer;

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Terraform Provider Rust");
    for (n, v) in env::vars() {
        println!("{}: {}\t", n, v);
    }

    // let server = server::MyProvider::default();
    // let socket = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 1234);

    // Server::builder()
    //     .add_service(ProviderServer::new(server))
    //     .serve(socket)
    //     .await?;

    Ok(())
}

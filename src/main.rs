use tonic::{transport::Server, Request, Response, Status};

use tfplugin5::provider_server::{Provider, ProviderServer};
use tfplugin5::get_provider_schema::Request as GetProviderSchemaRequest;
use tfplugin5::get_provider_schema::Response as GetProviderSchemaResponse;


pub mod tfplugin5 {
    tonic::include_proto!("tfplugin5"); // The string specified here must match the proto package name
}

#[derive(Debug, Default)]
pub struct MyProvider {}

#[tonic::async_trait]
impl Provider for MyProvider {
    async fn get_schema(
        &self, 
        request: Request<GetProviderSchemaRequest>,
    ) -> Result<Response<GetProviderSchemaResponse>, Status>{
        Ok()
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
        println!("Hello, world!");
        Ok()
}

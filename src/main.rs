use tonic::{transport::Server, Request, Response, Status};
use std::collections::HashMap;

// Import the generated rust code into module
pub mod tfplugin5_proto {
    tonic::include_proto!("tfplugin5");
}

use tfplugin5_proto::provider_server::{Provider, ProviderServer};

#[derive(Default)]
pub struct MyProvider {}

#[tonic::async_trait]
impl Provider for MyProvider {
    async fn get_schema(
             &self,
             request: tonic::Request<tfplugin5_proto::get_provider_schema::Request>,
         ) -> Result<tonic::Response<tfplugin5_proto::get_provider_schema::Response>, tonic::Status> {
        let resp = tfplugin5_proto::get_provider_schema::Response{
            provider: Option::None,
            resource_schemas: HashMap<String, tfplugin5_proto::Schema>::new();
        };
        Ok(Response::new(resp))
    }
    async fn prepare_provider_config(
             &self,
             request: tonic::Request<tfplugin5_proto::prepare_provider_config::Request>,
         ) -> Result<tonic::Response<tfplugin5_proto::prepare_provider_config::Response>, tonic::Status> {
    }

}

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello world");
    Ok(())
}
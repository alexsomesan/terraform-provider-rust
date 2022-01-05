use std::collections::HashMap;
use tonic::Response;

// Import the generated rust code into module
pub mod tfplugin5_proto {
    tonic::include_proto!("tfplugin5");
}

use tfplugin5_proto::provider_server::Provider;

#[derive(Default)]
pub struct MyProvider {}

use std::env;

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    print!("Terraform Provider Rust: ");
    for (n, v) in env::vars() {
        print!("{}: {}\t", n, v);
    }
    println!("Over and out");
    Ok(())
}

#[tonic::async_trait]
impl Provider for MyProvider {
    async fn get_schema(
        &self,
        _request: tonic::Request<tfplugin5_proto::get_provider_schema::Request>,
    ) -> Result<tonic::Response<tfplugin5_proto::get_provider_schema::Response>, tonic::Status>
    {
        let resp = tfplugin5_proto::get_provider_schema::Response {
            provider: Option::None,
            provider_meta: Option::None,
            diagnostics: Vec::new(),
            data_source_schemas: HashMap::new(),
            resource_schemas: HashMap::new(),
        };
        Ok(Response::new(resp))
    }

    async fn plan_resource_change(
        &self,
        _request: tonic::Request<tfplugin5_proto::plan_resource_change::Request>,
    ) -> Result<tonic::Response<tfplugin5_proto::plan_resource_change::Response>, tonic::Status>
    {
        Err(tonic::Status::unimplemented("not yet implemented"))
    }

    async fn apply_resource_change(
        &self,
        _request: tonic::Request<tfplugin5_proto::apply_resource_change::Request>,
    ) -> Result<tonic::Response<tfplugin5_proto::apply_resource_change::Response>, tonic::Status>
    {
        Err(tonic::Status::unimplemented("not yet implemented"))
    }

    async fn prepare_provider_config(
        &self,
        _request: tonic::Request<tfplugin5_proto::prepare_provider_config::Request>,
    ) -> Result<tonic::Response<tfplugin5_proto::prepare_provider_config::Response>, tonic::Status>
    {
        Err(tonic::Status::unimplemented("not yet implemented"))
    }

    async fn validate_resource_type_config(
        &self,
        _request: tonic::Request<tfplugin5_proto::validate_resource_type_config::Request>,
    ) -> Result<
        tonic::Response<tfplugin5_proto::validate_resource_type_config::Response>,
        tonic::Status,
    > {
        Err(tonic::Status::unimplemented("not yet implemented"))
    }

    async fn validate_data_source_config(
        &self,
        _request: tonic::Request<tfplugin5_proto::validate_data_source_config::Request>,
    ) -> Result<
        tonic::Response<tfplugin5_proto::validate_data_source_config::Response>,
        tonic::Status,
    > {
        Err(tonic::Status::unimplemented("not yet implemented"))
    }

    async fn upgrade_resource_state(
        &self,
        _request: tonic::Request<tfplugin5_proto::upgrade_resource_state::Request>,
    ) -> Result<tonic::Response<tfplugin5_proto::upgrade_resource_state::Response>, tonic::Status>
    {
        Err(tonic::Status::unimplemented("not yet implemented"))
    }

    async fn configure(
        &self,
        _request: tonic::Request<tfplugin5_proto::configure::Request>,
    ) -> Result<tonic::Response<tfplugin5_proto::configure::Response>, tonic::Status> {
        Err(tonic::Status::unimplemented("not yet implemented"))
    }

    async fn read_resource(
        &self,
        _request: tonic::Request<tfplugin5_proto::read_resource::Request>,
    ) -> Result<tonic::Response<tfplugin5_proto::read_resource::Response>, tonic::Status> {
        Err(tonic::Status::unimplemented("not yet implemented"))
    }

    async fn import_resource_state(
        &self,
        _request: tonic::Request<tfplugin5_proto::import_resource_state::Request>,
    ) -> Result<tonic::Response<tfplugin5_proto::import_resource_state::Response>, tonic::Status>
    {
        Err(tonic::Status::unimplemented("not yet implemented"))
    }

    async fn read_data_source(
        &self,
        _request: tonic::Request<tfplugin5_proto::read_data_source::Request>,
    ) -> Result<tonic::Response<tfplugin5_proto::read_data_source::Response>, tonic::Status> {
        Err(tonic::Status::unimplemented("not yet implemented"))
    }

    async fn stop(
        &self,
        _request: tonic::Request<tfplugin5_proto::stop::Request>,
    ) -> Result<tonic::Response<tfplugin5_proto::stop::Response>, tonic::Status> {
        Err(tonic::Status::unimplemented("not yet implemented"))
    }
}

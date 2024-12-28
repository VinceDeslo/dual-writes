use dotenv;
use tracing::info;
use tonic::{transport::Server, Request, Response, Status};
use tonic_reflection::server::Builder as ReflectionBuilder;

mod config;
use crate::config::{GrpcServerConfig, NatsConfig};

mod nats;
use crate::nats::NatsPublisher;

pub mod analytics {
    tonic::include_proto!("analytics");

    // only necessary for reflection
    pub const FILE_DESCRIPTOR_SET: &[u8]= include_bytes!("../proto/analytics-descriptor.pb");
}

use analytics::{AnalyticsRequest, AnalyticsResponse};
use analytics::analytics_server::{Analytics, AnalyticsServer};

#[derive(Debug, Default)]
pub struct AnalyticsService {}

#[tonic::async_trait]
impl Analytics for AnalyticsService {
    async fn process(
        &self,
        request: Request<AnalyticsRequest>,
    ) -> Result<Response<AnalyticsResponse>, Status> {
        info!("received a process analytics request: {:?}", request);

        let payload = request.into_inner();
        
        info!("processing analytics");
        
        let response = AnalyticsResponse {
            status: 200,
            message: format!("Processed analytics with id: {}", payload.id),
        };

        Ok(Response::new(response))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt::init();

    let nats_config = NatsConfig::from_env()?;

    if nats_config.enabled {
        let _nats_publisher = NatsPublisher::new(nats_config.clone()).await?;
    }

    let grpc_config = GrpcServerConfig::from_env()?;
    let grpc_socket_addr = grpc_config.socket_addr();

    let service = AnalyticsService::default();
    let server = AnalyticsServer::new(service);

    let reflection = ReflectionBuilder::configure()
        .register_encoded_file_descriptor_set(analytics::FILE_DESCRIPTOR_SET)
        .build_v1()?;

    info!("Server configuration: {:?}", grpc_config);
    info!("Server listening on {}", grpc_socket_addr);
    info!("Reflection enabled: {}", grpc_config.enable_reflection);

    if grpc_config.enable_reflection {
        Server::builder()
            .add_service(server)
            .add_service(reflection)
            .serve(grpc_socket_addr)
            .await?;
    } else {
        Server::builder()
            .add_service(server)
            .serve(grpc_socket_addr)
            .await?;
    }

    Ok(())
}

use dotenv;
use tracing::info;
use tonic::transport::Server;
use tonic_reflection::server::Builder as ReflectionBuilder;

mod config;
use crate::config::{GrpcServerConfig, NatsConfig};

mod nats;
use crate::nats::NatsPublisher;

mod service;
use crate::service::AnalyticsService;

pub mod analytics {
    tonic::include_proto!("analytics");

    // only necessary for reflection
    pub const FILE_DESCRIPTOR_SET: &[u8]= include_bytes!("../proto/analytics-descriptor.pb");
}
use analytics::analytics_server::AnalyticsServer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt::init();

    let nats_config = NatsConfig::from_env()?;
    let mut nats_publisher = None;
    if nats_config.enabled {
        nats_publisher = Some(
            NatsPublisher::new(nats_config.clone()).await?
        );
    }

    let grpc_config = GrpcServerConfig::from_env()?;
    let grpc_socket_addr = grpc_config.socket_addr();

    let service = AnalyticsService::new(nats_publisher);
    let server = AnalyticsServer::new(service);

    let reflection = ReflectionBuilder::configure()
        .register_encoded_file_descriptor_set(analytics::FILE_DESCRIPTOR_SET)
        .build_v1()?;

    info!("Server configuration: {:?}", grpc_config);
    info!("Server listening on {}", grpc_socket_addr);

    if grpc_config.enable_reflection {
        info!("Reflection enabled");
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

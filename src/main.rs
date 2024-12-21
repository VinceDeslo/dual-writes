use tracing::info;
use tonic::{transport::Server, Request, Response, Status};
use tonic_reflection::server::Builder as ReflectionBuilder;

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
    tracing_subscriber::fmt::init();

    let addr = "127.0.0.1:50051".parse()?;
    let service = AnalyticsService::default();
    let server = AnalyticsServer::new(service);
    let reflection = ReflectionBuilder::configure()
        .register_encoded_file_descriptor_set(analytics::FILE_DESCRIPTOR_SET)
        .build_v1()?;

    info!("Server listening on {}", addr);

    Server::builder()
        .add_service(server)
        .add_service(reflection)
        .serve(addr)
        .await?;

    Ok(())
}

use tracing::{info, error};
use tonic::{Request, Response, Status};

use crate::analytics::{AnalyticsRequest, AnalyticsResponse};
use crate::analytics::analytics_server::Analytics;
use crate::nats::{NatsPublisher, ANALYTICS_SUBJECT};

#[derive(Debug, Default)]
pub struct AnalyticsService {
    nats_publisher: Option<NatsPublisher>,
}

impl AnalyticsService {
    pub fn new(nats_publisher: Option<NatsPublisher>) -> Self {
        Self { nats_publisher }
    }
}

#[tonic::async_trait]
impl Analytics for AnalyticsService {
    async fn process(
        &self,
        request: Request<AnalyticsRequest>,
    ) -> Result<Response<AnalyticsResponse>, Status> {
        info!("received a process analytics request: {:?}", request);

        let payload = request.into_inner();
        
        info!("processing analytics");

        // TODO: Convert payload to internal format
        let analytics_data = "placeholder".to_string();

        match &self.nats_publisher {
            Some(nats_publisher) => {
                let result= nats_publisher
                    .publish(ANALYTICS_SUBJECT, &analytics_data)
                    .await
                    .map_err(|e| Status::internal(e.to_string()));

                if result.is_err() {
                    let error_message = "failed to publish analytics data";
                    error!("{}", error_message);
                    return Err(Status::internal(error_message));
                }
            }
            None => {
                let error_message = "publisher is not initialized";
                error!("{}", error_message);
                return Err(Status::internal(error_message));
            }
        }
        
        let response = AnalyticsResponse {
            status: 200,
            message: format!("Processed analytics with id: {}", payload.id),
        };

        Ok(Response::new(response))
    }
}
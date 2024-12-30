use tracing::{info, error};
use tonic::{Request, Response, Status};
use cloudevents::{AttributesReader, Event, EventBuilder, EventBuilderV10};
use crate::analytics::{AnalyticsRequest, AnalyticsResponse, AnalyticsData};
use crate::analytics::analytics_server::Analytics;
use crate::nats::{NatsPublisher, ANALYTICS_SUBJECT};
use crate::store::StoredAnalytics;

#[derive(Debug, Default)]
pub struct AnalyticsService {
    nats_publisher: Option<NatsPublisher>,
}

#[tonic::async_trait]
impl Analytics for AnalyticsService {
    async fn process(
        &self,
        request: Request<AnalyticsRequest>,
    ) -> Result<Response<AnalyticsResponse>, Status> {
        info!("received a process analytics request: {:?}", request);

        let payload = request.into_inner();
        let analytics_data: AnalyticsData;

        match payload.data {
            Some(data) => analytics_data = data,
            None => {
                let error_message = "analytics data is empty";
                error!("{}", error_message);
                return Err(Status::invalid_argument(error_message));
            }
        }
        
        let stored_analytics = self.store_analytics(&analytics_data).await?;
        let event = self.build_event(&payload.id, &stored_analytics).await?;
        let result = self.emit_event(&event).await;

        // rollback the insert for data consistency
        // this should probably be done in a transaction with an outbox instead
        // but this is a simple example so we'll just do it here
        if result.is_err() {
            self.delete_analytics(&stored_analytics.id).await?;
            error!("failed to emit analytics event with id: {}", stored_analytics.id);
            return Err(Status::data_loss(result.unwrap_err().to_string()));
        }

        let response = AnalyticsResponse {
            status: 200,
            message: format!("processed analytics with id: {}", payload.id),
        };

        Ok(Response::new(response))
    }
}

impl AnalyticsService {
    pub fn new(nats_publisher: Option<NatsPublisher>) -> Self {
        Self { nats_publisher }
    }

    async fn store_analytics(
        &self,
        analytics_data: &AnalyticsData,
    ) -> Result<StoredAnalytics, Status> {
        let stored_analytics = StoredAnalytics::from_analytics_data(analytics_data);
        info!("stored analytics with id: {}", stored_analytics.id);

        return Ok(stored_analytics);
    }

    async fn delete_analytics(
        &self,
        id: &str,
    ) -> Result<(), Status> {
        info!("deleted analytics with id: {}", id);
        return Ok(());
    }

    async fn build_event(
        &self,
        request_id: &str,
        stored_analytics: &StoredAnalytics,
    ) -> Result<Event, Status> {
        info!("building analytics event for request id: {}", request_id);

        let serialized_data = serde_json::to_string(&stored_analytics)
            .map_err(|e| Status::internal(e.to_string()))?;

        let event = EventBuilderV10::new()
            .id(stored_analytics.id.clone())
            .source("analytics-service.localdev.svc.cluster.local")
            .ty("analytics.processed")
            .data("application/json", serialized_data)
            .build()
            .map_err(|e| Status::internal(e.to_string()))?;

        return Ok(event);
    }

    async fn emit_event(
        &self,
        analytics_event: &Event,
    ) -> Result<(), Status> {
        match &self.nats_publisher {
            Some(nats_publisher) => {
                let result= nats_publisher
                    .publish(ANALYTICS_SUBJECT, analytics_event)
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

        info!("emitted analytics event with id: {}", analytics_event.id());
        Ok(())
    }
}

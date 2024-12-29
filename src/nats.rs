use tracing::info;
use async_nats::Client;
use serde::Serialize;
use std::error::Error;
use crate::config::NatsConfig;

pub const ANALYTICS_SUBJECT: &str = "events.analytics";

#[derive(Debug)]
pub struct NatsPublisher {
    client: Client,
}

impl NatsPublisher {
    pub async fn new(config: NatsConfig) -> Result<Self, Box<dyn Error>> {
        info!("NATS configuration: {:?}", config);
        info!("Connecting to NATS server at {}", config.url());
        let client = async_nats::connect(config.url()).await?;
        Ok(Self { client })
    }

    pub async fn publish<T: Serialize>(
        &self,
        subject: &str,
        payload: &T,
    ) -> Result<(), Box<dyn Error>> {
        let message = serde_json::to_vec(payload)?;
        self.client.publish(
            subject.to_string(), 
            message.into()
        ).await?;
        Ok(())
    }
}
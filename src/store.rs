use uuid::Uuid;
use std::time::Duration;
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

use crate::analytics::AnalyticsData;

#[derive(Debug, Serialize, Deserialize)]
pub struct StoredAnalytics {
    pub id: String,
    pub file_count: i32,
    pub vulns_count: i32,
    pub fixable_count: i32,
    pub scan_time: DateTime<Utc>,
    pub scan_duration: Duration,
}

// Internal representation of the analytics data
impl StoredAnalytics {
    pub fn from_analytics_data(data: &AnalyticsData) -> Self {
        let scan_time = data.scan_time.as_ref()
            .map(|t| DateTime::<Utc>::from_timestamp(
                t.seconds,
                t.nanos as u32,
            ))
            .flatten()
            .unwrap_or_default();

        let scan_duration = data.scan_duration.as_ref()
            .map(|d| Duration::new(
                d.seconds as u64,
                d.nanos as u32,
            ))
            .unwrap_or_default();

        Self {
            id: Uuid::new_v4().to_string(),
            file_count: data.file_count,
            vulns_count: data.vulns_count,
            fixable_count: data.fixable_count,
            scan_time: scan_time,
            scan_duration: scan_duration,
        }
    }
}

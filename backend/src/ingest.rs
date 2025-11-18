use crate::models::LogEvent;
use chrono::{DateTime, Utc};
use uuid::Uuid;
use anyhow::Result;

pub fn ingest_file(_path: &str) -> Result<Vec<LogEvent>> {
    // fake event for proof of life
    let mock = LogEvent {
        id: Uuid::new_v4(),
        timestamp: Utc::now(),
        service: "example".to_string(),
        level: "INFO".to_string(),
        message: "Stub log event".to_string(),
        fields: Default::default(),
    };

    Ok(vec![mock])
}
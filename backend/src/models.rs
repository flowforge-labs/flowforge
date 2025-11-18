use serde::{Serialize, Deserialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogEvent {
    pub id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub service: String,
    pub level: String, 
    pub message: String,
    pub fields: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Flow {
    pub id: Uuid,
    pub events: Vec<Uuid>,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub correlation_type: String,
}


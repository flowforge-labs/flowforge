use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid;
pub mod models;

pub fn ping() -> &'static str {
    "pong from core"
}

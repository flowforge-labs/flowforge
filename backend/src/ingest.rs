use crate::models::LogEvent;
use anyhow::{Result, anyhow};
use chrono::{DateTime, Utc};
use serde_json::Value;
use std::fs::File;
use std::io::{BufRead, BufReader};
use uuid::Uuid;

pub fn ingest_file(path: &str) -> Result<Vec<LogEvent>> {
    // fake event for proof of life
    // TODO: switch to fs::read_to_string
    let file = File::open(path).map_err(|e| anyhow!("Failed to open {}:, {}", path, e))?;

    let reader = BufReader::new(file);

    let mut events = Vec::new();

    for line in reader.lines() {
        let line = line?;

        // skip empty lines
        if (line.trim().is_empty()) {
            continue;
        }

        if line.trim_start().starts_with('{') {
            if let Ok(event) = parse_json_line(&line) {
                events.push(event);
                continue;
            }
        }

        let event = parse_plain_line(&line);
        events.push(event);
    }

    Ok(events)
}

fn parse_json_line(line: &str) -> Result<LogEvent> {
    let line_as_json: Value = serde_json::from_str(line)?;

    Ok(LogEvent {
        id: Uuid::new_v4(),
        timestamp: Utc::now(), // TODO: extract timestamp from v
        service: line_as_json
            .get("service")
            .and_then(|v| v.as_str())
            .unwrap_or("unknown")
            .to_string(),
        level: line_as_json
            .get("level")
            .and_then(|v| v.as_str())
            .unwrap_or("INFO")
            .to_string(),
        message: line_as_json
            .get("message")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string(),
        fields: extract_fields(&line_as_json),
    })
}

fn extract_fields(json: &Value) -> std::collections::HashMap<String, String> {
    let mut map = std::collections::HashMap::new();

    if let Some(obj) = json.as_object() {
        for (k, v) in obj {
            // skip used fields

            if ["service", "level", "message", "timestamp"].contains(&k.as_str()) {
                continue;
            }
            map.insert(k.clone(), v.to_string());
        }
    }
    map
}

fn parse_plain_line(line: &str) -> LogEvent {
    LogEvent {
        id: Uuid::new_v4(),
        timestamp: Utc::now(), // TODO: extract from line
        service: "unknown".into(),
        level: "INFO".into(),
        message: line.to_string(),
        fields: Default::default(),
    }
}

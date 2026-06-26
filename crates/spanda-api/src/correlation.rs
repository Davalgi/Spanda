//! Correlation ID generation and request tracing for Control Center APIs.
//!
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::time::{SystemTime, UNIX_EPOCH};

/// One traced API request for observability export.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TraceRecord {
    pub correlation_id: String,
    pub method: String,
    pub path: String,
    pub status: u16,
    pub timestamp_ms: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_ms: Option<f64>,
}

/// Ring buffer of recent API traces.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TraceLog {
    records: VecDeque<TraceRecord>,
    pub max_entries: usize,
}

impl TraceLog {
    pub fn new(max_entries: usize) -> Self {
        Self {
            records: VecDeque::new(),
            max_entries,
        }
    }

    pub fn push(&mut self, record: TraceRecord) {
        if self.records.len() >= self.max_entries {
            self.records.pop_front();
        }
        self.records.push_back(record);
    }

    pub fn list(&self) -> Vec<&TraceRecord> {
        self.records.iter().collect()
    }

    pub fn list_owned(&self) -> Vec<TraceRecord> {
        self.records.iter().cloned().collect()
    }

    pub fn from_records(max_entries: usize, records: Vec<TraceRecord>) -> Self {
        let mut log = Self::new(max_entries);
        for record in records {
            log.push(record);
        }
        log
    }
}

pub fn new_correlation_id() -> String {
    let ms = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis())
        .unwrap_or(0);
    format!("spanda-{ms}-{:04x}", rand_u16())
}

pub fn correlation_from_headers(raw: &str) -> Option<String> {
    for line in raw.lines() {
        if let Some(value) = line
            .strip_prefix("X-Correlation-ID:")
            .or_else(|| line.strip_prefix("x-correlation-id:"))
        {
            let trimmed = value.trim();
            if !trimmed.is_empty() {
                return Some(trimmed.to_string());
            }
        }
    }
    None
}

fn rand_u16() -> u16 {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.subsec_nanos())
        .unwrap_or(0);
    (nanos & 0xFFFF) as u16
}

pub fn now_ms() -> f64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs_f64() * 1000.0)
        .unwrap_or(0.0)
}

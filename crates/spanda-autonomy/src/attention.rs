//! Attention system — event prioritization and suppression.
//!
use crate::types::AutonomySeverity;
use serde::{Deserialize, Serialize};

/// Priority tier for events and signals.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EventPriority {
    Routine = 0,
    Elevated = 1,
    Important = 2,
    Urgent = 3,
    Critical = 4,
}

/// Signal priority within an attention window.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SignalPriority {
    pub signal: String,
    pub priority: EventPriority,
    pub severity: AutonomySeverity,
}

/// Rule to suppress repeated low-value alerts.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SuppressionRule {
    pub pattern: String,
    pub max_repeats: u32,
    pub window_secs: u64,
}

/// Attention policy configuration.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct AttentionPolicy {
    pub critical_first: bool,
    pub suppress_routine_when_critical: bool,
    pub suppression_rules: Vec<SuppressionRule>,
}

/// Sliding attention window for ranked events.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AttentionWindow {
    pub max_items: usize,
    pub items: Vec<AttentionScore>,
}

/// Computed attention score for an event.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AttentionScore {
    pub event_id: String,
    pub label: String,
    pub score: f64,
    pub priority: EventPriority,
    pub suppressed: bool,
}

/// Compute attention score from priority and severity.
pub fn compute_attention_score(
    event_id: &str,
    label: &str,
    priority: EventPriority,
    severity: AutonomySeverity,
) -> AttentionScore {
    let severity_boost = match severity {
        AutonomySeverity::Info => 0.0,
        AutonomySeverity::Low => 0.1,
        AutonomySeverity::Medium => 0.25,
        AutonomySeverity::High => 0.5,
        AutonomySeverity::Critical => 1.0,
    };
    let score = priority as u8 as f64 + severity_boost;
    AttentionScore {
        event_id: event_id.into(),
        label: label.into(),
        score,
        priority,
        suppressed: false,
    }
}

/// Rank events by attention score descending.
pub fn rank_events(mut scores: Vec<AttentionScore>, policy: &AttentionPolicy) -> AttentionWindow {
    scores.sort_by(|a, b| {
        b.score
            .partial_cmp(&a.score)
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    if policy.critical_first {
        scores.sort_by_key(|s| std::cmp::Reverse(s.priority));
    }
    let max = 50;
    AttentionWindow {
        max_items: max,
        items: scores.into_iter().take(max).collect(),
    }
}

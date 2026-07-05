//! Adaptive recovery — rule-based learning from historical outcomes.
//!
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Historical recovery attempt record.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RecoveryHistory {
    pub entity_id: String,
    pub strategy: String,
    pub success: bool,
    pub duration_ms: u64,
}

/// Success rate for a recovery strategy.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StrategySuccessRate {
    pub strategy: String,
    pub attempts: u32,
    pub successes: u32,
    pub rate: f64,
}

/// Preferred strategy based on historical success.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StrategyPreference {
    pub strategy: String,
    pub confidence: f64,
    pub reason: String,
}

/// Recovery confidence score for an entity.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RecoveryConfidence {
    pub entity_id: String,
    pub score: f64,
    pub preferred: Option<StrategyPreference>,
    pub rates: Vec<StrategySuccessRate>,
}

/// Adaptive recovery policy configuration.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct AdaptiveRecoveryPolicy {
    pub min_attempts: u32,
    pub escalate_below_rate: f64,
}

impl AdaptiveRecoveryPolicy {
    pub fn platform_defaults() -> Self {
        Self {
            min_attempts: 3,
            escalate_below_rate: 0.3,
        }
    }
}

/// Compute strategy success rates from history.
pub fn compute_strategy_preference(
    history: &[RecoveryHistory],
    policy: &AdaptiveRecoveryPolicy,
) -> Vec<StrategySuccessRate> {
    let mut counts: HashMap<String, (u32, u32)> = HashMap::new();
    for entry in history {
        let slot = counts.entry(entry.strategy.clone()).or_insert((0, 0));
        slot.0 += 1;
        if entry.success {
            slot.1 += 1;
        }
    }
    counts
        .into_iter()
        .map(|(strategy, (attempts, successes))| {
            let rate = if attempts == 0 {
                0.0
            } else {
                successes as f64 / attempts as f64
            };
            StrategySuccessRate {
                strategy,
                attempts,
                successes,
                rate,
            }
        })
        .filter(|r| r.attempts >= policy.min_attempts.max(1))
        .collect()
}

/// Compute recovery confidence for an entity from history.
pub fn compute_recovery_confidence(
    entity_id: &str,
    history: &[RecoveryHistory],
    policy: &AdaptiveRecoveryPolicy,
) -> RecoveryConfidence {
    let entity_history: Vec<_> = history
        .iter()
        .filter(|h| h.entity_id == entity_id)
        .cloned()
        .collect();
    let rates = compute_strategy_preference(&entity_history, policy);
    let preferred = rates
        .iter()
        .max_by(|a, b| {
            a.rate
                .partial_cmp(&b.rate)
                .unwrap_or(std::cmp::Ordering::Equal)
        })
        .map(|best| StrategyPreference {
            strategy: best.strategy.clone(),
            confidence: best.rate,
            reason: if best.rate < policy.escalate_below_rate {
                "Low success rate — escalate sooner".into()
            } else {
                "Historical success supports preference".into()
            },
        });
    let score = preferred.as_ref().map(|p| p.confidence).unwrap_or(0.5);
    RecoveryConfidence {
        entity_id: entity_id.into(),
        score,
        preferred,
        rates,
    }
}

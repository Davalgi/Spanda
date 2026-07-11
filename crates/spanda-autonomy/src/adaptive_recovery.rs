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

/// Mission abort / replan advice derived from recovery strategy preference.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MissionAbortReplanAdvice {
    pub should_abort: bool,
    pub should_replan: bool,
    pub preferred_strategy: Option<String>,
    pub confidence_score: f64,
    pub reasons: Vec<String>,
}

/// Field accuracy thresholds for adaptive recovery Stable claims.
pub mod field_accuracy_thresholds {
    /// Minimum historical attempts before a strategy preference is trusted.
    pub const MIN_ATTEMPTS: u32 = 3;
    /// Success rate below which the planner should escalate / abort sooner.
    pub const ESCALATE_BELOW_RATE: f64 = 0.30;
    /// Preferred strategy must hold at least this success rate over field soak.
    pub const STABLE_MIN_SUCCESS_RATE: f64 = 0.70;
    /// Minimum field soak days before accuracy claims (organizational gate).
    pub const FIELD_SOAK_DAYS: u32 = 30;
}

/// Surface recovery-confidence strategy preference for mission abort/replan.
pub fn mission_abort_replan_from_confidence(
    confidence: &RecoveryConfidence,
    policy: &AdaptiveRecoveryPolicy,
) -> MissionAbortReplanAdvice {
    // Translate adaptive recovery preference into mission planner abort/replan signals.
    //
    // Parameters:
    // - `confidence` — entity recovery confidence snapshot
    // - `policy` — min-attempts and escalate-below-rate thresholds
    //
    // Returns:
    // Abort/replan advice with human-readable reasons.
    //
    // Options:
    // Uses `policy.escalate_below_rate` and overall `confidence.score`.
    //
    // Example:
    // let advice = mission_abort_replan_from_confidence(&rc, &AdaptiveRecoveryPolicy::platform_defaults());

    let mut reasons = Vec::new();
    let preferred_strategy = confidence
        .preferred
        .as_ref()
        .map(|p| p.strategy.clone());

    // Prefer replan when the best strategy is below the escalate threshold.
    let low_preferred = confidence
        .preferred
        .as_ref()
        .is_some_and(|p| p.confidence < policy.escalate_below_rate);
    if low_preferred {
        reasons.push(format!(
            "Preferred recovery strategy '{}' success rate {:.0}% is below escalate threshold {:.0}% — replan before continue",
            preferred_strategy.as_deref().unwrap_or("unknown"),
            confidence.preferred.as_ref().map(|p| p.confidence * 100.0).unwrap_or(0.0),
            policy.escalate_below_rate * 100.0
        ));
    }

    // Abort when overall recovery confidence is critically low with enough history.
    let total_attempts: u32 = confidence.rates.iter().map(|r| r.attempts).sum();
    let should_abort =
        total_attempts >= policy.min_attempts.max(1) && confidence.score < policy.escalate_below_rate;
    if should_abort {
        reasons.push(format!(
            "Recovery confidence {:.0}% is below escalate threshold with {total_attempts} attempts — abort mission",
            confidence.score * 100.0
        ));
    }

    let should_replan = low_preferred || (!should_abort && confidence.preferred.is_some());
    if should_replan && !low_preferred {
        if let Some(ref strategy) = preferred_strategy {
            reasons.push(format!(
                "Mission replan should prefer recovery strategy '{strategy}' (confidence {:.0}%)",
                confidence.score * 100.0
            ));
        }
    }

    MissionAbortReplanAdvice {
        should_abort,
        should_replan,
        preferred_strategy,
        confidence_score: confidence.score,
        reasons,
    }
}


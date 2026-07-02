//! Conflict resolution for competing distributed decisions.

use crate::types::CONFLICT_PRECEDENCE;
use serde::{Deserialize, Serialize};

/// Competing decision from a layer.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CompetingDecision {
    pub layer_precedence: String,
    pub entity_id: String,
    pub action: String,
    pub reason: String,
}

/// Resolved outcome after applying precedence rules.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConflictResolution {
    pub winner: CompetingDecision,
    pub rejected: Vec<CompetingDecision>,
    pub precedence_applied: String,
}

/// Resolve conflicts using the global precedence order.
pub fn resolve_conflict(decisions: &[CompetingDecision]) -> Option<ConflictResolution> {
    // Description:
    //     Pick the winning decision by precedence, reject the rest.
    //
    // Parameters:
    // - `decisions` — competing decisions from multiple layers
    //
    // Returns:
    // Resolution with winner and rejected alternatives.
    //
    // Options:
    // None.
    //
    // Example:
    // let resolution = resolve_conflict(&decisions);

    if decisions.is_empty() {
        return None;
    }
    let mut best: Option<&CompetingDecision> = None;
    let mut best_rank = usize::MAX;
    for d in decisions {
        let rank = CONFLICT_PRECEDENCE
            .iter()
            .position(|p| *p == d.layer_precedence)
            .unwrap_or(usize::MAX);
        if rank < best_rank {
            best_rank = rank;
            best = Some(d);
        }
    }
    let winner = best?.clone();
    let rejected: Vec<_> = decisions
        .iter()
        .filter(|d| d.action != winner.action || d.entity_id != winner.entity_id)
        .cloned()
        .collect();
    let precedence_applied = CONFLICT_PRECEDENCE
        .get(best_rank)
        .unwrap_or(&"unknown")
        .to_string();
    Some(ConflictResolution {
        winner,
        rejected,
        precedence_applied,
    })
}

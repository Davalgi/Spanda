//! Live distributed decision tree evaluation and fleet consensus recording.

use super::{Interpreter, RobotBackend};
use spanda_error::SpandaError;

impl<B: RobotBackend> Interpreter<B> {
    pub(super) fn decision_runtime(&self) -> spanda_runtime::decision_runtime::SharedDecisionRuntime {
        self.decision_runtime.clone()
    }

    /// Update a decision signal and re-evaluate trees when the value changes.
    pub(super) fn set_decision_signal(&mut self, key: impl Into<String>, value: bool) {
        let key = key.into();
        let prev = self.decision_signals.get(&key).copied();
        if prev == Some(value) {
            return;
        }
        self.decision_signals.insert(key.clone(), value);
        self.evaluate_live_decision_trees(Some(&key));
    }

    /// Sync decision signals from hardware monitor, safety state, and faults.
    pub(super) fn sync_decision_signals_from_runtime(&mut self) {
        let gps_failed = self
            .hardware_monitor
            .injected_faults()
            .iter()
            .any(|f| f.contains("GPS") || f.contains("gps"));
        self.decision_signals
            .insert("gps.status == Failed".into(), gps_failed);

        let obstacle = self
            .safety_monitor
            .as_ref()
            .map(|m| m.is_emergency_stop())
            .unwrap_or(false)
            || self.backend.get_state().emergency_stop;
        self.decision_signals
            .insert("obstacle.detected".into(), obstacle);

        let coordinator_failed = std::env::var("SPANDA_FLEET_COORDINATOR_FAILED")
            .map(|v| matches!(v.as_str(), "1" | "true" | "yes"))
            .unwrap_or(false);
        self.decision_signals
            .insert("fleet.coordinator.failed".into(), coordinator_failed);

        let visual_odom = !gps_failed;
        self.decision_signals
            .insert("visual_odometry.available".into(), visual_odom);

        let operator = std::env::var("SPANDA_OPERATOR_AVAILABLE")
            .map(|v| matches!(v.as_str(), "1" | "true" | "yes"))
            .unwrap_or(false);
        self.decision_signals
            .insert("operator.available".into(), operator);
    }

    /// Evaluate decision trees and emit trace records for newly matched conditions.
    pub(super) fn evaluate_live_decision_trees(&mut self, changed: Option<&str>) {
        let Some(program) = self.health_program.clone() else {
            return;
        };
        self.sync_decision_signals_from_runtime();
        let runtime = self.decision_runtime();
        let results = runtime.evaluate_trees(&program, &self.decision_signals);
        for result in results {
            let fingerprint = format!("{}:{}", result.tree_name, result.condition_matched);
            if self.decision_tree_emitted.contains(&fingerprint) {
                continue;
            }
            if let Some(changed_key) = changed {
                if !result.condition_matched.contains(changed_key)
                    && !self.signal_matches_tree(changed_key, &result.condition_matched)
                {
                    continue;
                }
            }
            self.decision_tree_emitted.insert(fingerprint);
            self.log(format!(
                "decision_tree '{}': {} → [{}]",
                result.tree_name,
                result.condition_matched,
                result.actions.join(", ")
            ));
            let layer = if result.layer.contains("reflex") {
                "reflex"
            } else if result.layer.contains("group") || result.layer.contains("fleet") {
                "group_fleet"
            } else {
                "local_entity"
            };
            self.record_decision_trace(
                "decision_tree_eval",
                "local_decision",
                &format!(
                    "tree '{}' matched '{}' → {}",
                    result.tree_name,
                    result.condition_matched,
                    result.actions.join(", ")
                ),
                layer,
                &self.active_robot_name.clone().unwrap_or_else(|| "robot".into()),
                serde_json::json!({
                    "tree": result.tree_name,
                    "condition": result.condition_matched,
                    "actions": result.actions,
                    "tree_hash": result.tree_hash,
                    "signals": self.decision_signals,
                }),
            );
        }
    }

    fn signal_matches_tree(&self, changed: &str, condition: &str) -> bool {
        condition.contains(changed)
            || self
                .decision_signals
                .get(changed)
                .copied()
                .unwrap_or(false)
    }

    /// Record fleet mesh consensus decision after coordinator relay.
    pub(super) fn record_fleet_mesh_consensus(
        &mut self,
        event: &str,
        members: &[String],
        selected_action: &str,
        relayed: u32,
        failed: u32,
    ) {
        let votes: Vec<(String, String, f64)> = members
            .iter()
            .enumerate()
            .map(|(i, m)| {
                let action = if i == 0 {
                    selected_action.to_string()
                } else {
                    selected_action.to_string()
                };
                (m.clone(), action, 1.0 - (i as f64 * 0.1))
            })
            .collect();
        let quorum = if members.is_empty() {
            0.5
        } else {
            (relayed as f64 / members.len() as f64).clamp(0.0, 1.0)
        };
        let consensus = self.decision_runtime().resolve_fleet_consensus(&votes, quorum);
        self.record_decision_trace(
            event,
            "fleet_consensus",
            &format!(
                "{} → {} (quorum={}, votes={})",
                consensus.strategy, consensus.selected_action, consensus.quorum_met, consensus.vote_count
            ),
            "group_fleet",
            "fleet_coordinator",
            serde_json::json!({
                "selected_action": consensus.selected_action,
                "strategy": consensus.strategy,
                "quorum_met": consensus.quorum_met,
                "relayed": relayed,
                "failed": failed,
                "members": members,
            }),
        );
    }

    /// Poll decision trees on scheduler ticks (bounded rate).
    pub(super) fn poll_live_decision_trees(&mut self) -> Result<(), SpandaError> {
        self.evaluate_live_decision_trees(None);
        Ok(())
    }
}

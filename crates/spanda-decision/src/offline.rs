//! Offline decision policy support.

use crate::types::{DecisionLayer, DecisionPolicy};
use serde::{Deserialize, Serialize};
use spanda_ast::assurance_decl::OfflinePolicyDecl;
use spanda_ast::nodes::Program;

/// Offline operation policy extracted from program.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OfflinePolicySpec {
    pub name: String,
    pub max_duration_minutes: u32,
    pub allowed_actions: Vec<String>,
    pub forbidden_actions: Vec<String>,
}

/// Extract offline policies from a program.
pub fn extract_offline_policies(program: &Program) -> Vec<OfflinePolicySpec> {
    let Program::Program {
        offline_policies, ..
    } = program;
    offline_policies
        .iter()
        .map(|decl| {
            let OfflinePolicyDecl::OfflinePolicyDecl {
                name,
                max_duration_minutes,
                allowed_actions,
                forbidden_actions,
                ..
            } = decl;
            OfflinePolicySpec {
                name: name.clone(),
                max_duration_minutes: *max_duration_minutes,
                allowed_actions: allowed_actions.clone(),
                forbidden_actions: forbidden_actions.clone(),
            }
        })
        .collect()
}

/// Convert offline policy to a decision policy for the local cache.
pub fn offline_to_decision_policy(spec: &OfflinePolicySpec, version: &str) -> DecisionPolicy {
    DecisionPolicy {
        name: spec.name.clone(),
        version: version.into(),
        layer: DecisionLayer::LocalEntity,
        allowed_actions: spec.allowed_actions.clone(),
        forbidden_actions: spec.forbidden_actions.clone(),
        signature: None,
        expires_at_ms: None,
    }
}

/// Validate an offline action against policy and elapsed offline minutes.
pub fn validate_offline_action(
    spec: &OfflinePolicySpec,
    action: &str,
    offline_minutes: u32,
) -> Result<(), String> {
    // Description:
    //     Check whether an action is permitted while offline.
    //
    // Parameters:
    // - `spec` — offline policy
    // - `action` — proposed action
    // - `offline_minutes` — minutes since last central sync
    //
    // Returns:
    // Ok when permitted, Err with reason when blocked.
    //
    // Options:
    // None.
    //
    // Example:
    // validate_offline_action(&policy, "return_home", 15)?;

    if offline_minutes > spec.max_duration_minutes {
        return Err(format!(
            "offline duration {offline_minutes}m exceeds max {}m for policy '{}'",
            spec.max_duration_minutes, spec.name
        ));
    }
    if spec.forbidden_actions.iter().any(|a| a == action) {
        return Err(format!("action '{action}' forbidden while offline"));
    }
    if !spec.allowed_actions.is_empty() && !spec.allowed_actions.iter().any(|a| a == action) {
        return Err(format!("action '{action}' not in offline allowed list"));
    }
    Ok(())
}

//! Local policy cache for offline and edge operation.

use crate::types::DecisionPolicy;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Cached policy bundle maintained on an edge entity.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct LocalPolicyCache {
    pub entity_id: String,
    pub safety_rules: Vec<String>,
    pub recovery_playbooks: Vec<String>,
    pub mission_constraints: Vec<String>,
    pub trust_policy_version: String,
    pub capability_requirements: Vec<String>,
    pub approval_rules: Vec<String>,
    pub policies: HashMap<String, DecisionPolicy>,
    pub last_sync_ms: f64,
    pub signature: Option<String>,
}

impl LocalPolicyCache {
    /// Build an empty cache for an entity.
    pub fn new(entity_id: impl Into<String>) -> Self {
        Self {
            entity_id: entity_id.into(),
            ..Default::default()
        }
    }

    /// Look up a cached policy by name.
    pub fn get_policy(&self, name: &str) -> Option<&DecisionPolicy> {
        self.policies.get(name)
    }

    /// Insert or update a cached policy.
    pub fn upsert_policy(&mut self, policy: DecisionPolicy) {
        self.policies.insert(policy.name.clone(), policy);
    }
}

/// Populate cache from program-extracted policies.
pub fn build_policy_cache(
    entity_id: &str,
    policies: Vec<DecisionPolicy>,
    safety_rules: Vec<String>,
) -> LocalPolicyCache {
    let mut cache = LocalPolicyCache::new(entity_id);
    cache.safety_rules = safety_rules;
    for p in policies {
        cache.upsert_policy(p);
    }
    cache
}

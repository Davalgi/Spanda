//! Versioned governance policy assignment with signing and audit trail.
//!
use crate::policy::GovernancePolicyRef;
use crate::types::GovernancePolicyKind;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::fs;
use std::path::{Path, PathBuf};

/// Default on-disk policy assignment store path.
pub fn default_policy_store_path() -> PathBuf {
    PathBuf::from("control-center-governance-policies.json")
}

/// Assignment of a governance policy to an entity.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PolicyAssignment {
    pub id: String,
    pub entity_id: String,
    pub policy: GovernancePolicyRef,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub assigned_by: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub assigned_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub audit_id: Option<String>,
}

/// Append-only audit entry for policy mutations.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PolicyAuditEntry {
    pub id: String,
    pub action: String,
    pub assignment_id: String,
    pub entity_id: String,
    pub policy_name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub actor: Option<String>,
    pub at: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
}

/// On-disk policy assignment store.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct PolicyStore {
    #[serde(default)]
    pub assignments: Vec<PolicyAssignment>,
    #[serde(default)]
    pub audit: Vec<PolicyAuditEntry>,
}

impl PolicyStore {
    pub fn load(path: &Path) -> Self {
        fs::read_to_string(path)
            .ok()
            .and_then(|raw| serde_json::from_str(&raw).ok())
            .unwrap_or_default()
    }

    pub fn save(&self, path: &Path) -> Result<(), String> {
        if let Some(parent) = path.parent() {
            let _ = fs::create_dir_all(parent);
        }
        let raw = serde_json::to_string_pretty(self).map_err(|e| e.to_string())?;
        fs::write(path, raw).map_err(|e| e.to_string())
    }

    /// Sign a policy reference with a content hash (package-driven signing material).
    pub fn sign_policy(
        policy: &mut GovernancePolicyRef,
        signed_by: &str,
        material: &str,
    ) -> String {
        let mut hasher = Sha256::new();
        hasher.update(policy.kind.as_str().as_bytes());
        hasher.update(policy.name.as_bytes());
        hasher.update(policy.version.as_deref().unwrap_or("").as_bytes());
        hasher.update(material.as_bytes());
        let digest = hex::encode(hasher.finalize());
        policy.signature = Some(digest.clone());
        policy.signed_by = Some(signed_by.to_string());
        policy.signed_at = Some(chrono_now());
        digest
    }

    /// Assign a signed policy to an entity and append an audit entry.
    pub fn assign(
        &mut self,
        entity_id: &str,
        mut policy: GovernancePolicyRef,
        actor: Option<&str>,
        sign_material: Option<&str>,
    ) -> PolicyAssignment {
        if let Some(material) = sign_material {
            let signer = actor.unwrap_or("system");
            Self::sign_policy(&mut policy, signer, material);
        }
        let id = format!("pol-{}-{}", entity_id, self.assignments.len() + 1);
        let audit_id = format!("audit-{}", id);
        let assignment = PolicyAssignment {
            id: id.clone(),
            entity_id: entity_id.to_string(),
            policy: policy.clone(),
            assigned_by: actor.map(String::from),
            assigned_at: Some(chrono_now()),
            audit_id: Some(audit_id.clone()),
        };
        self.audit.push(PolicyAuditEntry {
            id: audit_id,
            action: "assign".into(),
            assignment_id: id.clone(),
            entity_id: entity_id.to_string(),
            policy_name: policy.name.clone(),
            actor: actor.map(String::from),
            at: chrono_now(),
            detail: Some(format!("kind={}", policy.kind.as_str())),
        });
        self.assignments.push(assignment.clone());
        assignment
    }

    /// Detach a policy assignment and append an audit entry.
    pub fn detach(&mut self, assignment_id: &str, actor: Option<&str>) -> bool {
        let Some(pos) = self.assignments.iter().position(|a| a.id == assignment_id) else {
            return false;
        };
        let assignment = self.assignments.remove(pos);
        self.audit.push(PolicyAuditEntry {
            id: format!("audit-detach-{}", assignment_id),
            action: "detach".into(),
            assignment_id: assignment_id.to_string(),
            entity_id: assignment.entity_id,
            policy_name: assignment.policy.name,
            actor: actor.map(String::from),
            at: chrono_now(),
            detail: None,
        });
        true
    }

    pub fn for_entity(&self, entity_id: &str) -> Vec<&PolicyAssignment> {
        self.assignments
            .iter()
            .filter(|a| a.entity_id == entity_id)
            .collect()
    }

    pub fn audit_for_entity(&self, entity_id: &str) -> Vec<&PolicyAuditEntry> {
        self.audit
            .iter()
            .filter(|e| e.entity_id == entity_id)
            .collect()
    }
}

/// Build a default policy reference for a kind/name pair.
pub fn policy_ref(
    kind: GovernancePolicyKind,
    name: &str,
    version: Option<&str>,
) -> GovernancePolicyRef {
    let mut policy = GovernancePolicyRef::new(kind, name);
    policy.version = version.map(String::from);
    policy
}

fn chrono_now() -> String {
    chrono::Utc::now().to_rfc3339()
}

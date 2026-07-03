//! Human accountability model for missions and deployments.
//!
use serde::{Deserialize, Serialize};

/// Contact reference for escalation and emergency response.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct AccountabilityContact {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization: Option<String>,
}

/// Operator certification record for human accountability.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct OperatorCertification {
    pub id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub issuer: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub issued_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub scopes: Vec<String>,
}

/// Approval chain step for governance workflows.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ApprovalChainStep {
    pub role: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub assignee: Option<String>,
    #[serde(default)]
    pub required: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub approved_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub approved_by: Option<String>,
}

/// Human accountability block for entity governance.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct HumanAccountability {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub responsible_person: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub responsible_organization: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mission_owner: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deployment_owner: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub approval_chain: Vec<ApprovalChainStep>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub emergency_contact: Option<AccountabilityContact>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub escalation_contact: Option<AccountabilityContact>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub operator_certifications: Vec<OperatorCertification>,
}

impl HumanAccountability {
    pub fn is_complete_for_production(&self) -> bool {
        self.responsible_person.is_some()
            && self.deployment_owner.is_some()
            && self.emergency_contact.is_some()
    }
}

//! Certification lifecycle tracking — independent from health posture.
//!
use crate::types::CertificationStatus;
use serde::{Deserialize, Serialize};

/// Evidence reference for certification audit trail.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CertificationEvidence {
    pub id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hash: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub collected_at: Option<String>,
}

/// Full certification record with lifecycle metadata.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CertificationRecord {
    pub id: String,
    pub status: CertificationStatus,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub certified_by: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub certified_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub evidence: Vec<CertificationEvidence>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub applicable_scope: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
}

impl CertificationRecord {
    pub fn draft(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            status: CertificationStatus::Draft,
            version: None,
            certified_by: None,
            certified_at: None,
            reason: None,
            evidence: vec![],
            applicable_scope: vec![],
            expires_at: None,
        }
    }

    pub fn is_valid(&self) -> bool {
        self.status.is_operational()
            && self
                .expires_at
                .as_ref()
                .map(|exp| !exp.is_empty())
                .unwrap_or(true)
    }
}

/// Certification summary for entity governance projection.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct EntityCertificationSummary {
    pub status: CertificationStatus,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub records: Vec<CertificationRecord>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub primary_record_id: Option<String>,
}

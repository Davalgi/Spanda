//! Persistent certification record store — independent from health posture.
//!
use crate::certification::{
    CertificationEvidence, CertificationRecord, EntityCertificationSummary,
};
use crate::types::CertificationStatus;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

/// Default on-disk certification store path.
pub fn default_certification_store_path() -> PathBuf {
    PathBuf::from("control-center-certifications.json")
}

/// On-disk certification record store.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct CertificationStore {
    #[serde(default)]
    pub records: Vec<CertificationRecord>,
}

impl CertificationStore {
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

    pub fn upsert(&mut self, record: CertificationRecord) {
        if let Some(existing) = self.records.iter_mut().find(|r| r.id == record.id) {
            *existing = record;
        } else {
            self.records.push(record);
        }
    }

    pub fn get(&self, id: &str) -> Option<&CertificationRecord> {
        self.records.iter().find(|r| r.id == id)
    }

    pub fn for_scope(&self, scope: &str) -> Vec<&CertificationRecord> {
        self.records
            .iter()
            .filter(|r| r.applicable_scope.iter().any(|s| s == scope))
            .collect()
    }

    pub fn summary_for_entity(&self, entity_id: &str) -> EntityCertificationSummary {
        let records: Vec<CertificationRecord> =
            self.for_scope(entity_id).into_iter().cloned().collect();
        let status = records
            .iter()
            .map(|r| r.status)
            .max()
            .unwrap_or(CertificationStatus::Draft);
        let primary_record_id = records
            .iter()
            .find(|r| r.status.is_operational())
            .or(records.first())
            .map(|r| r.id.clone());
        EntityCertificationSummary {
            status,
            records,
            primary_record_id,
        }
    }

    /// Transition a certification record to a new status with audit metadata.
    pub fn transition(
        &mut self,
        id: &str,
        status: CertificationStatus,
        certified_by: Option<&str>,
        reason: Option<&str>,
    ) -> Result<&CertificationRecord, String> {
        let record = self
            .records
            .iter_mut()
            .find(|r| r.id == id)
            .ok_or_else(|| format!("certification '{id}' not found"))?;
        record.status = status;
        if let Some(by) = certified_by {
            record.certified_by = Some(by.to_string());
        }
        if status.is_operational() {
            record.certified_at = Some(chrono::Utc::now().to_rfc3339());
        }
        if let Some(why) = reason {
            record.reason = Some(why.to_string());
        }
        Ok(record)
    }

    pub fn add_evidence(
        &mut self,
        id: &str,
        evidence: CertificationEvidence,
    ) -> Result<(), String> {
        let record = self
            .records
            .iter_mut()
            .find(|r| r.id == id)
            .ok_or_else(|| format!("certification '{id}' not found"))?;
        record.evidence.push(evidence);
        Ok(())
    }
}

/// Full certification report for CLI/API export.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CertificationReport {
    pub entity_id: Option<String>,
    pub records: Vec<CertificationRecord>,
    pub operational_count: usize,
    pub expired_count: usize,
    pub revoked_count: usize,
    pub disclaimer: String,
}

impl CertificationReport {
    pub fn from_store(store: &CertificationStore, entity_id: Option<&str>) -> Self {
        let records: Vec<CertificationRecord> = match entity_id {
            Some(id) => store.for_scope(id).into_iter().cloned().collect(),
            None => store.records.clone(),
        };
        let operational_count = records.iter().filter(|r| r.status.is_operational()).count();
        let expired_count = records
            .iter()
            .filter(|r| r.status == CertificationStatus::Expired)
            .count();
        let revoked_count = records
            .iter()
            .filter(|r| r.status == CertificationStatus::Revoked)
            .count();
        Self {
            entity_id: entity_id.map(String::from),
            records,
            operational_count,
            expired_count,
            revoked_count,
            disclaimer: "Spanda provides governance abstractions — not legal or regulatory advice."
                .into(),
        }
    }
}

pub fn format_certification_report(report: &CertificationReport, json: bool) -> String {
    if json {
        return serde_json::to_string_pretty(report).unwrap_or_else(|_| "{}".into());
    }
    let mut lines = vec!["Certification Report".into(), "====================".into()];
    if let Some(id) = report.entity_id.as_ref() {
        lines.push(format!("Entity: {id}"));
    }
    lines.push(format!("Records: {}", report.records.len()));
    lines.push(format!("Operational: {}", report.operational_count));
    lines.push(format!("Expired: {}", report.expired_count));
    lines.push(format!("Revoked: {}", report.revoked_count));
    lines.push(String::new());
    for record in &report.records {
        lines.push(format!(
            "  {} — {} (by: {}, scope: {})",
            record.id,
            record.status.as_str(),
            record.certified_by.as_deref().unwrap_or("-"),
            record.applicable_scope.join(", ")
        ));
        if let Some(reason) = record.reason.as_ref() {
            lines.push(format!("    reason: {reason}"));
        }
        if !record.evidence.is_empty() {
            lines.push(format!("    evidence: {} item(s)", record.evidence.len()));
        }
    }
    lines.push(String::new());
    lines.push(report.disclaimer.clone());
    lines.join("\n")
}

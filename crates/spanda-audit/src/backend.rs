//! backend support for Spanda.
//!
use crate::error::{AuditError, AuditResult};
use crate::record::{AuditExport, AuditRecord, Hash, RecordId, TransactionId};

/// Append-only audit storage backend.
pub trait AuditBackend {
    fn append(&mut self, record: AuditRecord) -> AuditResult<RecordId>;
    fn verify(&self, record_id: &RecordId) -> AuditResult<bool>;
    fn export(&self) -> AuditResult<AuditExport>;
    fn record_count(&self) -> usize;
}

/// Ledger backend for anchoring content hashes (blockchain-ready interface).
pub trait LedgerBackend: AuditBackend {
    fn anchor_hash(&mut self, hash: &Hash) -> AuditResult<TransactionId>;
    fn verify_anchor(&self, hash: &Hash) -> AuditResult<bool>;
}

/// In-memory append-only audit log.
#[derive(Debug, Default)]
pub struct LocalAuditBackend {
    records: Vec<AuditRecord>,
    provenance: Vec<crate::record::ProvenanceRecord>,
    mission: Option<crate::record::MissionRecord>,
}

impl LocalAuditBackend {
    pub fn new() -> Self {
        // Create a new instance.
        //
        // Parameters:
        // None.
        //
        // Returns:
        // A new instance of this type.
        //
        // Options:
        // None.
        //
        // Example:
        // let value = spanda_audit::backend::new();

        Self::default()
    }

    pub fn records(&self) -> &[AuditRecord] {
        // Records.
        //
        // Parameters:
        // - `self` — method receiver
        //
        // Returns:
        // &[AuditRecord].
        //
        // Options:
        // None.
        //
        // Example:
        // let result = instance.records();

        &self.records
    }

    pub fn last_hash(&self) -> Option<Hash> {
        // Last hash.
        //
        // Parameters:
        // - `self` — method receiver
        //
        // Returns:
        // Some value on success, otherwise none.
        //
        // Options:
        // None.
        //
        // Example:
        // let result = instance.last_hash();

        self.records.last().map(|r| r.hash.clone())
    }
}

impl AuditBackend for LocalAuditBackend {
    fn append(&mut self, record: AuditRecord) -> AuditResult<RecordId> {
        // Append.
        //
        // Parameters:
        // - `self` — method receiver
        // - `record` — input value
        //
        // Returns:
        // AuditResult<RecordId>.
        //
        // Options:
        // None.
        //
        // Example:
        // let result = instance.append(record);

        let id = record.id.clone();
        self.records.push(record);
        Ok(id)
    }

    fn verify(&self, record_id: &RecordId) -> AuditResult<bool> {
        // Verify.
        //
        // Parameters:
        // - `self` — method receiver
        // - `record_id` — input value
        //
        // Returns:
        // AuditResult<bool>.
        //
        // Options:
        // None.
        //
        // Example:
        // let result = instance.verify(record_id);

        let record = self
            .records
            .iter()
            .find(|r| r.id == *record_id)
            .ok_or_else(|| AuditError::NotFound(record_id.0.clone()))?;

        let expected = crate::crypto::sha256(&record.canonical_body());
        if expected != record.hash {
            return Err(AuditError::HashMismatch(record_id.0.clone()));
        }

        if let Some(sig) = &record.signature {
            let pub_key = record
                .signing_key
                .as_deref()
                .or(record.signer_id.as_deref())
                .unwrap_or("unknown");
            if !crate::crypto::verify_signature(&record.canonical_body(), sig, pub_key) {
                return Err(AuditError::InvalidSignature);
            }
        }
        Ok(true)
    }

    fn export(&self) -> AuditResult<AuditExport> {
        // Export.
        //
        // Parameters:
        // - `self` — method receiver
        //
        // Returns:
        // AuditResult<AuditExport>.
        //
        // Options:
        // None.
        //
        // Example:
        // let result = instance.export();

        Ok(AuditExport {
            records: self.records.clone(),
            provenance: self.provenance.clone(),
            mission: self.mission.clone(),
            exported_at: chrono::Utc::now(),
        })
    }

    fn record_count(&self) -> usize {
        // Return the number of record.
        //
        // Parameters:
        // - `self` — method receiver
        //
        // Returns:
        // Numeric result.
        //
        // Options:
        // None.
        //
        // Example:
        // let result = instance.record_count();

        self.records.len()
    }
}

/// JSON-serializing audit backend (stores in memory, exports as JSON).
#[derive(Debug, Default)]
pub struct JsonAuditBackend {
    inner: LocalAuditBackend,
}

impl JsonAuditBackend {
    pub fn new() -> Self {
        // Create a new instance.
        //
        // Parameters:
        // None.
        //
        // Returns:
        // A new instance of this type.
        //
        // Options:
        // None.
        //
        // Example:
        // let value = spanda_audit::backend::new();

        Self::default()
    }

    pub fn export_json(&self) -> AuditResult<String> {
        // Export json.
        //
        // Parameters:
        // - `self` — method receiver
        //
        // Returns:
        // AuditResult<String>.
        //
        // Options:
        // None.
        //
        // Example:
        // let result = instance.export_json();

        let export = self.export()?;
        serde_json::to_string_pretty(&export).map_err(|e| AuditError::Serialization(e.to_string()))
    }

    pub fn export_json_compact(&self) -> AuditResult<String> {
        // Export json compact.
        //
        // Parameters:
        // - `self` — method receiver
        //
        // Returns:
        // AuditResult<String>.
        //
        // Options:
        // None.
        //
        // Example:
        // let result = instance.export_json_compact();

        let export = self.export()?;
        serde_json::to_string(&export).map_err(|e| AuditError::Serialization(e.to_string()))
    }
}

impl AuditBackend for JsonAuditBackend {
    fn append(&mut self, record: AuditRecord) -> AuditResult<RecordId> {
        // Append.
        //
        // Parameters:
        // - `self` — method receiver
        // - `record` — input value
        //
        // Returns:
        // AuditResult<RecordId>.
        //
        // Options:
        // None.
        //
        // Example:
        // let result = instance.append(record);

        self.inner.append(record)
    }

    fn verify(&self, record_id: &RecordId) -> AuditResult<bool> {
        // Verify.
        //
        // Parameters:
        // - `self` — method receiver
        // - `record_id` — input value
        //
        // Returns:
        // AuditResult<bool>.
        //
        // Options:
        // None.
        //
        // Example:
        // let result = instance.verify(record_id);

        self.inner.verify(record_id)
    }

    fn export(&self) -> AuditResult<AuditExport> {
        // Export.
        //
        // Parameters:
        // - `self` — method receiver
        //
        // Returns:
        // AuditResult<AuditExport>.
        //
        // Options:
        // None.
        //
        // Example:
        // let result = instance.export();

        self.inner.export()
    }

    fn record_count(&self) -> usize {
        // Return the number of record.
        //
        // Parameters:
        // - `self` — method receiver
        //
        // Returns:
        // Numeric result.
        //
        // Options:
        // None.
        //
        // Example:
        // let result = instance.record_count();

        self.inner.record_count()
    }
}

/// Mock ledger that anchors hashes without connecting to real chains.
#[derive(Debug, Default)]
pub struct MockLedgerBackend {
    audit: LocalAuditBackend,
    anchors: Vec<(Hash, TransactionId)>,
    next_tx: u64,
}

impl MockLedgerBackend {
    pub fn new() -> Self {
        // Create a new instance.
        //
        // Parameters:
        // None.
        //
        // Returns:
        // A new instance of this type.
        //
        // Options:
        // None.
        //
        // Example:
        // let value = spanda_audit::backend::new();

        Self {
            next_tx: 1,
            ..Default::default()
        }
    }

    pub fn anchored_count(&self) -> usize {
        // Return the number of anchored.
        //
        // Parameters:
        // - `self` — method receiver
        //
        // Returns:
        // Numeric result.
        //
        // Options:
        // None.
        //
        // Example:
        // let result = instance.anchored_count();

        self.anchors.len()
    }
}

impl AuditBackend for MockLedgerBackend {
    fn append(&mut self, record: AuditRecord) -> AuditResult<RecordId> {
        // Append.
        //
        // Parameters:
        // - `self` — method receiver
        // - `record` — input value
        //
        // Returns:
        // AuditResult<RecordId>.
        //
        // Options:
        // None.
        //
        // Example:
        // let result = instance.append(record);

        self.audit.append(record)
    }

    fn verify(&self, record_id: &RecordId) -> AuditResult<bool> {
        // Verify.
        //
        // Parameters:
        // - `self` — method receiver
        // - `record_id` — input value
        //
        // Returns:
        // AuditResult<bool>.
        //
        // Options:
        // None.
        //
        // Example:
        // let result = instance.verify(record_id);

        self.audit.verify(record_id)
    }

    fn export(&self) -> AuditResult<AuditExport> {
        // Export.
        //
        // Parameters:
        // - `self` — method receiver
        //
        // Returns:
        // AuditResult<AuditExport>.
        //
        // Options:
        // None.
        //
        // Example:
        // let result = instance.export();

        self.audit.export()
    }

    fn record_count(&self) -> usize {
        // Return the number of record.
        //
        // Parameters:
        // - `self` — method receiver
        //
        // Returns:
        // Numeric result.
        //
        // Options:
        // None.
        //
        // Example:
        // let result = instance.record_count();

        self.audit.record_count()
    }
}

impl LedgerBackend for MockLedgerBackend {
    fn anchor_hash(&mut self, hash: &Hash) -> AuditResult<TransactionId> {
        // Anchor hash.
        //
        // Parameters:
        // - `self` — method receiver
        // - `hash` — input value
        //
        // Returns:
        // AuditResult<TransactionId>.
        //
        // Options:
        // None.
        //
        // Example:
        // let result = instance.anchor_hash(hash);

        let tx = TransactionId(format!("mock-tx-{}", self.next_tx));
        self.next_tx += 1;
        self.anchors.push((hash.clone(), tx.clone()));
        Ok(tx)
    }

    fn verify_anchor(&self, hash: &Hash) -> AuditResult<bool> {
        // Verify anchor.
        //
        // Parameters:
        // - `self` — method receiver
        // - `hash` — input value
        //
        // Returns:
        // AuditResult<bool>.
        //
        // Options:
        // None.
        //
        // Example:
        // let result = instance.verify_anchor(hash);

        Ok(self.anchors.iter().any(|(h, _)| h == hash))
    }
}

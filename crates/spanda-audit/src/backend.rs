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
        // Description:
        //     Construct a new instance.
        //
        // Inputs:
        //     None.
        //
        // Outputs:
        //     result: Self
        //         Return value from `new`.
        //
        // Example:
        //     let value = spanda_audit::backend::new();

        // Build the result via default.
        Self::default()
    }

    pub fn records(&self) -> &[AuditRecord] {
        // Description:
        //     Records.
        //
        // Inputs:
        //     &self: input value
        //         Caller-supplied &self.
        //
        // Outputs:
        //     result: &[AuditRecord]
        //         Return value from `records`.
        //
        // Example:
        //     let result = spanda_audit::backend::records(&self);

        // Return records from this handle.
        &self.records
    }

    pub fn last_hash(&self) -> Option<Hash> {
        // Description:
        //     Last hash.
        //
        // Inputs:
        //     &self: input value
        //         Caller-supplied &self.
        //
        // Outputs:
        //     result: Option<Hash>
        //         Return value from `last_hash`.
        //
        // Example:
        //     let result = spanda_audit::backend::last_hash(&self);

        // Transform self and continue the chain.
        self.records.last().map(|r| r.hash.clone())
    }
}

impl AuditBackend for LocalAuditBackend {
    fn append(&mut self, record: AuditRecord) -> AuditResult<RecordId> {
        // Description:
        //     Append.
        //
        // Inputs:
        //     &mut self: input value
        //         Caller-supplied &mut self.
        //     record: AuditRecord
        //         Caller-supplied record.
        //
        // Outputs:
        //     result: AuditResult<RecordId>
        //         Return value from `append`.
        //
        // Example:
        //     let result = spanda_audit::backend::append(&mut self, record);

        // Compute id for the following logic.
        let id = record.id.clone();
        self.records.push(record);
        Ok(id)
    }

    fn verify(&self, record_id: &RecordId) -> AuditResult<bool> {
        // Description:
        //     Verify.
        //
        // Inputs:
        //     &self: input value
        //         Caller-supplied &self.
        //     record_id: &RecordId
        //         Caller-supplied record id.
        //
        // Outputs:
        //     result: AuditResult<bool>
        //         Return value from `verify`.
        //
        // Example:
        //     let result = spanda_audit::backend::verify(&self, record_id);

        // Compute record for the following logic.
        let record = self
            .records
            .iter()
            .find(|r| r.id == *record_id)
            .ok_or_else(|| AuditError::NotFound(record_id.0.clone()))?;
        let expected = crate::crypto::sha256(&record.canonical_body());

        // Take the branch when expected differs from hash.
        if expected != record.hash {
            return Err(AuditError::HashMismatch(record_id.0.clone()));
        }

        // Emit output when signature provides a sig.
        if let Some(sig) = &record.signature {
            let pub_key = record
                .signing_key
                .as_deref()
                .or(record.signer_id.as_deref())
                .unwrap_or("unknown");

            // Take the branch when canonical body is false.
            if !crate::crypto::verify_signature(&record.canonical_body(), sig, pub_key) {
                return Err(AuditError::InvalidSignature);
            }
        }
        Ok(true)
    }

    fn export(&self) -> AuditResult<AuditExport> {
        // Description:
        //     Export.
        //
        // Inputs:
        //     &self: input value
        //         Caller-supplied &self.
        //
        // Outputs:
        //     result: AuditResult<AuditExport>
        //         Return value from `export`.
        //
        // Example:
        //     let result = spanda_audit::backend::export(&self);

        // Return the success value to the caller.
        Ok(AuditExport {
            records: self.records.clone(),
            provenance: self.provenance.clone(),
            mission: self.mission.clone(),
            exported_at: chrono::Utc::now(),
        })
    }

    fn record_count(&self) -> usize {
        // Description:
        //     Record count.
        //
        // Inputs:
        //     &self: input value
        //         Caller-supplied &self.
        //
        // Outputs:
        //     result: usize
        //         Return value from `record_count`.
        //
        // Example:
        //     let result = spanda_audit::backend::record_count(&self);

        // Call len on the current instance.
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
        // Description:
        //     Construct a new instance.
        //
        // Inputs:
        //     None.
        //
        // Outputs:
        //     result: Self
        //         Return value from `new`.
        //
        // Example:
        //     let value = spanda_audit::backend::new();

        // Build the result via default.
        Self::default()
    }

    pub fn export_json(&self) -> AuditResult<String> {
        // Description:
        //     Export json.
        //
        // Inputs:
        //     &self: input value
        //         Caller-supplied &self.
        //
        // Outputs:
        //     result: AuditResult<String>
        //         Return value from `export_json`.
        //
        // Example:
        //     let result = spanda_audit::backend::export_json(&self);

        // Compute export for the following logic.
        let export = self.export()?;
        serde_json::to_string_pretty(&export).map_err(|e| AuditError::Serialization(e.to_string()))
    }

    pub fn export_json_compact(&self) -> AuditResult<String> {
        // Description:
        //     Export json compact.
        //
        // Inputs:
        //     &self: input value
        //         Caller-supplied &self.
        //
        // Outputs:
        //     result: AuditResult<String>
        //         Return value from `export_json_compact`.
        //
        // Example:
        //     let result = spanda_audit::backend::export_json_compact(&self);

        // Compute export for the following logic.
        let export = self.export()?;
        serde_json::to_string(&export).map_err(|e| AuditError::Serialization(e.to_string()))
    }
}

impl AuditBackend for JsonAuditBackend {
    fn append(&mut self, record: AuditRecord) -> AuditResult<RecordId> {
        // Description:
        //     Append.
        //
        // Inputs:
        //     &mut self: input value
        //         Caller-supplied &mut self.
        //     record: AuditRecord
        //         Caller-supplied record.
        //
        // Outputs:
        //     result: AuditResult<RecordId>
        //         Return value from `append`.
        //
        // Example:
        //     let result = spanda_audit::backend::append(&mut self, record);

        // Call append on the current instance.
        self.inner.append(record)
    }

    fn verify(&self, record_id: &RecordId) -> AuditResult<bool> {
        // Description:
        //     Verify.
        //
        // Inputs:
        //     &self: input value
        //         Caller-supplied &self.
        //     record_id: &RecordId
        //         Caller-supplied record id.
        //
        // Outputs:
        //     result: AuditResult<bool>
        //         Return value from `verify`.
        //
        // Example:
        //     let result = spanda_audit::backend::verify(&self, record_id);

        // Call verify on the current instance.
        self.inner.verify(record_id)
    }

    fn export(&self) -> AuditResult<AuditExport> {
        // Description:
        //     Export.
        //
        // Inputs:
        //     &self: input value
        //         Caller-supplied &self.
        //
        // Outputs:
        //     result: AuditResult<AuditExport>
        //         Return value from `export`.
        //
        // Example:
        //     let result = spanda_audit::backend::export(&self);

        // Call export on the current instance.
        self.inner.export()
    }

    fn record_count(&self) -> usize {
        // Description:
        //     Record count.
        //
        // Inputs:
        //     &self: input value
        //         Caller-supplied &self.
        //
        // Outputs:
        //     result: usize
        //         Return value from `record_count`.
        //
        // Example:
        //     let result = spanda_audit::backend::record_count(&self);

        // Call record count on the current instance.
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
        // Description:
        //     Construct a new instance.
        //
        // Inputs:
        //     None.
        //
        // Outputs:
        //     result: Self
        //         Return value from `new`.
        //
        // Example:
        //     let value = spanda_audit::backend::new();

        // Assemble the struct fields and return it.
        Self {
            next_tx: 1,
            ..Default::default()
        }
    }

    pub fn anchored_count(&self) -> usize {
        // Description:
        //     Anchored count.
        //
        // Inputs:
        //     &self: input value
        //         Caller-supplied &self.
        //
        // Outputs:
        //     result: usize
        //         Return value from `anchored_count`.
        //
        // Example:
        //     let result = spanda_audit::backend::anchored_count(&self);

        // Call len on the current instance.
        self.anchors.len()
    }
}

impl AuditBackend for MockLedgerBackend {
    fn append(&mut self, record: AuditRecord) -> AuditResult<RecordId> {
        // Description:
        //     Append.
        //
        // Inputs:
        //     &mut self: input value
        //         Caller-supplied &mut self.
        //     record: AuditRecord
        //         Caller-supplied record.
        //
        // Outputs:
        //     result: AuditResult<RecordId>
        //         Return value from `append`.
        //
        // Example:
        //     let result = spanda_audit::backend::append(&mut self, record);

        // Call append on the current instance.
        self.audit.append(record)
    }

    fn verify(&self, record_id: &RecordId) -> AuditResult<bool> {
        // Description:
        //     Verify.
        //
        // Inputs:
        //     &self: input value
        //         Caller-supplied &self.
        //     record_id: &RecordId
        //         Caller-supplied record id.
        //
        // Outputs:
        //     result: AuditResult<bool>
        //         Return value from `verify`.
        //
        // Example:
        //     let result = spanda_audit::backend::verify(&self, record_id);

        // Call verify on the current instance.
        self.audit.verify(record_id)
    }

    fn export(&self) -> AuditResult<AuditExport> {
        // Description:
        //     Export.
        //
        // Inputs:
        //     &self: input value
        //         Caller-supplied &self.
        //
        // Outputs:
        //     result: AuditResult<AuditExport>
        //         Return value from `export`.
        //
        // Example:
        //     let result = spanda_audit::backend::export(&self);

        // Call export on the current instance.
        self.audit.export()
    }

    fn record_count(&self) -> usize {
        // Description:
        //     Record count.
        //
        // Inputs:
        //     &self: input value
        //         Caller-supplied &self.
        //
        // Outputs:
        //     result: usize
        //         Return value from `record_count`.
        //
        // Example:
        //     let result = spanda_audit::backend::record_count(&self);

        // Call record count on the current instance.
        self.audit.record_count()
    }
}

impl LedgerBackend for MockLedgerBackend {
    fn anchor_hash(&mut self, hash: &Hash) -> AuditResult<TransactionId> {
        // Description:
        //     Anchor hash.
        //
        // Inputs:
        //     &mut self: input value
        //         Caller-supplied &mut self.
        //     hash: &Hash
        //         Caller-supplied hash.
        //
        // Outputs:
        //     result: AuditResult<TransactionId>
        //         Return value from `anchor_hash`.
        //
        // Example:
        //     let result = spanda_audit::backend::anchor_hash(&mut self, hash);

        // Compute tx for the following logic.
        let tx = TransactionId(format!("mock-tx-{}", self.next_tx));
        self.next_tx += 1;
        self.anchors.push((hash.clone(), tx.clone()));
        Ok(tx)
    }

    fn verify_anchor(&self, hash: &Hash) -> AuditResult<bool> {
        // Description:
        //     Verify anchor.
        //
        // Inputs:
        //     &self: input value
        //         Caller-supplied &self.
        //     hash: &Hash
        //         Caller-supplied hash.
        //
        // Outputs:
        //     result: AuditResult<bool>
        //         Return value from `verify_anchor`.
        //
        // Example:
        //     let result = spanda_audit::backend::verify_anchor(&self, hash);

        // Return the success value to the caller.
        Ok(self.anchors.iter().any(|(h, _)| h == hash))
    }
}

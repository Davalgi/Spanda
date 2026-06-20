//! Audit error types and result alias for the Spanda audit crate.

use thiserror::Error;

/// Errors raised by audit backends and the audit runtime.
#[derive(Debug, Error)]
pub enum AuditError {
    #[error("audit record not found: {0}")]
    NotFound(String),
    #[error("signature verification failed")]
    InvalidSignature,
    #[error("hash mismatch for record {0}")]
    HashMismatch(String),
    #[error("anchor not found for hash {0}")]
    AnchorNotFound(String),
    #[error("serialization error: {0}")]
    Serialization(String),
    #[error("{0}")]
    Other(String),
}

/// Convenience alias for audit operations that may fail with [`AuditError`].
pub type AuditResult<T> = Result<T, AuditError>;

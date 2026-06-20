use thiserror::Error;

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

pub type AuditResult<T> = Result<T, AuditError>;

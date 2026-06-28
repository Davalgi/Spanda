//! Structured errors shared across Spanda SDK language bindings.
//!
use thiserror::Error;

/// Top-level SDK error type.
#[derive(Debug, Error)]
pub enum SpandaError {
    #[error("validation error: {0}")]
    Validation(String),

    #[error("readiness error: {0}")]
    Readiness(String),

    #[error("verification error: {0}")]
    Verification(String),

    #[error("security error: {0}")]
    Security(String),

    #[error("connection error: {0}")]
    Connection(String),

    #[error("permission error: {0}")]
    Permission(String),

    #[error("api error ({status}): {message}")]
    Api { status: u16, message: String },
}

impl SpandaError {
    pub fn validation(message: impl Into<String>) -> Self {
        Self::Validation(message.into())
    }

    pub fn connection(message: impl Into<String>) -> Self {
        Self::Connection(message.into())
    }

    pub fn permission(message: impl Into<String>) -> Self {
        Self::Permission(message.into())
    }

    pub fn from_status(status: u16, message: impl Into<String>) -> Self {
        match status {
            401 | 403 => Self::Permission(message.into()),
            400 => Self::Validation(message.into()),
            _ => Self::Api {
                status,
                message: message.into(),
            },
        }
    }
}

pub type SpandaResult<T> = Result<T, SpandaError>;

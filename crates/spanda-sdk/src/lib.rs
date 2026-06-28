//! Official Rust SDK for Spanda — thin HTTP client over Control Center API v1.
//!
//! Core platform logic lives in Rust runtime crates; this crate delegates to
//! `spanda-api` REST endpoints without duplicating business rules.

pub mod client;
pub mod error;
pub mod stream;
pub mod types;

pub use client::{AuthConfig, SpandaClient, SpandaClientBuilder};
pub use error::SpandaError;
pub use stream::EventStream;
pub use types::*;

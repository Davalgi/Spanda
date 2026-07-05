//! Official Rust SDK for Spanda — thin HTTP client over Control Center API v1.
//!
//! Core platform logic lives in Rust runtime crates; this crate delegates to
//! `spanda-api` REST endpoints without duplicating business rules.
//!
//! Enable feature `grpc` for a native tonic client.

pub mod client;
pub mod error;
pub mod stream;
pub mod types;

#[cfg(feature = "grpc")]
pub mod grpc;

pub use client::{
    AttentionClient, AuthConfig, AutonomyClient, CertificationClient, ComplianceClient,
    DeploymentProfileClient, FusionClient, GovernanceClient, HomeostasisClient, ImmunityClient,
    MemoryClient, ReflexClient, RiskClient, SpandaClient, SpandaClientBuilder,
};
pub use error::SpandaError;
pub use stream::EventStream;
pub use types::*;

#[cfg(feature = "grpc")]
pub use grpc::GrpcClient;

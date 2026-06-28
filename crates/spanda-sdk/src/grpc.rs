//! Optional native gRPC client (tonic) for Control Center.
//!
#[cfg(feature = "grpc")]
pub mod client;

#[cfg(feature = "grpc")]
pub use client::GrpcClient;

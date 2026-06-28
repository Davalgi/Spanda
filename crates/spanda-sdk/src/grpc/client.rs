//! Native tonic gRPC client — optional `grpc` feature on `spanda-sdk`.
//!
use crate::error::{SpandaError, SpandaResult};
use serde_json::Value;
use tonic::transport::Channel;

pub mod spanda_v1 {
    tonic::include_proto!("spanda.v1");
}

use spanda_v1::control_center_client::ControlCenterClient;
use spanda_v1::JsonBodyRequest;

/// Async gRPC client for Control Center (`spanda.v1.ControlCenter`).
pub struct GrpcClient {
    inner: ControlCenterClient<Channel>,
}

impl GrpcClient {
    /// Connect to a gRPC endpoint (for example `http://127.0.0.1:50051`).
    pub async fn connect(endpoint: impl Into<String>) -> SpandaResult<Self> {
        let channel = Channel::from_shared(endpoint.into())
            .map_err(|e| SpandaError::connection(e.to_string()))?
            .connect()
            .await
            .map_err(|e| SpandaError::connection(e.to_string()))?;
        Ok(Self {
            inner: ControlCenterClient::new(channel),
        })
    }

    /// Blocking connect helper for scripts without an async runtime.
    pub fn connect_blocking(endpoint: impl Into<String>) -> SpandaResult<Self> {
        tokio::runtime::Runtime::new()
            .map_err(|e| SpandaError::connection(e.to_string()))?
            .block_on(Self::connect(endpoint))
    }

    fn parse_json(raw: String) -> SpandaResult<Value> {
        serde_json::from_str(&raw).map_err(|e| SpandaError::validation(e.to_string()))
    }

    /// Evaluate program readiness via `EvaluateProgramReadiness`.
    pub async fn readiness(&mut self, file: &str) -> SpandaResult<Value> {
        let body = serde_json::json!({ "file": file }).to_string();
        let resp = self
            .inner
            .evaluate_program_readiness(JsonBodyRequest { body_json: body })
            .await
            .map_err(|e| SpandaError::connection(e.to_string()))?;
        Self::parse_json(resp.into_inner().json)
    }

    /// List unified entities via `ListEntities`.
    pub async fn list_entities(&mut self) -> SpandaResult<Value> {
        let resp = self
            .inner
            .list_entities(spanda_v1::Empty {})
            .await
            .map_err(|e| SpandaError::connection(e.to_string()))?;
        Self::parse_json(resp.into_inner().json)
    }
}

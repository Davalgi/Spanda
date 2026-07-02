//! Environment configuration for Twin Cloud SaaS endpoints.

use std::env;

/// Connection settings for Twin Cloud HTTP API.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TwinCloudConfig {
    pub base_url: String,
    pub api_key: Option<String>,
    pub tenant_id: String,
}

impl TwinCloudConfig {
    /// Load config from `SPANDA_TWIN_CLOUD_URL`, optional API key, and tenant id env vars.
    pub fn from_env() -> Option<Self> {
        let base_url = env::var("SPANDA_TWIN_CLOUD_URL")
            .ok()
            .or_else(|| env::var("SPANDA_CONTROL_CENTER_URL").ok())
            .map(|value| value.trim().trim_end_matches('/').to_string())
            .filter(|value| !value.is_empty())?;
        Some(Self {
            base_url,
            api_key: env::var("SPANDA_TWIN_CLOUD_API_KEY")
                .ok()
                .or_else(|| env::var("SPANDA_API_KEY").ok())
                .map(|value| value.trim().to_string())
                .filter(|value| !value.is_empty()),
            tenant_id: env::var("SPANDA_TWIN_CLOUD_TENANT")
                .ok()
                .or_else(|| env::var("SPANDA_TENANT_ID").ok())
                .unwrap_or_else(|| "default".into()),
        })
    }
}

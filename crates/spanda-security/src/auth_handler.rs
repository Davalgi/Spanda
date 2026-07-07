//! Unified authentication handler for Control Center REST and gRPC boundaries.
//!
use crate::api_key_hash::{api_key_pepper, constant_time_eq, hash_api_key_token, verify_api_key_token};
use crate::rbac::{ApiKeyStore, RbacContext};
use crate::session_token::SessionTokenIssuer;
use serde::{Deserialize, Serialize};

/// How the current request principal was authenticated.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum AuthKind {
    #[default]
    ApiKey,
    Session,
}

/// Policy for optional authentication on read-only HTTP routes.
#[derive(Debug, Clone)]
pub struct ReadAuthPolicy {
    pub require_sensitive_reads: bool,
    pub require_all_reads: bool,
}

impl Default for ReadAuthPolicy {
    fn default() -> Self {
        Self::from_env()
    }
}

impl ReadAuthPolicy {
    /// Load read-auth policy from environment.
    pub fn from_env() -> Self {
        let require_sensitive_reads = std::env::var("SPANDA_API_REQUIRE_AUTH_READS")
            .map(|value| value == "1" || value.eq_ignore_ascii_case("true"))
            .unwrap_or(false);
        let require_all_reads = std::env::var("SPANDA_API_REQUIRE_AUTH_ALL_READS")
            .map(|value| value == "1" || value.eq_ignore_ascii_case("true"))
            .unwrap_or(false);
        Self {
            require_sensitive_reads,
            require_all_reads,
        }
    }

    /// Return `true` for health, version, and instance probes that stay public.
    pub fn is_public_read(&self, path: &str) -> bool {
        matches!(
            path,
            "/v1/health" | "/v1/version" | "/v1/instance" | "/v1/tenant"
        )
    }

    /// Return `true` when a GET request must include a valid bearer principal.
    pub fn requires_auth(&self, method: &str, path: &str) -> bool {
        if method != "GET" {
            return false;
        }
        if self.is_public_read(path) {
            return false;
        }
        if self.require_all_reads {
            return true;
        }
        if !self.require_sensitive_reads {
            return false;
        }
        Self::is_sensitive_read(path)
    }

    fn is_sensitive_read(path: &str) -> bool {
        const PREFIXES: &[&str] = &[
            "/v1/dashboard",
            "/v1/devices",
            "/v1/fleet",
            "/v1/programs/source",
            "/v1/secrets",
            "/v1/audit",
            "/v1/compliance",
            "/v1/admin",
            "/v1/entities",
            "/v1/robots",
            "/v1/config",
            "/v1/sre",
            "/v1/operator",
            "/v1/drift",
            "/v1/ota",
            "/v1/trust",
            "/v1/recovery",
            "/v1/autonomy",
            "/v1/decisions",
            "/v1/humans",
            "/v1/reports",
            "/v1/digital-thread",
            "/v1/executive",
            "/v1/rbac/me",
            "/v1/auth/session",
        ];
        PREFIXES.iter().any(|prefix| path.starts_with(prefix))
    }
}

/// Central authentication handler combining API keys, session JWTs, and read policy.
#[derive(Debug, Clone, Default)]
pub struct AuthHandler {
    pub read_policy: ReadAuthPolicy,
    pub sessions: SessionTokenIssuer,
}

impl AuthHandler {
    /// Build handler with default environment-backed settings.
    pub fn new() -> Self {
        Self::default()
    }

    /// Normalize `Authorization` header or raw bearer value.
    pub fn extract_bearer(raw: Option<&str>) -> Option<String> {
        let token = raw?.trim();
        if token.is_empty() {
            return None;
        }
        let stripped = token
            .strip_prefix("Bearer ")
            .or_else(|| token.strip_prefix("bearer "))
            .unwrap_or(token)
            .trim();
        if stripped.is_empty() {
            None
        } else {
            Some(stripped.to_string())
        }
    }

    /// Authenticate a bearer value against session JWTs and API keys.
    pub fn authenticate(&self, api_keys: &ApiKeyStore, bearer: Option<&str>) -> Option<RbacContext> {
        let token = Self::extract_bearer(bearer)?;
        if SessionTokenIssuer::looks_like_jwt(&token) {
            if let Ok(ctx) = self.sessions.verify(&token) {
                return Some(ctx);
            }
        }
        api_keys.authenticate(Some(&token))
    }

    /// Enforce optional read authentication policy.
    pub fn ensure_read_auth(
        &self,
        method: &str,
        path: &str,
        ctx: Option<&RbacContext>,
    ) -> Result<(), &'static str> {
        if self.read_policy.requires_auth(method, path) && ctx.is_none() {
            return Err("read_auth_required");
        }
        Ok(())
    }
}

/// Hash a new API key token for persistence.
pub fn hash_new_api_key_token(token: &str) -> String {
    hash_api_key_token(token, &api_key_pepper())
}

/// Verify a presented API key against stored material.
pub fn verify_api_key_record(token: &str, token_plain: &str, token_hash: Option<&str>) -> bool {
    if let Some(expected) = token_hash {
        return verify_api_key_token(token, &api_key_pepper(), expected);
    }
    if token_plain.is_empty() {
        return false;
    }
    constant_time_eq(token.as_bytes(), token_plain.as_bytes())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rbac::{ApiKeyRecord, Role};

    #[test]
    fn sensitive_reads_require_auth_when_enabled() {
        let policy = ReadAuthPolicy {
            require_sensitive_reads: true,
            require_all_reads: false,
        };
        assert!(policy.requires_auth("GET", "/v1/dashboard"));
        assert!(!policy.requires_auth("GET", "/v1/health"));
        assert!(!policy.requires_auth("POST", "/v1/alerts/test"));
    }

    #[test]
    fn auth_handler_accepts_api_key() {
        let handler = AuthHandler::new();
        let mut store = ApiKeyStore::new();
        store.keys.push(ApiKeyRecord {
            key_id: "k1".into(),
            token: "secret".into(),
            token_hash: None,
            role: Role::Operator,
            label: None,
            tenant_id: "default".into(),
        });
        let ctx = handler
            .authenticate(&store, Some("Bearer secret"))
            .expect("auth");
        assert_eq!(ctx.auth_kind, AuthKind::ApiKey);
    }
}

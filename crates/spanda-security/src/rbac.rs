//! Role-based access control for Spanda Control Center and APIs.
//!
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Enterprise operator roles (v1 — four primary roles plus safety and audit).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Role {
    Administrator,
    Developer,
    Operator,
    Supervisor,
    SafetyOfficer,
    Auditor,
    Guest,
}

impl Role {
    pub fn parse(s: &str) -> Self {
        match s.to_ascii_lowercase().as_str() {
            "administrator" | "admin" => Self::Administrator,
            "developer" | "dev" => Self::Developer,
            "operator" => Self::Operator,
            "supervisor" => Self::Supervisor,
            "safety_officer" | "safety" => Self::SafetyOfficer,
            "auditor" => Self::Auditor,
            "guest" => Self::Guest,
            _ => Self::Guest,
        }
    }
}

/// Mutating actions guarded by RBAC.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RbacAction {
    Deploy,
    Operate,
    Approve,
    Override,
    Shutdown,
    Recover,
    Delete,
    Provision,
}

/// Authenticated request context after API key or session validation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RbacContext {
    pub key_id: String,
    pub role: Role,
    #[serde(default = "default_tenant_field")]
    pub tenant_id: String,
    #[serde(default)]
    pub auth_kind: crate::auth_handler::AuthKind,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
}

fn default_tenant_field() -> String {
    crate::tenant::default_tenant_id()
}

impl RbacContext {
    /// Build RBAC context for a static API key principal.
    pub fn api_key(key_id: impl Into<String>, role: Role, tenant_id: impl Into<String>) -> Self {
        Self {
            key_id: key_id.into(),
            role,
            tenant_id: tenant_id.into(),
            auth_kind: crate::auth_handler::AuthKind::ApiKey,
            user_id: None,
        }
    }
}

/// API key record. File-backed keys persist `token_hash` only; env keys keep plaintext in memory.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ApiKeyRecord {
    pub key_id: String,
    /// Plaintext token — in-memory only (env default or immediately after create).
    #[serde(default)]
    pub token: String,
    /// HMAC-SHA256 hex digest for file-backed keys.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub token_hash: Option<String>,
    pub role: Role,
    #[serde(default)]
    pub label: Option<String>,
    #[serde(default = "default_tenant_field")]
    pub tenant_id: String,
}

/// In-memory API key store for Control Center v1.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ApiKeyStore {
    pub keys: Vec<ApiKeyRecord>,
}

impl ApiKeyStore {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_env() -> Self {
        let tenant_id = crate::tenant::default_tenant_id();
        let mut store = Self::new();
        if let Ok(token) = std::env::var("SPANDA_API_KEY") {
            store.keys.push(ApiKeyRecord {
                key_id: "env-default".into(),
                token,
                token_hash: None,
                role: Role::Administrator,
                label: Some("SPANDA_API_KEY".into()),
                tenant_id: tenant_id.clone(),
            });
        }
        store
    }

    pub fn from_env_and_file() -> Self {
        let mut store = Self::from_env();
        if let Ok(path) = std::env::var("SPANDA_API_KEYS_FILE") {
            if let Ok(content) = std::fs::read_to_string(path) {
                if let Ok(keys) = serde_json::from_str::<Vec<ApiKeyRecord>>(&content) {
                    store
                        .keys
                        .extend(keys.into_iter().map(Self::normalize_loaded_key));
                }
            }
        }
        store
    }

    fn normalize_loaded_key(mut key: ApiKeyRecord) -> ApiKeyRecord {
        // Migrate legacy plaintext file keys to hashed storage on load.
        if key.token_hash.is_none() && !key.token.is_empty() {
            key.token_hash = Some(crate::auth_handler::hash_new_api_key_token(&key.token));
            key.token.clear();
        }
        key
    }

    pub fn authenticate(&self, bearer: Option<&str>) -> Option<RbacContext> {
        let token = bearer?.trim();
        if token.is_empty() {
            return None;
        }
        self.keys.iter().find_map(|key| {
            if !crate::auth_handler::verify_api_key_record(
                token,
                &key.token,
                key.token_hash.as_deref(),
            ) {
                return None;
            }
            Some(RbacContext {
                key_id: key.key_id.clone(),
                role: key.role,
                tenant_id: key.tenant_id.clone(),
                auth_kind: crate::auth_handler::AuthKind::ApiKey,
                user_id: None,
            })
        })
    }

    pub fn check_tenant(ctx: Option<&RbacContext>, server_tenant: &str) -> bool {
        ctx.is_some_and(|context| crate::tenant::tenant_matches(server_tenant, &context.tenant_id))
    }

    pub fn check_scoped(
        ctx: Option<&RbacContext>,
        server_tenant: &str,
        action: RbacAction,
    ) -> bool {
        match ctx {
            Some(context) if crate::tenant::tenant_matches(server_tenant, &context.tenant_id) => {
                Self::authorize(context.role, action)
            }
            _ => false,
        }
    }

    pub fn authorize(role: Role, action: RbacAction) -> bool {
        use RbacAction::*;
        use Role::*;
        match (role, action) {
            (Administrator, _) => true,
            (Supervisor, _) => !matches!(action, Delete),
            (Developer, Deploy | Operate) => true,
            (Operator, Operate | Shutdown | Recover) => true,
            (SafetyOfficer, Operate | Approve | Shutdown) => true,
            (Auditor, _) => false,
            (Guest, _) => false,
            _ => false,
        }
    }

    pub fn check(ctx: Option<&RbacContext>, action: RbacAction) -> bool {
        match ctx {
            Some(c) => Self::authorize(c.role, action),
            None => false,
        }
    }

    /// Path for durable API key storage (env override or `.spanda/api-keys.json`).
    pub fn default_api_keys_file_path() -> std::path::PathBuf {
        std::env::var("SPANDA_API_KEYS_FILE")
            .map(std::path::PathBuf::from)
            .unwrap_or_else(|_| {
                std::env::var("SPANDA_CONTROL_CENTER_STATE_DIR")
                    .map(std::path::PathBuf::from)
                    .unwrap_or_else(|_| std::path::PathBuf::from(".spanda"))
                    .join("api-keys.json")
            })
    }

    /// Keys persisted to disk (excludes the in-memory `SPANDA_API_KEY` env default).
    pub fn file_backed_keys(&self) -> Vec<ApiKeyRecord> {
        self.keys
            .iter()
            .filter(|key| key.key_id != "env-default")
            .cloned()
            .collect()
    }

    /// Write file-backed keys to [`default_api_keys_file_path`].
    pub fn persist_file_keys(&self) -> Result<(), String> {
        let path = Self::default_api_keys_file_path();
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).map_err(|error| error.to_string())?;
        }
        let persisted: Vec<PersistedApiKeyRecord> = self
            .file_backed_keys()
            .into_iter()
            .map(PersistedApiKeyRecord::from)
            .collect();
        let payload =
            serde_json::to_string_pretty(&persisted).map_err(|error| error.to_string())?;
        std::fs::write(path, payload).map_err(|error| error.to_string())
    }
}

/// On-disk API key shape — never stores plaintext tokens.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct PersistedApiKeyRecord {
    pub key_id: String,
    pub token_hash: String,
    pub role: Role,
    #[serde(default)]
    pub label: Option<String>,
    #[serde(default = "default_tenant_field")]
    pub tenant_id: String,
}

impl From<ApiKeyRecord> for PersistedApiKeyRecord {
    fn from(record: ApiKeyRecord) -> Self {
        let token_hash = record
            .token_hash
            .or_else(|| {
                if record.token.is_empty() {
                    None
                } else {
                    Some(crate::auth_handler::hash_new_api_key_token(&record.token))
                }
            })
            .unwrap_or_default();
        Self {
            key_id: record.key_id,
            token_hash,
            role: record.role,
            label: record.label,
            tenant_id: record.tenant_id,
        }
    }
}

/// Permission matrix for documentation and UI.
pub fn permission_matrix() -> HashMap<String, Vec<String>> {
    let roles = [
        Role::Administrator,
        Role::Developer,
        Role::Operator,
        Role::Supervisor,
        Role::SafetyOfficer,
        Role::Auditor,
        Role::Guest,
    ];
    let actions = [
        RbacAction::Deploy,
        RbacAction::Operate,
        RbacAction::Approve,
        RbacAction::Override,
        RbacAction::Shutdown,
        RbacAction::Recover,
        RbacAction::Delete,
        RbacAction::Provision,
    ];
    let mut matrix = HashMap::new();
    for role in roles {
        let allowed: Vec<String> = actions
            .iter()
            .filter(|a| ApiKeyStore::authorize(role, **a))
            .map(|a| format!("{a:?}"))
            .collect();
        matrix.insert(format!("{role:?}"), allowed);
    }
    matrix
}

/// Generate a random 256-bit Control Center API key token (hex-encoded).
pub fn generate_api_key_token() -> String {
    // Produce a cryptographically random Bearer token for operator auth.
    //
    // Parameters:
    // None.
    //
    // Returns:
    // Lowercase hex string (64 characters).
    //
    // Options:
    // None.
    //
    // Example:
    // let token = generate_api_key_token();

    use rand::Rng;
    let mut rng = rand::thread_rng();
    let bytes: [u8; 32] = rng.gen();
    hex::encode(bytes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn operator_can_recover_not_deploy() {
        assert!(ApiKeyStore::authorize(Role::Operator, RbacAction::Recover));
        assert!(!ApiKeyStore::authorize(Role::Operator, RbacAction::Deploy));
    }

    #[test]
    fn generated_api_key_token_is_64_hex_chars() {
        let token = generate_api_key_token();
        assert_eq!(token.len(), 64);
        assert!(token.chars().all(|c| c.is_ascii_hexdigit()));
    }

    #[test]
    fn env_key_authenticates() {
        std::env::set_var("SPANDA_API_KEY", "test-token-123");
        let store = ApiKeyStore::from_env();
        let ctx = store.authenticate(Some("test-token-123"));
        assert_eq!(ctx.unwrap().role, Role::Administrator);
        std::env::remove_var("SPANDA_API_KEY");
    }
}

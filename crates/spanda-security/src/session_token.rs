//! Short-lived HS256 session JWTs for Control Center operator sign-in (OIDC and refresh).
//!
use crate::rbac::{RbacContext, Role};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

const DEFAULT_ISSUER: &str = "spanda-control-center";
const DEFAULT_TTL_SECS: u64 = 900;
const DEFAULT_REFRESH_WINDOW_SECS: u64 = 86_400;

/// Claims carried in a Control Center session JWT.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SessionClaims {
    pub sub: String,
    pub key_id: String,
    pub role: String,
    pub tenant_id: String,
    pub exp: usize,
    pub iat: usize,
    pub iss: String,
}

/// Issues and verifies short-lived operator session tokens.
#[derive(Debug, Clone)]
pub struct SessionTokenIssuer {
    secret: String,
    issuer: String,
    ttl_secs: u64,
    refresh_window_secs: u64,
}

impl Default for SessionTokenIssuer {
    fn default() -> Self {
        Self::from_env()
    }
}

impl SessionTokenIssuer {
    /// Build issuer settings from environment variables.
    pub fn from_env() -> Self {
        let secret = std::env::var("SPANDA_SESSION_JWT_SECRET")
            .or_else(|_| std::env::var("SPANDA_API_KEY_PEPPER"))
            .unwrap_or_else(|_| crate::api_key_hash::api_key_pepper());
        let issuer =
            std::env::var("SPANDA_SESSION_JWT_ISSUER").unwrap_or_else(|_| DEFAULT_ISSUER.into());
        let ttl_secs = std::env::var("SPANDA_SESSION_TTL_SECS")
            .ok()
            .and_then(|value| value.parse().ok())
            .unwrap_or(DEFAULT_TTL_SECS);
        let refresh_window_secs = std::env::var("SPANDA_SESSION_REFRESH_WINDOW_SECS")
            .ok()
            .and_then(|value| value.parse().ok())
            .unwrap_or(DEFAULT_REFRESH_WINDOW_SECS);
        Self {
            secret,
            issuer,
            ttl_secs,
            refresh_window_secs,
        }
    }

    /// Return configured session TTL in seconds.
    pub fn ttl_secs(&self) -> u64 {
        self.ttl_secs
    }

    /// Issue a signed session JWT for an operator directory user.
    pub fn issue(
        &self,
        user_id: &str,
        role: Role,
        tenant_id: &str,
        now_secs: u64,
    ) -> Result<String, String> {
        // Mint a bearer JWT for browser and SDK session auth.
        //
        // Parameters:
        // - `user_id` — operator directory id (OIDC `sub` or Spanda user id)
        // - `role` — RBAC role mapped from IdP groups or directory record
        // - `tenant_id` — tenant scope for the session
        // - `now_secs` — current unix timestamp
        //
        // Returns:
        // Compact serialized JWT string.
        //
        // Options:
        // TTL from `SPANDA_SESSION_TTL_SECS` (default 900).
        //
        // Example:
        // let token = issuer.issue("alice", Role::Operator, "default", now)?;

        let claims = SessionClaims {
            sub: user_id.to_string(),
            key_id: format!("session-{user_id}"),
            role: format!("{role:?}").to_ascii_lowercase(),
            tenant_id: tenant_id.to_string(),
            iat: now_secs as usize,
            exp: (now_secs + self.ttl_secs) as usize,
            iss: self.issuer.clone(),
        };
        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.secret.as_bytes()),
        )
        .map_err(|error| error.to_string())
    }

    /// Validate a bearer JWT and map it to RBAC context.
    pub fn verify(&self, token: &str) -> Result<RbacContext, String> {
        // Decode and validate signature, issuer, and expiry.
        //
        // Parameters:
        // - `token` — compact JWT from `Authorization: Bearer`
        //
        // Returns:
        // RBAC context for the session principal.
        //
        // Options:
        // None.
        //
        // Example:
        // let ctx = issuer.verify(bearer_token)?;

        let mut validation = Validation::default();
        validation.set_issuer(&[self.issuer.as_str()]);
        let data = decode::<SessionClaims>(
            token,
            &DecodingKey::from_secret(self.secret.as_bytes()),
            &validation,
        )
        .map_err(|error| error.to_string())?;
        let claims = data.claims;
        let role = Role::parse(&claims.role);
        Ok(RbacContext {
            key_id: claims.key_id,
            role,
            tenant_id: claims.tenant_id,
            auth_kind: crate::auth_handler::AuthKind::Session,
            user_id: Some(claims.sub),
        })
    }

    /// Refresh a session when it is expired but still inside the refresh window.
    pub fn refresh(&self, token: &str, now_secs: u64) -> Result<String, String> {
        // Re-issue a JWT when the prior token expired recently.
        //
        // Parameters:
        // - `token` — prior session JWT (may be expired)
        // - `now_secs` — current unix timestamp
        //
        // Returns:
        // New session JWT.
        //
        // Options:
        // Refresh window from `SPANDA_SESSION_REFRESH_WINDOW_SECS`.
        //
        // Example:
        // let next = issuer.refresh(old_token, now)?;

        let mut validation = Validation::default();
        validation.set_issuer(&[self.issuer.as_str()]);
        validation.validate_exp = false;
        let data = decode::<SessionClaims>(
            token,
            &DecodingKey::from_secret(self.secret.as_bytes()),
            &validation,
        )
        .map_err(|error| error.to_string())?;
        let claims = data.claims;
        if now_secs > claims.exp as u64 + self.refresh_window_secs {
            return Err("refresh window elapsed".into());
        }
        let role = Role::parse(&claims.role);
        self.issue(&claims.sub, role, &claims.tenant_id, now_secs)
    }

    /// Return `true` when the bearer value looks like a JWT (three base64url segments).
    pub fn looks_like_jwt(token: &str) -> bool {
        let parts: Vec<&str> = token.split('.').collect();
        parts.len() == 3 && parts.iter().all(|part| !part.is_empty())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn issue_and_verify_round_trip() {
        std::env::set_var("SPANDA_SESSION_JWT_SECRET", "test-session-secret");
        let issuer = SessionTokenIssuer::from_env();
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|duration| duration.as_secs())
            .unwrap_or(0);
        let token = issuer
            .issue("alice", Role::Operator, "default", now)
            .expect("issue");
        assert!(SessionTokenIssuer::looks_like_jwt(&token));
        let ctx = issuer.verify(&token).expect("verify");
        assert_eq!(ctx.user_id.as_deref(), Some("alice"));
        assert_eq!(ctx.role, Role::Operator);
        std::env::remove_var("SPANDA_SESSION_JWT_SECRET");
    }
}

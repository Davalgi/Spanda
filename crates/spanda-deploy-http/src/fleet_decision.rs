//! Fleet mesh distributed decision HTTP client.

use crate::{http_request, HttpResponse};
use serde::{Deserialize, Serialize};

/// One competing decision vote on the fleet mesh.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FleetCompetingDecision {
    pub layer_precedence: String,
    pub entity_id: String,
    pub action: String,
    pub reason: String,
}

/// Resolved fleet decision conflict from mesh coordinator.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FleetConflictResolution {
    pub winner: FleetCompetingDecision,
    pub rejected: Vec<FleetCompetingDecision>,
    pub precedence_applied: String,
}

/// Decision vote posted to a fleet mesh coordinator.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FleetDecisionVoteIngestRequest {
    pub round_id: String,
    pub entity_id: String,
    pub action: String,
    pub layer_precedence: String,
    pub reason: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fleet_name: Option<String>,
}

/// Result of ingesting one decision vote.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FleetDecisionVoteIngestResponse {
    pub ok: bool,
    pub votes: u32,
}

/// Resolved fleet decision conflict from mesh coordinator.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FleetDecisionConflictResponse {
    pub ok: bool,
    pub round_id: String,
    #[serde(default)]
    pub fleet_name: String,
    pub resolution: FleetConflictResolution,
}

/// Nonce registration request for shared fleet registry.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FleetDecisionNonceRegisterRequest {
    pub nonce: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub entity_id: Option<String>,
}

/// Nonce registration response from mesh coordinator.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FleetDecisionNonceRegisterResponse {
    pub ok: bool,
    pub accepted: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    pub registry_size: u32,
}

fn mesh_decision_url(mesh_url: &str, suffix: &str, query: &str) -> String {
    let path = if suffix.is_empty() {
        "v1/fleet/decisions".to_string()
    } else {
        format!("v1/fleet/decisions/{suffix}")
    };
    let base = if mesh_url.ends_with('/') {
        format!("{mesh_url}{path}")
    } else {
        format!("{mesh_url}/{path}")
    };
    if query.is_empty() {
        base
    } else {
        format!("{base}?{query}")
    }
}

/// Ingest a decision vote into the mesh coordinator.
pub fn ingest_fleet_decision_vote(
    mesh_url: &str,
    request: &FleetDecisionVoteIngestRequest,
    token: Option<&str>,
) -> Result<FleetDecisionVoteIngestResponse, String> {
    let body = serde_json::to_string(request).map_err(|error| error.to_string())?;
    let response = http_request(
        "POST",
        &mesh_decision_url(mesh_url, "vote/ingest", ""),
        Some(&body),
        token,
    )?;
    parse_vote_ingest_response(response)
}

/// Fetch resolved fleet decision conflict for a round.
pub fn fetch_fleet_decision_conflict(
    mesh_url: &str,
    round_id: &str,
    token: Option<&str>,
) -> Result<FleetDecisionConflictResponse, String> {
    let query = format!("round={round_id}");
    let response = http_request(
        "GET",
        &mesh_decision_url(mesh_url, "conflicts", &query),
        None,
        token,
    )?;
    if (200..300).contains(&response.status) {
        return serde_json::from_str(&response.body)
            .map_err(|error| format!("invalid fleet decision conflict JSON: {error}"));
    }
    Err(format!(
        "fleet decision conflict HTTP {}: {}",
        response.status, response.body
    ))
}

/// Register a nonce on the shared fleet mesh registry.
pub fn register_fleet_decision_nonce(
    mesh_url: &str,
    request: &FleetDecisionNonceRegisterRequest,
    token: Option<&str>,
) -> Result<FleetDecisionNonceRegisterResponse, String> {
    let body = serde_json::to_string(request).map_err(|error| error.to_string())?;
    let response = http_request(
        "POST",
        &mesh_decision_url(mesh_url, "nonce/register", ""),
        Some(&body),
        token,
    )?;
    if (200..300).contains(&response.status) {
        return serde_json::from_str(&response.body)
            .map_err(|error| format!("invalid nonce register JSON: {error}"));
    }
    if response.status == 409 {
        return serde_json::from_str(&response.body).map_err(|error| error.to_string());
    }
    Err(format!(
        "fleet nonce register HTTP {}: {}",
        response.status, response.body
    ))
}

/// Fetch fleet mesh decision and nonce registry diagnostics.
pub fn fetch_fleet_decision_mesh_status(
    mesh_url: &str,
    token: Option<&str>,
) -> Result<serde_json::Value, String> {
    let nonce = http_request(
        "GET",
        &mesh_decision_url(mesh_url, "nonce/status", ""),
        None,
        token,
    )?;
    let status = http_request("GET", &mesh_base_url(mesh_url, "v1/status"), None, token)?;
    let nonce_json: serde_json::Value =
        serde_json::from_str(&nonce.body).unwrap_or_else(|_| serde_json::json!({ "raw": nonce.body }));
    let status_json: serde_json::Value =
        serde_json::from_str(&status.body).unwrap_or_else(|_| serde_json::json!({ "raw": status.body }));
    Ok(serde_json::json!({
        "ok": true,
        "mesh_url": mesh_url,
        "nonce": nonce_json,
        "mesh_status": status_json,
    }))
}

fn mesh_base_url(mesh_url: &str, path: &str) -> String {
    if mesh_url.ends_with('/') {
        format!("{mesh_url}{path}")
    } else {
        format!("{mesh_url}/{path}")
    }
}

fn parse_vote_ingest_response(response: HttpResponse) -> Result<FleetDecisionVoteIngestResponse, String> {
    if (200..300).contains(&response.status) {
        return serde_json::from_str(&response.body)
            .map_err(|error| format!("invalid fleet decision vote ingest JSON: {error}"));
    }
    Err(format!(
        "fleet decision vote ingest HTTP {}: {}",
        response.status, response.body
    ))
}

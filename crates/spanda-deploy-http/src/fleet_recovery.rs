//! Fleet mesh recovery HTTP client types and coordinator POST helper.
//!
use crate::{http_request, parse_http_url};
use serde::{Deserialize, Serialize};

/// Recovery command posted to the fleet mesh coordinator.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FleetRecoveryRequest {
    pub action: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fleet_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub from_robot: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub members: Vec<String>,
}

/// Result of broadcasting a recovery command to fleet agents.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FleetRecoveryResponse {
    pub ok: bool,
    pub relayed: u32,
    pub failed: u32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

/// Post a recovery command to a running fleet mesh coordinator.
pub fn relay_recovery_via_mesh(
    mesh_url: &str,
    request: &FleetRecoveryRequest,
    token: Option<&str>,
) -> Result<FleetRecoveryResponse, String> {
    // Description:
    //     Relay recovery via mesh.
    //
    // Inputs:
    //     mesh_url: &str
    //         Fleet mesh coordinator base URL.
    //     request: &FleetRecoveryRequest
    //         Recovery command payload to broadcast.
    //     token: Option<&str>
    //         Optional bearer token for authenticated mesh calls.
    //
    // Outputs:
    //     result: Result<FleetRecoveryResponse, String>
    //         Return value from `relay_recovery_via_mesh`.
    //
    // Example:

    //     let result = spanda_deploy_http::fleet_recovery::relay_recovery_via_mesh(mesh_url, reques, oken);

    let parsed = parse_http_url(mesh_url)?;
    let url = format!(
        "{}://{}:{}/v1/fleet/recovery",
        parsed.scheme, parsed.host, parsed.port
    );
    let payload = serde_json::to_string(request).map_err(|e| e.to_string())?;
    let response = http_request("POST", &url, Some(&payload), token)?;
    if response.status >= 400 {
        return Err(format!(
            "fleet mesh recovery HTTP {}: {}",
            response.status, response.body
        ));
    }
    serde_json::from_str(&response.body).map_err(|e| format!("invalid fleet recovery JSON: {e}"))
}

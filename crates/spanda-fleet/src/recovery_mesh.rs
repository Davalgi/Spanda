//! Fleet recovery command relay through the mesh coordinator.
//!
use crate::remote::{lookup_fleet_agent, relay_peer_deliveries, FleetAgentRegistry};
use crate::PeerDelivery;
use serde::{Deserialize, Serialize};
use spanda_deploy_http::{http_request, parse_http_url, HttpResponse};

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

/// Build peer deliveries for a fleet recovery command.
pub fn recovery_deliveries_for_request(request: &FleetRecoveryRequest) -> Vec<PeerDelivery> {
    // Turn a mesh recovery request into per-robot peer deliveries.
    let from_robot = request
        .from_robot
        .clone()
        .unwrap_or_else(|| "coordinator".into());
    let targets: Vec<String> = if !request.members.is_empty() {
        request.members.clone()
    } else {
        Vec::new()
    };
    targets
        .into_iter()
        .map(|to_robot| PeerDelivery {
            from_robot: from_robot.clone(),
            to_robot,
            topic: "fleet_recovery".into(),
            step: request.action.clone(),
            delivered: false,
        })
        .collect()
}

/// Relay a recovery command to registered fleet agents.
pub fn relay_fleet_recovery(
    request: &FleetRecoveryRequest,
    registry: &FleetAgentRegistry,
) -> FleetRecoveryResponse {
    // Fan out recovery peer messages to every targeted fleet agent.
    let deliveries = recovery_deliveries_for_request(request);
    if deliveries.is_empty() {
        return FleetRecoveryResponse {
            ok: false,
            relayed: 0,
            failed: 0,
            error: Some("no fleet members targeted for recovery".into()),
        };
    }
    let (relayed, failed) = relay_peer_deliveries(&deliveries, registry);
    FleetRecoveryResponse {
        ok: failed == 0,
        relayed,
        failed,
        error: if failed > 0 {
            Some(format!("{failed} fleet recovery relay(s) failed"))
        } else {
            None
        },
    }
}

fn recovery_http_response(response: &FleetRecoveryResponse) -> HttpResponse {
    HttpResponse {
        status: 200,
        body: serde_json::to_string(response).unwrap_or_else(|_| "{}".into()),
    }
}

/// Handle `POST /v1/fleet/recovery` on the mesh coordinator.
pub fn handle_fleet_recovery_post(body: &str, registry: &FleetAgentRegistry) -> HttpResponse {
    // Parse the recovery payload and relay it to fleet agents.
    let Ok(request) = serde_json::from_str::<FleetRecoveryRequest>(body) else {
        return HttpResponse {
            status: 400,
            body: r#"{"ok":false,"error":"invalid fleet recovery payload"}"#.into(),
        };
    };
    if request.action.trim().is_empty() {
        return HttpResponse {
            status: 400,
            body: r#"{"ok":false,"error":"recovery action required"}"#.into(),
        };
    }
    let response = relay_fleet_recovery(&request, registry);
    recovery_http_response(&response)
}

/// Post a recovery command to a running fleet mesh coordinator.
pub fn relay_recovery_via_mesh(
    mesh_url: &str,
    request: &FleetRecoveryRequest,
    token: Option<&str>,
) -> Result<FleetRecoveryResponse, String> {
    // Send a recovery command to the mesh coordinator HTTP endpoint.
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

/// Return fleet members with registered remote agents.
pub fn registered_recovery_members(
    members: &[String],
    registry: &FleetAgentRegistry,
) -> Vec<String> {
    members
        .iter()
        .filter(|member| lookup_fleet_agent(registry, member).is_some())
        .cloned()
        .collect()
}

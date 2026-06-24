//! Fleet takeover command relay through the mesh coordinator.
//!
use crate::remote::{lookup_fleet_agent, relay_peer_deliveries, FleetAgentRegistry};
use crate::PeerDelivery;
use spanda_deploy_http::HttpResponse;

pub use spanda_deploy_http::{
    relay_continuity_via_mesh, FleetContinuityRequest, FleetContinuityResponse,
};

/// Build peer deliveries for a fleet takeover command.
pub fn continuity_deliveries_for_request(request: &FleetContinuityRequest) -> Vec<PeerDelivery> {
    let from_robot = request
        .from_robot
        .clone()
        .unwrap_or_else(|| request.failed_robot.clone());
    let payload = serde_json::to_string(request).unwrap_or_default();
    let targets: Vec<String> = if !request.members.is_empty() {
        request.members.clone()
    } else if let Some(successor) = &request.successor {
        vec![successor.clone(), request.failed_robot.clone()]
    } else {
        Vec::new()
    };
    targets
        .into_iter()
        .map(|to_robot| PeerDelivery {
            from_robot: from_robot.clone(),
            to_robot,
            topic: "fleet_takeover".into(),
            step: payload.clone(),
            delivered: false,
        })
        .collect()
}

/// Relay a takeover command to registered fleet agents.
pub fn relay_fleet_continuity(
    request: &FleetContinuityRequest,
    registry: &FleetAgentRegistry,
) -> FleetContinuityResponse {
    let deliveries = continuity_deliveries_for_request(request);
    if deliveries.is_empty() {
        return FleetContinuityResponse {
            ok: false,
            relayed: 0,
            failed: 0,
            error: Some("no fleet members targeted for takeover".into()),
        };
    }
    let (relayed, failed) = relay_peer_deliveries(&deliveries, registry);
    FleetContinuityResponse {
        ok: failed == 0,
        relayed,
        failed,
        error: if failed > 0 {
            Some(format!("{failed} fleet takeover relay(s) failed"))
        } else {
            None
        },
    }
}

fn continuity_http_response(response: &FleetContinuityResponse) -> HttpResponse {
    HttpResponse {
        status: 200,
        body: serde_json::to_string(response).unwrap_or_else(|_| "{}".into()),
    }
}

/// Handle `POST /v1/fleet/continuity` on the mesh coordinator.
pub fn handle_fleet_continuity_post(body: &str, registry: &FleetAgentRegistry) -> HttpResponse {
    let Ok(request) = serde_json::from_str::<FleetContinuityRequest>(body) else {
        return HttpResponse {
            status: 400,
            body: r#"{"ok":false,"error":"invalid fleet continuity payload"}"#.into(),
        };
    };
    if request.failed_robot.trim().is_empty() {
        return HttpResponse {
            status: 400,
            body: r#"{"ok":false,"error":"failed_robot required"}"#.into(),
        };
    }
    let response = relay_fleet_continuity(&request, registry);
    continuity_http_response(&response)
}

/// Return fleet members with registered remote agents.
pub fn registered_continuity_members(
    members: &[String],
    registry: &FleetAgentRegistry,
) -> Vec<String> {
    members
        .iter()
        .filter(|member| lookup_fleet_agent(registry, member).is_some())
        .cloned()
        .collect()
}

//! Fleet recovery command relay through the mesh coordinator.
//!
use crate::remote::{lookup_fleet_agent, relay_peer_deliveries, FleetAgentRegistry};
use crate::PeerDelivery;
use spanda_deploy_http::HttpResponse;

pub use spanda_deploy_http::{
    relay_recovery_via_mesh, FleetRecoveryRequest, FleetRecoveryResponse,
};

/// Build peer deliveries for a fleet recovery command.
pub fn recovery_deliveries_for_request(request: &FleetRecoveryRequest) -> Vec<PeerDelivery> {
    // Description:
    //     Recovery deliveries for request.
    //
    // Inputs:
    //     request: &FleetRecoveryRequest
    //         Caller-supplied request.
    //
    // Outputs:
    //     result: Vec<PeerDelivery>
    //         Return value from `recovery_deliveries_for_request`.
    //
    // Example:
    //     let result = spanda_fleet::recovery_mesh::recovery_deliveries_for_request(reques);
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
    // Description:
    //     Relay fleet recovery.
    //
    // Inputs:
    //     request: &FleetRecoveryRequest
    //         Caller-supplied request.
    //     registry: &FleetAgentRegistry
    //         Caller-supplied registry.
    //
    // Outputs:
    //     result: FleetRecoveryResponse
    //         Return value from `relay_fleet_recovery`.
    //
    // Example:
    //     let result = spanda_fleet::recovery_mesh::relay_fleet_recovery(reques, registry);
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
    // Description:
    //     Recovery http response.
    //
    // Inputs:
    //     response: &FleetRecoveryResponse
    //         Caller-supplied response.
    //
    // Outputs:
    //     result: HttpResponse
    //         Return value from `recovery_http_response`.
    //
    // Example:

    //     let result = spanda_fleet::recovery_mesh::recovery_http_response(response);

    HttpResponse {
        status: 200,
        body: serde_json::to_string(response).unwrap_or_else(|_| "{}".into()),
    }
}

/// Handle `POST /v1/fleet/recovery` on the mesh coordinator.
pub fn handle_fleet_recovery_post(body: &str, registry: &FleetAgentRegistry) -> HttpResponse {
    // Description:
    //     Handle fleet recovery post.
    //
    // Inputs:
    //     body: &str
    //         Caller-supplied body.
    //     registry: &FleetAgentRegistry
    //         Caller-supplied registry.
    //
    // Outputs:
    //     result: HttpResponse
    //         Return value from `handle_fleet_recovery_post`.
    //
    // Example:
    //     let result = spanda_fleet::recovery_mesh::handle_fleet_recovery_post(body, registry);
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

/// Return fleet members with registered remote agents.
pub fn registered_recovery_members(
    members: &[String],
    registry: &FleetAgentRegistry,
) -> Vec<String> {
    // Description:
    //     Registered recovery members.
    //
    // Inputs:
    //     embers: &[String]
    //         Caller-supplied embers.
    //     registry: &FleetAgentRegistry
    //         Caller-supplied registry.
    //
    // Outputs:
    //     result: Vec<String>
    //         Return value from `registered_recovery_members`.
    //
    // Example:

    //     let result = spanda_fleet::recovery_mesh::registered_recovery_members(embers, registry);

    members
        .iter()
        .filter(|member| lookup_fleet_agent(registry, member).is_some())
        .cloned()
        .collect()
}

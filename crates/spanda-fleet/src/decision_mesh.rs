//! Fleet mesh distributed decision vote ingest, conflict resolution, and shared nonce registry.

use crate::mesh::FleetMeshState;
use spanda_decision::{resolve_conflict, CompetingDecision};
use spanda_deploy_http::{
    FleetCompetingDecision, FleetConflictResolution, FleetDecisionConflictResponse,
    FleetDecisionNonceRegisterRequest, FleetDecisionNonceRegisterResponse,
    FleetDecisionVoteIngestRequest, FleetDecisionVoteIngestResponse, HttpResponse,
};

/// Handle `POST /v1/fleet/decisions/vote/ingest` on the mesh coordinator.
pub fn handle_fleet_decision_vote_ingest_post(
    body: &str,
    state: &mut FleetMeshState,
) -> HttpResponse {
    let payload: FleetDecisionVoteIngestRequest = match serde_json::from_str(body) {
        Ok(value) => value,
        Err(_) => {
            return HttpResponse {
                status: 400,
                body: r#"{"ok":false,"error":"invalid fleet decision vote payload"}"#.into(),
            };
        }
    };
    if payload.round_id.trim().is_empty() || payload.entity_id.trim().is_empty() {
        return HttpResponse {
            status: 400,
            body: r#"{"ok":false,"error":"round_id and entity_id are required"}"#.into(),
        };
    }
    let vote = CompetingDecision {
        layer_precedence: payload.layer_precedence,
        entity_id: payload.entity_id,
        action: payload.action,
        reason: payload.reason,
    };
    state
        .decision_votes
        .entry(payload.round_id.clone())
        .or_default()
        .push(vote);
    if let Some(fleet_name) = payload.fleet_name {
        state.decision_fleet_name = fleet_name;
    }
    state.decision_ingest_total = state.decision_ingest_total.saturating_add(1);
    let response = FleetDecisionVoteIngestResponse {
        ok: true,
        votes: state
            .decision_votes
            .get(&payload.round_id)
            .map(|v| v.len() as u32)
            .unwrap_or(0),
    };
    HttpResponse {
        status: 200,
        body: serde_json::to_string(&response).unwrap_or_else(|_| r#"{"ok":false}"#.into()),
    }
}

/// Handle `GET /v1/fleet/decisions/conflicts` — resolve competing votes for a round.
pub fn handle_fleet_decision_conflicts_get(path: &str, state: &FleetMeshState) -> HttpResponse {
    let round_id = parse_round_query(path).unwrap_or_default();
    if round_id.is_empty() {
        return HttpResponse {
            status: 400,
            body: r#"{"ok":false,"error":"round query parameter required"}"#.into(),
        };
    }
    match resolve_mesh_decision_conflict(&round_id, state) {
        Ok(body) => HttpResponse { status: 200, body },
        Err(error) => HttpResponse {
            status: 400,
            body: format!(r#"{{"ok":false,"error":"{error}"}}"#),
        },
    }
}

/// Resolve competing decision votes for a mesh round.
pub fn resolve_mesh_decision_conflict(
    round_id: &str,
    state: &FleetMeshState,
) -> Result<String, String> {
    let votes = state
        .decision_votes
        .get(round_id)
        .ok_or_else(|| format!("no decision votes for round '{round_id}'"))?;
    if votes.is_empty() {
        return Err("decision vote list is empty".into());
    }
    let resolution = resolve_conflict(votes).ok_or_else(|| "conflict resolution failed".to_string())?;
    let response = FleetDecisionConflictResponse {
        ok: true,
        round_id: round_id.to_string(),
        fleet_name: state.decision_fleet_name.clone(),
        resolution: FleetConflictResolution {
            winner: FleetCompetingDecision {
                layer_precedence: resolution.winner.layer_precedence,
                entity_id: resolution.winner.entity_id,
                action: resolution.winner.action,
                reason: resolution.winner.reason,
            },
            rejected: resolution
                .rejected
                .into_iter()
                .map(|d| FleetCompetingDecision {
                    layer_precedence: d.layer_precedence,
                    entity_id: d.entity_id,
                    action: d.action,
                    reason: d.reason,
                })
                .collect(),
            precedence_applied: resolution.precedence_applied,
        },
    };
    serde_json::to_string(&response).map_err(|e| e.to_string())
}

/// Handle `POST /v1/fleet/decisions/nonce/register` — shared fleet nonce registry.
pub fn handle_fleet_decision_nonce_register_post(
    body: &str,
    state: &mut FleetMeshState,
) -> HttpResponse {
    let payload: FleetDecisionNonceRegisterRequest = match serde_json::from_str(body) {
        Ok(value) => value,
        Err(_) => {
            return HttpResponse {
                status: 400,
                body: r#"{"ok":false,"error":"invalid nonce register payload"}"#.into(),
            };
        }
    };
    if payload.nonce.trim().is_empty() {
        return HttpResponse {
            status: 400,
            body: r#"{"ok":false,"error":"nonce is required"}"#.into(),
        };
    }
    let result = state.decision_nonce_registry.register(&payload.nonce);
    state.decision_nonce_register_total = state.decision_nonce_register_total.saturating_add(1);
    let response = FleetDecisionNonceRegisterResponse {
        ok: result.is_ok(),
        accepted: result.is_ok(),
        error: result.err(),
        registry_size: state.decision_nonce_registry.seen.len() as u32,
    };
    HttpResponse {
        status: if response.accepted { 200 } else { 409 },
        body: serde_json::to_string(&response).unwrap_or_else(|_| r#"{"ok":false}"#.into()),
    }
}

/// Handle `GET /v1/fleet/decisions/nonce/status` — diagnostics for shared nonce registry.
pub fn handle_fleet_decision_nonce_status_get(state: &FleetMeshState) -> HttpResponse {
    HttpResponse {
        status: 200,
        body: serde_json::to_string(&serde_json::json!({
            "ok": true,
            "registry_size": state.decision_nonce_registry.seen.len(),
            "register_total": state.decision_nonce_register_total,
            "updated_at_ms": state.decision_nonce_registry.updated_at_ms,
        }))
        .unwrap_or_else(|_| "{}".into()),
    }
}

fn parse_round_query(path: &str) -> Option<String> {
    let (_, query) = path.split_once('?')?;
    for pair in query.split('&') {
        if let Some(value) = pair.strip_prefix("round=") {
            return Some(value.to_string());
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ingests_votes_and_resolves_split_brain() {
        let mut state = FleetMeshState::default();
        let round = "round-1";
        for (entity, action, precedence) in [
            ("RoverA", "continue_mission", "fleet_coordination"),
            ("RoverB", "emergency_stop", "safety_kill_switch"),
        ] {
            let ingest = handle_fleet_decision_vote_ingest_post(
                &serde_json::to_string(&FleetDecisionVoteIngestRequest {
                    round_id: round.into(),
                    entity_id: entity.into(),
                    action: action.into(),
                    layer_precedence: precedence.into(),
                    reason: "test".into(),
                    fleet_name: Some("PatrolFleet".into()),
                })
                .unwrap(),
                &mut state,
            );
            assert_eq!(ingest.status, 200);
        }
        let resolved = resolve_mesh_decision_conflict(round, &state).expect("resolve");
        assert!(resolved.contains("emergency_stop"));
    }

    #[test]
    fn shared_nonce_registry_rejects_replay() {
        let mut state = FleetMeshState::default();
        let body = serde_json::to_string(&FleetDecisionNonceRegisterRequest {
            nonce: "mesh-nonce-1".into(),
            entity_id: Some("RoverA".into()),
        })
        .unwrap();
        let first = handle_fleet_decision_nonce_register_post(&body, &mut state);
        assert_eq!(first.status, 200);
        let replay = handle_fleet_decision_nonce_register_post(&body, &mut state);
        assert_eq!(replay.status, 409);
    }
}

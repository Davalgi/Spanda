//! REST API stubs for bio-inspired resilient autonomy (CLI/SDK parity).
//!
use spanda_autonomy::attention::{compute_attention_score, AttentionPolicy, EventPriority};
use spanda_autonomy::types::AutonomySeverity;
use spanda_autonomy::{
    enrich_entity_autonomy, evaluate_homeostasis, evaluate_quarantine_decision,
    list_reflex_actions, rank_events, EntityAutonomyContext, HomeostasisPolicy, ImmunePolicy,
    StabilityMetric,
};
use spanda_deploy_http::HttpResponse;

use crate::handlers::json_ok;
use crate::state::ControlCenterState;

const API_VERSION: &str = "v1";

fn entity_not_found(message: &str) -> HttpResponse {
    HttpResponse {
        status: 404,
        body: serde_json::json!({ "ok": false, "error": message }).to_string(),
    }
}

/// GET /v1/autonomy/reflex — list platform reflex actions.
pub fn list_reflex(state: &ControlCenterState) -> HttpResponse {
    let _ = state;
    json_ok(&serde_json::json!({
        "version": API_VERSION,
        "reflexes": list_reflex_actions(),
    }))
}

/// GET /v1/autonomy/homeostasis — platform homeostasis summary.
pub fn homeostasis_summary(state: &ControlCenterState) -> HttpResponse {
    let registry = state.entity_registry();
    let policy = HomeostasisPolicy::platform_defaults();
    let reports: Vec<_> = registry
        .entities
        .values()
        .take(20)
        .map(|entity| {
            let metrics = demo_metrics();
            evaluate_homeostasis(entity, &metrics, &policy)
        })
        .collect();
    json_ok(&serde_json::json!({
        "version": API_VERSION,
        "reports": reports,
    }))
}

/// GET /v1/autonomy/immunity — immunity scan across entities.
pub fn immunity_scan(state: &ControlCenterState) -> HttpResponse {
    let registry = state.entity_registry();
    let policy = ImmunePolicy::platform_defaults();
    let decisions: Vec<_> = registry
        .entities
        .values()
        .map(|e| evaluate_quarantine_decision(e, &policy))
        .filter(|d| d.quarantine)
        .collect();
    json_ok(&serde_json::json!({
        "version": API_VERSION,
        "quarantined": decisions,
    }))
}

/// GET /v1/autonomy/attention — attention queue placeholder.
pub fn attention_queue(state: &ControlCenterState) -> HttpResponse {
    let _ = state;
    let scores = vec![
        compute_attention_score(
            "evt-1",
            "collision_imminent",
            EventPriority::Critical,
            AutonomySeverity::Critical,
        ),
        compute_attention_score(
            "evt-2",
            "routine_telemetry",
            EventPriority::Routine,
            AutonomySeverity::Info,
        ),
    ];
    let window = rank_events(scores, &AttentionPolicy::default());
    json_ok(&serde_json::json!({
        "version": API_VERSION,
        "attention": window,
    }))
}

/// GET /v1/entities/{id}/autonomy — entity autonomy profile.
pub fn entity_autonomy(state: &ControlCenterState, entity_id: &str) -> HttpResponse {
    let registry = state.entity_registry();
    let Some(mut entity) = registry.get(entity_id).cloned() else {
        return entity_not_found(&format!("entity not found: {entity_id}"));
    };
    let ctx = EntityAutonomyContext {
        metrics: demo_metrics(),
        ..Default::default()
    };
    enrich_entity_autonomy(&mut entity, &ctx);
    json_ok(&serde_json::json!({
        "version": API_VERSION,
        "entity_id": entity_id,
        "autonomy": entity.autonomy,
    }))
}

fn demo_metrics() -> Vec<StabilityMetric> {
    vec![
        StabilityMetric {
            name: "cpu_pct".into(),
            value: 45.0,
            unit: "pct".into(),
        },
        StabilityMetric {
            name: "memory_pct".into(),
            value: 58.0,
            unit: "pct".into(),
        },
    ]
}

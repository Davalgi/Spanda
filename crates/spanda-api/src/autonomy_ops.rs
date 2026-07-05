//! REST API for bio-inspired resilient autonomy (CLI/SDK parity).
//!
use spanda_autonomy::attention::{compute_attention_score, AttentionPolicy, EventPriority};
use spanda_autonomy::reflex::{evaluate_reflex_priority, ReflexTrace};
use spanda_autonomy::types::AutonomySeverity;
use spanda_autonomy::{
    enrich_entity_autonomy, evaluate_homeostasis, evaluate_quarantine_decision,
    list_reflex_actions, rank_events, recovery_confidence_from_history, EntityAutonomyContext,
    HomeostasisPolicy, ImmunePolicy,
};
use spanda_autonomy::adaptive_recovery::RecoveryHistory;
use spanda_deploy_http::HttpResponse;

use crate::handlers::json_ok;
use crate::recovery_plugins::orchestrator_for_state;
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

/// GET /v1/autonomy/reflex/traces — reflex trace catalog from runtime buffer + defaults.
pub fn list_reflex_traces(state: &ControlCenterState) -> HttpResponse {
    let registry = state.entity_registry();
    let entity_id = registry
        .entities
        .keys()
        .next()
        .cloned()
        .unwrap_or_else(|| "platform".into());
    let recorded = spanda_autonomy::list_recorded_reflex_traces();
    if !recorded.is_empty() {
        return json_ok(&serde_json::json!({
            "version": API_VERSION,
            "traces": recorded,
            "source": "runtime",
        }));
    }
    let actions = list_reflex_actions();
    let traces: Vec<ReflexTrace> = ["emergency", "obstacle", "thermal"]
        .iter()
        .filter_map(|hint| {
            evaluate_reflex_priority(&actions, hint).map(|action| ReflexTrace {
                reflex_id: action.id.clone(),
                entity_id: entity_id.clone(),
                trigger: action.trigger.clone(),
                action_taken: action.action.clone(),
                timestamp: chrono::Utc::now().to_rfc3339(),
                priority: action.priority,
            })
        })
        .collect();
    json_ok(&serde_json::json!({
        "version": API_VERSION,
        "traces": traces,
        "source": "catalog",
    }))
}

/// GET /v1/autonomy/homeostasis — platform homeostasis summary from entity signals.
pub fn homeostasis_summary(state: &ControlCenterState) -> HttpResponse {
    let registry = state.entity_registry();
    let policy = HomeostasisPolicy::platform_defaults();
    let reports: Vec<_> = registry
        .entities
        .values()
        .take(20)
        .map(|entity| {
            let ctx = EntityAutonomyContext::from_entity(entity);
            evaluate_homeostasis(entity, &ctx.metrics, &policy)
        })
        .collect();
    json_ok(&serde_json::json!({
        "version": API_VERSION,
        "reports": reports,
    }))
}

/// GET /v1/autonomy/immunity — immunity scan across entities (trust/tamper integration).
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

/// GET /v1/autonomy/attention — attention queue from entity health/readiness signals.
pub fn attention_queue(state: &ControlCenterState) -> HttpResponse {
    let registry = state.entity_registry();
    let mut scores = vec![compute_attention_score(
        "platform",
        "routine_telemetry",
        EventPriority::Routine,
        AutonomySeverity::Info,
    )];
    for entity in registry.entities.values().take(10) {
        let (priority, severity, label) = match entity.health_status {
            spanda_config::EntityHealthStatus::Critical => (
                EventPriority::Critical,
                AutonomySeverity::Critical,
                format!("health_critical:{}", entity.id),
            ),
            spanda_config::EntityHealthStatus::Degraded => (
                EventPriority::Urgent,
                AutonomySeverity::High,
                format!("health_degraded:{}", entity.id),
            ),
            _ => continue,
        };
        scores.push(compute_attention_score(
            &entity.id,
            &label,
            priority,
            severity,
        ));
    }
    let window = rank_events(scores, &AttentionPolicy::default());
    json_ok(&serde_json::json!({
        "version": API_VERSION,
        "attention": window,
    }))
}

/// GET /v1/entities/{id}/autonomy — entity autonomy profile with runtime enrichment.
pub fn entity_autonomy(state: &ControlCenterState, entity_id: &str) -> HttpResponse {
    let registry = state.entity_registry();
    let Some(mut entity) = registry.get(entity_id).cloned() else {
        return entity_not_found(&format!("entity not found: {entity_id}"));
    };
    let orchestrator = orchestrator_for_state(state);
    let recovery_history: Vec<RecoveryHistory> = orchestrator
        .history()
        .recent(50)
        .into_iter()
        .filter(|e| e.entities_involved.iter().any(|id| id == entity_id))
        .map(|e| RecoveryHistory {
            entity_id: entity_id.into(),
            strategy: format!("{:?}", e.strategy),
            success: e.status == spanda_runtime::recovery_types::RecoveryStatus::Success,
            duration_ms: e.duration_secs.saturating_mul(1000),
        })
        .collect();
    let ctx = EntityAutonomyContext::from_entity(&entity).with_recovery_history(recovery_history);
    enrich_entity_autonomy(&mut entity, &ctx);
    let recovery_confidence = recovery_confidence_from_history(
        entity_id,
        &ctx.recovery_history,
    );
    json_ok(&serde_json::json!({
        "version": API_VERSION,
        "entity_id": entity_id,
        "autonomy": entity.autonomy,
        "recovery_confidence_score": recovery_confidence,
    }))
}

/// GET /v1/autonomy/fusion — sensory fusion confidence summary across entities.
pub fn fusion_summary(state: &ControlCenterState) -> HttpResponse {
    use spanda_autonomy::fusion::{fuse_observations, ConfidencePolicy};
    let registry = state.entity_registry();
    let mut summaries = Vec::new();
    for entity in registry.entities.values().take(20) {
        let ctx = EntityAutonomyContext::from_entity(entity);
        if ctx.sensor_readings.is_empty() {
            continue;
        }
        let fused = fuse_observations(
            &format!("entity:{}", entity.id),
            &ctx.sensor_readings,
            &ConfidencePolicy::default(),
        );
        summaries.push(serde_json::json!({
            "entity_id": entity.id,
            "score": fused.confidence.score,
            "meets_policy": fused.confidence.meets_policy,
            "conflicts": fused.conflicts.len(),
            "sources": ctx.sensor_readings.len(),
        }));
    }
    json_ok(&serde_json::json!({
        "version": API_VERSION,
        "fusion": summaries,
    }))
}

/// GET /v1/autonomy/memory — operational memory references across entities.
pub fn memory_summary(state: &ControlCenterState) -> HttpResponse {
    let registry = state.entity_registry();
    let entries: Vec<_> = registry
        .entities
        .values()
        .take(20)
        .filter_map(|entity| {
            entity.autonomy.as_ref().and_then(|profile| {
                profile.memory_refs.as_ref().map(|refs| {
                    serde_json::json!({
                        "entity_id": entity.id,
                        "working": refs.working.len(),
                        "episodic": refs.episodic.len(),
                        "semantic": refs.semantic.len(),
                        "procedural": refs.procedural.len(),
                        "reflex": refs.reflex.len(),
                    })
                })
            })
        })
        .collect();
    json_ok(&serde_json::json!({
        "version": API_VERSION,
        "memory": entries,
    }))
}

/// JSON string helper for gRPC parity.
pub fn list_reflex_json(state: &ControlCenterState) -> String {
    list_reflex(state).body
}

/// JSON string helper for gRPC parity.
pub fn list_reflex_traces_json(state: &ControlCenterState) -> String {
    list_reflex_traces(state).body
}

/// JSON string helper for gRPC parity.
pub fn homeostasis_summary_json(state: &ControlCenterState) -> String {
    homeostasis_summary(state).body
}

/// JSON string helper for gRPC parity.
pub fn immunity_scan_json(state: &ControlCenterState) -> String {
    immunity_scan(state).body
}

/// JSON string helper for gRPC parity.
pub fn attention_queue_json(state: &ControlCenterState) -> String {
    attention_queue(state).body
}

/// JSON string helper for gRPC parity.
pub fn fusion_summary_json(state: &ControlCenterState) -> String {
    fusion_summary(state).body
}

/// JSON string helper for gRPC parity.
pub fn memory_summary_json(state: &ControlCenterState) -> String {
    memory_summary(state).body
}

/// JSON string helper for gRPC parity.
pub fn entity_autonomy_json(state: &ControlCenterState, entity_id: &str) -> String {
    entity_autonomy(state, entity_id).body
}

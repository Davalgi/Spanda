//! Registry-wide autonomy profile attachment and enrichment.
//!
use crate::entity::{enrich_entity_autonomy, EntityAutonomyContext};
use crate::immunity::{evaluate_quarantine_decision, ImmunePolicy};
use crate::memory::enrich_entity_memory_refs;
use crate::reflex::list_reflex_actions;
use spanda_config::entity::{EntityRecord, EntityRegistry, EntityTrustStatus};
use spanda_config::entity_autonomy::{EntityAutonomyProfile, EntityReflexSummary};

/// Attach default autonomy stubs to every entity in the registry.
pub fn apply_registry_autonomy_profiles(registry: &mut EntityRegistry) {
    for entity in registry.entities.values_mut() {
        apply_tamper_trust_signals(entity);
        attach_entity_autonomy_stub(entity);
        let ctx = EntityAutonomyContext::from_entity(entity);
        enrich_entity_autonomy(entity, &ctx);
        sync_immunity_quarantine_metadata(entity);
    }
}

fn apply_tamper_trust_signals(entity: &mut EntityRecord) {
    if entity.metadata.get("tamper.detected") == Some(&"true".into()) {
        entity.trust_status = EntityTrustStatus::Compromised;
    }
    if entity.metadata.get("sensor.spoofing_detected") == Some(&"true".into()) {
        entity.trust_status = EntityTrustStatus::Untrusted;
    }
}

fn sync_immunity_quarantine_metadata(entity: &mut EntityRecord) {
    let decision = evaluate_quarantine_decision(entity, &ImmunePolicy::platform_defaults());
    if decision.quarantine {
        entity
            .metadata
            .insert("autonomy.quarantined".into(), "true".into());
        if let Some(profile) = entity.autonomy.as_mut() {
            if let Some(status) = profile.immunity_status.as_mut() {
                status.quarantined = true;
                status.threat_level = Some("elevated".into());
            }
        }
    }
}

fn attach_entity_autonomy_stub(entity: &mut EntityRecord) {
    if entity.autonomy.is_some() {
        return;
    }
    let reflexes: Vec<EntityReflexSummary> = list_reflex_actions()
        .into_iter()
        .map(|action| EntityReflexSummary {
            id: action.id,
            name: action.name,
            priority: action.priority,
            enabled: action.enabled,
            last_triggered_at: None,
        })
        .collect();
    entity.autonomy = Some(EntityAutonomyProfile {
        reflexes: reflexes.clone(),
        memory_refs: Some(enrich_entity_memory_refs(
            entity,
            &reflexes.iter().map(|r| r.id.clone()).collect::<Vec<_>>(),
        )),
        ..Default::default()
    });
}

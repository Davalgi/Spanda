//! Registry-wide autonomy profile attachment and enrichment.
//!
use crate::entity::{enrich_entity_autonomy, EntityAutonomyContext};
use crate::reflex::list_reflex_actions;
use spanda_config::entity::{EntityRecord, EntityRegistry};
use spanda_config::entity_autonomy::{EntityAutonomyProfile, EntityMemoryRefs, EntityReflexSummary};

/// Attach default autonomy stubs to every entity in the registry.
pub fn apply_registry_autonomy_profiles(registry: &mut EntityRegistry) {
    for entity in registry.entities.values_mut() {
        attach_entity_autonomy_stub(entity);
        let ctx = EntityAutonomyContext::from_entity(entity);
        enrich_entity_autonomy(entity, &ctx);
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
        reflexes,
        memory_refs: Some(EntityMemoryRefs {
            semantic: vec![format!("entity:{}", entity.id)],
            ..Default::default()
        }),
        ..Default::default()
    });
}

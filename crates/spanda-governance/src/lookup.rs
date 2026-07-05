//! Resolve entity governance posture for live decision enforcement.
//!
use spanda_config::entity::EntityRecord;
use spanda_config::{
    apply_entity_mutation_overlay, default_entity_overlay_path, load_entity_overlay,
    ConfigResolver, SpandaManifest,
};
use std::env;
use std::path::PathBuf;

/// Load an entity record for governance checks from overlay and project config.
pub fn lookup_entity_for_governance(entity_id: &str) -> Option<EntityRecord> {
    // Resolve entity posture for live decision authorization.
    //
    // Parameters:
    // - `entity_id` — entity identifier from the decision runtime
    //
    // Returns:
    // Entity record when found in overlay or resolved project config.
    //
    // Options:
    // Uses `SPANDA_ENTITY_OVERLAY_PATH`, `SPANDA_PROJECT_ROOT`, and cwd.
    //
    // Example:
    // let entity = lookup_entity_for_governance("robot:amr-01")?;

    let overlay = load_entity_overlay(&default_entity_overlay_path());
    if let Some(record) = overlay.entities.get(entity_id) {
        return Some(record.clone());
    }

    let root = project_root()?;
    let resolved = ConfigResolver::new().resolve_from_dir(&root).ok()?;
    let mut registry = resolved.entity_registry();
    apply_entity_mutation_overlay(&mut registry, &overlay);
    registry.get(entity_id).cloned()
}

fn project_root() -> Option<PathBuf> {
    if let Ok(root) = env::var("SPANDA_PROJECT_ROOT") {
        let path = PathBuf::from(root);
        if path.is_dir() {
            return Some(path);
        }
    }
    let cwd = env::current_dir().ok()?;
    SpandaManifest::find_project_root(&cwd)
}

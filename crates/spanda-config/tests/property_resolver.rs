//! Property-style config resolver tests.

use spanda_config::{ConfigResolver, EntityQuery, build_entity_registry};
use std::path::PathBuf;

#[test]
fn config_resolver_never_panics_on_missing_project() {
    // Missing project roots must fail gracefully.
    let missing = PathBuf::from("/tmp/spanda-does-not-exist-release-hardening");
    let result = std::panic::catch_unwind(|| {
        ConfigResolver::new()
            .with_validation(false)
            .resolve_from_dir(&missing)
    });
    assert!(result.is_ok(), "resolver must not panic");
    assert!(result.unwrap().is_err(), "missing project must error");
}

#[test]
fn entity_graph_query_is_stable_for_empty_registry() {
    // Empty registries must return empty query results.
    let warehouse = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/warehouse");
    let resolved = ConfigResolver::new()
        .with_validation(false)
        .resolve_from_dir(&warehouse)
        .expect("warehouse fixture");
    let registry = build_entity_registry(&resolved);
    let result = registry.query(&EntityQuery {
        entity_type: None,
        kind: None,
        health_status: None,
        readiness_status: None,
        trust_status: None,
        lifecycle_state: None,
        tag: None,
        label: None,
        provider: None,
        package: None,
        firmware_version: None,
        assigned_to: None,
        depends_on: None,
        participates_in: None,
        parent_id: None,
        search: None,
    });
    assert!(result.count > 0);
    assert_eq!(result.count, result.entities.len());
}

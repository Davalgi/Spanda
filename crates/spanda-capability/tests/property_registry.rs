//! Property-style capability registry tests.

use spanda_capability::{capability_registry, lookup_capability};

#[test]
fn capability_registry_never_panics_on_unknown_names() {
    // Unknown capability names must resolve without panicking.
    let names = ["", "lidar.read", "no.such.capability", "!!!", &"a".repeat(128)];
    for name in names {
        let _ = std::panic::catch_unwind(|| {
            let _ = lookup_capability(name);
        })
        .expect("capability registry must not panic");
    }
}

#[test]
fn capability_registry_is_non_empty_and_unique() {
    // Registry entries must remain non-empty with unique names.
    let entries = capability_registry();
    assert!(!entries.is_empty());
    let mut names: Vec<_> = entries.iter().map(|e| e.name.clone()).collect();
    names.sort();
    names.dedup();
    assert_eq!(names.len(), entries.len());
}

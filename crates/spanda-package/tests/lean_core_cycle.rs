//! Ensure spanda-package does not depend on spanda-core (Phase 4 cycle break).
//!
#[test]
fn package_manifest_has_no_spanda_core_dependency() {
    let manifest = std::fs::read_to_string(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("Cargo.toml"),
    )
    .expect("read Cargo.toml");
    assert!(
        !manifest.contains("spanda-core"),
        "spanda-package must not depend on spanda-core"
    );
}

#[test]
fn package_dependency_tree_excludes_spanda_core() {
    let output = std::process::Command::new("cargo")
        .args(["tree", "-p", "spanda-package", "--prefix", "none"])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("cargo tree");
    assert!(output.status.success(), "cargo tree failed");
    let tree = String::from_utf8_lossy(&output.stdout);
    assert!(
        !tree.lines().any(|line| line.trim() == "spanda-core"),
        "transitive dependency tree must not include spanda-core:\n{tree}"
    );
}

#[test]
fn permissive_permissions_use_hardware_catalog() {
    let perms = spanda_package::validation::ApplicationPermissions::permissive();
    assert!(perms.hardware_targets.iter().any(|t| t == "JetsonOrin"));
}

//! Property-style package and plugin manifest parser tests.

use spanda_package::manifest::PackageManifest;

#[test]
fn package_manifest_parser_never_panics_on_garbage() {
    // Garbage TOML must not panic the manifest parser.
    let samples = [
        "",
        "not toml",
        "[[[",
        "[package]\nname = ",
        "[package]\nname = \"x\"\nversion = \"0.1.0\"",
        &"x = 1\n".repeat(1000),
        "[dependencies]\nspanda-gps = \"0.1\"",
    ];
    for sample in samples {
        let _ = std::panic::catch_unwind(|| {
            let _ = PackageManifest::parse_str(sample);
        })
        .expect("manifest parser must not panic");
    }
}

#[test]
fn package_manifest_round_trips_minimal_valid() {
    // Minimal valid manifests must parse and retain identity fields.
    let manifest = PackageManifest::parse_str(
        r#"
        [package]
        name = "demo"
        version = "0.1.0"
        description = "demo package"
        license = "Apache-2.0"
        "#,
    )
    .expect("valid manifest");
    assert_eq!(manifest.package.name, "demo");
    assert_eq!(manifest.package.version, "0.1.0");
}

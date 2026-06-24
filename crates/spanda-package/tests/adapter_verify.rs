//! Adapter package manifest verification tests.

use spanda_package::{
    adapter_verify_ok, nav2_adapter_metadata, verify_adapter_package, verify_manifest_adapter,
    AdapterVerifySeverity, PackageManifest,
};

#[test]
fn nav2_example_package_matches_registry_metadata() {
    // Description:
    //     Nav2 example package matches registry metadata.
    //
    // Inputs:
    //     None.
    //
    // Outputs:
    //     None.
    //
    // Example:

    //     let result = spanda_package::adapter_verify::nav2_example_package_matches_registry_metadata();

    let manifest = PackageManifest::parse_str(include_str!(
        "../../../examples/packages/nav2_adapter_package/spanda.toml"
    ))
    .expect("parse manifest");
    let issues = verify_manifest_adapter(&manifest, &nav2_adapter_metadata());
    assert!(adapter_verify_ok(&issues), "issues: {issues:?}");
}

#[test]
fn missing_adapter_section_fails_verify() {
    // Description:
    //     Missing adapter section fails verify.
    //
    // Inputs:
    //     None.
    //
    // Outputs:
    //     None.
    //
    // Example:

    //     let result = spanda_package::adapter_verify::missing_adapter_section_fails_verify();

    let manifest = PackageManifest::parse_str(
        r#"
[package]
name = "empty_adapter"
version = "0.1.0"
"#,
    )
    .expect("parse");
    let issues = verify_manifest_adapter(&manifest, &nav2_adapter_metadata());
    assert!(issues
        .iter()
        .any(|i| i.severity == AdapterVerifySeverity::Error));
}

#[test]
fn verify_adapter_package_by_import_path() {
    // Description:
    //     Verify adapter package by import path.
    //
    // Inputs:
    //     None.
    //
    // Outputs:
    //     None.
    //
    // Example:

    //     let result = spanda_package::adapter_verify::verify_adapter_package_by_import_path();

    let manifest = PackageManifest::parse_str(include_str!(
        "../../../examples/packages/nav2_adapter_package/spanda.toml"
    ))
    .expect("parse");
    let issues = verify_adapter_package(&manifest, Some("navigation.nav2"), None).expect("verify");
    assert!(adapter_verify_ok(&issues));
}

//! Validate package `[adapter]` sections against framework registry metadata.

use crate::adapter::{adapter_metadata_for_import, adapter_metadata_for_package, AdapterMetadata};
use crate::error::{PackageError, PackageResult};
use crate::manifest::PackageManifest;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdapterVerifyIssue {
    pub severity: AdapterVerifySeverity,
    pub message: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AdapterVerifySeverity {
    Pass,
    Warning,
    Error,
}

fn missing_provides(expected: &AdapterMetadata, actual: &AdapterMetadata) -> Vec<String> {
    // Description:
    //     Missing provides.
    //
    // Inputs:
    //     expected: &AdapterMetadata
    //         Caller-supplied expected.
    //     actual: &AdapterMetadata
    //         Caller-supplied actual.
    //
    // Outputs:
    //     result: Vec<String>
    //         Return value from `missing_provides`.
    //
    // Example:

    //     let result = spanda_package::adapter_verify::missing_provides(expected, actual);

    expected
        .provides
        .iter()
        .filter(|symbol| !actual.provides.iter().any(|p| p == *symbol))
        .cloned()
        .collect()
}

fn missing_requires(expected: &AdapterMetadata, actual: &AdapterMetadata) -> Vec<String> {
    // Description:
    //     Missing requires.
    //
    // Inputs:
    //     expected: &AdapterMetadata
    //         Caller-supplied expected.
    //     actual: &AdapterMetadata
    //         Caller-supplied actual.
    //
    // Outputs:
    //     result: Vec<String>
    //         Return value from `missing_requires`.
    //
    // Example:

    //     let result = spanda_package::adapter_verify::missing_requires(expected, actual);

    expected
        .requires
        .iter()
        .filter(|symbol| !actual.requires.iter().any(|r| r == *symbol))
        .cloned()
        .collect()
}

/// Compare a package manifest adapter section against expected registry metadata.
pub fn verify_manifest_adapter(
    manifest: &PackageManifest,
    expected: &AdapterMetadata,
) -> Vec<AdapterVerifyIssue> {
    // Description:
    //     Verify manifest adapter.
    //
    // Inputs:
    //     anifes: &PackageManifest
    //         Caller-supplied anifes.
    //     expected: &AdapterMetadata
    //         Caller-supplied expected.
    //
    // Outputs:
    //     result: Vec<AdapterVerifyIssue>
    //         Return value from `verify_manifest_adapter`.
    //
    // Example:

    //     let result = spanda_package::adapter_verify::verify_manifest_adapter(anifes, expected);

    let actual = &manifest.adapter;
    let mut issues = Vec::new();
    if actual.provides.is_empty() && actual.requires.is_empty() {
        issues.push(AdapterVerifyIssue {
            severity: AdapterVerifySeverity::Error,
            message: format!(
                "Package '{}' missing [adapter] provides/requires for production adapter scaffolding",
                manifest.package.name
            ),
        });
        return issues;
    }

    let missing_provides = missing_provides(expected, actual);
    if missing_provides.is_empty() {
        issues.push(AdapterVerifyIssue {
            severity: AdapterVerifySeverity::Pass,
            message: format!(
                "Package '{}' adapter provides cover expected symbols",
                manifest.package.name
            ),
        });
    } else {
        issues.push(AdapterVerifyIssue {
            severity: AdapterVerifySeverity::Error,
            message: format!(
                "Package '{}' adapter missing provides: {}",
                manifest.package.name,
                missing_provides.join(", ")
            ),
        });
    }

    let missing_requires = missing_requires(expected, actual);
    if missing_requires.is_empty() {
        issues.push(AdapterVerifyIssue {
            severity: AdapterVerifySeverity::Pass,
            message: format!(
                "Package '{}' adapter requires cover expected runtime capabilities",
                manifest.package.name
            ),
        });
    } else {
        issues.push(AdapterVerifyIssue {
            severity: AdapterVerifySeverity::Warning,
            message: format!(
                "Package '{}' adapter missing recommended requires: {}",
                manifest.package.name,
                missing_requires.join(", ")
            ),
        });
    }

    issues
}

/// Verify a project manifest against a framework import path or registry package name.
pub fn verify_adapter_package(
    manifest: &PackageManifest,
    import_path: Option<&str>,
    package_name: Option<&str>,
) -> PackageResult<Vec<AdapterVerifyIssue>> {
    // Description:
    //     Verify adapter package.
    //
    // Inputs:
    //     anifes: &PackageManifest
    //         Caller-supplied anifes.
    //     import_path: Option<&str>
    //         Caller-supplied import path.
    //     package_name: Option<&str>
    //         Caller-supplied package name.
    //
    // Outputs:
    //     result: PackageResult<Vec<AdapterVerifyIssue>>
    //         Return value from `verify_adapter_package`.
    //
    // Example:
    //     let result = spanda_package::adapter_verify::verify_adapter_package(anifes, import_path, package_name);
    let expected = import_path
        .and_then(adapter_metadata_for_import)
        .or_else(|| package_name.and_then(adapter_metadata_for_package))
        .ok_or_else(|| {
            PackageError::Validation(
                "No adapter metadata registered for requested import/package".into(),
            )
        })?;
    Ok(verify_manifest_adapter(manifest, &expected))
}

/// Return true when verification issues contain no errors.
pub fn adapter_verify_ok(issues: &[AdapterVerifyIssue]) -> bool {
    // Description:
    //     Adapter verify ok.
    //
    // Inputs:
    //     issues: &[AdapterVerifyIssue]
    //         Caller-supplied issues.
    //
    // Outputs:
    //     result: bool
    //         Return value from `adapter_verify_ok`.
    //
    // Example:

    //     let result = spanda_package::adapter_verify::adapter_verify_ok(issues);

    !issues
        .iter()
        .any(|issue| issue.severity == AdapterVerifySeverity::Error)
}

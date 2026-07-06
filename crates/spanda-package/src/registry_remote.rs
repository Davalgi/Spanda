//! Optional remote registry mirror via `SPANDA_REGISTRY_URL`.
//!
//! Fetches `index.json` from the configured base URL (curl when available).
//! Entries merge with `LOCAL_REGISTRY` for search and dependency resolution.

use crate::category::PackageCategory;
use crate::registry::RegistryEntry;
use crate::safety::SafetyLevel;
use serde::{Deserialize, Serialize};
use std::fs;
use std::sync::OnceLock;

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct RemoteRegistryEntry {
    pub name: String,
    pub description: String,
    pub versions: Vec<String>,
    pub category: String,
    pub license: String,
    #[serde(default)]
    pub import_paths: Vec<String>,
    #[serde(default)]
    pub version_checksums: std::collections::BTreeMap<String, String>,
    #[serde(default)]
    pub version_signatures:
        std::collections::BTreeMap<String, crate::registry_sign::RegistryVersionSignature>,
}

static REMOTE_CACHE: OnceLock<Vec<RemoteRegistryEntry>> = OnceLock::new();

pub fn registry_base_url() -> Option<String> {
    // Description:
    //     Registry base url.
    //
    // Inputs:
    //     None.
    //
    // Outputs:
    //     result: Option<String>
    //         Return value from `registry_base_url`.
    //
    // Example:

    //     let result = spanda_package::registry_remote::registry_base_url();

    const DEFAULT_REGISTRY: &str = "https://raw.githubusercontent.com/Davalgi/Spanda/main/registry";

    // Produce var as the result.
    match std::env::var("SPANDA_REGISTRY_URL") {
        Ok(url) if url.trim().is_empty() => None,
        Ok(url) => Some(url.trim_end_matches('/').to_string()),
        Err(_) => Some(DEFAULT_REGISTRY.to_string()),
    }
}

pub fn fetch_index_json(url: &str) -> Result<String, String> {
    // Description:
    //     Fetch index json.
    //
    // Inputs:
    //     url: &str
    //         Caller-supplied url.
    //
    // Outputs:
    //     result: Result<String, String>
    //         Return value from `fetch_index_json`.
    //
    // Example:
    //     let result = spanda_package::registry_remote::fetch_index_json(rl);
    // use path when file url path is present.

    // Emit output when file url path provides a path.
    if let Some(path) = super::registry_fetch::file_url_path(url) {
        return fs::read_to_string(&path)
            .map_err(|e| format!("failed to read registry index at {}: {e}", path.display()));
    }

    // Handle the success value from new.
    if let Ok(output) = std::process::Command::new("curl")
        .args(["-fsSL", url])
        .output()
    {
        // Handle output when the subprocess succeeds.
        if output.status.success() {
            return String::from_utf8(output.stdout)
                .map_err(|e| format!("registry response is not UTF-8: {e}"));
        }
    }
    Err(format!("failed to fetch registry index from {url}"))
}

pub fn load_remote_registry() -> Vec<RemoteRegistryEntry> {
    // Description:
    //     Load remote registry.
    //
    // Inputs:
    //     None.
    //
    // Outputs:
    //     result: Vec<RemoteRegistryEntry>
    //         Return value from `load_remote_registry`.
    //
    // Example:
    //     let result = spanda_package::registry_remote::load_remote_registry();

    // Produce REMOTE CACHE as the result.
    REMOTE_CACHE
        .get_or_init(|| {
            let Some(base) = registry_base_url() else {
                return Vec::new();
            };
            let url = format!("{base}/index.json");

            // Match on fetch index json and handle each case.
            match fetch_index_json(&url) {
                Ok(body) => serde_json::from_str(&body).unwrap_or_else(|e| {
                    eprintln!("Warning: invalid remote registry JSON at {url}: {e}");
                    Vec::new()
                }),
                Err(err) => {
                    eprintln!("Warning: {err}");
                    Vec::new()
                }
            }
        })
        .clone()
}

pub fn find_remote_entry(name: &str) -> Option<RemoteRegistryEntry> {
    // Description:
    //     Find remote entry.
    //
    // Inputs:
    //     name: &str
    //         Caller-supplied name.
    //
    // Outputs:
    //     result: Option<RemoteRegistryEntry>
    //         Return value from `find_remote_entry`.
    //
    // Example:
    //     let result = spanda_package::registry_remote::find_remote_entry(name);

    // Produce load remote registry as the result.
    load_remote_registry()
        .into_iter()
        .find(|entry| entry.name == name)
}

pub fn search_remote_registry(query: &str) -> Vec<RemoteRegistryEntry> {
    // Description:
    //     Search remote registry.
    //
    // Inputs:
    //     query: &str
    //         Caller-supplied query.
    //
    // Outputs:
    //     result: Vec<RemoteRegistryEntry>
    //         Return value from `search_remote_registry`.
    //
    // Example:
    //     let result = spanda_package::registry_remote::search_remote_registry(query);

    // Compute q for the following logic.
    let q = query.to_lowercase();
    load_remote_registry()
        .into_iter()
        .filter(|entry| {
            entry.name.to_lowercase().contains(&q)
                || entry.description.to_lowercase().contains(&q)
                || entry.category.to_lowercase().contains(&q)
        })
        .collect()
}

pub fn remote_category(name: &str) -> PackageCategory {
    // Description:
    //     Remote category.
    //
    // Inputs:
    //     name: &str
    //         Caller-supplied name.
    //
    // Outputs:
    //     result: PackageCategory
    //         Return value from `remote_category`.
    //
    // Example:
    //     let result = spanda_package::registry_remote::remote_category(name);

    // Produce Robotics) as the result.
    name.parse().unwrap_or(PackageCategory::Robotics)
}

pub fn remote_safety_level(name: &str) -> SafetyLevel {
    // Description:
    //     Remote safety level.
    //
    // Inputs:
    //     name: &str
    //         Caller-supplied name.
    //
    // Outputs:
    //     result: SafetyLevel
    //         Return value from `remote_safety_level`.
    //
    // Example:
    //     let result = spanda_package::registry_remote::remote_safety_level(name);

    // Match on name and handle each case.
    match name {
        "spanda-ros2" | "spanda-opencv" | "spanda-yolo" | "spanda-mqtt" => {
            SafetyLevel::SimulationOnly
        }
        "spanda-python-bridge" | "spanda-cpp-bridge" => SafetyLevel::HardwareSafe,
        _ => SafetyLevel::Experimental,
    }
}

pub fn remote_as_static_view(entry: &RemoteRegistryEntry) -> RegistryEntryView<'_> {
    // Description:
    //     Remote as static view.
    //
    // Inputs:
    //     entry: &RemoteRegistryEntry
    //         Caller-supplied entry.
    //
    // Outputs:
    //     result: RegistryEntryView<'_>
    //         Return value from `remote_as_static_view`.
    //
    // Example:
    //     let result = spanda_package::registry_remote::remote_as_static_view(entry);

    // Produce RegistryEntryView as the result.
    RegistryEntryView {
        name: entry.name.as_str(),
        description: entry.description.as_str(),
        versions: entry.versions.iter().map(String::as_str).collect(),
        category: remote_category(&entry.category),
        license: entry.license.as_str(),
        import_paths: entry.import_paths.iter().map(String::as_str).collect(),
        safety_level: remote_safety_level(&entry.name),
    }
}

/// Unified view for local static entries and remote owned entries.
#[derive(Debug, Clone)]
pub struct RegistryEntryView<'a> {
    pub name: &'a str,
    pub description: &'a str,
    pub versions: Vec<&'a str>,
    pub category: PackageCategory,
    pub license: &'a str,
    pub import_paths: Vec<&'a str>,
    pub safety_level: SafetyLevel,
}

impl<'a> From<&'static RegistryEntry> for RegistryEntryView<'a> {
    fn from(entry: &'static RegistryEntry) -> Self {
        // From.
        //
        // Parameters:
        // - `entry` — input value
        //
        // Returns:
        // A new instance of this type.
        //
        // Options:
        // None.
        //
        // Example:
        // let result = spanda_package::registry_remote::from(entry);

        RegistryEntryView {
            name: entry.name,
            description: entry.description,
            versions: entry.versions.to_vec(),
            category: entry.category,
            license: entry.license,
            import_paths: entry.import_paths.to_vec(),
            safety_level: entry.safety_level(),
        }
    }
}

/// Build a remote-style entry from an on-disk `packages/registry/<name>` tree.
fn entry_from_ondisk_package(name: &str) -> Option<RemoteRegistryEntry> {
    // Resolve official packages that exist in the monorepo source tree.
    //
    // Parameters:
    // - `name` — package name (for example `spanda-wifi`)
    //
    // Returns:
    // A registry entry when `packages/registry/<name>/spanda.toml` is present.
    //
    // Options:
    // None.
    //
    // Example:
    // let entry = entry_from_ondisk_package("spanda-gps");

    let dir = super::registry::registry_package_dir(name)?;
    let manifest = super::manifest::PackageManifest::load(&dir.join("spanda.toml")).ok()?;
    Some(RemoteRegistryEntry {
        name: manifest.package.name,
        description: manifest
            .package
            .description
            .unwrap_or_else(|| format!("On-disk registry package {name}")),
        versions: vec![manifest.package.version],
        category: manifest
            .categories
            .first()
            .map(|c| c.as_str().to_string())
            .unwrap_or_else(|| "robotics".into()),
        license: manifest
            .package
            .license
            .unwrap_or_else(|| "Apache-2.0".into()),
        import_paths: Vec::new(),
        version_checksums: Default::default(),
        version_signatures: Default::default(),
    })
}

pub fn lookup_registry_entry(name: &str) -> Option<RegistryEntryLookup> {
    // Resolve a package from the local stub, remote/hosted index, or on-disk tree.
    //
    // Parameters:
    // - `name` — registry package name
    //
    // Returns:
    // A lookup entry when the package is known to any registry source.
    //
    // Options:
    // None.
    //
    // Example:
    // let entry = lookup_registry_entry("spanda-wifi");

    // Prefer the compile-time local stub for well-known framework packages.
    if let Some(entry) = super::registry::find_registry_entry(name) {
        return Some(RegistryEntryLookup::Local(entry));
    }
    // Use the configured remote or monorepo index when available.
    if let Some(entry) = find_remote_entry(name) {
        return Some(RegistryEntryLookup::Remote(entry));
    }
    // Fall back to packages present under packages/registry/ in a source checkout.
    entry_from_ondisk_package(name).map(RegistryEntryLookup::Remote)
}

#[derive(Debug, Clone)]
pub enum RegistryEntryLookup {
    Local(&'static RegistryEntry),
    Remote(RemoteRegistryEntry),
}

impl RegistryEntryLookup {
    pub fn name(&self) -> &str {
        // Description:
        //     Name.
        //
        // Inputs:
        //     &self: input value
        //         Caller-supplied &self.
        //
        // Outputs:
        //     result: &str
        //         Return value from `name`.
        //
        // Example:
        //     let result = spanda_package::registry_remote::name(&self);

        // Dispatch based on the enum variant or current state.
        match self {
            RegistryEntryLookup::Local(entry) => entry.name,
            RegistryEntryLookup::Remote(entry) => &entry.name,
        }
    }

    pub fn versions(&self) -> Vec<String> {
        // Description:
        //     Versions.
        //
        // Inputs:
        //     &self: input value
        //         Caller-supplied &self.
        //
        // Outputs:
        //     result: Vec<String>
        //         Return value from `versions`.
        //
        // Example:
        //     let result = spanda_package::registry_remote::versions(&self);

        // Dispatch based on the enum variant or current state.
        match self {
            RegistryEntryLookup::Local(entry) => {
                entry.versions.iter().map(|v| (*v).to_string()).collect()
            }
            RegistryEntryLookup::Remote(entry) => entry.versions.clone(),
        }
    }

    pub fn registry_label(&self) -> &'static str {
        // Description:
        //     Registry label.
        //
        // Inputs:
        //     &self: input value
        //         Caller-supplied &self.
        //
        // Outputs:
        //     result: &'static str
        //         Return value from `registry_label`.
        //
        // Example:
        //     let result = spanda_package::registry_remote::registry_label(&self);

        // Dispatch based on the enum variant or current state.
        match self {
            RegistryEntryLookup::Local(_) => "local",
            RegistryEntryLookup::Remote(_) => "remote",
        }
    }

    pub fn version_sha256(&self, version: &str) -> Option<String> {
        // Description:
        //     Version sha256.
        //
        // Inputs:
        //     &self: input value
        //         Caller-supplied &self.
        //     version: &str
        //         Caller-supplied version.
        //
        // Outputs:
        //     result: Option<String>
        //         Return value from `version_sha256`.
        //
        // Example:

        //     let result = spanda_package::registry_remote::version_sha256(&self, version);

        match self {
            RegistryEntryLookup::Local(_) => None,
            RegistryEntryLookup::Remote(entry) => entry.version_checksums.get(version).cloned(),
        }
    }

    pub fn version_signature(
        &self,
        version: &str,
    ) -> Option<crate::registry_sign::RegistryVersionSignature> {
        // Description:
        //     Version signature.
        //
        // Inputs:
        //     &self: input value
        //         Caller-supplied &self.
        //     version: &str
        //         Caller-supplied version.
        //
        // Outputs:
        //     result: Option<crate::registry_sign::RegistryVersionSignature>
        //         Return value from `version_signature`.
        //
        // Example:

        //     let result = spanda_package::registry_remote::version_signature(&self, version);

        match self {
            RegistryEntryLookup::Local(_) => None,
            RegistryEntryLookup::Remote(entry) => entry.version_signatures.get(version).cloned(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn remote_disabled_without_env() {
        // Description:
        //     Remote disabled without env.
        //
        // Inputs:
        //     None.
        //
        // Outputs:
        //     None.
        //
        // Example:

        //     let result = spanda_package::registry_remote::remote_disabled_without_env();

        let _guard = crate::testing::env_lock();
        // Remote disabled without env.
        //
        // Parameters:
        // None.
        //
        // Returns:
        // Nothing.
        //
        // Options:
        // None.
        //
        // Example:
        // let result = spanda_package::registry_remote::remote_disabled_without_env();

        std::env::remove_var("SPANDA_REGISTRY_URL");
        assert!(registry_base_url().is_some());
        std::env::set_var("SPANDA_REGISTRY_URL", "");
        assert!(registry_base_url().is_none());
    }
}

#[cfg(test)]
mod ondisk_lookup_tests {
    use super::*;

    #[test]
    fn looks_up_ondisk_wifi_package() {
        let _guard = crate::testing::env_lock();
        std::env::remove_var("SPANDA_REGISTRY_URL");
        let entry = lookup_registry_entry("spanda-wifi");
        assert!(
            entry.is_some(),
            "spanda-wifi should resolve from packages/registry"
        );
        assert_eq!(entry.unwrap().name(), "spanda-wifi");
    }
}

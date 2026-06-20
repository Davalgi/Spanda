//! `spanda.toml` manifest parsing, validation, and project root discovery.
//!
use crate::adapter::AdapterMetadata;
use crate::category::PackageCategory;
use crate::dependency::DependencySpec;
use crate::error::{PackageError, PackageResult};
use crate::hardware_req::{CapabilityRequirements, HardwareRequirements};
use crate::safety::SafetyMetadata;
use semver::Version;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

pub const MANIFEST_FILENAME: &str = "spanda.toml";

/// Root manifest structure for `spanda.toml`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PackageManifest {
    pub package: PackageSection,
    #[serde(default)]
    pub dependencies: HashMap<String, DependencySpec>,
    #[serde(default)]
    pub dev_dependencies: HashMap<String, DependencySpec>,
    #[serde(default)]
    pub hardware: HardwareSection,
    #[serde(default)]
    pub capabilities: CapabilityRequirements,
    #[serde(default)]
    pub requires_hardware: HardwareRequirements,
    #[serde(default)]
    pub safety: SafetyMetadata,
    #[serde(default)]
    pub adapter: AdapterMetadata,
    #[serde(default)]
    pub categories: Vec<PackageCategory>,
    #[serde(default)]
    pub license_compat: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PackageSection {
    pub name: String,
    pub version: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub license: Option<String>,
    #[serde(default)]
    pub authors: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct HardwareSection {
    #[serde(default)]
    pub targets: Vec<String>,
}

impl PackageManifest {
    pub fn parse_str(content: &str) -> PackageResult<Self> {
        // Parse str.
        //
        // Parameters:
        // - `content` — input value
        //
        // Returns:
        // PackageResult<Self>.
        //
        // Options:
        // None.
        //
        // Example:
        // let result = spanda_package::manifest::parse_str(content);

        let mut manifest: Self = toml::from_str(content)?;
        manifest.safety.normalize();
        Ok(manifest)
    }

    pub fn load(path: &Path) -> PackageResult<Self> {
        // Load the value.
        //
        // Parameters:
        // - `path` — input value
        //
        // Returns:
        // PackageResult<Self>.
        //
        // Options:
        // None.
        //
        // Example:
        // let result = spanda_package::manifest::load(path);

        let content = std::fs::read_to_string(path).map_err(PackageError::from)?;
        Self::parse_str(&content)
    }

    pub fn load_from_dir(dir: &Path) -> PackageResult<Self> {
        // Load from dir.
        //
        // Parameters:
        // - `dir` — input value
        //
        // Returns:
        // PackageResult<Self>.
        //
        // Options:
        // None.
        //
        // Example:
        // let result = spanda_package::manifest::load_from_dir(dir);

        Self::load(&dir.join(MANIFEST_FILENAME))
    }

    pub fn save(&self, path: &Path) -> PackageResult<()> {
        // Save the value.
        //
        // Parameters:
        // - `self` — method receiver
        // - `path` — input value
        //
        // Returns:
        // PackageResult<()>.
        //
        // Options:
        // None.
        //
        // Example:
        // let result = instance.save(path);

        let content = toml::to_string_pretty(self)?;
        std::fs::write(path, content).map_err(PackageError::from)?;
        Ok(())
    }

    pub fn version(&self) -> PackageResult<Version> {
        // Version.
        //
        // Parameters:
        // - `self` — method receiver
        //
        // Returns:
        // PackageResult<Version>.
        //
        // Options:
        // None.
        //
        // Example:
        // let result = instance.version();

        crate::dependency::parse_version(&self.package.version)
    }

    pub fn all_dependencies(&self) -> impl Iterator<Item = (&str, &DependencySpec)> {
        // All dependencies.
        //
        // Parameters:
        // - `self` — method receiver
        //
        // Returns:
        // impl Iterator<Item = (&str, &DependencySpec)>.
        //
        // Options:
        // None.
        //
        // Example:
        // let result = instance.all_dependencies();

        self.dependencies
            .iter()
            .chain(self.dev_dependencies.iter())
            .map(|(k, v)| (k.as_str(), v))
    }
}

/// Find the project root by walking up from `start` looking for spanda.toml.
pub fn find_project_root(start: &Path) -> Option<PathBuf> {
    // Find project root.
    //
    // Parameters:
    // - `start` — input value
    //
    // Returns:
    // Some value on success, otherwise none.
    //
    // Options:
    // None.
    //
    // Example:
    // let result = spanda_package::manifest::find_project_root(start);

    let mut dir = if start.is_file() {
        start.parent()?.to_path_buf()
    } else {
        start.to_path_buf()
    };
    loop {
        if dir.join(MANIFEST_FILENAME).is_file() {
            return Some(dir);
        }
        if !dir.pop() {
            return None;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const WAREHOUSE: &str = r#"
[package]
name = "warehouse_robot"
version = "0.1.0"
description = "Warehouse robot controller"
license = "Apache-2.0"

[dependencies]
spanda-ros2 = "0.1.0"
spanda-vision = "0.1.0"
spanda-navigation = "0.1.0"

[hardware]
targets = ["RoverV1", "JetsonOrin"]

[capabilities]
required = [
  "camera.read",
  "lidar.read",
  "motion.propose",
  "actuator.execute.safe"
]
"#;

    #[test]
    fn parses_warehouse_manifest() {
        // Parses warehouse manifest.
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
        // let result = spanda_package::manifest::parses_warehouse_manifest();

        let m = PackageManifest::parse_str(WAREHOUSE).unwrap();
        assert_eq!(m.package.name, "warehouse_robot");
        assert_eq!(m.dependencies.len(), 3);
        assert_eq!(m.hardware.targets, vec!["RoverV1", "JetsonOrin"]);
        assert_eq!(m.capabilities.required.len(), 4);
    }
}

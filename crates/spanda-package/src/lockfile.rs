//! lockfile support for Spanda.
//!
use crate::dependency::LockedDependency;
use crate::error::{PackageError, PackageResult};
use crate::manifest::PackageManifest;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::path::Path;

pub const LOCKFILE_FILENAME: &str = "spanda.lock";

/// Resolved dependency graph written to `spanda.lock`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Lockfile {
    pub version: u32,
    pub package: LockPackageInfo,
    pub dependencies: BTreeMap<String, LockedDependency>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LockPackageInfo {
    pub name: String,
    pub version: String,
}

impl Lockfile {
    pub fn new(manifest: &PackageManifest, deps: BTreeMap<String, LockedDependency>) -> Self {
        // Description:
        //     Construct a new instance.
        //
        // Inputs:
        //     anifes: &PackageManifest
        //         Caller-supplied anifes.
        //     deps: BTreeMap<String, LockedDependency>
        //         Caller-supplied deps.
        //
        // Outputs:
        //     result: Self
        //         Return value from `new`.
        //
        // Example:
        //     let value = spanda_package::lockfile::new(anifes, deps);

        // Assemble the struct fields and return it.
        Self {
            version: 1,
            package: LockPackageInfo {
                name: manifest.package.name.clone(),
                version: manifest.package.version.clone(),
            },
            dependencies: deps,
        }
    }

    pub fn parse_str(content: &str) -> PackageResult<Self> {
        // Description:
        //     Parse str.
        //
        // Inputs:
        //     conten: &str
        //         Caller-supplied conten.
        //
        // Outputs:
        //     result: PackageResult<Self>
        //         Return value from `parse_str`.
        //
        // Example:
        //     let result = spanda_package::lockfile::parse_str(conten);

        // Produce to string as the result.
        serde_json::from_str(content).map_err(|e| PackageError::Lockfile(e.to_string()))
    }

    pub fn load(path: &Path) -> PackageResult<Self> {
        // Description:
        //     Load.
        //
        // Inputs:
        //     path: &Path
        //         Caller-supplied path.
        //
        // Outputs:
        //     result: PackageResult<Self>
        //         Return value from `load`.
        //
        // Example:
        //     let result = spanda_package::lockfile::load(path);

        // Compute content for the following logic.
        let content = std::fs::read_to_string(path).map_err(PackageError::from)?;
        Self::parse_str(&content)
    }

    pub fn load_from_dir(dir: &Path) -> PackageResult<Self> {
        // Description:
        //     Load from dir.
        //
        // Inputs:
        //     dir: &Path
        //         Caller-supplied dir.
        //
        // Outputs:
        //     result: PackageResult<Self>
        //         Return value from `load_from_dir`.
        //
        // Example:
        //     let result = spanda_package::lockfile::load_from_dir(dir);

        // Build the result via join.
        Self::load(&dir.join(LOCKFILE_FILENAME))
    }

    pub fn save(&self, path: &Path) -> PackageResult<()> {
        // Description:
        //     Save.
        //
        // Inputs:
        //     &self: input value
        //         Caller-supplied &self.
        //     path: &Path
        //         Caller-supplied path.
        //
        // Outputs:
        //     result: PackageResult<()>
        //         Return value from `save`.
        //
        // Example:
        //     let result = spanda_package::lockfile::save(&self, path);

        // Compute content for the following logic.
        let content = serde_json::to_string_pretty(self)
            .map_err(|e| PackageError::Lockfile(e.to_string()))?;
        std::fs::write(path, content).map_err(PackageError::from)?;
        Ok(())
    }

    pub fn save_to_dir(&self, dir: &Path) -> PackageResult<()> {
        // Description:
        //     Save to dir.
        //
        // Inputs:
        //     &self: input value
        //         Caller-supplied &self.
        //     dir: &Path
        //         Caller-supplied dir.
        //
        // Outputs:
        //     result: PackageResult<()>
        //         Return value from `save_to_dir`.
        //
        // Example:
        //     let result = spanda_package::lockfile::save_to_dir(&self, dir);

        // Call save on the current instance.
        self.save(&dir.join(LOCKFILE_FILENAME))
    }
}

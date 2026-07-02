//! Plugin install state, enable/disable lifecycle, and on-disk store.

use crate::audit::PluginAuditLog;
use crate::compatibility::require_compatible;
use crate::error::{PluginError, PluginResult};
use crate::hooks::{hook_context, parse_enabled_hooks, PluginHook};
use crate::loader::PluginLoader;
use crate::manifest::{PluginManifest, MANIFEST_FILENAME};
use crate::registry::{lookup_plugin_entry, PluginTrustTier};
use crate::security::{
    registry_trust_tier, validate_install_security, validate_registry_entry,
    SecurityValidationReport,
};
use serde::{Deserialize, Serialize};
use spanda_package::registry_sign::RegistryVersionSignature;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

pub const PLUGIN_STORE_DIR: &str = ".spanda/plugins";
pub const STATE_FILENAME: &str = "state.json";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PluginState {
    Installed,
    Enabled,
    Disabled,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InstalledPlugin {
    pub name: String,
    pub version: String,
    pub state: PluginState,
    pub trust_tier: String,
    pub plugin_type: String,
    pub install_path: PathBuf,
    #[serde(default)]
    pub dangerous_approved: bool,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PluginStoreState {
    pub plugins: HashMap<String, InstalledPlugin>,
}

pub struct PluginStore {
    root: PathBuf,
    state: PluginStoreState,
    audit: PluginAuditLog,
}

impl PluginStore {
    pub fn open(project_root: &Path) -> PluginResult<Self> {
        let root = project_root.join(PLUGIN_STORE_DIR);
        fs::create_dir_all(&root)?;
        let state_path = root.join(STATE_FILENAME);
        let state = if state_path.is_file() {
            serde_json::from_str(&fs::read_to_string(&state_path)?)?
        } else {
            PluginStoreState::default()
        };
        Ok(Self {
            root,
            state,
            audit: PluginAuditLog::default(),
        })
    }

    pub fn root(&self) -> &Path {
        &self.root
    }

    pub fn audit_log(&self) -> &PluginAuditLog {
        &self.audit
    }

    pub fn list(&self) -> Vec<&InstalledPlugin> {
        let mut items: Vec<_> = self.state.plugins.values().collect();
        items.sort_by(|a, b| a.name.cmp(&b.name));
        items
    }

    pub fn get(&self, name: &str) -> Option<&InstalledPlugin> {
        self.state.plugins.get(name)
    }

    fn save(&self) -> PluginResult<()> {
        let state_path = self.root.join(STATE_FILENAME);
        fs::write(state_path, serde_json::to_string_pretty(&self.state)?)?;
        Ok(())
    }

    pub fn install_from_dir(
        &mut self,
        source_dir: &Path,
        host_version: &str,
        dangerous_approved: bool,
    ) -> PluginResult<InstalledPlugin> {
        validate_registry_entry_from_dir(source_dir)?;
        let manifest = PluginManifest::load_from_dir(source_dir)?;
        let trust_tier = registry_trust_tier(&manifest.plugin.name);
        let artifact_path = manifest.artifact_path(source_dir, "wasm");
        let digest = if artifact_path.is_file() {
            Some(crate::security::sha256_file(&artifact_path)?)
        } else {
            None
        };
        let signature = lookup_plugin_entry(&manifest.plugin.name)
            .and_then(|entry| entry.version_signature(&manifest.plugin.version).cloned());

        let security = validate_install_security(
            &manifest,
            host_version,
            digest.as_deref(),
            signature.as_ref(),
            trust_tier,
            dangerous_approved,
        )?;
        if !security.approved {
            return Err(PluginError::Security(format!(
                "plugin install rejected: {}",
                security.detail.join("; ")
            )));
        }

        let compat = crate::compatibility::validate_spanda_version(&manifest, host_version)?;
        require_compatible(&compat)?;

        let dest = self.root.join(&manifest.plugin.name);
        if dest.exists() {
            fs::remove_dir_all(&dest)?;
        }
        copy_dir_recursive(source_dir, &dest)?;

        let record = InstalledPlugin {
            name: manifest.plugin.name.clone(),
            version: manifest.plugin.version.clone(),
            state: PluginState::Installed,
            trust_tier: trust_tier.as_str().to_string(),
            plugin_type: manifest.plugin_type().as_str().to_string(),
            install_path: dest,
            dangerous_approved,
        };
        self.state
            .plugins
            .insert(record.name.clone(), record.clone());
        self.save()?;
        self.audit
            .record(&record.name, "install", "plugin installed");
        self.dispatch_hook(&record.name, PluginHook::OnInstall, serde_json::json!({}))?;
        Ok(record)
    }

    pub fn uninstall(&mut self, name: &str) -> PluginResult<()> {
        let record = self
            .state
            .plugins
            .get(name)
            .ok_or_else(|| PluginError::Runtime(format!("plugin not installed: {name}")))?
            .clone();
        self.dispatch_hook(name, PluginHook::OnUninstall, serde_json::json!({}))?;
        if record.install_path.is_dir() {
            fs::remove_dir_all(&record.install_path)?;
        }
        self.state.plugins.remove(name);
        self.save()?;
        self.audit.record(name, "uninstall", "plugin removed");
        Ok(())
    }

    pub fn enable(&mut self, name: &str) -> PluginResult<()> {
        let record = self
            .state
            .plugins
            .get_mut(name)
            .ok_or_else(|| PluginError::Runtime(format!("plugin not installed: {name}")))?;
        record.state = PluginState::Enabled;
        self.save()?;
        self.audit.record(name, "enable", "plugin enabled");
        self.dispatch_hook(name, PluginHook::OnEnable, serde_json::json!({}))?;
        Ok(())
    }

    pub fn disable(&mut self, name: &str) -> PluginResult<()> {
        let record = self
            .state
            .plugins
            .get_mut(name)
            .ok_or_else(|| PluginError::Runtime(format!("plugin not installed: {name}")))?;
        record.state = PluginState::Disabled;
        self.save()?;
        self.audit.record(name, "disable", "plugin disabled");
        self.dispatch_hook(name, PluginHook::OnDisable, serde_json::json!({}))?;
        Ok(())
    }

    pub fn set_trust(&mut self, name: &str, tier: PluginTrustTier) -> PluginResult<()> {
        let record = self
            .state
            .plugins
            .get_mut(name)
            .ok_or_else(|| PluginError::Runtime(format!("plugin not installed: {name}")))?;
        record.trust_tier = tier.as_str().to_string();
        self.save()?;
        self.audit.record(
            name,
            "trust",
            format!("trust tier set to {}", tier.as_str()),
        );
        Ok(())
    }

    pub fn inspect(&self, name: &str) -> PluginResult<PluginInspectReport> {
        let record = self
            .state
            .plugins
            .get(name)
            .ok_or_else(|| PluginError::Runtime(format!("plugin not installed: {name}")))?
            .clone();
        let manifest = PluginManifest::load_from_dir(&record.install_path)?;
        Ok(PluginInspectReport {
            installed: record,
            manifest,
        })
    }

    pub fn dispatch_hook(
        &mut self,
        name: &str,
        hook: PluginHook,
        payload: serde_json::Value,
    ) -> PluginResult<crate::hooks::HookExecutionResult> {
        let record = self
            .state
            .plugins
            .get(name)
            .ok_or_else(|| PluginError::Runtime(format!("plugin not installed: {name}")))?
            .clone();
        if record.state == PluginState::Disabled
            && !matches!(
                hook,
                PluginHook::OnDisable | PluginHook::OnUninstall | PluginHook::OnInstall
            )
        {
            return Err(PluginError::Runtime(format!("plugin disabled: {name}")));
        }
        let manifest = PluginManifest::load_from_dir(&record.install_path)?;
        let enabled = parse_enabled_hooks(&manifest.hooks.enabled)?;
        if !enabled.is_empty() && !enabled.contains(&hook) {
            return Ok(crate::hooks::HookExecutionResult {
                hook: hook.as_str().to_string(),
                success: true,
                output: None,
                message: Some("hook not declared in manifest".into()),
            });
        }
        let loaded = PluginLoader::load(&record.install_path, &mut self.audit)?;
        let ctx = hook_context(name, hook, payload);
        loaded.execute_hook(hook, &ctx, &mut self.audit)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PluginInspectReport {
    pub installed: InstalledPlugin,
    pub manifest: PluginManifest,
}

pub struct PluginManager {
    store: PluginStore,
    host_version: String,
}

impl PluginManager {
    pub fn open(project_root: &Path, host_version: impl Into<String>) -> PluginResult<Self> {
        Ok(Self {
            store: PluginStore::open(project_root)?,
            host_version: host_version.into(),
        })
    }

    pub fn store(&self) -> &PluginStore {
        &self.store
    }

    pub fn store_mut(&mut self) -> &mut PluginStore {
        &mut self.store
    }

    pub fn host_version(&self) -> &str {
        &self.host_version
    }

    pub fn validate_security(
        &self,
        manifest: &PluginManifest,
        digest: Option<&str>,
        signature: Option<&RegistryVersionSignature>,
        dangerous_approved: bool,
    ) -> PluginResult<SecurityValidationReport> {
        let tier = registry_trust_tier(&manifest.plugin.name);
        validate_install_security(
            manifest,
            &self.host_version,
            digest,
            signature,
            tier,
            dangerous_approved,
        )
    }
}

fn validate_registry_entry_from_dir(source_dir: &Path) -> PluginResult<()> {
    let manifest_path = source_dir.join(MANIFEST_FILENAME);
    if !manifest_path.is_file() {
        return Err(PluginError::Manifest(format!(
            "missing {MANIFEST_FILENAME} in {}",
            source_dir.display()
        )));
    }
    let manifest = PluginManifest::load_from_dir(source_dir)?;
    validate_registry_entry(&manifest.plugin.name)
}

fn copy_dir_recursive(src: &Path, dst: &Path) -> PluginResult<()> {
    fs::create_dir_all(dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let file_type = entry.file_type()?;
        let target = dst.join(entry.file_name());
        if file_type.is_dir() {
            copy_dir_recursive(&entry.path(), &target)?;
        } else {
            fs::copy(entry.path(), target)?;
        }
    }
    Ok(())
}

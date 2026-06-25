//! JSON configuration compatibility for machine-generated configs.
//!
use crate::error::{ConfigError, ConfigResult};
use std::path::Path;

/// Load a configuration file as TOML or JSON based on extension.
pub fn load_config_value(path: &Path) -> ConfigResult<toml::Value> {
    // Load TOML or JSON configuration into a generic value tree.
    //
    // Parameters:
    // - `path` — path to `.toml` or `.json` config file
    //
    // Returns:
    // Parsed configuration value.
    //
    // Options:
    // None.
    //
    // Example:
    // let value = load_config_value(path)?;

    let content = std::fs::read_to_string(path).map_err(|source| ConfigError::Io {
        path: path.to_path_buf(),
        source,
    })?;
    parse_config_str(&content, path)
}

pub fn parse_config_str(content: &str, label: &Path) -> ConfigResult<toml::Value> {
    // Parse TOML or JSON text into a generic value tree.
    //
    // Parameters:
    // - `content` — raw file contents
    // - `label` — path used in error messages
    //
    // Returns:
    // Parsed configuration value.
    //
    // Options:
    // None.
    //
    // Example:
    // let value = parse_config_str(text, path)?;

    let ext = label
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("toml")
        .to_ascii_lowercase();
    if ext == "json" {
        let json: serde_json::Value =
            serde_json::from_str(content).map_err(|source| ConfigError::JsonParse {
                path: label.to_path_buf(),
                source,
            })?;
        let toml_str = toml::to_string(&json).map_err(|e| ConfigError::InvalidManifest {
            detail: e.to_string(),
        })?;
        toml::from_str(&toml_str).map_err(|source| ConfigError::TomlParse {
            path: label.to_path_buf(),
            source,
        })
    } else {
        toml::from_str(content).map_err(|source| ConfigError::TomlParse {
            path: label.to_path_buf(),
            source,
        })
    }
}

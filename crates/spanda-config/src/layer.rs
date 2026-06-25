//! Configuration layer types and merge strategies.
//!
use crate::manifest::MergeStrategyHint;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// A single configuration layer in the merge stack.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConfigLayer {
    pub name: String,
    pub path: PathBuf,
    pub depth: usize,
    pub content: toml::Value,
}

/// Strategy for merging array values across configuration layers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum ConfigMergeStrategy {
    #[default]
    Replace,
    Append,
    MergeById,
}

impl From<MergeStrategyHint> for ConfigMergeStrategy {
    fn from(hint: MergeStrategyHint) -> Self {
        match hint {
            MergeStrategyHint::Replace => Self::Replace,
            MergeStrategyHint::Append => Self::Append,
            MergeStrategyHint::MergeById => Self::MergeById,
        }
    }
}

/// Directed edge in the configuration dependency graph.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConfigGraphEdge {
    pub from: String,
    pub to: String,
    pub layer_kind: String,
}

/// Full configuration dependency graph for inspection.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct ConfigGraph {
    pub nodes: Vec<String>,
    pub edges: Vec<ConfigGraphEdge>,
    pub merge_order: Vec<String>,
}

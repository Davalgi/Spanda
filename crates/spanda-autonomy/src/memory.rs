//! Operational memory model — engineering memory categories.
//!
use serde::{Deserialize, Serialize};

/// Memory category in the operational memory model.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MemoryCategory {
    Reflex,
    Working,
    Episodic,
    Semantic,
    Procedural,
}

/// Fast local rules and safety reflexes.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReflexMemory {
    pub rules: Vec<String>,
}

/// Current mission context.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WorkingMemory {
    pub context_keys: Vec<String>,
}

/// Mission traces, incidents, replays.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EpisodicMemory {
    pub trace_ids: Vec<String>,
}

/// Entity graph, knowledge graph, relationships.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SemanticMemory {
    pub graph_refs: Vec<String>,
}

/// Recovery playbooks, decision policies, procedures.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProceduralMemory {
    pub playbook_ids: Vec<String>,
}

/// Full operational memory model for an entity.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct OperationalMemoryModel {
    pub reflex: ReflexMemory,
    pub working: WorkingMemory,
    pub episodic: EpisodicMemory,
    pub semantic: SemanticMemory,
    pub procedural: ProceduralMemory,
}

impl Default for ReflexMemory {
    fn default() -> Self {
        Self { rules: vec![] }
    }
}

impl Default for WorkingMemory {
    fn default() -> Self {
        Self {
            context_keys: vec![],
        }
    }
}

impl Default for EpisodicMemory {
    fn default() -> Self {
        Self { trace_ids: vec![] }
    }
}

impl Default for SemanticMemory {
    fn default() -> Self {
        Self { graph_refs: vec![] }
    }
}

impl Default for ProceduralMemory {
    fn default() -> Self {
        Self {
            playbook_ids: vec![],
        }
    }
}

/// Map a trace or artifact reference to a memory category.
pub fn categorize_memory(artifact_kind: &str) -> MemoryCategory {
    match artifact_kind {
        "reflex_rule" | "safety_reflex" | "kill_switch" => MemoryCategory::Reflex,
        "mission_context" | "active_mission" | "working_state" => MemoryCategory::Working,
        "trace" | "replay" | "incident" => MemoryCategory::Episodic,
        "entity_graph" | "knowledge_graph" | "relationship" => MemoryCategory::Semantic,
        "playbook" | "recovery_policy" | "decision_policy" | "procedure" => {
            MemoryCategory::Procedural
        }
        _ => MemoryCategory::Working,
    }
}

/// Map trace artifact to memory category and reference id.
pub fn map_trace_to_memory(trace_id: &str, artifact_kind: &str) -> (MemoryCategory, String) {
    (categorize_memory(artifact_kind), trace_id.into())
}

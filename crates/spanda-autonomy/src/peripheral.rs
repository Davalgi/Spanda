//! Peripheral autonomy layer — distributed execution hierarchy.
//!
use serde::{Deserialize, Serialize};

/// Node role in the peripheral autonomy hierarchy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PeripheralNodeRole {
    ControlCenter,
    RegionalCoordinator,
    FleetCoordinator,
    EntityRuntime,
    ReflexController,
}

/// A node in the peripheral autonomy hierarchy.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PeripheralNode {
    pub id: String,
    pub role: PeripheralNodeRole,
    pub parent_id: Option<String>,
    pub entity_id: Option<String>,
    pub offline_capable: bool,
}

/// Regional / site coordinator.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RegionalCoordinator {
    pub id: String,
    pub site: String,
    pub fleet_ids: Vec<String>,
}

/// Edge coordinator for site-level autonomy.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EdgeCoordinator {
    pub id: String,
    pub region_id: String,
    pub connected: bool,
}

/// Local autonomy node — bounded entity runtime.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LocalAutonomyNode {
    pub entity_id: String,
    pub decision_tree_signed: bool,
    pub offline_policy_signed: bool,
}

/// Full peripheral autonomy layer stack.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct PeripheralAutonomyLayer {
    pub nodes: Vec<PeripheralNode>,
}

impl PeripheralAutonomyLayer {
    /// Build default hierarchy for an entity under optional fleet and region.
    pub fn default_for_entity(
        entity_id: &str,
        fleet_id: Option<&str>,
        region_id: Option<&str>,
    ) -> Self {
        let mut nodes = vec![PeripheralNode {
            id: "control-center".into(),
            role: PeripheralNodeRole::ControlCenter,
            parent_id: None,
            entity_id: None,
            offline_capable: false,
        }];
        if let Some(region) = region_id {
            nodes.push(PeripheralNode {
                id: format!("region:{region}"),
                role: PeripheralNodeRole::RegionalCoordinator,
                parent_id: Some("control-center".into()),
                entity_id: None,
                offline_capable: true,
            });
        }
        if let Some(fleet) = fleet_id {
            let parent = region_id
                .map(|r| format!("region:{r}"))
                .unwrap_or_else(|| "control-center".into());
            nodes.push(PeripheralNode {
                id: format!("fleet:{fleet}"),
                role: PeripheralNodeRole::FleetCoordinator,
                parent_id: Some(parent),
                entity_id: None,
                offline_capable: true,
            });
        }
        let runtime_parent = fleet_id
            .map(|f| format!("fleet:{f}"))
            .or_else(|| region_id.map(|r| format!("region:{r}")))
            .unwrap_or_else(|| "control-center".into());
        nodes.push(PeripheralNode {
            id: format!("entity:{entity_id}"),
            role: PeripheralNodeRole::EntityRuntime,
            parent_id: Some(runtime_parent),
            entity_id: Some(entity_id.into()),
            offline_capable: true,
        });
        nodes.push(PeripheralNode {
            id: format!("reflex:{entity_id}"),
            role: PeripheralNodeRole::ReflexController,
            parent_id: Some(format!("entity:{entity_id}")),
            entity_id: Some(entity_id.into()),
            offline_capable: true,
        });
        Self { nodes }
    }
}

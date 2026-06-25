//! Operational policy declarations for verify-time rule enforcement.
//!
use crate::nodes::Span;
use serde::{Deserialize, Serialize};

/// Single operational policy rule inside a `policy` block.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind")]
pub enum OperationalPolicyRule {
    MaxSpeed {
        limit_mps: f64,
        span: Span,
    },
    RequiresKillSwitch {
        span: Span,
    },
    RequiresCapability {
        capabilities: Vec<String>,
        span: Span,
    },
    MinReadinessScore {
        score: u32,
        span: Span,
    },
    OperationHours {
        range: String,
        span: Span,
    },
}

/// Named operational policy (`policy WarehousePolicy { ... }`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind")]
pub enum OperationalPolicyDecl {
    OperationalPolicyDecl {
        name: String,
        rules: Vec<OperationalPolicyRule>,
        span: Span,
    },
}

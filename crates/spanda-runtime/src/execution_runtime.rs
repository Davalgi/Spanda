//! Execution runtime selection — LLVM native primary with interpreter LTS fallback.
//!
use serde::{Deserialize, Serialize};

/// How `spanda run` / `spanda sim` choose between native LLVM and the interpreter.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum ExecutionRuntime {
    /// Try native when the program lowers to eligible SIR and clang is available; else interpreter.
    #[default]
    Auto,
    /// Require native codegen; fail when unavailable or ineligible.
    Native,
    /// Force the tree-walking interpreter (long-term support path).
    Interpreter,
}

impl ExecutionRuntime {
    /// Resolve runtime mode from CLI `--runtime` and `SPANDA_RUNTIME`.
    pub fn resolve(flag: Option<&str>) -> Self {
        if let Some(from_flag) = flag.and_then(Self::parse) {
            return from_flag;
        }
        std::env::var("SPANDA_RUNTIME")
            .ok()
            .as_deref()
            .and_then(Self::parse)
            .unwrap_or_default()
    }

    fn parse(value: &str) -> Option<Self> {
        match value.to_ascii_lowercase().as_str() {
            "auto" | "primary" | "default" => Some(Self::Auto),
            "native" | "llvm" => Some(Self::Native),
            "interpreter" | "interpreted" | "lts" => Some(Self::Interpreter),
            _ => None,
        }
    }

    /// Whether the dispatch layer should attempt native codegen first.
    pub fn prefers_native(self) -> bool {
        matches!(self, Self::Auto | Self::Native)
    }

    /// Whether interpreter execution is permitted when native is unavailable.
    pub fn allows_interpreter_fallback(self) -> bool {
        matches!(self, Self::Auto | Self::Interpreter)
    }
}

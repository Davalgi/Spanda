//! Chaos engineering experiments for Spanda autonomous system programs.
//!
pub mod experiment;

pub use experiment::{
    default_injections, format_chaos_report, normalize_injection, run_chaos_experiment,
    ChaosExperimentOptions, ChaosFormat, ChaosInjectionResult, ChaosReport,
};

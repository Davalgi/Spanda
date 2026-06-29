//! Injectable runtime fault detection boundary for interpreter polling.
//!
use crate::fault_primitives::{
    empty_fault_scan_report, faults_from_hardware_signals, record_fault_in_trace,
};
use crate::fault_types::{FaultScanOptions, FaultScanReport, RuntimeFault};
use crate::replay::MissionTrace;
use spanda_ast::nodes::Program;
use std::sync::Arc;

pub use crate::fault_types::{
    FaultEvidence, FaultTimeline, RuntimeFaultKind, RuntimeHealth, RuntimeHealthStatus,
};

/// Extension points for runtime fault scanning and trace recording.
pub trait FaultRuntime: Send + Sync {
    fn faults_from_hardware_signals(
        &self,
        faults: &[String],
        events: &[String],
        sim_time_ms: f64,
    ) -> Vec<RuntimeFault>;

    fn scan_program_faults(
        &self,
        program: &Program,
        source_label: &str,
        options: &FaultScanOptions,
    ) -> FaultScanReport;

    fn record_fault_in_trace(
        &self,
        trace: &mut MissionTrace,
        fault: &RuntimeFault,
        sim_time_ms: f64,
    );
}

/// Built-in fault runtime using kernel primitives without full AST fault scanning.
#[derive(Debug, Default, Clone, Copy)]
pub struct BuiltinFaultRuntime;

impl FaultRuntime for BuiltinFaultRuntime {
    fn faults_from_hardware_signals(
        &self,
        faults: &[String],
        events: &[String],
        sim_time_ms: f64,
    ) -> Vec<RuntimeFault> {
        faults_from_hardware_signals(faults, events, sim_time_ms)
    }

    fn scan_program_faults(
        &self,
        _program: &Program,
        source_label: &str,
        options: &FaultScanOptions,
    ) -> FaultScanReport {
        empty_fault_scan_report(source_label, options)
    }

    fn record_fault_in_trace(
        &self,
        trace: &mut MissionTrace,
        fault: &RuntimeFault,
        sim_time_ms: f64,
    ) {
        record_fault_in_trace(trace, fault, sim_time_ms);
    }
}

/// Shared fault runtime handle passed through run options at the driver boundary.
pub type SharedFaultRuntime = Arc<dyn FaultRuntime>;

/// Default built-in fault runtime for direct interpreter use without runtime-faults crate.
pub fn default_fault_runtime() -> SharedFaultRuntime {
    Arc::new(BuiltinFaultRuntime)
}

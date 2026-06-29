//! Built-in fault signal conversion and mission trace recording primitives.
//!
use crate::fault_types::{
    FaultEvidence, FaultScanOptions, FaultScanReport, RuntimeFault, RuntimeFaultKind,
    RuntimeHealth, RuntimeHealthStatus,
};
use crate::replay::MissionTrace;

/// Convert hardware monitor fault/event names into structured runtime faults.
pub fn faults_from_hardware_signals(
    faults: &[String],
    events: &[String],
    sim_time_ms: f64,
) -> Vec<RuntimeFault> {
    // Map hardware monitor labels into runtime fault records for interpreter polling.
    //
    // Parameters:
    // - `faults` — active fault labels from the hardware monitor
    // - `events` — active event labels from the hardware monitor
    // - `sim_time_ms` — current simulation time in milliseconds
    //
    // Returns:
    // Structured runtime fault records for each signal.
    //
    // Options:
    // None.
    //
    // Example:
    // let faults = faults_from_hardware_signals(&hw_faults, &hw_events, sim_ms);

    let mut out = Vec::new();
    for fault in faults {
        let (kind, status) = hardware_label_to_fault(fault);
        out.push(RuntimeFault {
            kind,
            target: fault.clone(),
            status,
            message: format!("Hardware fault: {fault}"),
            evidence: FaultEvidence {
                related_events: vec![fault.clone()],
                ..Default::default()
            },
            detected_at_ms: sim_time_ms,
        });
    }
    for event in events {
        if faults.contains(event) {
            continue;
        }
        let (kind, status) = hardware_label_to_fault(event);
        out.push(RuntimeFault {
            kind,
            target: event.clone(),
            status,
            message: format!("Hardware event: {event}"),
            evidence: FaultEvidence {
                related_events: vec![event.clone()],
                ..Default::default()
            },
            detected_at_ms: sim_time_ms,
        });
    }
    out
}

/// Record a runtime fault in a mission trace.
pub fn record_fault_in_trace(trace: &mut MissionTrace, fault: &RuntimeFault, sim_time_ms: f64) {
    // Append a fault event frame to a mission trace for replay analysis.
    //
    // Parameters:
    // - `trace` — mission trace to append to
    // - `fault` — detected runtime fault
    // - `sim_time_ms` — simulation timestamp
    //
    // Returns:
    // None (modifies trace in place).
    //
    // Options:
    // None.
    //
    // Example:
    // record_fault_in_trace(&mut trace, &fault, 1000.0);

    let event = fault_kind_to_trace_event(&fault.kind);
    let payload = serde_json::json!({
        "kind": fault.kind.as_str(),
        "target": fault.target,
        "status": fault.status.as_str(),
        "message": fault.message,
        "evidence": fault.evidence,
    });
    trace.record(sim_time_ms, event, payload);
}

fn hardware_label_to_fault(label: &str) -> (RuntimeFaultKind, RuntimeHealthStatus) {
    let lower = label.to_ascii_lowercase();
    if lower.contains("crash") || lower.contains("critical") {
        (RuntimeFaultKind::ProcessCrash, RuntimeHealthStatus::Crashed)
    } else if lower.contains("offline") || lower.contains("camera") {
        (
            RuntimeFaultKind::SensorDriverCrash,
            RuntimeHealthStatus::Degraded,
        )
    } else if lower.contains("gps") || lower.contains("degraded") {
        (
            RuntimeFaultKind::TaskStarvation,
            RuntimeHealthStatus::Degraded,
        )
    } else if lower.contains("heartbeat") {
        (
            RuntimeFaultKind::HeartbeatLoss,
            RuntimeHealthStatus::Degraded,
        )
    } else if lower.contains("memory") {
        (
            RuntimeFaultKind::MemoryPressure,
            RuntimeHealthStatus::Warning,
        )
    } else if lower.contains("watchdog") {
        (
            RuntimeFaultKind::WatchdogTimeout,
            RuntimeHealthStatus::Critical,
        )
    } else {
        (
            RuntimeFaultKind::AbnormalShutdown,
            RuntimeHealthStatus::Warning,
        )
    }
}

fn fault_kind_to_trace_event(kind: &RuntimeFaultKind) -> &'static str {
    match kind {
        RuntimeFaultKind::ProcessCrash
        | RuntimeFaultKind::RuntimePanic
        | RuntimeFaultKind::ProviderCrash
        | RuntimeFaultKind::PackageCrash => "fault_crash",
        RuntimeFaultKind::UnexpectedReboot | RuntimeFaultKind::OsReboot => "fault_reboot",
        RuntimeFaultKind::WatchdogTimeout => "fault_watchdog_timeout",
        RuntimeFaultKind::MemoryLeak => "fault_memory_growth",
        RuntimeFaultKind::RestartLoop => "fault_restart_loop",
        RuntimeFaultKind::CpuOverload
        | RuntimeFaultKind::MemoryPressure
        | RuntimeFaultKind::DiskPressure => "fault_resource_pressure",
        RuntimeFaultKind::HeartbeatLoss => "fault_heartbeat_loss",
        RuntimeFaultKind::Deadlock | RuntimeFaultKind::TaskStarvation => "fault_deadlock",
        RuntimeFaultKind::OutOfMemory => "fault_oom",
        _ => "fault_event",
    }
}

/// Build an empty fault scan report for built-in fault runtimes without AST scanning.
pub fn empty_fault_scan_report(source_label: &str, options: &FaultScanOptions) -> FaultScanReport {
    FaultScanReport {
        source: source_label.into(),
        health: RuntimeHealth {
            overall: RuntimeHealthStatus::Healthy,
            heartbeats: Vec::new(),
            processes: Vec::new(),
            active_faults: Vec::new(),
            uptime_ms: options.sim_time_ms,
            boot_id: None,
        },
        faults: Vec::new(),
        timeline: Vec::new(),
        heartbeats_configured: 0,
        memory_watches_configured: 0,
        resource_watches_configured: 0,
        restart_policies_configured: 0,
        passed: true,
    }
}

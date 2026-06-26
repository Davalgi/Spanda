//! Runtime health context for live readiness evaluation.

use spanda_ast::nodes::{Program, RobotDecl, SensorDecl};
use spanda_config::{health_inject_faults, ResolvedSystemConfig};
use spanda_hal::HardwareMonitor;

/// Live fault and event signals used during readiness evaluation.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct RuntimeReadinessContext {
    pub faults: Vec<String>,
    pub events: Vec<String>,
}

const DEFAULT_HEALTH_FAULTS: &[&str] = &["GPSDegraded", "CameraOffline", "RobotHealthCritical"];

/// Register robot sensors/actuators on a hardware monitor from the program AST.
pub fn seed_hardware_monitor(program: &Program, monitor: &mut HardwareMonitor) {
    // Description:
    //     Seed hardware monitor.
    //
    // Inputs:
    //     progra: &Program
    //         Caller-supplied progra.
    //     onitor: &mut HardwareMonitor
    //         Caller-supplied onitor.
    //
    // Outputs:
    //     None.
    //
    // Example:

    //     let result = spanda_readiness::runtime::seed_hardware_monitor(progra, onitor);

    let Program::Program { robots, .. } = program;
    for robot in robots {
        let RobotDecl::RobotDecl {
            sensors, actuators, ..
        } = robot;
        for sensor in sensors {
            let SensorDecl::SensorDecl {
                name, sensor_type, ..
            } = sensor;
            monitor.register_sensor(name, sensor_type);
        }
        for actuator in actuators {
            let spanda_ast::nodes::ActuatorDecl::ActuatorDecl {
                name,
                actuator_type,
                ..
            } = actuator;
            monitor.register_actuator(name, actuator_type);
        }
    }
}

/// Build runtime readiness context from a program and optional fault injection.
pub fn build_runtime_context(
    program: &Program,
    inject_health_faults: bool,
) -> RuntimeReadinessContext {
    build_runtime_context_with_config(program, inject_health_faults, None)
}

/// Build runtime readiness context using per-robot `[health]` policies when configured.
pub fn build_runtime_context_with_config(
    program: &Program,
    inject_health_faults: bool,
    config: Option<&ResolvedSystemConfig>,
) -> RuntimeReadinessContext {
    // Description:
    //     Build runtime context.
    //
    // Inputs:
    //     progra: &Program
    //         Caller-supplied progra.
    //     inject_health_faults: bool
    //         Caller-supplied inject health faults.
    //
    // Outputs:
    //     result: RuntimeReadinessContext
    //         Return value from `build_runtime_context`.
    //
    // Example:

    //     let result = spanda_readiness::runtime::build_runtime_context(progra, inject_health_faults);

    let mut monitor = HardwareMonitor::default();
    seed_hardware_monitor(program, &mut monitor);
    if inject_health_faults {
        let faults = health_faults_for_runtime(program, config);
        for fault in faults {
            monitor.inject_fault(fault);
        }
    }
    RuntimeReadinessContext {
        faults: monitor.runtime_faults(),
        events: monitor.runtime_events(),
    }
}

fn health_faults_for_runtime(
    program: &Program,
    config: Option<&ResolvedSystemConfig>,
) -> Vec<String> {
    let mut faults = Vec::new();
    if let Some(cfg) = config {
        let robot_ids: Vec<String> = if !cfg.robot_ids().is_empty() {
            cfg.robot_ids().into_iter().map(str::to_owned).collect()
        } else {
            let Program::Program { robots, .. } = program;
            robots
                .iter()
                .map(|RobotDecl::RobotDecl { name, .. }| name.clone())
                .collect()
        };
        for id in robot_ids {
            faults.extend(health_inject_faults(cfg, &id));
        }
        faults.sort();
        faults.dedup();
    }
    if faults.is_empty() {
        DEFAULT_HEALTH_FAULTS
            .iter()
            .map(|fault| (*fault).to_string())
            .collect()
    } else {
        faults
    }
}

impl RuntimeReadinessContext {
    /// Capture faults and events from an existing hardware monitor instance.
    pub fn from_monitor(monitor: &HardwareMonitor) -> Self {
        // Description:
        //     From monitor.
        //
        // Inputs:
        //     onitor: &HardwareMonitor
        //         Caller-supplied onitor.
        //
        // Outputs:
        //     result: Self
        //         Return value from `from_monitor`.
        //
        // Example:

        //     let result = spanda_readiness::runtime::from_monitor(onitor);

        Self {
            faults: monitor.runtime_faults(),
            events: monitor.runtime_events(),
        }
    }
}

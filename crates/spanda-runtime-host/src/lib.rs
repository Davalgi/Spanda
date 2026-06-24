//! Default `RuntimeHost` wiring for navigation, SLAM, and connectivity.
//!
pub mod nav2_adapter;
pub mod robotics_validation;
pub mod slam_adapter;
pub mod type_check_host;

pub use type_check_host::{core_type_check_host, CoreTypeCheckHost};

use spanda_comm::TransportKind;
use spanda_connectivity::adapter_bridge;
use spanda_runtime::{RuntimeHost, RuntimeValue};
use std::collections::HashSet;

/// Default host wiring domain adapters from `spanda-core` into `spanda-runtime`.
pub struct CoreRuntimeHost;

impl RuntimeHost for CoreRuntimeHost {
    fn slam_import_known(&self, path: &str) -> bool {
        // Description:
        //     Slam import known.
        //
        // Inputs:
        //     &self: input value
        //         Caller-supplied &self.
        //     path: &str
        //         Caller-supplied path.
        //
        // Outputs:
        //     result: bool
        //         Return value from `slam_import_known`.
        //
        // Example:

        //     let result = spanda_runtime_host::slam_import_known(&self, path);

        slam_adapter::slam_import_paths().contains(&path)
    }

    fn navigation_import_known(&self, path: &str) -> bool {
        // Description:
        //     Navigation import known.
        //
        // Inputs:
        //     &self: input value
        //         Caller-supplied &self.
        //     path: &str
        //         Caller-supplied path.
        //
        // Outputs:
        //     result: bool
        //         Return value from `navigation_import_known`.
        //
        // Example:

        //     let result = spanda_runtime_host::navigation_import_known(&self, path);

        nav2_adapter::nav2_import_paths().contains(&path)
    }

    fn invoke_nav2_bridge(&self, goal: &str) -> Option<String> {
        // Description:
        //     Invoke nav2 bridge.
        //
        // Inputs:
        //     &self: input value
        //         Caller-supplied &self.
        //     goal: &str
        //         Caller-supplied goal.
        //
        // Outputs:
        //     result: Option<String>
        //         Return value from `invoke_nav2_bridge`.
        //
        // Example:

        //     let result = spanda_runtime_host::invoke_nav2_bridge(&self, goal);

        adapter_bridge::invoke_nav2_bridge(goal)
    }

    fn invoke_slam_bridge(&self, op: &str) -> Option<String> {
        // Description:
        //     Invoke slam bridge.
        //
        // Inputs:
        //     &self: input value
        //         Caller-supplied &self.
        //     op: &str
        //         Caller-supplied op.
        //
        // Outputs:
        //     result: Option<String>
        //         Return value from `invoke_slam_bridge`.
        //
        // Example:

        //     let result = spanda_runtime_host::invoke_slam_bridge(&self, op);

        adapter_bridge::invoke_slam_bridge(op)
    }

    fn connectivity_link_to_transport(&self, link: &str) -> TransportKind {
        // Description:
        //     Connectivity link to transport.
        //
        // Inputs:
        //     &self: input value
        //         Caller-supplied &self.
        //     link: &str
        //         Caller-supplied link.
        //
        // Outputs:
        //     result: TransportKind
        //         Return value from `connectivity_link_to_transport`.
        //
        // Example:

        //     let result = spanda_runtime_host::connectivity_link_to_transport(&self, link);

        spanda_connectivity_runtime::connectivity_link_to_transport(link)
    }

    fn hardware_event_to_connectivity(&self, event: &str) -> Option<(&'static str, &'static str)> {
        // Description:
        //     Hardware event to connectivity.
        //
        // Inputs:
        //     &self: input value
        //         Caller-supplied &self.
        //     even: &str
        //         Caller-supplied even.
        //
        // Outputs:
        //     result: Option<(&'static str, &'static str)>
        //         Return value from `hardware_event_to_connectivity`.
        //
        // Example:

        //     let result = spanda_runtime_host::hardware_event_to_connectivity(&self, even);

        spanda_connectivity::hardware_event_to_connectivity(event)
    }

    fn fault_to_connectivity(&self, fault: &str) -> Option<(&'static str, &'static str)> {
        // Description:
        //     Fault to connectivity.
        //
        // Inputs:
        //     &self: input value
        //         Caller-supplied &self.
        //     faul: &str
        //         Caller-supplied faul.
        //
        // Outputs:
        //     result: Option<(&'static str, &'static str)>
        //         Return value from `fault_to_connectivity`.
        //
        // Example:

        //     let result = spanda_runtime_host::fault_to_connectivity(&self, faul);

        spanda_connectivity::fault_to_connectivity(fault)
    }

    fn is_link_impaired(&self, link: &str, faults: &HashSet<String>) -> bool {
        // Description:
        //     Is link impaired.
        //
        // Inputs:
        //     &self: input value
        //         Caller-supplied &self.
        //     link: &str
        //         Caller-supplied link.
        //     faults: &HashSet<String>
        //         Caller-supplied faults.
        //
        // Outputs:
        //     result: bool
        //         Return value from `is_link_impaired`.
        //
        // Example:

        //     let result = spanda_runtime_host::is_link_impaired(&self, link, faults);

        spanda_connectivity::is_link_impaired(link, faults)
    }

    fn apply_gps_position_faults(
        &self,
        faults: &HashSet<String>,
        true_lat: f64,
        true_lon: f64,
        sim_time_ms: f64,
    ) -> (f64, f64, f64) {
        // Description:
        //     Apply gps position faults.
        //
        // Inputs:
        //     &self: input value
        //         Caller-supplied &self.
        //     faults: &HashSet<String>
        //         Caller-supplied faults.
        //     rue_la: f64
        //         Caller-supplied rue la.
        //     rue_lon: f64
        //         Caller-supplied rue lon.
        //     sim_time_ms: f64
        //         Caller-supplied sim time ms.
        //
        // Outputs:
        //     result: (f64, f64, f64)
        //         Return value from `apply_gps_position_faults`.
        //
        // Example:

        //     let result = spanda_runtime_host::apply_gps_position_faults(&self, faults, rue_la, rue_lon, sim_time_ms);

        spanda_connectivity::apply_gps_position_faults(faults, true_lat, true_lon, sim_time_ms)
    }

    fn geofence_contains(
        &self,
        center_lat: f64,
        center_lon: f64,
        radius_m: f64,
        lat: f64,
        lon: f64,
    ) -> bool {
        // Description:
        //     Geofence contains.
        //
        // Inputs:
        //     &self: input value
        //         Caller-supplied &self.
        //     center_la: f64
        //         Caller-supplied center la.
        //     center_lon: f64
        //         Caller-supplied center lon.
        //     radius_: f64
        //         Caller-supplied radius.
        //     la: f64
        //         Caller-supplied la.
        //     lon: f64
        //         Caller-supplied lon.
        //
        // Outputs:
        //     result: bool
        //         Return value from `geofence_contains`.
        //
        // Example:

        //     let result = spanda_runtime_host::geofence_contains(&self, center_la, center_lon, radius_, la, lon);

        let fence = spanda_connectivity::GeofenceRuntime {
            name: "runtime".into(),
            center_lat,
            center_lon,
            radius_m,
        };
        spanda_connectivity::geofence_contains(&fence, lat, lon)
    }

    fn is_modem_bearer(&self, link: &str) -> bool {
        // Description:
        //     Is modem bearer.
        //
        // Inputs:
        //     &self: input value
        //         Caller-supplied &self.
        //     link: &str
        //         Caller-supplied link.
        //
        // Outputs:
        //     result: bool
        //         Return value from `is_modem_bearer`.
        //
        // Example:

        //     let result = spanda_runtime_host::is_modem_bearer(&self, link);

        spanda_connectivity::is_modem_bearer(link)
    }

    fn apply_gps_reading_faults(
        &self,
        reading: RuntimeValue,
        faults: &HashSet<String>,
        true_lat: f64,
        true_lon: f64,
        sim_time_ms: f64,
    ) -> RuntimeValue {
        // Description:
        //     Apply gps reading faults.
        //
        // Inputs:
        //     &self: input value
        //         Caller-supplied &self.
        //     reading: RuntimeValue
        //         Caller-supplied reading.
        //     faults: &HashSet<String>
        //         Caller-supplied faults.
        //     rue_la: f64
        //         Caller-supplied rue la.
        //     rue_lon: f64
        //         Caller-supplied rue lon.
        //     sim_time_ms: f64
        //         Caller-supplied sim time ms.
        //
        // Outputs:
        //     result: RuntimeValue
        //         Return value from `apply_gps_reading_faults`.
        //
        // Example:

        //     let result = spanda_runtime_host::apply_gps_reading_faults(&self, reading, faults, rue_la, rue_lon, sim_time_ms);

        spanda_connectivity_runtime::apply_gps_reading_faults(
            reading,
            faults,
            true_lat,
            true_lon,
            sim_time_ms,
        )
    }

    fn runtime_sim_identity(&self, link: &str, attested: bool) -> RuntimeValue {
        // Description:
        //     Runtime sim identity.
        //
        // Inputs:
        //     &self: input value
        //         Caller-supplied &self.
        //     link: &str
        //         Caller-supplied link.
        //     attested: bool
        //         Caller-supplied attested.
        //
        // Outputs:
        //     result: RuntimeValue
        //         Return value from `runtime_sim_identity`.
        //
        // Example:

        //     let result = spanda_runtime_host::runtime_sim_identity(&self, link, attested);

        spanda_connectivity_runtime::runtime_sim_identity(link, attested)
    }
}

/// Shared core runtime host instance for interpreter wiring.
pub fn core_runtime_host() -> &'static CoreRuntimeHost {
    // Description:
    //     Core runtime host.
    //
    // Inputs:
    //     None.
    //
    // Outputs:
    //     result: &'static CoreRuntimeHost
    //         Return value from `core_runtime_host`.
    //
    // Example:

    //     let result = spanda_runtime_host::core_runtime_host();

    &CoreRuntimeHost
}

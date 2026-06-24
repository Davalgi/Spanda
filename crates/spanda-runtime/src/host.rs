//! Host hooks for domain-specific interpreter behavior in `spanda-core`.
//!

use crate::value::RuntimeValue;
use spanda_ast::comm_decl::TransportKind;
use std::collections::HashSet;

/// Domain-specific runtime services supplied by the embedding application.
pub trait RuntimeHost {
    /// Whether an import path enables SLAM adapter hooks.
    fn slam_import_known(&self, path: &str) -> bool;

    /// Whether an import path enables navigation adapter hooks.
    fn navigation_import_known(&self, path: &str) -> bool;

    /// Invoke an external Nav2 bridge when the host configures one.
    fn invoke_nav2_bridge(&self, goal: &str) -> Option<String> {
        // Description:
        //     Invoke nav2 bridge.
        //
        // Inputs:
        //     &self: value
        //         Caller-supplied &self.
        //     goal: &str
        //         Caller-supplied goal.
        //
        // Outputs:
        //     result: Option<String>
        //         Return value from `invoke_nav2_bridge`.
        //
        // Example:

        //     let result = spanda_runtime::host::invoke_nav2_bridge(&self, goal);

        let _ = goal;
        None
    }

    /// Invoke an external SLAM bridge when the host configures one.
    fn invoke_slam_bridge(&self, op: &str) -> Option<String> {
        // Description:
        //     Invoke slam bridge.
        //
        // Inputs:
        //     &self: value
        //         Caller-supplied &self.
        //     op: &str
        //         Caller-supplied op.
        //
        // Outputs:
        //     result: Option<String>
        //         Return value from `invoke_slam_bridge`.
        //
        // Example:

        //     let result = spanda_runtime::host::invoke_slam_bridge(&self, op);

        let _ = op;
        None
    }

    /// Map an active connectivity link name to the default transport backend.
    fn connectivity_link_to_transport(&self, link: &str) -> TransportKind {
        // Description:
        //     Connectivity link to transport.
        //
        // Inputs:
        //     &self: value
        //         Caller-supplied &self.
        //     link: &str
        //         Caller-supplied link.
        //
        // Outputs:
        //     result: TransportKind
        //         Return value from `connectivity_link_to_transport`.
        //
        // Example:

        //     let result = spanda_runtime::host::connectivity_link_to_transport(&self, link);

        let _ = link;
        TransportKind::Sim
    }

    /// Map a hardware event to a connectivity trigger `(domain, event)` pair.
    fn hardware_event_to_connectivity(&self, event: &str) -> Option<(&'static str, &'static str)> {
        // Description:
        //     Hardware event to connectivity.
        //
        // Inputs:
        //     &self: value
        //         Caller-supplied &self.
        //     even: &str
        //         Caller-supplied even.
        //
        // Outputs:
        //     result: Option<(&'static str, &'static str)>
        //         Return value from `hardware_event_to_connectivity`.
        //
        // Example:

        //     let result = spanda_runtime::host::hardware_event_to_connectivity(&self, even);

        let _ = event;
        None
    }

    /// Map a fault string to a connectivity trigger `(domain, event)` pair.
    fn fault_to_connectivity(&self, fault: &str) -> Option<(&'static str, &'static str)> {
        // Description:
        //     Fault to connectivity.
        //
        // Inputs:
        //     &self: value
        //         Caller-supplied &self.
        //     faul: &str
        //         Caller-supplied faul.
        //
        // Outputs:
        //     result: Option<(&'static str, &'static str)>
        //         Return value from `fault_to_connectivity`.
        //
        // Example:

        //     let result = spanda_runtime::host::fault_to_connectivity(&self, faul);

        let _ = fault;
        None
    }

    /// Return true when the active link should be considered impaired by current faults.
    fn is_link_impaired(&self, link: &str, faults: &HashSet<String>) -> bool {
        // Description:
        //     Is link impaired.
        //
        // Inputs:
        //     &self: value
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

        //     let result = spanda_runtime::host::is_link_impaired(&self, link, faults);

        let _ = (link, faults);
        false
    }

    /// Apply GPS drift/spoof faults to the true lat/lon at simulation time.
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
        //     &self: value
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

        //     let result = spanda_runtime::host::apply_gps_position_faults(&self, faults, rue_la, rue_lon, sim_time_ms);

        let _ = (faults, sim_time_ms);
        (true_lat, true_lon, 1.0)
    }

    /// Return true when `(lat, lon)` is inside a geofence circle.
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
        //     &self: value
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

        //     let result = spanda_runtime::host::geofence_contains(&self, center_la, center_lon, radius_, la, lon);

        let _ = (center_lat, center_lon, radius_m, lat, lon);
        false
    }

    /// Return true when the link name denotes a cellular or satellite modem bearer.
    fn is_modem_bearer(&self, link: &str) -> bool {
        // Description:
        //     Is modem bearer.
        //
        // Inputs:
        //     &self: value
        //         Caller-supplied &self.
        //     link: &str
        //         Caller-supplied link.
        //
        // Outputs:
        //     result: bool
        //         Return value from `is_modem_bearer`.
        //
        // Example:

        //     let result = spanda_runtime::host::is_modem_bearer(&self, link);

        let _ = link;
        false
    }

    /// Rewrite a GPS/GNSS sensor reading after applying simulation faults.
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
        //     &self: value
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

        //     let result = spanda_runtime::host::apply_gps_reading_faults(&self, reading, faults, rue_la, rue_lon, sim_time_ms);

        let _ = (faults, true_lat, true_lon, sim_time_ms);
        reading
    }

    /// Produce a [`SimIdentity`]-shaped runtime object for SIM/eSIM attestation simulation.
    fn runtime_sim_identity(&self, link: &str, attested: bool) -> RuntimeValue {
        // Description:
        //     Runtime sim identity.
        //
        // Inputs:
        //     &self: value
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

        //     let result = spanda_runtime::host::runtime_sim_identity(&self, link, attested);

        let _ = (link, attested);
        RuntimeValue::Void
    }
}

/// Return true when any import path enables SLAM adapter behavior.
pub fn imports_enable_slam(paths: &[&str], host: &dyn RuntimeHost) -> bool {
    // Description:
    //     Imports enable slam.
    //
    // Inputs:
    //     paths: &[&str]
    //         Caller-supplied paths.
    //     hos: &dyn RuntimeHost
    //         Caller-supplied hos.
    //
    // Outputs:
    //     result: bool
    //         Return value from `imports_enable_slam`.
    //
    // Example:

    //     let result = spanda_runtime::host::imports_enable_slam(paths, hos);

    paths.iter().any(|path| host.slam_import_known(path))
}

/// Return true when any import path enables navigation adapter behavior.
pub fn imports_enable_navigation(paths: &[&str], host: &dyn RuntimeHost) -> bool {
    // Description:
    //     Imports enable navigation.
    //
    // Inputs:
    //     paths: &[&str]
    //         Caller-supplied paths.
    //     hos: &dyn RuntimeHost
    //         Caller-supplied hos.
    //
    // Outputs:
    //     result: bool
    //         Return value from `imports_enable_navigation`.
    //
    // Example:

    //     let result = spanda_runtime::host::imports_enable_navigation(paths, hos);

    paths.iter().any(|path| host.navigation_import_known(path))
}

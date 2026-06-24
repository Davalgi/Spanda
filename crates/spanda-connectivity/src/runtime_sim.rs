//! Runtime-facing connectivity simulation helpers without AST/runtime dependencies.
//!
use std::collections::HashSet;

/// Runtime geofence circle loaded from a program declaration.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GeofenceRuntime {
    pub name: String,
    pub center_lat: f64,
    pub center_lon: f64,
    pub radius_m: f64,
}

/// Haversine distance in meters between two WGS84 points.
pub fn haversine_m(lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> f64 {
    // Description:
    //     Haversine m.
    //
    // Inputs:
    //     lat1: f64
    //         Caller-supplied lat1.
    //     lon1: f64
    //         Caller-supplied lon1.
    //     lat2: f64
    //         Caller-supplied lat2.
    //     lon2: f64
    //         Caller-supplied lon2.
    //
    // Outputs:
    //     result: f64
    //         Return value from `haversine_m`.
    //
    // Example:

    //     let result = spanda_connectivity::runtime_sim::haversine_m(lat1, lon1, lat2, lon2);

    const R: f64 = 6_371_000.0;
    let d_lat = (lat2 - lat1).to_radians();
    let d_lon = (lon2 - lon1).to_radians();
    let a = (d_lat / 2.0).sin().powi(2)
        + lat1.to_radians().cos() * lat2.to_radians().cos() * (d_lon / 2.0).sin().powi(2);
    2.0 * R * a.sqrt().asin()
}

/// Return true when `(lat, lon)` is inside the geofence circle.
pub fn geofence_contains(fence: &GeofenceRuntime, lat: f64, lon: f64) -> bool {
    // Description:
    //     Geofence contains.
    //
    // Inputs:
    //     fence: &GeofenceRuntime
    //         Caller-supplied fence.
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

    //     let result = spanda_connectivity::runtime_sim::geofence_contains(fence, la, lon);

    haversine_m(lat, lon, fence.center_lat, fence.center_lon) <= fence.radius_m
}

/// Map hardware monitor events to connectivity trigger `(domain, event)` pairs.
pub fn hardware_event_to_connectivity(event: &str) -> Option<(&'static str, &'static str)> {
    // Description:
    //     Hardware event to connectivity.
    //
    // Inputs:
    //     even: &str
    //         Caller-supplied even.
    //
    // Outputs:
    //     result: Option<(&'static str, &'static str)>
    //         Return value from `hardware_event_to_connectivity`.
    //
    // Example:

    //     let result = spanda_connectivity::runtime_sim::hardware_event_to_connectivity(even);

    match event {
        "GpsFailure" => Some(("gps", "lost")),
        _ => None,
    }
}

/// Map comm bus fault names to connectivity trigger pairs.
pub fn fault_to_connectivity(fault: &str) -> Option<(&'static str, &'static str)> {
    // Description:
    //     Fault to connectivity.
    //
    // Inputs:
    //     faul: &str
    //         Caller-supplied faul.
    //
    // Outputs:
    //     result: Option<(&'static str, &'static str)>
    //         Return value from `fault_to_connectivity`.
    //
    // Example:

    //     let result = spanda_connectivity::runtime_sim::fault_to_connectivity(faul);

    match fault {
        "NetworkOutage" | "LteOutage" | "SatelliteOutage" | "WeakWifi" => {
            Some(("network", "disconnected"))
        }
        "BluetoothDisconnect" => Some(("bluetooth", "device_disconnected")),
        "FiveGHandoff" => Some(("cellular", "roaming")),
        "GpsSpoofing" => Some(("gps", "spoofed")),
        "GpsDrift" => Some(("gps", "drift")),
        _ => None,
    }
}

/// Apply GPS drift/spoofing simulation to WGS84 coordinates.
pub fn apply_gps_position_faults(
    faults: &HashSet<String>,
    true_lat: f64,
    true_lon: f64,
    sim_time_ms: f64,
) -> (f64, f64, f64) {
    // Description:
    //     Apply gps position faults.
    //
    // Inputs:
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

    //     let result = spanda_connectivity::runtime_sim::apply_gps_position_faults(faults, rue_la, rue_lon, sim_time_ms);

    if faults.contains("GpsSpoofing") {
        return (true_lat + 0.009, true_lon + 0.012, 0.3);
    }
    if faults.contains("GpsDrift") {
        let drift_m = (sim_time_ms / 1000.0) * 0.05;
        let d_deg = drift_m / 111_000.0;
        return (true_lat + d_deg, true_lon + d_deg * 0.5, 0.8);
    }
    (true_lat, true_lon, 1.0)
}

/// Return true when simulation faults disable the given connectivity link.
pub fn is_link_impaired(link: &str, faults: &HashSet<String>) -> bool {
    // Description:
    //     Is link impaired.
    //
    // Inputs:
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

    //     let result = spanda_connectivity::runtime_sim::is_link_impaired(link, faults);

    let link = link.to_ascii_lowercase();
    for fault in faults {
        match fault.as_str() {
            "NetworkOutage" => {
                if super::is_satellite_link(&link) || link == "bluetooth" || link == "ble" {
                    continue;
                }
                if super::is_wifi_link(&link)
                    || super::is_cellular_link(&link)
                    || link == "network"
                    || link == "ethernet"
                {
                    return true;
                }
            }
            "WeakWifi" if super::is_wifi_link(&link) || link == "network" => return true,
            "LteOutage" if super::is_cellular_link(&link) => return true,
            "SatelliteOutage" if super::is_satellite_link(&link) => return true,
            _ => {}
        }
    }
    false
}

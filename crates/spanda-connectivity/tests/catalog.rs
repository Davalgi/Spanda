//! Connectivity catalog tests.

use spanda_connectivity::{
    connectivity_key_to_profile_tokens, connectivity_link_to_transport, is_wifi_link,
    ConnectivityTransport,
};

#[test]
fn wifi_maps_to_mqtt_transport() {
    // Description:
    //     Wifi maps to mqtt transport.
    //
    // Inputs:
    //     None.
    //
    // Outputs:
    //     None.
    //
    // Example:

    //     let result = spanda_connectivity::catalog::wifi_maps_to_mqtt_transport();

    assert_eq!(
        connectivity_link_to_transport("wifi"),
        ConnectivityTransport::Mqtt
    );
    assert!(is_wifi_link("WiFi6"));
}

#[test]
fn gps_key_maps_to_profile_tokens() {
    // Description:
    //     Gps key maps to profile tokens.
    //
    // Inputs:
    //     None.
    //
    // Outputs:
    //     None.
    //
    // Example:

    //     let result = spanda_connectivity::catalog::gps_key_maps_to_profile_tokens();

    assert_eq!(connectivity_key_to_profile_tokens("gps"), vec!["GPS"]);
}

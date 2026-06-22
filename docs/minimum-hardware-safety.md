# Minimum Hardware Safety

Missions and `requires_capability` blocks define minimum hardware requirements. The safety verifier rejects deployment when requirements are not met.

## Mission requirements

```spanda
mission Patrol {
    requires capabilities [
        gps_navigation,
        obstacle_avoidance,
        emergency_stop,
        telemetry_streaming
    ];
}
```

## Explicit capability requirements

```spanda
requires_capability obstacle_avoidance {
    any_of sensors [Lidar, DepthCamera, Radar];
    actuator DifferentialDrive;
    safety emergency_stop;
}

requires_capability remote_control {
    connectivity any_of [WiFi, LTE, FiveG, Bluetooth];
    security signed_commands;
}
```

## CLI

```bash
spanda verify rover.sd --minimum-capabilities
spanda safety check rover.sd --capabilities
spanda trace capabilities rover.sd
```

## Error example

```
ERROR: Mission requires obstacle_avoidance.
Missing minimum hardware:
  - sensor: Lidar OR DepthCamera OR Radar
Suggested fixes:
  - Add Lidar sensor to hardware profile
  - Install spanda-nav package
```

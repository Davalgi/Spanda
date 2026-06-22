# Robot Capabilities

Robots expose high-level capabilities derived from hardware, packages, providers, and safety rules.

## Declared capabilities

```spanda
robot Rover {
    uses hardware RoverV1;
    exposes capabilities [
        autonomous_navigation,
        gps_navigation,
        obstacle_avoidance,
        emergency_stop,
        telemetry_streaming
    ];
}
```

## Inferred capabilities

The verifier infers capabilities from component combinations:

| Components | Inferred capability |
|------------|---------------------|
| GPS + DifferentialDrive | `gps_navigation` |
| Lidar/Camera + nav package | `obstacle_avoidance` |
| MQTT + WiFi/LTE | `telemetry_streaming` |
| Kill switch + drive actuator | `emergency_stop` |

## CLI

```bash
spanda robot capabilities rover.sd
spanda robot capabilities rover.sd --json
spanda safety check rover.sd --capabilities
```

See [Minimum Hardware Safety](./minimum-hardware-safety.md) for mission requirement checks.

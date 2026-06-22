# Hardware Capabilities

Hardware profiles can expose explicit capabilities beyond type lists.

## Example

```spanda
hardware RoverV1 {
    sensor gps: GPS {
        capabilities [read_location, read_altitude, read_heading];
        accuracy: "2 m";
    }

    actuator wheels: DifferentialDrive {
        capabilities [move_forward, rotate, stop, emergency_stop];
        max_speed: 1.5 m/s;
    }
}
```

## CLI

```bash
spanda hardware capabilities rover.sd
spanda verify rover.sd --capabilities
spanda verify rover.sd --capabilities-json
```

## Verification

- Hardware exists but does not expose required capability → error
- Capability parameter below minimum → warning/error
- Declared but unused capability → optional warning

See [Robot Capabilities](./robot-capabilities.md) for high-level capability exposure.

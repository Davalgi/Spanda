# Hardware Traceability

Maps declared hardware dependencies to source usage, packages, providers, capabilities, and safety rules.

## Example

```spanda
hardware RoverV1 {
    sensors: [GPS, Camera, Lidar];
    actuators: [DifferentialDrive];
    connectivity: [WiFi, LTE];
}

robot Rover {
    sensor gps: GPS;
    actuator wheels: DifferentialDrive;
    uses hardware RoverV1;
}
```

## CLI

```bash
spanda trace hardware rover.sd
spanda verify rover.sd --traceability
```

## Validation

| Condition | Severity |
|-----------|----------|
| Declared but unused hardware | Warning |
| Used but undeclared hardware | Error |
| Missing provider | Error |
| Actuator without safety gate | Warning |
| Network without connectivity requirement | Warning |

See [Capability Traceability](./capability-traceability.md).

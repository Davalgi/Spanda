# Search and Rescue Mesh

End-to-end scenario: discover heterogeneous robots, find thermal camera capability, route through trusted relay nodes, survive partition.

```bash
CONFIG=crates/spanda-config/tests/fixtures/warehouse/spanda.toml

spanda mesh discover --config "$CONFIG"
spanda mesh find --capability thermal_camera --config "$CONFIG"
spanda mesh find --capability relay_node --config "$CONFIG"
spanda mesh health --config "$CONFIG"
spanda mesh topology --config "$CONFIG"
```

Combine with Control Center **Mesh** tab when running `spanda control-center`.

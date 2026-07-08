# Capability Discovery

Find entities advertising a specific capability through the mesh.

```bash
CONFIG=crates/spanda-config/tests/fixtures/warehouse/spanda.toml

spanda mesh capabilities --config "$CONFIG"
spanda mesh find --capability thermal_camera --config "$CONFIG" --json
```

See [docs/mesh-capability-routing.md](../../../docs/mesh-capability-routing.md).

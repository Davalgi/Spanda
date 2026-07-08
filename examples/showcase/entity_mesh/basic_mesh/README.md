# Basic Entity Mesh

Discover entities from the warehouse demo config and inspect topology.

```bash
CONFIG=crates/spanda-config/tests/fixtures/warehouse/spanda.toml

spanda mesh discover --config "$CONFIG"
spanda mesh list --config "$CONFIG"
spanda mesh topology --config "$CONFIG"
spanda mesh health --config "$CONFIG"
```

See [docs/entity-mesh.md](../../../docs/entity-mesh.md).

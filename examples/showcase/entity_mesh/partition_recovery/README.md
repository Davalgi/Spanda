# Partition Recovery

Simulate a partition and inspect merge report.

```bash
CONFIG=crates/spanda-config/tests/fixtures/warehouse/spanda.toml

spanda mesh simulate-partition <entity-id> --config "$CONFIG" --json
spanda mesh merge-report --config "$CONFIG"
```

See [docs/mesh-partition-handling.md](../../../docs/mesh-partition-handling.md).

# Leader Election

Mesh coordinator election (communication role only) appears in health output.

```bash
CONFIG=crates/spanda-config/tests/fixtures/warehouse/spanda.toml

spanda mesh health --config "$CONFIG" --json
```

See [docs/mesh-leader-election.md](../../../docs/mesh-leader-election.md).

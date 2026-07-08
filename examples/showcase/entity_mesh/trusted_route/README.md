# Trusted Route

Compute a trust-weighted route between two entities.

```bash
CONFIG=crates/spanda-config/tests/fixtures/warehouse/spanda.toml

spanda mesh route <source-entity> <target-entity> --config "$CONFIG"
```

Safety-critical routes reject untrusted relays. See [docs/mesh-security.md](../../../docs/mesh-security.md).

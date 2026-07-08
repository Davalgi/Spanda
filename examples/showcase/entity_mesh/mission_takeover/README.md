# Mission Takeover via Mesh

Mission delegation locates capable trusted replacements; **takeover still flows through Recovery Orchestrator**.

```bash
CONFIG=crates/spanda-config/tests/fixtures/warehouse/spanda.toml

spanda mesh list --config "$CONFIG"
spanda recovery plan --config "$CONFIG"   # takeover authority remains here
```

See [docs/entity-mesh.md](../../../docs/entity-mesh.md) compatibility rules.

# Autonomous Entity Mesh — field pilot (30-day)

Operational pilot to validate **Autonomous Entity Mesh** before **Experimental → Stable**
promotion. Separate from the enterprise-ops soak clock — Entity Mesh has its own start file and
weekly smoke checklist.

**Promotion checklist:** [entity-mesh-stable-promotion.md](./entity-mesh-stable-promotion.md)

---

## Start the pilot clock (one-time)

```bash
chmod +x scripts/entity_mesh_field_soak_init.sh
./scripts/entity_mesh_field_soak_init.sh
```

Writes `.spanda/entity-mesh-field-soak-start.txt` (UTC `YYYY-MM-DD`). The file is gitignored;
record the start date in fleet CMDB or a pilot tracking issue.

| Variable | Default |
|----------|---------|
| `SPANDA_ENTITY_MESH_FIELD_SOAK_START_FILE` | `.spanda/entity-mesh-field-soak-start.txt` |
| `SPANDA_FIELD_SOAK_MIN_DAYS` | `30` |

---

## Weekly pilot smoke

Run at least once per week during the soak window:

```bash
./scripts/entity_mesh_smoke.sh
```

With Control Center and gRPC (matches CI Integration):

```bash
spanda control-center serve --config crates/spanda-config/tests/fixtures/warehouse/spanda.toml &
CC_PID=$!
sleep 2
./scripts/entity_mesh_smoke.sh --grpc-bind 127.0.0.1:50051
kill "$CC_PID" 2>/dev/null || true
```

Manual CLI checks on the warehouse fixture:

```bash
export CONFIG=crates/spanda-config/tests/fixtures/warehouse/spanda.toml
spanda mesh discover --config "$CONFIG"
spanda mesh health --config "$CONFIG"
spanda mesh topology --config "$CONFIG"
spanda mesh graph --config "$CONFIG"
```

Control Center: open the **Entity Mesh** tab — verify coordinator, Discover/Refresh, and topology
graph.

---

## Pilot scope

| In scope | Out of scope (future) |
|----------|------------------------|
| Discovery, topology, health, partitions | Live MQTT/DDS/ROS2 mesh transport wiring |
| Trust-aware routing on warehouse entities | Multi-site production mesh relay |
| REST + SDK + gRPC smoke parity | Python gRPC via `spanda-sdk[grpc]` **0.5.9+** |
| Partition simulate + merge report | CC graph time-travel / replay overlays |

---

## Exit criteria (Stable promotion)

1. **30 days** elapsed on the Entity Mesh soak file.
2. `./scripts/entity_mesh_stable_promotion_gate.sh` passes (soak + audit prep + crate tests +
   smoke).
3. External security audit sign-off on `.spanda/security-audit-prep.json`.
4. Update `docs/feature-status.md` — Autonomous Entity Mesh row → **Stable**.

CI Nightly runs implementation checks only (`SPANDA_ENTITY_MESH_SKIP_SOAK=1
SPANDA_ENTITY_MESH_SKIP_AUDIT=1`).

---

## Related

- [entity-mesh.md](./entity-mesh.md) · [examples/showcase/entity_mesh/](../examples/showcase/entity_mesh/)
- [field-soak-gate.md](./field-soak-gate.md) · [organizational-gates.md](./organizational-gates.md)

# Fleet distributed field validation

Multi-process fleet orchestration with live HTTP agents and mesh coordinator.

## Workflow

```bash
# Start agents + mesh (see scripts/fleet_field_validation.sh)
spanda fleet agent start --robot ScoutA --bind 127.0.0.1:19701
spanda fleet agent start --robot ScoutB --bind 127.0.0.1:19702
spanda fleet agent register ScoutA http://127.0.0.1:19701
spanda fleet agent register ScoutB http://127.0.0.1:19702
spanda fleet mesh start --bind 127.0.0.1:19703

# Orchestrate peer missions locally, via HTTP relay, and via mesh
spanda fleet orchestrate examples/robotics/fleet_peer_missions.sd
spanda fleet orchestrate examples/robotics/fleet_peer_missions.sd --remote
spanda fleet orchestrate examples/robotics/fleet_peer_missions.sd --mesh-url http://127.0.0.1:19703

# One-command smoke (agents + mesh + tests)
./scripts/fleet_field_validation.sh
```

## Related examples

| Path | Role |
|------|------|
| `examples/robotics/fleet_peer_missions.sd` | Peer mesh mission handoffs |
| `examples/robotics/golden_path_deploy.sh` | Full robotics golden path (certify, deploy, fleet, swarm) |
| `examples/showcase/fleet_recovery/` | Recovery CLI showcase |
| `examples/showcase/continuity/` | Continuity CLI showcase |

See [fleet-distributed.md](../../docs/fleet-distributed.md) · [mission-continuity.md](../../docs/mission-continuity.md) · [self-healing.md](../../docs/self-healing.md).

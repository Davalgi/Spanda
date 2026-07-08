# Mesh Leader / Coordinator Election

Coordinator election selects a **communication/coordinator role only**. It does **not** grant safety, mission, or actuator authority.

## Methods

- Configured coordinator
- Backup coordinator
- Trust-weighted election
- Readiness-weighted election
- Capability-based election
- Quorum-based election

Untrusted entities **cannot** become coordinator (`min_trust` gate).

## Types

- `MeshLeader` — elected leader evidence
- `MeshCoordinator` — active coordinator state

## Use cases

- Fleet coordinator failure
- Swarm coordinator failure
- Site coordinator failure
- Partitioned group operation

## CLI / API

Coordinator status appears in `spanda mesh health` and `GET /v1/mesh/health`.

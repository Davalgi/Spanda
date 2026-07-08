# Mesh Sync

Mesh sync propagates operational state across partitions on reconnection. **Secrets are never synced.**

## Synchronized fields

- Entity state (health, readiness, trust)
- Mission progress summary
- Decision trace counts
- Recovery event counts
- Audit event counts
- Telemetry summary
- Configuration version

## Types

- `MeshSyncState` — per-entity sync snapshot
- `MeshMergePlan` — planned sync and graph update actions

## Conflict resolution

Conflicts (mission state, duplicate leader, diverged recovery, capability ads) use existing **decision conflict precedence** via `MeshMergePolicy`.

## API

Merge results: `GET /v1/mesh/merge-report`

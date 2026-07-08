# Mesh Partition Handling

Partitions are detected when reachable node clusters become disconnected.

## Behavior

When a partition occurs:

1. Identify affected entities and clusters
2. Elect a **local coordinator** (communication role)
3. Apply `MeshPartitionPolicy` (pause unsafe missions, allow safe missions if configured)
4. Record `MeshPartitionReport` evidence
5. Sync when reconnected via `MeshMergePlan` / `MeshMergeReport`

## Types

- `MeshPartition`, `MeshCluster`
- `MeshPartitionPolicy`, `MeshPartitionReport`
- `MeshMergePolicy`, `MeshMergePlan`, `MeshMergeReport`
- `MeshConflict` — resolved using decision conflict precedence

## CLI

```bash
spanda mesh simulate-partition robot-a robot-b
spanda mesh merge-report
```

## API

`GET /v1/mesh/partitions` · `POST /v1/mesh/simulate-partition` · `GET /v1/mesh/merge-report`

Partition mode **blocks high-risk mission starts** (more restrictive than normal mode).

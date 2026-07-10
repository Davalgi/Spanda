# spanda-telemetry(1)

## NAME

telemetry — Query the persistent telemetry store written by `--persist-telemetry` or `SPANDA_TELEMETRY_STORE=1`.

## SYNOPSIS

```
spanda telemetry list|latest|heartbeats|devices|stats|export|prometheus|otlp|push|serve|sessions|replay|info [flags]
```

## DESCRIPTION

Query the persistent telemetry store written by `--persist-telemetry` or `SPANDA_TELEMETRY_STORE=1`.

## OPTIONS

`list` — filter by device, sensor, task, session, kind, since, limit
`latest` — most recent device metric, sensor read, task heartbeat, or device liveness
`heartbeats` / `devices` — index sidecar for tasks and devices
`stats` — event counts (includes session and runtime_metrics)
`info` — backend, paths, retention, migration backup
`sessions` — list persisted run sessions with linked mission traces
`replay` — replay the mission trace linked to a session (`--record` runs)
`export` — copy event log (JSONL from SQLite when needed)
`prometheus` — Prometheus text exposition
`otlp` — OTLP/JSON metrics export
`push` — POST OTLP/JSON to a remote collector (`--endpoint` or `SPANDA_OTLP_ENDPOINT`)
`serve` — HTTP server (`/metrics`, `/otlp/v1/metrics`, `/healthz`)

## EXAMPLES

```bash
spanda telemetry stats
spanda telemetry info
spanda telemetry push --endpoint http://localhost:4318/v1/metrics
spanda telemetry sessions --json
spanda telemetry replay --session rover-123 --deterministic
```

## EXIT STATUS

0 on success; 1 when the store cannot be read.

## FILES

`.spanda/telemetry-store.jsonl` or `.spanda/telemetry-store.db` when `SPANDA_TELEMETRY_BACKEND=sqlite` (override with `SPANDA_TELEMETRY_STORE_PATH`).

## SEE ALSO

spanda-run(1), spanda-sim(1), [spanda(1)](./spanda.md), [spanda-reference.md](../spanda-reference.md)

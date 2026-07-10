# spanda-run(1)

## NAME

run — Execute a Spanda program on the interpreter backend.

## SYNOPSIS

```
spanda run [--json] [--verbose] [--trace-*] [--record] [--persist-telemetry] <file.sd>
```

## DESCRIPTION

Execute a Spanda program on the interpreter backend.

## OPTIONS

`--trace-scheduler`, `--trace-tasks`, `--trace-triggers`, `--trace-events` — scheduler telemetry
`--trace-realtime`, `--metrics-json` — realtime metrics
`--record` — write mission trace
`--persist-telemetry` — append device/sensor/heartbeat events to `.spanda/telemetry-store.jsonl`

## EXAMPLES

```bash
spanda run examples/rover.sd
spanda run robot.sd --trace-realtime --metrics-json
spanda run rover.sd --persist-telemetry
```

## EXIT STATUS

0 on successful execution; 1 on runtime or compile errors.

## FILES

Mission traces when using `--record` (default: `mission.trace`). Persistent telemetry when using `--persist-telemetry` (`.spanda/telemetry-store.jsonl`).

## SEE ALSO

spanda-sim(1), spanda-replay(1), [spanda(1)](./spanda.md), [spanda-reference.md](../spanda-reference.md)

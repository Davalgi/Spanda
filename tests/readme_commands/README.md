# README command smoke & golden-output tests

Release-hardening harness for every `spanda` invocation shown in the root
[`README.md`](../../README.md) and for flagship golden-output commands.

## Layout

| Path | Role |
|------|------|
| [`commands.toml`](./commands.toml) | Command manifest (smoke + golden) |
| [`run.sh`](./run.sh) | Runner — smoke exit codes and golden snapshots |
| [`golden/`](./golden/) | Expected stdout snapshots (normalized) |

## Run

```bash
# Smoke only (exit code + required markers)
./tests/readme_commands/run.sh

# Compare golden snapshots
./tests/readme_commands/run.sh --golden

# Regenerate goldens after intentional output changes
SPANDA_UPDATE_GOLDENS=1 ./tests/readme_commands/run.sh --golden
```

`SPANDA_BIN` may point at a prebuilt `spanda` binary (CI sets this).

## Normalization

Snapshots replace absolute repository paths with `<ROOT>` and collapse runs of
blank lines so machine-local paths do not cause false failures.

## Policy

- Do **not** invent README commands — only entries in `commands.toml`.
- Update goldens only when output changes are intentional.
- Prefer fixing the CLI over weakening assertions.

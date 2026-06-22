# Deterministic Replay

Record and replay simulation traces for regression and incident analysis.

## Recording

```bash
spanda sim rover.sd --record
```

Produces a JSON mission trace (`.trace`, version 2 when state snapshots are present) with scheduler events, **provider dispatch events** (`provider_call` with module, function, and provider key), and embedded robot state (pose, velocity, e-stop, active mode) on each recorded frame.

## Replay modes

**Inspect frames** (default):

```bash
spanda replay mission.trace
spanda replay mission.trace --from T+00:30
```

**Deterministic verification** — re-run the source program and compare event sequences:

```bash
spanda replay mission.trace --deterministic
```

**Frame-by-frame playback** — apply recorded state snapshots without re-executing program logic:

```bash
spanda replay mission.trace --playback
```

Playback uses wall-clock pacing between frames by default. Offsets accept milliseconds or `T+mm:ss` / `T+hh:mm:ss` forms.

Twin replay integrates with existing `twin { replay true; }` blocks.

## Incident workflow (Phase 24)

Combine mission trace recording, twin export, and optional cloud upload for post-incident review:

```bash
# 1. Record a mission trace during simulation
spanda sim examples/showcase/autonomous_rover/src/rover.sd --record

# 2. Export twin replay JSON
spanda twin export examples/communication/twin_replay_golden.sd --out incident-replay.json

# 3. Optional cloud upload (mock HTTP server in CI)
export SPANDA_CLOUD_UPLOAD_URL=http://127.0.0.1:18770/upload
./scripts/twin_cloud_golden_path.sh
```

Golden path index: [tier-3-golden-paths.md](tier-3-golden-paths.md)

## Digital twin replay export (JSON)

Export the twin replay buffer after simulation for post-incident review:

```bash
spanda twin export examples/communication/twin_replay_golden.sd --out twin-replay.json
spanda sim examples/showcase/digital_twin_demo.sd --twin-export twin-replay.json
```

JSON shape:

```json
{
  "twin": "RobotTwin",
  "mirrors": ["pose", "velocity"],
  "frame_count": 12,
  "frames": [
    { "frame": 0, "fields": { "pose": { "...": "..." } } }
  ]
}
```

## Golden traces in git

Runtime `--record` output is ignored by default (`*.trace` in `.gitignore`). Committed reference traces are allowed under:

- `examples/**/*.trace` — demo or walkthrough replays
- `tests/golden/**/*.trace` — CI golden fixtures paired with `tests/golden/manifest.json`

See [realtime](realtime.md).

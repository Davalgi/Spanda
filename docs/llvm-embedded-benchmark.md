# LLVM embedded benchmark slice (Jetson / Pi)

Cross-compile validation for **aarch64-linux-gnu** — the primary triple for NVIDIA Jetson Orin/Nano and Raspberry Pi 5 class boards. This is a **CI-friendly smoke slice**, not a full hardware benchmark suite.

**Related:** [compiler-backend-roadmap.md](./compiler-backend-roadmap.md) · [tier-3-golden-paths.md](./tier-3-golden-paths.md) · [hardware-compatibility.md](./hardware-compatibility.md)

---

## What it validates

| Step | Command | Pass criteria |
|------|---------|---------------|
| Type-check | `spanda check hello_world.sd` | No diagnostics |
| LLVM IR | `spanda llvm-ir --target-triple aarch64-unknown-linux-gnu` | `.ll` file emitted |
| Cross link (Linux CI only) | `spanda compile-native --target-triple aarch64-unknown-linux-gnu` | Binary when `aarch64-linux-gnu-gcc` is installed |

The script **always validates IR** for the aarch64 triple. Native linking runs only on Linux when `aarch64-linux-gnu-gcc` is present; macOS dev machines skip the link step after IR succeeds.

---

## Run locally

```bash
cargo build -p spanda-cli --release --features llvm
./scripts/llvm_embedded_golden_path.sh
./scripts/llvm_embedded_golden_path.sh examples/showcase/world_model_patrol.sd
```

Optional override:

```bash
SPANDA_BIN=target/release/spanda ./scripts/llvm_embedded_golden_path.sh
```

---

## On-device benchmark (manual)

After copying a natively built binary to hardware (or building on-device with the same triple):

```bash
# On Jetson / Pi (Linux aarch64)
time ./hello_world_native
spanda run rover.sd --metrics-json   # interpreter baseline for comparison
```

Record wall time and RSS for interpreter vs native when promoting LLVM from experimental to beta ([compiler-backend-roadmap.md](./compiler-backend-roadmap.md)).

---

## CI

| Job | Script | Notes |
|-----|--------|-------|
| `llvm-golden-path` | [llvm_golden_path.sh](../scripts/llvm_golden_path.sh) | Host-native codegen |
| `llvm-embedded-golden-path` | [llvm_embedded_golden_path.sh](../scripts/llvm_embedded_golden_path.sh) | aarch64 triple smoke |

---

## HAL profiles (planned)

`--hal-profile jetson-orin` and `--hal-profile rpi5` will select conditional compilation flags once HAL-backed codegen lands. Until then, use `--target-triple` only.

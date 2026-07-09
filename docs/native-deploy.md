# Native deploy (LLVM)

Spanda uses **LLVM native codegen as the primary runtime** for `spanda run` and `spanda sim`
when programs lower to eligible SIR and clang is available. The tree-walking interpreter
remains the **long-term support (LTS)** path and is used automatically when native execution
is unavailable or the program contains statements unsupported by native codegen.

## Runtime selection

| Mode | Flag / env | Behavior |
|------|------------|----------|
| **Auto** (default) | `--runtime auto` or `SPANDA_RUNTIME=auto` | Try native first; fall back to interpreter LTS with a warning |
| **Native** | `--runtime native` or `SPANDA_RUNTIME=native` | Require native; error if unavailable |
| **Interpreter LTS** | `--runtime interpreter` or `SPANDA_RUNTIME=interpreter` | Force interpreter only |

```bash
spanda run --runtime auto examples/showcase/killer_demo.sd
SPANDA_RUNTIME=interpreter spanda sim rover.sd
```

Build the CLI with the `llvm` feature (default) for native-primary dispatch.

## Quick start

```bash
cargo build -p spanda --release --features llvm
spanda compile-native examples/showcase/killer_demo.sd
# → target/spanda-native/spanda-program
```

Or via deploy:

```bash
spanda deploy --target native examples/showcase/killer_demo.sd
```

## Commands

| Command | Output |
|---------|--------|
| `spanda llvm-ir <file.sd>` | LLVM IR (`.ll`) for inspection |
| `spanda compile-native <file.sd>` | LLVM IR + linked binary under `target/spanda-native/` |
| `spanda deploy --target native <file.sd>` | Same as `compile-native` with deploy-oriented defaults |

### Flags

| Flag | Purpose |
|------|---------|
| `--out <path>` | Output binary path |
| `--target-triple <triple>` | Cross-compile triple (e.g. `aarch64-unknown-linux-gnu`) |
| `--hal-profile <name>` | HAL profile baked into codegen metadata |

## Requirements

- **clang** on `PATH` (links `libspanda_rt`)
- Programs must lower to SIR successfully (`spanda check` first)

Embedded cross-build example (CI: `llvm-embedded-golden-path`):

```bash
spanda compile-native --target-triple aarch64-unknown-linux-gnu \
  --hal-profile jetson examples/showcase/killer_demo.sd
```

## When to use native vs interpreter

| Runtime | Best for |
|---------|----------|
| **Auto / native** (`spanda run`, `spanda sim`) | Production deploys, edge nodes with clang, eligible programs |
| **Interpreter LTS** (`--runtime interpreter`) | Full language surface, triggers, agents, development |
| **Prebuilt binary** (`compile-native`, `deploy --target native`) | Fixed behaviors shipped as standalone executables |
| WASM (`deploy --target wasm`) | Browser and lightweight embed targets |

Native codegen covers a **subset** of the language today. Use `spanda check` and
[known-limitations.md](./known-limitations.md) before requiring native-only execution.

## CI

| Job | Script |
|-----|--------|
| `llvm-golden-path` | `scripts/llvm_golden_path.sh` |
| `llvm-embedded-golden-path` | `scripts/llvm_embedded_golden_path.sh` |
| `sensor-pipeline-golden-path` | `scripts/sensor_pipeline_golden_path.sh` |

## Related

- [compiler-backend-roadmap.md](./compiler-backend-roadmap.md)
- [llvm-embedded-benchmark.md](./llvm-embedded-benchmark.md)
- [hardware-compatibility.md](./hardware-compatibility.md)

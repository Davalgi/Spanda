# Chapter 5 — The ten commands you'll actually use

Back to [index](./README.md)

Stick this on a sticky note. Full CLI reference: [spanda-reference.md](../spanda-reference.md) and
[man/](../man/).

---

## The essential ten

| # | Command | When to use it |
|---|---------|----------------|
| 1 | `spanda check file.sd` | Before every commit — does it compile? |
| 2 | `spanda run file.sd` | Run in simulation |
| 3 | `spanda sim file.sd` | Run with verbose sim output |
| 4 | `spanda verify file.sd` | Does it fit the hardware profile? |
| 5 | `spanda test` | Run `test "..."` blocks in your project |
| 6 | `spanda init name` | Create a new project folder |
| 7 | `spanda fmt file.sd` | Auto-format source |
| 8 | `spanda lint file.sd` | Extra style/safety lint |
| 9 | `spanda replay mission.trace` | Inspect a recorded mission |
| 10 | `spanda fleet run file.sd` | Multi-robot in-process sim |

---

## Copy-paste recipes

**New project**

```bash
spanda init my_bot && cd my_bot
spanda check src/main.sd
spanda run src/main.sd
```

**Check before push**

```bash
spanda check src/main.sd
spanda test
spanda verify src/main.sd --target RoverV1
```

**Debug “why did it stop?”**

```bash
spanda run robot.sd --trace-scheduler --trace-tasks --trace-triggers
```

**Record and replay**

```bash
spanda sim robot.sd --record
spanda replay robot.trace --deterministic
```

---

## Flags worth knowing

| Flag | Command | What it does |
|------|---------|--------------|
| `--json` | `check`, `verify`, `run` | Machine-readable output for CI |
| `--target RoverV1` | `verify` | Check one hardware profile |
| `--all-targets` | `verify` | Check every `deploy` line |
| `--record` | `sim` | Write a `.trace` mission file |
| `--deterministic` | `replay` | Re-run source and compare frames |
| `--trace-triggers` | `run` | Log trigger firings |

---

## Commands you can ignore until later

| Command | When you'll care |
|---------|------------------|
| `spanda codegen` | Native/WASM emit (experimental) |
| `spanda llvm-ir` | Compiler backend hacking |
| `spanda debug` | Step-through debugging (DAP) |
| `spanda publish` | Package registry |
| `spanda reference` | Regenerate API docs |

---

## npm shortcuts (from repo root)

```bash
npm run build:rust          # build spanda CLI
npm run spanda:native -- check examples/hello_world.sd
npm test                    # TypeScript + golden tests
```

---

**Next:** [Oops — common mistakes](./06-common-mistakes.md)

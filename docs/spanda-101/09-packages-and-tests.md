# Lesson 9 — Packages and tests

**Goal:** Structure a Spanda project with manifests, dependencies, and in-language tests.

**Examples:**

- [`examples/basics/07_in_language_tests.sd`](../../examples/basics/07_in_language_tests.sd)
- [`examples/packages/basic_project/`](../../examples/packages/basic_project/)
- [`examples/packages/local_dependency/`](../../examples/packages/local_dependency/)

References: [packages.md](../packages.md), [spanda-toml.md](../spanda-toml.md)

---

## Project layout

```bash
spanda init my_rover
```

Creates:

```
my_rover/
  spanda.toml       # manifest
  src/main.sd       # entry point
```

### Manifest basics

```toml
[package]
name = "my_rover"
version = "0.1.0"

[hardware]
targets = ["RoverV1"]

[safety]
level = "experimental"
```

---

## In-language tests

Top-level test blocks run with `spanda test`:

```spanda
module basics.math;

export fn clamp_speed(speed: Float) -> Float {
  return speed;
}

test "clamp accepts in-range speed" {
  assert(true);
}
```

```bash
spanda test examples/basics/07_in_language_tests.sd
spanda test --project    # all tests in current package
```

Failed `assert(...)` calls fail the test run — wire these into CI alongside `spanda check`.

---

## Dependencies

Registry dependency (when registry is configured):

```toml
[dependencies]
spanda-navigation = "0.1.0"
```

Local path dependency:

```toml
[dependencies]
shared_utils = { path = "shared_utils" }
```

See `examples/packages/local_dependency/` for a working layout.

---

## Build and install

```bash
spanda build --project .
spanda install
```

`build` resolves dependencies and prepares the project; exact artifacts depend on your targets (interpreter today, native codegen experimental).

---

## Try it

```bash
spanda check examples/basics/07_in_language_tests.sd
spanda test examples/basics/07_in_language_tests.sd

spanda check examples/packages/basic_project/src/main.sd
```

---

## Exercise

1. Run `spanda init lesson9_bot`
2. Add a `test "robot has lidar"` block with `assert(true)` (expand later with real checks)
3. Run `spanda test` from the project directory

---

**Next:** [Lesson 10 — End-to-end patrol](./10-end-to-end-patrol.md)

# Verification vocabulary

Spanda uses **verify / verification / certify** for three different, lighter mechanisms.
None of them is **formal verification** (model checking, theorem proving, or IEC toolchains).

| Mechanism | What it is | What it is not |
|-----------|------------|----------------|
| **`spanda verify`** (alias: **`spanda compatibility`**) | Hardware / deploy **fit checking** — sensors, actuators, memory, timing, optional policy/certify metadata gates | Formal proof of safety or standards compliance |
| **`verify { }` / `assert { }`** | **Runtime assertions** evaluated after behavior/task execution (and at end of run) | Compile-time proof; not the same as `spanda verify` |
| **`certify ISO13849 { … }`** (and IEC/ISO variants) | **Declared metadata** recorded at verify/CI time for intent and packaging | A certification body result or runtime safety proof |
| **`requires` / `ensures` / `invariant`** | **Runtime contracts** on behaviors and tasks (`ensures` is checked after the body) | Hoare-logic proofs or static postcondition discharge |

## Preferred wording

- Prefer **hardware compatibility check** when talking about `spanda verify`.
- Prefer **`assert { }`** for new runtime assertion blocks; `verify { }` remains supported and
  emits a lint warning pointing at this vocabulary split.
- Always describe **`certify`** as **declared metadata**, never as “certified” or “certified safe.”
- Describe **`ensures`** as a **runtime postcondition**, not a verified proof.

## CLI aliases

```bash
spanda verify robot.sd --target RoverV1
spanda compatibility robot.sd --target RoverV1   # clearer alias; same implementation
```

## Related docs

- [hardware-compatibility.md](./hardware-compatibility.md) — `spanda verify`
- [spanda-language.md](./spanda-language.md) — `assert` / `verify` blocks and contracts
- [known-limitations.md](./known-limitations.md) — honesty bounds
- [man/spanda-verify.md](./man/spanda-verify.md)

# spanda-verify(1)

## NAME

verify — Check hardware compatibility for a deploy target (not formal verification). Alias: spanda compatibility.

## SYNOPSIS

```
spanda verify [--json] [--target <profile>] [--all-targets] [--simulate] <file.sd>
```

## DESCRIPTION

Check hardware compatibility for a deploy target (not formal verification). Alias: spanda compatibility.

## OPTIONS

`--target` — hardware profile name
`--all-targets` — compatibility matrix
`--simulate` — include simulator checks
`--json` — JSON report
`--strict-certify` — fail when certify metadata is missing/incomplete (metadata only)

## EXAMPLES

```bash
spanda verify robot.sd --target RoverV1
spanda compatibility robot.sd --all-targets --simulate
```

## EXIT STATUS

0 when compatible; 1 on compatibility failures or errors.

## FILES

Hardware profile definitions in the program or `hardware/` package paths.

## SEE ALSO

spanda-check(1), spanda-run(1), [verification-vocabulary.md](../verification-vocabulary.md), [spanda(1)](./spanda.md), [spanda-reference.md](../spanda-reference.md)

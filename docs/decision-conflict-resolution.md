# Decision Conflict Resolution

When multiple layers produce competing decisions, Spanda applies a **fixed precedence order**.

## Precedence (highest first)

1. **Safety / kill switch** — always wins
2. **Local immediate safety** — reflex layer reactions
3. **Trust / security block** — compromised or untrusted sources blocked
4. **Human emergency override** — operator e-stop and manual override
5. **Control center policy** — strategic governance
6. **Fleet coordination** — group-level reassignment and delegation
7. **Local optimization** — entity-level efficiency decisions

## Example conflicts

| Local robot | Fleet coordinator | Control center | Winner |
|-------------|-------------------|----------------|--------|
| Continue | Reassign | — | Fleet (unless safety/trust blocks) |
| Continue | — | Abort | Control center |
| E-stop | Continue | Continue | Safety reflex |
| Continue | Continue | — | Local (lowest precedence when no higher conflict) |

## API and engine

The `resolve_conflict()` function in `spanda-decision` applies precedence automatically. Rejected alternatives are recorded in `DistributedDecisionRecord.rejected_alternatives`.

## Consensus vs conflict

**Consensus** (quorum, majority, trust-weighted voting) applies within Layer 2 for fleet/swarm agreement.

**Conflict resolution** applies **across layers** when decisions disagree.

## Simulation

```bash
spanda decision simulate mission.sd --network-partition
spanda decision simulate mission.sd --fleet-coordinator-failure
```

Attack scenarios (`spanda decision simulate-attack split_brain_coordinator`) test split-brain handling via quorum and backup leader promotion.

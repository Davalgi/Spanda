# Agentic Programming

Spanda supports safety-gated agentic programming with tool permissions, memory scopes, and approval gates.

## Example

```spanda
agent Planner {
    goal "Navigate safely";
    tools [camera, lidar, map];
    memory short_term;
    policy safe_only;

    can [ read(lidar), propose_motion ];

    plan {
        let proposal = planner.reason(goal);
        let action = safety.validate(proposal);
        return action;
    }
}
```

## Rules

- Agents cannot directly execute actuators unless permitted and safety-gated
- High-risk actions require approval or `SafeAction`
- Reasoning traces captured for audit when `audit` is configured
- `ActionProposal` must pass through `safety.validate` before actuator execution

## Runtime

`spanda-ai` provides `AgentRuntime` with mock and live AI paths. Capability enforcement runs in the interpreter.

See [Architecture — AI Safety](./architecture.md) and [Feature Status](./feature-status.md).

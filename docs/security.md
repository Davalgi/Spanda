# Security

Spanda separates **language-level agent capabilities**, **robot-level package permissions**, **trust tiers**, **secrets**, and **secure communication** policies. Audit and blockchain-related libraries integrate through the package capability system and `spanda-security` runtime.

## Architecture

```text
Agent can [...]          → runtime ACL for AI / comm actions
Robot permissions [...]  → package capability grants at runtime
Package [capabilities]   → manifest validation (spanda-package)
Trust level              → device / endpoint policy tier
Secure { ... }           → signed + capability-gated topics/services/actions
Audit integration        → security events recorded via AuditRuntime
```

The `spanda-security` crate (`crates/spanda-security/`) provides:

| Module | Purpose |
|--------|---------|
| `identity` | `RobotIdentity` with trust metadata |
| `secrets` | `SecretStore`, env/literal secret resolution |
| `capability` | `CapabilitySet`, `Permission`, runtime enforcement |
| `signed` | `SignedMessage` envelopes for secure comm |
| `trust` | `TrustLevel` (`untrusted` … `certified`) |
| `permissions` | `PackagePermissions` bridge |
| `secure_comm` | `SecurePolicy` for topics/services/actions |
| `runtime` | `SecurityContext` used by the interpreter |

## Capability layers

### 1. Agent capabilities (`.sd`)

```spanda
agent planner {
  can [ read(lidar), publish(cmd), subscribe(scan), plan ];
  plan { ... }
}
```

Enforced at runtime for sensor reads, AI actions, and **communication** (`publish`, `subscribe`, `call`, `execute`, `discover`).

### 2. Robot permissions (`.sd`)

```spanda
robot Rover {
  permissions [ audit.write, identity.sign, identity.verify, ledger.anchor ];
}
```

Grants package-level capabilities to the running program. When an `audit`, `identity`, or `mock_ledger` block is declared without an explicit `permissions` block, the runtime auto-grants the corresponding caps for backward compatibility.

### 3. Package capabilities (`spanda.toml`)

```toml
[capabilities]
required = [
  "camera.read",
  "audit.write",
  "identity.sign",
  "ledger.anchor",
  "network.outbound"
]
```

Validated by `spanda-package` before install/publish.

### 4. Known capabilities

| Capability | Risk | Description |
|------------|------|-------------|
| `audit.write` | High | Append tamper-evident audit records |
| `audit.read` | Low | Export/read audit logs |
| `identity.sign` | High | Sign telemetry and mission logs |
| `identity.verify` | Low | Verify device signatures |
| `ledger.anchor` | High | Anchor content hashes (async, non-control-path) |
| `network.outbound` | High | Outbound network access |
| `actuator.execute` | High | Direct actuator control |
| `actuator.execute.safe` | Medium | Actuator control via `SafeAction` only |

High-risk capabilities produce **validation warnings** when packages declare them without application approval.

## Identity

```spanda
identity RobotIdentity {
  id: "rover-001";
  public_key: "pub-key-rover-001";
}
```

Device identity is attached to `SecurityContext` and used for signing audit records, secure topics, and provenance.

## Secrets

```spanda
secret api_key from env("API_KEY");
secret dev_key from "literal-dev-key";
```

Secrets resolve at runtime through `SecretStore`. Values are never logged in plaintext; audit entries use redacted labels. Secret names are available as opaque `Secret` bindings in robot scope and can be passed to `sign(data, key)`.

## Trust levels

```spanda
trust trusted;   // untrusted | restricted | trusted | certified
```

Trust tiers gate secure endpoints. A robot at `restricted` cannot publish to a topic with `min_trust = trusted`.

## Secure topics, services, and actions

```spanda
topic cmd: Velocity publish on "/cmd" secure {
  signed = true;
  min_trust = trusted;
  requires = [ identity.verify, identity.sign ];
};

service reset: ResetService secure {
  signed = true;
  requires = [ identity.verify ];
};
```

At runtime:

- **Outbound** (`publish`, `send_goal`, `execute`): checks capabilities, trust, and signs payload when `signed = true`
- **Inbound** (`subscribe`, `call`): verifies trust and signature policy
- **Audit**: security events (`security.publish`, `security.audit.record`, …) append to the audit log when configured

### Crypto (`std.crypto`)

Spanda uses **Ed25519** signatures (via `ed25519-dalek` in Rust, `@noble/ed25519` in TypeScript):

- `sha256(data)` — SHA-256 content hash (hex)
- `sign(data, key_material)` — Ed25519 signature (hex, 128 chars)
- `verify_signature(data, signature, public_key_or_material)` — verify signature

Signing material is hashed to a 32-byte seed. A 64-character hex string is treated as a raw public key for verification.

### Strict permissions mode

When a robot declares `permissions [ ... ]`, **strict mode** is enabled: capability auto-grants from `identity`, `audit`, and `mock_ledger` blocks are disabled. You must explicitly list every capability the program needs.

```spanda
robot R {
  permissions [ audit.write, audit.read ];  // strict — no auto-grant
  audit A { record robot.pose; }
}
```

## Audit integration

Security events flow through `SecurityContext::audit_event()` into `AuditRuntime`:

- Capability denials (when checked before operations)
- Secure publish/subscribe operations
- `audit.record` / `audit.export` / `audit.create_provenance` gated by `audit.write`, `audit.read`, `identity.sign`

See [audit-provenance.md](./audit-provenance.md) for audit block syntax and provenance.

## Safety levels

Package safety levels (`experimental` → `certified`) are validated in `spanda-package` and documented separately. Runtime `TrustLevel` complements package safety for communication policy.

## Rules for audit/blockchain libraries

1. **Never** block robot control on ledger confirmation
2. **Never** send actuation commands through blockchain transports
3. Declare all required capabilities in `spanda.toml` and grant them via robot `permissions`
4. Use `AuditBackend` / `LedgerBackend` traits — do not extend core syntax for chain-specific features
5. Prefer `hardware_safe` or `certified` safety levels for production audit packages

## Related

- [audit-provenance.md](./audit-provenance.md)
- [spanda-toml.md](./spanda-toml.md)
- [packages.md](./packages.md)
- [examples/std/security.sd](../examples/std/security.sd)

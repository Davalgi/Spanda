# Mesh Security

Mesh communication reuses Spanda **secure messaging** — mesh does not bypass signing, encryption
policy, or trust boundaries.

## Message envelope

Every `MeshMessage` includes: `message_id`, `source_entity`, `target_entity`, `target_capability`,
`route`, `priority`, `ttl`, `timestamp`, `nonce`, `signature`, `encryption_required`,
`trust_requirement`, `payload_hash`, and optional `SignedMessage` envelope.

## Validation

The mesh security layer rejects:

- Unsigned high-risk commands
- Untrusted relays for safety-critical routes
- Replayed messages (nonce registry)
- Expired messages (TTL)
- Routes through compromised or low-trust entities

## Trust requirements

`MeshTrustRequirement` controls `minimum_trust_score`, `require_signed`, `require_identity_match`,
and `block_untrusted_relays`.

## Regression tests

See `crates/spanda-entity-mesh/tests/security_regression.rs`.

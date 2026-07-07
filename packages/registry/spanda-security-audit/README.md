# spanda-security-audit

Third-party security audit checklist and prep workflow for Spanda Control Center.

## Checklist

- [ ] API keys stored outside git; rotation documented
- [ ] File-backed keys persist `token_hash` only (`SPANDA_API_KEY_PEPPER` set in production)
- [ ] `SPANDA_SESSION_JWT_SECRET` set when OIDC SSO is enabled
- [ ] `SPANDA_API_REQUIRE_AUTH_READS` or ingress policy when API is network-exposed
- [ ] `SPANDA_TENANT_ID` enforced on mismatched keys (403)
- [ ] RBAC matrix matches mutation endpoints under test
- [ ] Session JWT TTL and refresh window documented (`SPANDA_SESSION_TTL_SECS`)
- [ ] Secret vault never returns raw values in REST responses
- [ ] Mutation audit export (CEF/JSONL) suitable for SIEM retention
- [ ] Encrypted config snapshots when `SPANDA_CONFIG_SNAPSHOT_KEY` set
- [ ] Rate limiting behavior documented per tier

## Prep script

```bash
./scripts/security_audit_prep.sh
```

Produces `.spanda/security-audit-prep.json` for auditor intake.

## Related

- [docs/authentication.md](../../../docs/authentication.md)
- [docs/security-audit-third-party.md](../../../docs/security-audit-third-party.md)

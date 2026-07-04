# Human Accountability

Every mission and deployment should have clear **human ownership** for governance, approval, and emergency response.

## Fields

| Field | Purpose |
|-------|---------|
| `responsible_person` | Primary accountable individual |
| `responsible_organization` | Owning organization |
| `mission_owner` | Mission authority |
| `deployment_owner` | Deployment/environment authority |
| `approval_chain` | Ordered approval steps with roles and assignees |
| `emergency_contact` | Incident escalation |
| `escalation_contact` | Operational escalation |
| `operator_certifications` | Required operator credentials |

## Configuration

```toml
[governance]
responsible_person = "j.smith@hospital.org"
responsible_organization = "Acme Robotics Ops"
mission_owner = "mission.control@hospital.org"
deployment_owner = "facilities@hospital.org"
```

Approval chains, emergency contacts, and escalation contacts are configured in `spanda.governance.toml` and shown on the Control Center Governance tab (Responsible owners). Policy assignments use `POST /v1/governance/policies/assign`.

## Validation

Production and mission-critical maturity stages require:

- Responsible person
- Deployment owner
- Emergency contact

Governance validation reports `GOV_ACCOUNTABILITY` when these are missing.

## Entity projection

Accountability maps to `EntityRecord.owner` and `governance.responsible_person` metadata for API and Control Center display.

## Audit

Changes to accountability fields should flow through config approval workflows (`spanda-config` approvals) and appear in the mutation audit trail.

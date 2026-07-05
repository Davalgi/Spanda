# Habituation and Sensitization

**Status: Experimental**

## Purpose

Reduce alert fatigue while escalating repeated or worsening issues.

## Types

`HabituationPolicy`, `SensitizationPolicy`, `AlertSuppression`, `AlertEscalation`,
`RepetitionPattern`, `AlertFatigueMetric`

## Examples

- Same harmless warning repeated 100 times → suppress
- Minor network glitches increasing → escalate
- Repeated recovery for same device → create incident
- Repeated low battery warnings → maintenance recommendation

## CLI

```bash
spanda alerts analyze
spanda alerts fatigue-report
```

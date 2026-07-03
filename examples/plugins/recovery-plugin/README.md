# Recovery plugin example

Demonstrates `[recovery.extensions]` contributions for the Recovery Orchestrator:

- **playbook** — merged into `spanda recovery playbooks`
- **strategy** — resolved during simulation when named
- **validator** — registered validator extensions (orchestrator requires safe plans when validators exist)

Enable with `spanda plugin install examples/plugins/recovery-plugin` from the repo root.

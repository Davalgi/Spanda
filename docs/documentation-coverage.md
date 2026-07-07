# Documentation Coverage Report

Generated: 2026-07-07

This report is produced by `scripts/validate_documentation.py`. See [coding-standards.md](./coding-standards.md) for the required docstring format.

## Summary

| Metric | Count |
|--------|------:|
| Total methods / functions audited | 9420 |
| Fully documented (structured standard) | 4346 |
| Undocumented or incomplete | 5074 |
| Coverage | 46.1% |

## Coverage by module

| Module | Total | Documented | Coverage |
|--------|------:|-----------:|---------:|
| `python/scripts` | 126 | 54 | 42.9% |
| `rust/spanda-adr` | 10 | 1 | 10.0% |
| `rust/spanda-ai` | 74 | 74 | 100.0% |
| `rust/spanda-api` | 930 | 14 | 1.5% |
| `rust/spanda-assurance` | 206 | 111 | 53.9% |
| `rust/spanda-ast` | 23 | 23 | 100.0% |
| `rust/spanda-audit` | 81 | 54 | 66.7% |
| `rust/spanda-autonomy` | 82 | 0 | 0.0% |
| `rust/spanda-bridge` | 31 | 31 | 100.0% |
| `rust/spanda-capability` | 63 | 54 | 85.7% |
| `rust/spanda-certify` | 21 | 19 | 90.5% |
| `rust/spanda-chaos` | 17 | 4 | 23.5% |
| `rust/spanda-cli` | 602 | 174 | 28.9% |
| `rust/spanda-codegen` | 9 | 9 | 100.0% |
| `rust/spanda-comm` | 55 | 30 | 54.5% |
| `rust/spanda-compliance` | 58 | 3 | 5.2% |
| `rust/spanda-concurrency` | 17 | 17 | 100.0% |
| `rust/spanda-config` | 467 | 30 | 6.4% |
| `rust/spanda-connectivity` | 22 | 22 | 100.0% |
| `rust/spanda-connectivity-runtime` | 23 | 20 | 87.0% |
| `rust/spanda-contract` | 6 | 4 | 66.7% |
| `rust/spanda-core` | 325 | 318 | 97.8% |
| `rust/spanda-dap` | 8 | 8 | 100.0% |
| `rust/spanda-debug` | 7 | 7 | 100.0% |
| `rust/spanda-decision` | 188 | 15 | 8.0% |
| `rust/spanda-deploy-http` | 35 | 23 | 65.7% |
| `rust/spanda-diff` | 18 | 2 | 11.1% |
| `rust/spanda-docs` | 69 | 53 | 76.8% |
| `rust/spanda-driver` | 56 | 54 | 96.4% |
| `rust/spanda-error` | 3 | 3 | 100.0% |
| `rust/spanda-estimate` | 17 | 2 | 11.8% |
| `rust/spanda-explain` | 12 | 7 | 58.3% |
| `rust/spanda-ffi` | 14 | 14 | 100.0% |
| `rust/spanda-fleet` | 146 | 96 | 65.8% |
| `rust/spanda-format` | 60 | 58 | 96.7% |
| `rust/spanda-generate` | 22 | 8 | 36.4% |
| `rust/spanda-governance` | 81 | 5 | 6.2% |
| `rust/spanda-graph` | 53 | 2 | 3.8% |
| `rust/spanda-hal` | 40 | 40 | 100.0% |
| `rust/spanda-hardware` | 35 | 35 | 100.0% |
| `rust/spanda-interpreter` | 327 | 236 | 72.2% |
| `rust/spanda-lexer` | 22 | 22 | 100.0% |
| `rust/spanda-lib-registry` | 44 | 44 | 100.0% |
| `rust/spanda-lint` | 13 | 13 | 100.0% |
| `rust/spanda-llvm` | 42 | 42 | 100.0% |
| `rust/spanda-modules` | 6 | 6 | 100.0% |
| `rust/spanda-node` | 8 | 8 | 100.0% |
| `rust/spanda-ops` | 84 | 0 | 0.0% |
| `rust/spanda-ota` | 78 | 61 | 78.2% |
| `rust/spanda-package` | 254 | 219 | 86.2% |
| `rust/spanda-parser` | 213 | 188 | 88.3% |
| `rust/spanda-plugin` | 163 | 3 | 1.8% |
| `rust/spanda-policy` | 29 | 6 | 20.7% |
| `rust/spanda-providers` | 244 | 156 | 63.9% |
| `rust/spanda-readiness` | 230 | 101 | 43.9% |
| `rust/spanda-recovery` | 115 | 12 | 10.4% |
| `rust/spanda-regex-lang` | 6 | 6 | 100.0% |
| `rust/spanda-risk` | 13 | 0 | 0.0% |
| `rust/spanda-ros2-rclrs-native` | 9 | 9 | 100.0% |
| `rust/spanda-rt` | 23 | 23 | 100.0% |
| `rust/spanda-runtime` | 430 | 236 | 54.9% |
| `rust/spanda-runtime-faults` | 55 | 22 | 40.0% |
| `rust/spanda-runtime-host` | 39 | 39 | 100.0% |
| `rust/spanda-safety` | 22 | 22 | 100.0% |
| `rust/spanda-score` | 10 | 2 | 20.0% |
| `rust/spanda-sdk` | 274 | 0 | 0.0% |
| `rust/spanda-security` | 239 | 135 | 56.5% |
| `rust/spanda-sir` | 42 | 42 | 100.0% |
| `rust/spanda-spoofing` | 53 | 10 | 18.9% |
| `rust/spanda-tamper` | 143 | 28 | 19.6% |
| `rust/spanda-telemetry-store` | 192 | 1 | 0.5% |
| `rust/spanda-threat` | 12 | 2 | 16.7% |
| `rust/spanda-transport` | 47 | 47 | 100.0% |
| `rust/spanda-transport-dds` | 18 | 18 | 100.0% |
| `rust/spanda-transport-mqtt` | 24 | 24 | 100.0% |
| `rust/spanda-transport-ros2` | 68 | 68 | 100.0% |
| `rust/spanda-transport-routing` | 72 | 61 | 84.7% |
| `rust/spanda-transport-websocket` | 19 | 19 | 100.0% |
| `rust/spanda-trust` | 31 | 3 | 9.7% |
| `rust/spanda-twin-cloud` | 28 | 0 | 0.0% |
| `rust/spanda-typecheck` | 145 | 142 | 97.9% |
| `rust/spanda-wasm` | 13 | 7 | 53.8% |
| `rust/spanda-whatif` | 14 | 0 | 0.0% |
| `spanda/examples` | 2 | 2 | 100.0% |
| `ts/ai` | 23 | 23 | 100.0% |
| `ts/cli` | 54 | 23 | 42.6% |
| `ts/comm` | 7 | 0 | 0.0% |
| `ts/ffi` | 11 | 11 | 100.0% |
| `ts/hal` | 2 | 2 | 100.0% |
| `ts/lexer` | 8 | 8 | 100.0% |
| `ts/lib` | 9 | 9 | 100.0% |
| `ts/lsp` | 32 | 28 | 87.5% |
| `ts/modules` | 4 | 3 | 75.0% |
| `ts/native` | 9 | 9 | 100.0% |
| `ts/navigation` | 1 | 1 | 100.0% |
| `ts/network` | 2 | 2 | 100.0% |
| `ts/parser` | 173 | 163 | 94.2% |
| `ts/providers` | 12 | 11 | 91.7% |
| `ts/registry` | 6 | 0 | 0.0% |
| `ts/root` | 593 | 258 | 43.5% |
| `ts/ros2` | 3 | 3 | 100.0% |
| `ts/runtime` | 104 | 75 | 72.1% |
| `ts/safety` | 4 | 1 | 25.0% |
| `ts/security` | 30 | 30 | 100.0% |
| `ts/simulator` | 2 | 2 | 100.0% |
| `ts/soc` | 3 | 3 | 100.0% |
| `ts/transport` | 32 | 32 | 100.0% |
| `ts/types` | 51 | 50 | 98.0% |
| `ts/units` | 8 | 8 | 100.0% |
| `ts/web` | 210 | 8 | 3.8% |

## Coverage by language

| Language | Total | Documented | Coverage |
|----------|------:|-----------:|---------:|
| python | 132 | 54 | 40.9% |
| rust | 7899 | 3527 | 44.7% |
| spanda | 2 | 2 | 100.0% |
| typescript | 1387 | 763 | 55.0% |

## Remaining gaps (public APIs, sample)

Public APIs missing one or more required sections. Run `python3 scripts/validate_documentation.py --warn` for the full list.

- `crates/spanda-adr/src/generate.rs:236` `format_adr_report` — Description section, Inputs section, Outputs section, Example section
- `crates/spanda-api/src/admin_ops.rs:17` `not_found_response` — documentation block, Description section, Inputs section, Outputs section, Example section
- `crates/spanda-api/src/admin_ops.rs:24` `require_admin` — documentation block, Description section, Inputs section, Outputs section, Example section
- `crates/spanda-api/src/admin_ops.rs:34` `admin_api_keys_list` — Description section, Inputs section, Outputs section, Example section
- `crates/spanda-api/src/admin_ops.rs:70` `admin_api_keys_create` — Description section, Inputs section, Outputs section, Example section
- `crates/spanda-api/src/admin_ops.rs:121` `admin_api_keys_patch` — Description section, Inputs section, Outputs section, Example section
- `crates/spanda-api/src/admin_ops.rs:161` `admin_api_keys_delete` — Description section, Inputs section, Outputs section, Example section
- `crates/spanda-api/src/admin_ops.rs:188` `admin_integrations_summary` — Description section, Inputs section, Outputs section, Example section
- `crates/spanda-api/src/admin_ops.rs:222` `find_mission_entity_id` — documentation block, Description section, Inputs section, Outputs section, Example section
- `crates/spanda-api/src/admin_ops.rs:310` `operator_missions_list` — Description section, Inputs section, Outputs section, Example section
- `crates/spanda-api/src/admin_ops.rs:339` `operator_mission_pause` — Description section, Inputs section, Outputs section, Example section
- `crates/spanda-api/src/admin_ops.rs:365` `operator_mission_resume` — Description section, Inputs section, Outputs section, Example section
- `crates/spanda-api/src/admin_ops.rs:391` `operator_mission_cancel` — Description section, Inputs section, Outputs section, Example section
- `crates/spanda-api/src/admin_ops.rs:417` `route_admin` — Description section, Inputs section, Outputs section, Example section
- `crates/spanda-api/src/admin_ops.rs:489` `admin_api_keys_list_json` — documentation block, Description section, Inputs section, Outputs section, Example section
- `crates/spanda-api/src/admin_ops.rs:493` `admin_api_keys_create_json` — documentation block, Description section, Inputs section, Outputs section, Example section
- `crates/spanda-api/src/admin_ops.rs:501` `admin_api_keys_patch_json` — documentation block, Description section, Inputs section, Outputs section, Example section
- `crates/spanda-api/src/admin_ops.rs:510` `admin_api_keys_delete_json` — documentation block, Description section, Inputs section, Outputs section, Example section
- `crates/spanda-api/src/admin_ops.rs:518` `admin_integrations_summary_json` — documentation block, Description section, Inputs section, Outputs section, Example section
- `crates/spanda-api/src/admin_ops.rs:525` `operator_missions_list_json` — documentation block, Description section, Inputs section, Outputs section, Example section
- `crates/spanda-api/src/admin_ops.rs:529` `operator_mission_pause_json` — documentation block, Description section, Inputs section, Outputs section, Example section
- `crates/spanda-api/src/admin_ops.rs:537` `operator_mission_resume_json` — documentation block, Description section, Inputs section, Outputs section, Example section
- `crates/spanda-api/src/admin_ops.rs:545` `operator_mission_cancel_json` — documentation block, Description section, Inputs section, Outputs section, Example section
- `crates/spanda-api/src/admin_users.rs:29` `default_enabled` — documentation block, Description section, Inputs section, Outputs section, Example section
- `crates/spanda-api/src/admin_users.rs:41` `find` — documentation block, Description section, Inputs section, Outputs section, Example section
- `crates/spanda-api/src/admin_users.rs:44` `find_mut` — documentation block, Description section, Inputs section, Outputs section, Example section
- `crates/spanda-api/src/admin_users.rs:49` `users_path` — documentation block, Description section, Inputs section, Outputs section, Example section
- `crates/spanda-api/src/admin_users.rs:53` `hydrate_admin_users` — documentation block, Description section, Inputs section, Outputs section, Example section
- `crates/spanda-api/src/admin_users.rs:62` `persist_admin_users` — documentation block, Description section, Inputs section, Outputs section, Example section
- `crates/spanda-api/src/admin_users.rs:112` `admin_users_list` — Description section, Inputs section, Outputs section, Example section
- `crates/spanda-api/src/admin_users.rs:137` `admin_users_create` — Description section, Inputs section, Outputs section, Example section
- `crates/spanda-api/src/admin_users.rs:196` `admin_users_patch` — Description section, Inputs section, Outputs section, Example section
- `crates/spanda-api/src/admin_users.rs:242` `admin_users_delete` — Description section, Inputs section, Outputs section, Example section
- `crates/spanda-api/src/admin_users.rs:267` `admin_users_list_json` — documentation block, Description section, Inputs section, Outputs section, Example section
- `crates/spanda-api/src/admin_users.rs:271` `route_admin_users` — documentation block, Description section, Inputs section, Outputs section, Example section
- `crates/spanda-api/src/admin_users.rs:294` `import_oidc_directory` — Description section, Inputs section, Outputs section, Example section
- `crates/spanda-api/src/alert_channels.rs:20` `channels_path` — documentation block, Description section, Inputs section, Outputs section, Example section
- `crates/spanda-api/src/alert_channels.rs:24` `hydrate_alert_channels` — documentation block, Description section, Inputs section, Outputs section, Example section
- `crates/spanda-api/src/alert_channels.rs:41` `persist_alert_channels` — documentation block, Description section, Inputs section, Outputs section, Example section
- `crates/spanda-api/src/alert_channels.rs:60` `admin_alert_channels_get` — Description section, Inputs section, Outputs section, Example section
- `crates/spanda-api/src/alert_channels.rs:83` `admin_alert_channels_put` — Description section, Inputs section, Outputs section, Example section
- `crates/spanda-api/src/alert_channels.rs:112` `admin_alert_channels_get_json` — documentation block, Description section, Inputs section, Outputs section, Example section
- `crates/spanda-api/src/alert_channels.rs:119` `admin_alert_channels_put_json` — documentation block, Description section, Inputs section, Outputs section, Example section
- `crates/spanda-api/src/alert_channels.rs:127` `route_alert_channels` — documentation block, Description section, Inputs section, Outputs section, Example section
- `crates/spanda-api/src/audit_log.rs:10` `default_mutation_audit_path` — Description section, Inputs section, Outputs section, Example section
- `crates/spanda-api/src/audit_log.rs:17` `maybe_record_mutation` — Description section, Inputs section, Outputs section, Example section
- `crates/spanda-api/src/audit_log.rs:54` `record_grpc_mutation` — Description section, Inputs section, Outputs section, Example section
- `crates/spanda-api/src/audit_log.rs:112` `read_mutation_audit_lines` — Description section, Inputs section, Outputs section, Example section
- `crates/spanda-api/src/audit_log.rs:124` `export_mutation_audit_cef` — Description section, Inputs section, Outputs section, Example section
- `crates/spanda-api/src/audit_log.rs:168` `export_mutation_audit_jsonl` — Description section, Inputs section, Outputs section, Example section
- `crates/spanda-api/src/auth_routes.rs:12` `route_auth` — Description section, Inputs section, Outputs section, Example section
- `crates/spanda-api/src/autonomy_ops.rs:28` `list_reflex` — Description section, Inputs section, Outputs section, Example section
- `crates/spanda-api/src/autonomy_ops.rs:37` `list_reflex_traces` — Description section, Inputs section, Outputs section, Example section
- `crates/spanda-api/src/autonomy_ops.rs:75` `homeostasis_summary` — Description section, Inputs section, Outputs section, Example section
- `crates/spanda-api/src/autonomy_ops.rs:94` `immunity_scan` — Description section, Inputs section, Outputs section, Example section
- `crates/spanda-api/src/autonomy_ops.rs:110` `attention_queue` — Description section, Inputs section, Outputs section, Example section
- `crates/spanda-api/src/autonomy_ops.rs:144` `entity_autonomy` — Description section, Inputs section, Outputs section, Example section
- `crates/spanda-api/src/autonomy_ops.rs:174` `fusion_summary` — Description section, Inputs section, Outputs section, Example section
- `crates/spanda-api/src/autonomy_ops.rs:203` `memory_summary` — Description section, Inputs section, Outputs section, Example section
- `crates/spanda-api/src/autonomy_ops.rs:231` `list_reflex_json` — Description section, Inputs section, Outputs section, Example section
- `crates/spanda-api/src/autonomy_ops.rs:236` `list_reflex_traces_json` — Description section, Inputs section, Outputs section, Example section
- `crates/spanda-api/src/autonomy_ops.rs:241` `homeostasis_summary_json` — Description section, Inputs section, Outputs section, Example section
- `crates/spanda-api/src/autonomy_ops.rs:246` `immunity_scan_json` — Description section, Inputs section, Outputs section, Example section
- `crates/spanda-api/src/autonomy_ops.rs:251` `attention_queue_json` — Description section, Inputs section, Outputs section, Example section
- `crates/spanda-api/src/autonomy_ops.rs:256` `fusion_summary_json` — Description section, Inputs section, Outputs section, Example section
- `crates/spanda-api/src/autonomy_ops.rs:261` `memory_summary_json` — Description section, Inputs section, Outputs section, Example section
- `crates/spanda-api/src/autonomy_ops.rs:266` `entity_autonomy_json` — Description section, Inputs section, Outputs section, Example section
- `crates/spanda-api/src/control_center_extras.rs:468` `oidc_public_config` — Description section, Inputs section, Outputs section, Example section
- `crates/spanda-api/src/control_center_extras.rs:479` `oidc_userinfo_entry` — Description section, Inputs section, Outputs section, Example section
- `crates/spanda-api/src/control_center_extras.rs:486` `oidc_build_authorize_url` — Description section, Inputs section, Outputs section, Example section
- `crates/spanda-api/src/control_center_extras.rs:541` `oidc_exchange_code` — Description section, Inputs section, Outputs section, Example section
- `crates/spanda-api/src/control_center_extras.rs:814` `admin_oidc_authorize_url` — Description section, Inputs section, Outputs section, Example section
- `crates/spanda-api/src/control_center_extras.rs:833` `admin_oidc_oauth_callback` — Description section, Inputs section, Outputs section, Example section
- `crates/spanda-api/src/control_center_ui.rs:23` `content_type_for_path` — Description section, Inputs section, Outputs section, Example section
- `crates/spanda-api/src/control_center_ui.rs:41` `serve_static` — Description section, Inputs section, Outputs section, Example section
- `crates/spanda-api/src/correlation.rs:27` `new` — documentation block, Description section, Inputs section, Outputs section, Example section
- `crates/spanda-api/src/correlation.rs:33` `push` — documentation block, Description section, Inputs section, Outputs section, Example section
- `crates/spanda-api/src/correlation.rs:40` `list` — documentation block, Description section, Inputs section, Outputs section, Example section
- `crates/spanda-api/src/correlation.rs:44` `list_owned` — documentation block, Description section, Inputs section, Outputs section, Example section
- `crates/spanda-api/src/correlation.rs:48` `from_records` — documentation block, Description section, Inputs section, Outputs section, Example section
- `crates/spanda-api/src/correlation.rs:57` `new_correlation_id` — documentation block, Description section, Inputs section, Outputs section, Example section
- `crates/spanda-api/src/correlation.rs:65` `correlation_from_headers` — documentation block, Description section, Inputs section, Outputs section, Example section
- `crates/spanda-api/src/correlation.rs:88` `now_ms` — documentation block, Description section, Inputs section, Outputs section, Example section
- `crates/spanda-api/src/decision_ops.rs:73` `list_decisions` — Description section, Inputs section, Outputs section, Example section
- `crates/spanda-api/src/decision_ops.rs:91` `entity_decisions` — Description section, Inputs section, Outputs section, Example section
- `crates/spanda-api/src/decision_ops.rs:119` `simulate_decisions` — Description section, Inputs section, Outputs section, Example section
- `crates/spanda-api/src/decision_ops.rs:141` `escalate_decision` — Description section, Inputs section, Outputs section, Example section
- `crates/spanda-api/src/decision_ops.rs:158` `list_escalations` — Description section, Inputs section, Outputs section, Example section
- `crates/spanda-api/src/decision_ops.rs:169` `list_decision_policies` — Description section, Inputs section, Outputs section, Example section
- `crates/spanda-api/src/decision_ops.rs:183` `list_decision_traces` — Description section, Inputs section, Outputs section, Example section
- `crates/spanda-api/src/decision_ops.rs:234` `list_decision_policy_cache` — Description section, Inputs section, Outputs section, Example section
- `crates/spanda-api/src/decision_ops.rs:265` `list_decisions_json` — Description section, Inputs section, Outputs section, Example section
- `crates/spanda-api/src/decision_ops.rs:270` `entity_decisions_json` — Description section, Inputs section, Outputs section, Example section
- `crates/spanda-api/src/decision_ops.rs:275` `simulate_decisions_json` — Description section, Inputs section, Outputs section, Example section
- `crates/spanda-api/src/decision_ops.rs:280` `list_decision_traces_json` — Description section, Inputs section, Outputs section, Example section
- `crates/spanda-api/src/decision_ops.rs:285` `list_decision_policy_cache_json` — Description section, Inputs section, Outputs section, Example section
- `crates/spanda-api/src/decision_ops.rs:290` `list_decision_policies_json` — Description section, Inputs section, Outputs section, Example section
- `crates/spanda-api/src/decision_ops.rs:295` `fleet_decision_mesh_status` — Description section, Inputs section, Outputs section, Example section
- `crates/spanda-api/src/decision_ops.rs:329` `fleet_decision_mesh_conflicts` — Description section, Inputs section, Outputs section, Example section
- `crates/spanda-api/src/differentiation_ops.rs:26` `analytics_what_if` — Description section, Inputs section, Outputs section, Example section
- … and 2203 more public APIs

## CI enforcement

CI runs `python3 scripts/validate_documentation.py --warn --report` on every pull request. Warnings are emitted for public APIs that lack structured documentation; builds do not fail yet.

## Regenerating

```bash
python3 scripts/validate_documentation.py --report
python3 scripts/migrate_legacy_inline_docs.py
python3 scripts/add_structured_api_docs.py
python3 scripts/normalize_inline_docs.py
```

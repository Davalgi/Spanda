/**
 * Span-aware distributed decision diagnostics for IDE and check JSON fallbacks.
 * @module
 */

import type { Program } from "./ast/nodes.js";

export type DecisionDiagnostic = {
  message: string;
  line: number;
  column: number;
  severity: string;
  category: string;
  suggested_fix?: string;
};

/** Collect decision-tree, offline-policy, and authority diagnostics mirroring Rust. */
export function collectDecisionDiagnostics(program: Program): DecisionDiagnostic[] {
  const diags: DecisionDiagnostic[] = [];
  const decisionTrees = program.decisionTrees ?? [];
  const offlinePolicies = program.offlinePolicies ?? [];
  const robots = program.robots ?? [];

  if (decisionTrees.length === 0 && offlinePolicies.length === 0) {
    const hasAuthority = robots.some(
      (r) =>
        (r.localDecisionAuthority?.length ?? 0) > 0 ||
        (r.requiresCentralApproval?.length ?? 0) > 0,
    );
    if (hasAuthority) {
      diags.push({
        message: "Entity declares decision authority but no decision_tree or offline_policy",
        line: 1,
        column: 1,
        severity: "info",
        category: "decision:authority",
        suggested_fix:
          "decision_tree LocalRecovery local {\n    when gps.status == Failed { enter degraded_mode; }\n}",
      });
    }
  }

  for (const tree of decisionTrees) {
    if (tree.branches.length === 0) {
      diags.push({
        message: `decision_tree '${tree.name}' has no when branches`,
        line: tree.span.start.line,
        column: tree.span.start.column,
        severity: "warning",
        category: "decision:tree",
        suggested_fix: `decision_tree ${tree.name} local {\n    when condition { action; }\n}`,
      });
    }
  }

  for (const policy of offlinePolicies) {
    if (policy.maxDurationMinutes === 0) {
      diags.push({
        message: `offline_policy '${policy.name}' has zero max_duration`,
        line: policy.span.start.line,
        column: policy.span.start.column,
        severity: "error",
        category: "decision:offline",
        suggested_fix: `offline_policy ${policy.name} {\n    max_duration = 30 min;\n}`,
      });
    }
    if (policy.allowedActions.length === 0) {
      diags.push({
        message: `offline_policy '${policy.name}' has no allowed_actions`,
        line: policy.span.start.line,
        column: policy.span.start.column,
        severity: "warning",
        category: "decision:offline",
      });
    }
    if (policy.forbiddenActions.length === 0) {
      diags.push({
        message: `offline_policy '${policy.name}' should forbid high-risk actions while offline`,
        line: policy.span.start.line,
        column: policy.span.start.column,
        severity: "info",
        category: "decision:offline",
        suggested_fix:
          "forbidden_actions [disable_safety, accept_unknown_device, update_firmware]",
      });
    }
  }

  for (const robot of robots) {
    const local = robot.localDecisionAuthority ?? [];
    const central = robot.requiresCentralApproval ?? [];
    if (local.length === 0 && central.length === 0) {
      continue;
    }
    for (const action of central) {
      if (local.includes(action)) {
        diags.push({
          message: `robot '${robot.name}': '${action}' is both local and requires central approval`,
          line: robot.span.start.line,
          column: robot.span.start.column,
          severity: "error",
          category: "decision:authority",
        });
      }
    }
  }

  return diags;
}

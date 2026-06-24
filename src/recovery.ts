/**
 * TypeScript self-healing recovery analysis (native CLI fallback).
 * @module
 */

import type { Program } from "./ast/nodes.js";
import { evaluateReadinessTs, type ReadinessOptions } from "./readiness.js";

export type RecoveryStatus =
  | "Success"
  | "PartialSuccess"
  | "Failed"
  | "Aborted"
  | "Unsafe";

export type RecoveryContext = {
  issue: string;
  diagnosis?: string;
  classification?: string;
  level: number;
};

export type PlannedRecoveryAction = {
  description: string;
  risk: string;
  requiresApproval: boolean;
  order: number;
};

export type RecoveryPlan = {
  name: string;
  failure: string;
  diagnosis: string;
  risk: string;
  actions: PlannedRecoveryAction[];
};

export type RecoveryEvidence = {
  failure: string;
  diagnosis: string;
  plan: string;
  safety_validation: string;
  recovery_actions: string[];
  outcome: string;
  operator_approval: string | null;
  verification: string;
};

export type RecoveryResult = {
  plan: string;
  status: RecoveryStatus;
  executed_actions: string[];
  failed_actions: string[];
  verification_outcome: string;
  evidence: RecoveryEvidence;
};

export type RecoveryReadiness = {
  recovery_ready: boolean;
  risk: string;
  readiness_score: number;
  blockers: string[];
};

export type RecoveryReport = {
  policies: Array<{ name: string; triggers: Array<[string, string[]]> }>;
  plans: RecoveryPlan[];
  results: RecoveryResult[];
  readiness: RecoveryReadiness;
  passed: boolean;
};

function classifyFailure(issue: string): string {
  const lower = issue.toLowerCase();
  if (lower.includes("gps") || lower.includes("sensor")) return "SensorFailure";
  if (lower.includes("actuator") || lower.includes("motor")) return "ActuatorFailure";
  if (lower.includes("lte") || lower.includes("wifi") || lower.includes("connect")) {
    return "ConnectivityFailure";
  }
  if (lower.includes("provider")) return "ProviderFailure";
  if (lower.includes("fleet")) return "FleetFailure";
  if (lower.includes("safety")) return "SafetyFailure";
  return "Unknown";
}

function inferDiagnosis(issue: string): string {
  const lower = issue.toLowerCase();
  if (lower.includes("gps")) return "Satellite lock lost";
  if (lower.includes("lidar")) return "Lidar point cloud unavailable";
  if (lower.includes("lte") || lower.includes("wifi")) return "Connectivity link down";
  return "Root cause under investigation";
}

function extractPolicies(program: Program): RecoveryReport["policies"] {
  const specs: RecoveryReport["policies"] = [];
  for (const decl of program.recoveryPolicies ?? []) {
    specs.push({
      name: decl.name,
      triggers: decl.branches.map((b) => [b.condition, b.actions]),
    });
  }
  for (const decl of program.mitigations ?? []) {
    specs.push({
      name: decl.name,
      triggers: decl.branches.map((b) => [b.condition, b.actions]),
    });
  }
  return specs;
}

function parseAction(text: string, order: number): PlannedRecoveryAction {
  const lower = text.toLowerCase();
  const risk =
    lower.includes("unsafe") || lower.includes("restart fleet") || lower.includes("open gate")
      ? "High"
      : lower.includes("halt") || lower.includes("emergency")
        ? "Critical"
        : "Low";
  return {
    description: text,
    risk,
    requiresApproval: risk === "High" || risk === "Critical" || lower.includes("resume mission"),
    order,
  };
}

function actionsForIssue(program: Program, issue: string): PlannedRecoveryAction[] {
  const lower = issue.toLowerCase();
  const actions: PlannedRecoveryAction[] = [];
  let order = 0;
  for (const policy of extractPolicies(program)) {
    for (const [condition, policyActions] of policy.triggers) {
      if (lower.includes(condition.toLowerCase())) {
        for (const action of policyActions) {
          order += 1;
          actions.push(parseAction(action, order));
        }
      }
    }
  }
  if (actions.length > 0) return actions;
  if (lower.includes("gps")) {
    return [
      "switch_to visual_odometry",
      "reduce_speed 0.5 m/s",
      "enter degraded_mode",
    ].map((a, i) => parseAction(a, i + 1));
  }
  return [parseAction("enter safe_mode", 1)];
}

function planRecovery(program: Program, context: RecoveryContext): RecoveryPlan {
  const diagnosis = context.diagnosis ?? inferDiagnosis(context.issue);
  const actions = actionsForIssue(program, context.issue);
  const risk = actions.some((a) => a.risk === "Critical")
    ? "Critical"
    : actions.some((a) => a.risk === "High")
      ? "High"
      : "Low";
  return {
    name: `recovery_${context.issue.replace(/[. ]/g, "_")}`,
    failure: context.issue,
    diagnosis,
    risk,
    actions,
  };
}

function executePlan(
  program: Program,
  plan: RecoveryPlan,
  options: ReadinessOptions,
): RecoveryResult {
  const readiness = evaluateReadinessTs(program, options);
  const executed: string[] = [];
  const failed: string[] = [];
  for (const action of plan.actions) {
    const unsafe = action.description.toLowerCase().includes("unsafe");
    const readinessOk =
      readiness.mission_ready || (action.risk === "Low" && readiness.score.total > 0);
    if (unsafe || !readinessOk) {
      failed.push(action.description);
      continue;
    }
    executed.push(action.description);
  }
  const status: RecoveryStatus =
    failed.length === 0 && executed.length > 0
      ? "Success"
      : executed.length > 0
        ? "PartialSuccess"
        : "Failed";
  const evidence: RecoveryEvidence = {
    failure: plan.failure,
    diagnosis: plan.diagnosis,
    plan: plan.name,
    safety_validation: plan.actions.every((a) => !a.description.toLowerCase().includes("unsafe"))
      ? "PASS"
      : "FAIL",
    recovery_actions: executed,
    outcome: status,
    operator_approval: null,
    verification: status === "Success" ? "Recovery verified" : "Recovery incomplete",
  };
  return {
    plan: plan.name,
    status,
    executed_actions: executed,
    failed_actions: failed,
    verification_outcome: evidence.verification,
    evidence,
  };
}

export function evaluateRecoveryTs(
  program: Program,
  context?: RecoveryContext,
  options: ReadinessOptions = {},
): RecoveryReport {
  const policies = extractPolicies(program);
  const contexts: RecoveryContext[] = context
    ? [context]
    : policies.length > 0
      ? policies.flatMap((p) =>
          p.triggers.map(([cond]) => ({
            issue: cond,
            level: 2,
          })),
        )
      : [{ issue: "gps.failed", diagnosis: "Satellite lock lost", level: 2 }];
  const plans = contexts.map((ctx) => planRecovery(program, ctx));
  const results = plans.map((plan) => executePlan(program, plan, options));
  const readinessScore = evaluateReadinessTs(program, options).score.total;
  const passed = results.every((r) => r.status !== "Unsafe" && r.status !== "Failed");
  return {
    policies,
    plans,
    results,
    readiness: {
      recovery_ready: passed,
      risk: plans[0]?.risk ?? "Unknown",
      readiness_score: readinessScore,
      blockers: passed ? [] : ["One or more recovery actions failed validation"],
    },
    passed,
  };
}

export function simulateFailureRecoveryTs(
  program: Program,
  failureKind: string,
  options: ReadinessOptions = {},
): RecoveryReport {
  return evaluateRecoveryTs(
    program,
    {
      issue: `${failureKind} failure`,
      diagnosis: inferDiagnosis(failureKind),
      classification: classifyFailure(failureKind),
      level: 3,
    },
    options,
  );
}

export function formatRecoveryReport(report: RecoveryReport): string {
  const plan = report.plans[0];
  const result = report.results[0];
  if (!plan || !result) {
    return `Recovery Ready: ${report.readiness.recovery_ready ? "YES" : "NO"}\n`;
  }
  return [
    `Issue:\n${plan.failure}\n`,
    `Diagnosis:\n${plan.diagnosis}\n`,
    `Recovery:\n${plan.actions[0]?.description ?? "none"}\n`,
    `Risk:\n${plan.risk}\n`,
    `Safety Validation:\n${result.evidence.safety_validation}\n`,
    `Outcome:\n${result.status}`,
    `Recovery Ready: ${report.readiness.recovery_ready ? "YES" : "NO"}`,
  ].join("\n");
}

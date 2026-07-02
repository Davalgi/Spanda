/** RBAC constants and helpers shared by Control Center panels. @module */

export type RbacAction =
  | "Deploy"
  | "Operate"
  | "Approve"
  | "Override"
  | "Shutdown"
  | "Recover"
  | "Delete"
  | "Provision";

export type ControlCenterTab =
  | "dashboard"
  | "entities"
  | "devices"
  | "fleet"
  | "discovery"
  | "provisioning"
  | "mapping"
  | "config"
  | "health"
  | "readiness"
  | "drift"
  | "alerts"
  | "security"
  | "ota"
  | "sre"
  | "compliance"
  | "audit"
  | "decisions"
  | "recovery"
  | "digital-thread"
  | "adas"
  | "humans"
  | "smart-spaces"
  | "executive"
  | "analytics"
  | "twins"
  | "traceability"
  | "administration"
  | "mission"
  | "operator"
  | "assurance"
  | "diagnosis"
  | "simulation"
  | "replay";

export const RBAC_ACTIONS: RbacAction[] = [
  "Deploy",
  "Operate",
  "Approve",
  "Override",
  "Shutdown",
  "Recover",
  "Delete",
  "Provision",
];

export const TAB_ACCESS: Record<ControlCenterTab, string[]> = {
  dashboard: ["*"],
  entities: ["administrator", "supervisor", "developer"],
  devices: ["*"],
  fleet: ["*"],
  discovery: ["administrator", "supervisor", "developer", "operator", "safety_officer"],
  provisioning: ["administrator", "supervisor", "developer", "operator", "safety_officer"],
  mapping: ["*"],
  config: ["administrator", "supervisor", "developer"],
  health: ["*"],
  readiness: ["*"],
  drift: ["administrator", "supervisor", "developer"],
  alerts: ["*"],
  security: ["*"],
  ota: ["administrator", "supervisor", "developer"],
  sre: ["*"],
  compliance: ["administrator", "supervisor", "developer", "auditor", "safety_officer"],
  audit: ["administrator", "supervisor", "developer", "auditor"],
  decisions: ["administrator", "supervisor", "developer"],
  recovery: ["administrator", "supervisor", "operator", "safety_officer"],
  "digital-thread": ["*"],
  traceability: ["*"],
  analytics: ["administrator", "supervisor", "developer", "auditor"],
  twins: ["*"],
  operator: ["administrator", "supervisor", "operator", "safety_officer"],
  assurance: ["*"],
  diagnosis: ["*"],
  adas: ["*"],
  humans: ["*"],
  "smart-spaces": ["*"],
  executive: ["*"],
  administration: ["administrator"],
  mission: ["administrator", "supervisor", "operator", "safety_officer"],
  simulation: ["administrator", "supervisor", "developer"],
  replay: ["*"],
};

export const ROLE_META: Record<
  string,
  { label: string; summary: string }
> = {
  guest: {
    label: "Guest",
    summary: "Read-only dashboards. Paste a Bearer token to unlock role-specific operations.",
  },
  auditor: {
    label: "Auditor",
    summary: "Read-only compliance and audit views. No mutation actions.",
  },
  operator: {
    label: "Operator",
    summary: "Day-to-day fleet operations: incidents, provisioning, quarantine, recovery.",
  },
  developer: {
    label: "Developer",
    summary: "Deploy and configure: snapshots, drift, OTA plans, entities, decisions.",
  },
  safety_officer: {
    label: "Safety Officer",
    summary: "Safety approvals, shutdown workflows, and compliance export.",
  },
  supervisor: {
    label: "Supervisor",
    summary: "Full operational control except delete.",
  },
  administrator: {
    label: "Administrator",
    summary: "Full access to every tab and mutation action.",
  },
};

export type RbacContext = {
  key_id?: string;
  role?: string;
  permissions?: string[];
  tenant_id?: string;
};

export function roleKey(role: string | undefined): string {
  if (!role) return "guest";
  if (typeof role === "string") {
    const normalized = role.replace(/^Role::/, "").toLowerCase();
    return normalized || "guest";
  }
  return "guest";
}

export function tabAllowed(tab: ControlCenterTab, effectiveRole: string): boolean {
  const access = TAB_ACCESS[tab];
  if (!access) return true;
  if (access.includes("*")) return true;
  return access.includes(effectiveRole);
}

export function canAction(ctx: RbacContext | null, action: RbacAction): boolean {
  if (!ctx?.permissions) return false;
  return ctx.permissions.includes(action);
}

export function authStorageKey(apiHost: string): string {
  return `spanda.control_center.bearer_token.v1:${apiHost}`;
}

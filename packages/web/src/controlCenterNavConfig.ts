/** Control Center navigation groups and tab labels. @module */

import { type ControlCenterTab } from "./controlCenterRbac";

export type NavGroup = {
  id: string;
  label: string;
  tabs: ControlCenterTab[];
};

export const CONTROL_CENTER_NAV_GROUPS: NavGroup[] = [
  {
    id: "overview",
    label: "Overview",
    tabs: ["dashboard", "executive"],
  },
  {
    id: "fleet",
    label: "Fleet & devices",
    tabs: [
      "devices",
      "fleet",
      "discovery",
      "provisioning",
      "mapping",
      "entities",
      "operator",
      "mission",
    ],
  },
  {
    id: "health",
    label: "Health & incidents",
    tabs: [
      "health",
      "readiness",
      "alerts",
      "assurance",
      "diagnosis",
      "recovery",
      "sre",
    ],
  },
  {
    id: "governance",
    label: "Governance & config",
    tabs: [
      "security",
      "compliance",
      "audit",
      "drift",
      "config",
      "ota",
      "decisions",
    ],
  },
  {
    id: "digital-twin",
    label: "Digital twin & replay",
    tabs: ["digital-thread", "traceability", "twins", "simulation", "replay"],
  },
  {
    id: "analytics",
    label: "Analytics & domains",
    tabs: ["analytics", "adas", "humans", "smart-spaces"],
  },
  {
    id: "admin",
    label: "Administration",
    tabs: ["administration"],
  },
];

export const TAB_DESCRIPTIONS: Partial<Record<ControlCenterTab, string>> = {
  dashboard: "Fleet-wide summary — devices, agents, and open alerts.",
  devices: "Device pool lifecycle, trust, and assignment.",
  fleet: "Fleets, robots, and connected agents.",
  discovery: "Scan transports and register new hardware.",
  provisioning: "Trust, provision, and assign devices to robots.",
  health: "Device pool health rollup and breakdown.",
  alerts: "Operational alerts sorted by severity.",
  readiness: "Mission readiness impact and blocked devices.",
  sre: "Availability, SLOs, incidents, and observability traces.",
  recovery: "Incident playbooks, orchestration, and rollback.",
  mapping: "Logical names mapped to physical devices and redundancy groups.",
  config: "Publish approval queue for configuration snapshots.",
  drift: "Compare live config against saved baselines.",
  security: "Package trust evaluation and RBAC permission matrix.",
  ota: "Plan and execute fleet firmware rollouts.",
  compliance: "Accreditation profiles and signed evidence export.",
  audit: "Immutable mutation audit trail for compliance.",
  decisions: "Distributed decision layers, escalations, and live trace audit.",
  executive: "Cross-domain KPI scorecard for leadership dashboards.",
  analytics: "What-if, mission risk, trust graph, and differentiation analytics.",
  twins: "Twin Cloud registry — persisted mission twin snapshots.",
  traceability: "Device trust and logical name mapping for audit chains.",
  adas: "ADAS vehicle health, trust, readiness, and OTA status.",
  humans: "Operators, wearables, HRI sessions, and mission approvals.",
  "smart-spaces": "Smart building facilities, zones, energy, and occupancy.",
  entities: "Unified entity graph — browse, search, and inspect relationships.",
  administration: "API keys, users, alert channels, and integrations.",
  "digital-thread": "Capability-to-device graph across the stack.",
};

export function tabLabel(name: ControlCenterTab): string {
  if (name === "adas") return "ADAS";
  if (name === "humans") return "Humans";
  if (name === "smart-spaces") return "Smart Spaces";
  if (name === "sre") return "SRE";
  if (name === "ota") return "OTA";
  if (name === "twins") return "Twin Cloud";
  if (name === "administration") return "Administration";
  if (name === "mission") return "Mission";
  if (name === "simulation") return "Simulation";
  if (name === "replay") return "Replay";
  if (name === "provisioning") return "Provisioning";
  if (name === "discovery") return "Discovery";
  return name.charAt(0).toUpperCase() + name.slice(1).replace(/-/g, " ");
}

export function allNavTabs(): ControlCenterTab[] {
  return CONTROL_CENTER_NAV_GROUPS.flatMap((group) => group.tabs);
}

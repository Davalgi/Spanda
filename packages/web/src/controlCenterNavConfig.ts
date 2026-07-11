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
      "fleet-map",
      "discovery",
      "provisioning",
      "mapping",
      "entities",
      "operator",
      "mission",
      "continuity",
      "mesh",
    ],
  },
  {
    id: "health",
    label: "Health & incidents",
    tabs: [
      "health",
      "readiness",
      "trends",
      "telemetry",
      "alerts",
      "assurance",
      "diagnosis",
      "recovery",
      "resilient-autonomy",
      "sre",
      "chaos",
    ],
  },
  {
    id: "governance",
    label: "Governance & config",
    tabs: [
      "security",
      "compliance",
      "governance",
      "audit",
      "drift",
      "config",
      "ota",
      "decisions",
      "differentiation",
      "reports",
    ],
  },
  {
    id: "digital-twin",
    label: "Digital twin & replay",
    tabs: ["digital-thread", "traceability", "twins", "simulation", "replay", "playground"],
  },
  {
    id: "analytics",
    label: "Analytics & domains",
    tabs: [
      "analytics",
      "adas",
      "humans",
      "smart-spaces",
      "sar",
      "healthcare",
      "warehouse",
      "agriculture",
      "maritime",
    ],
  },
  {
    id: "admin",
    label: "Administration",
    tabs: ["administration", "about", "marketplace"],
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
  assurance: "Assurance policy summary and program-level assure checks.",
  diagnosis: "Diagnosis policy summary and program-level diagnose checks.",
  readiness: "Mission readiness impact and blocked devices.",
  sre: "Availability, SLOs, incidents, and observability traces.",
  recovery: "Incident playbooks, orchestration, and rollback.",
  mapping: "Logical names mapped to physical devices and redundancy groups.",
  config: "Publish approval queue for configuration snapshots.",
  drift: "Compare live config against saved baselines.",
  security: "Package trust evaluation and RBAC permission matrix.",
  ota: "Plan and execute fleet firmware rollouts.",
  compliance: "Accreditation profiles and signed evidence export.",
  governance: "Deployment profiles, compliance validation, certification, risk, and accountability.",
  audit: "Immutable mutation audit trail for compliance.",
  decisions: "Distributed decision layers, escalations, and live trace audit.",
  differentiation: "Mission contracts, explainability, and decision audit trail for the loaded program.",
  executive: "Cross-domain KPI scorecard for leadership dashboards.",
  analytics: "What-if, mission risk, trust graph, and differentiation analytics.",
  twins: "Twin Cloud registry — persisted mission twin snapshots.",
  traceability:
    "Capability and hardware traceability matrix, entity chain, and device pool mapping.",
  adas: "ADAS vehicle health, trust, readiness, and OTA status.",
  humans: "Operators, wearables, HRI sessions, and mission approvals.",
  "smart-spaces": "Smart building facilities, zones, energy, and occupancy.",
  sar: "Search & Rescue — remote expert, mission approve, AR sessions, continuity.",
  healthcare: "Connected healthcare — wearables, human readiness, medical compliance.",
  warehouse: "Warehouse automation — fleet delivery, pick missions, continuity.",
  agriculture: "Agriculture — field fleet map and readiness trends.",
  maritime: "Maritime — fleet map, incidents, and pre-departure readiness.",
  entities: "Unified entity graph — browse, search, and inspect relationships.",
  administration: "API keys, users, alert channels, and integrations.",
  about: "Platform, UI, desktop, REST, and gRPC component versions for this instance.",
  "digital-thread": "Lifecycle graph (requirement → retirement) with device overlays and phase-path query.",
  telemetry: "Live WebSocket telemetry, traces, and alert stream.",
  trends: "Readiness history slopes and forecasted mission risk.",
  continuity: "Takeover, delegation, and mission pause during continuity events.",
  mesh: "Autonomous Entity Mesh — topology, trusted routes, partitions, and coordinator status.",
  "fleet-map": "Geospatial or grid map of robots, agents, and devices.",
  reports: "Scheduled compliance report delivery and preview.",
  playground: "In-browser WASM check and run for Spanda programs.",
  marketplace: "Installed plugins and Control Center panel extensions.",
  chaos: "Fault injection catalog and chaos simulation.",
  "resilient-autonomy":
    "Cognitive & Resilience — reflex events, attention queue, homeostasis, platform immunity, operational memory, damage risk, recovery confidence.",
};

export function tabLabel(name: ControlCenterTab): string {
  if (name === "adas") return "ADAS";
  if (name === "humans") return "Humans";
  if (name === "smart-spaces") return "Smart Spaces";
  if (name === "sar") return "SAR";
  if (name === "healthcare") return "Healthcare";
  if (name === "warehouse") return "Warehouse";
  if (name === "agriculture") return "Agriculture";
  if (name === "maritime") return "Maritime";
  if (name === "sre") return "SRE";
  if (name === "ota") return "OTA";
  if (name === "twins") return "Twin Cloud";
  if (name === "administration") return "Administration";
  if (name === "about") return "About";
  if (name === "mission") return "Mission";
  if (name === "simulation") return "Simulation";
  if (name === "replay") return "Replay";
  if (name === "provisioning") return "Provisioning";
  if (name === "discovery") return "Discovery";
  if (name === "resilient-autonomy") return "Cognitive & Resilience";
  if (name === "fleet-map") return "Fleet map";
  if (name === "telemetry") return "Telemetry";
  if (name === "trends") return "Trends";
  if (name === "continuity") return "Continuity";
  if (name === "reports") return "Reports";
  if (name === "playground") return "Playground";
  if (name === "marketplace") return "Marketplace";
  if (name === "chaos") return "Chaos";
  if (name === "mesh") return "Entity Mesh";
  return name.charAt(0).toUpperCase() + name.slice(1).replace(/-/g, " ");
}

export function allNavTabs(): ControlCenterTab[] {
  return CONTROL_CENTER_NAV_GROUPS.flatMap((group) => group.tabs);
}

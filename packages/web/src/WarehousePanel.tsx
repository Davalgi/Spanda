/** Warehouse automation Control Center domain dashboard. @module */

import type { ControlCenterTab, RbacAction } from "./controlCenterRbac";
import { SolutionDomainPanel } from "./SolutionDomainPanel";

type Props = {
  baseUrl: string;
  authHeaders: () => HeadersInit;
  can: (action: RbacAction) => boolean;
  hasToken: boolean;
  onNavigate: (tab: ControlCenterTab) => void;
};

const CONFIG = {
  id: "warehouse",
  title: "Warehouse Automation",
  hint: "Fleet delivery, AR pick missions, continuity, and readiness for warehouse robots.",
  exampleConfig: "examples/end_to_end/warehouse_delivery/spanda.toml",
  exampleProgram: "examples/end_to_end/warehouse_delivery/src/main.sd",
  deepLinks: [
    { tab: "fleet" as const, label: "Fleet" },
    { tab: "mission" as const, label: "Mission" },
    { tab: "continuity" as const, label: "Continuity" },
    { tab: "readiness" as const, label: "Readiness" },
  ],
  domainTags: ["warehouse", "delivery", "pick", "amr"],
};

const EXTRA_LOADS = [
  { key: "fleet", path: "/v1/fleet/agents", label: "Fleet agents" },
  { key: "continuity", path: "/v1/continuity/status", label: "Continuity status" },
  { key: "readiness", path: "/v1/readiness/impact", label: "Readiness impact" },
];

export function WarehousePanel(props: Props) {
  // Compose warehouse ops from fleet, continuity, and readiness APIs.
  return <SolutionDomainPanel {...props} config={CONFIG} extraLoads={EXTRA_LOADS} />;
}

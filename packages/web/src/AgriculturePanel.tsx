/** Agriculture Control Center domain dashboard. @module */

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
  id: "agriculture",
  title: "Agriculture",
  hint: "Field patrol fleet map and readiness trends for agricultural robots.",
  exampleConfig: "examples/solutions/agriculture/spanda.toml",
  exampleProgram: "examples/solutions/agriculture/field_patrol.sd",
  deepLinks: [
    { tab: "fleet-map" as const, label: "Fleet map" },
    { tab: "trends" as const, label: "Readiness trends" },
    { tab: "fleet" as const, label: "Fleet" },
    { tab: "telemetry" as const, label: "Telemetry" },
  ],
};

const EXTRA_LOADS = [
  { key: "forecast", path: "/v1/analytics/readiness-forecast", label: "Readiness forecast" },
  { key: "fleet", path: "/v1/fleet/agents", label: "Fleet agents" },
];

export function AgriculturePanel(props: Props) {
  // Compose agriculture ops from fleet map and readiness forecast APIs.
  return <SolutionDomainPanel {...props} config={CONFIG} extraLoads={EXTRA_LOADS} />;
}

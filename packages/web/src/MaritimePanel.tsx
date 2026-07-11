/** Maritime Control Center domain dashboard. @module */

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
  id: "maritime",
  title: "Maritime",
  hint: "Fleet map, incident workflow, and pre-departure readiness for maritime systems.",
  exampleConfig: "examples/solutions/maritime/spanda.toml",
  exampleProgram: "examples/solutions/maritime/harbor_patrol.sd",
  deepLinks: [
    { tab: "fleet-map" as const, label: "Fleet map" },
    { tab: "sre" as const, label: "SRE / incidents" },
    { tab: "readiness" as const, label: "Readiness" },
    { tab: "alerts" as const, label: "Alerts" },
  ],
};

const EXTRA_LOADS = [
  { key: "sre", path: "/v1/sre/summary", label: "SRE summary" },
  { key: "alerts", path: "/v1/alerts", label: "Alerts" },
  { key: "readiness", path: "/v1/readiness/impact", label: "Readiness impact" },
];

export function MaritimePanel(props: Props) {
  // Compose maritime ops from fleet map, SRE, and readiness APIs.
  return <SolutionDomainPanel {...props} config={CONFIG} extraLoads={EXTRA_LOADS} />;
}

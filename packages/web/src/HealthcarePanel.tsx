/** Healthcare Control Center domain dashboard. @module */

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
  id: "healthcare",
  title: "Connected Healthcare",
  hint: "Wearable health, human readiness, and medical compliance workflows.",
  exampleConfig: "examples/solutions/spatial-computing/spanda.toml",
  exampleProgram:
    "examples/solutions/spatial-computing/wearable-health/health_patrol.sd",
  deepLinks: [
    { tab: "humans" as const, label: "Humans" },
    { tab: "smart-spaces" as const, label: "Smart Spaces" },
    { tab: "compliance" as const, label: "Compliance" },
    { tab: "readiness" as const, label: "Readiness" },
  ],
  complianceProfile: "medical",
  domainTags: ["healthcare", "medical", "wearable", "patient", "health"],
};

const EXTRA_LOADS = [
  { key: "humans", path: "/v1/humans", label: "Humans directory" },
  { key: "wearables", path: "/v1/wearables", label: "Wearables" },
  { key: "health", path: "/v1/human-health/policy", label: "Human health policy" },
  { key: "team", path: "/v1/humans/readiness", label: "Team readiness" },
];

export function HealthcarePanel(props: Props) {
  // Compose healthcare signals from humans, wearables, and health policy APIs.
  return <SolutionDomainPanel {...props} config={CONFIG} extraLoads={EXTRA_LOADS} />;
}

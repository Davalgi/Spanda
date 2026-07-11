/** Search & Rescue Control Center domain dashboard. @module */

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
  id: "sar",
  title: "Search & Rescue",
  hint: "Remote expert, mission approve, AR annotate, and continuity for SAR field teams.",
  exampleConfig: "examples/solutions/spatial-computing/spanda.toml",
  exampleProgram:
    "examples/solutions/spatial-computing/search-and-rescue-ar/sar_mission.sd",
  deepLinks: [
    { tab: "humans" as const, label: "Humans / remote expert" },
    { tab: "mission" as const, label: "Mission" },
    { tab: "continuity" as const, label: "Continuity" },
    { tab: "diagnosis" as const, label: "Diagnosis" },
  ],
};

const EXTRA_LOADS = [
  { key: "hri", path: "/v1/hri/sessions", label: "HRI / remote expert sessions" },
  { key: "collab", path: "/v1/hri/collaboration", label: "Collaboration graph" },
  { key: "continuity", path: "/v1/continuity/status", label: "Continuity status" },
];

export function SarPanel(props: Props) {
  // Compose SAR ops from Humans, mission approvals, HRI sessions, and continuity APIs.
  return <SolutionDomainPanel {...props} config={CONFIG} extraLoads={EXTRA_LOADS} />;
}

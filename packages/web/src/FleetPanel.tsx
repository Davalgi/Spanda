import { CcEmptyState, CcMiniStats, CcSection } from "./controlCenterUi";

export type FleetRow = {
  id: string;
  robot_count: number;
};

export type RobotRow = {
  id: string;
  model?: string;
  hardware_profile?: string;
};

export type AgentRow = {
  robot_name: string;
  url: string;
  token?: string;
};

type Props = {
  fleets: FleetRow[];
  robots: RobotRow[];
  agents: AgentRow[];
  loading?: boolean;
};

export function FleetPanel({ fleets, robots, agents, loading }: Props) {
  const connectedAgents = agents.length;
  const totalRobots = robots.length;

  return (
    <div className="cc-panel">
      <CcMiniStats
        items={[
          { label: "Fleets", value: fleets.length },
          { label: "Robots", value: totalRobots },
          { label: "Connected agents", value: connectedAgents, tone: connectedAgents > 0 ? "ok" : "warn" },
        ]}
      />

      <div className="cc-panel-grid">
        <CcSection title="Fleets" hint="Logical groupings of robots.">
          {loading && fleets.length === 0 ? (
            <CcEmptyState title="Loading fleets…" />
          ) : fleets.length === 0 ? (
            <CcEmptyState
              title="No fleets configured"
              description="Fleets appear when robots are organized in your deployment config."
            />
          ) : (
            <ul className="cc-card-list">
              {fleets.map((fleet) => (
                <li key={fleet.id} className="cc-card-item">
                  <span className="cc-card-item-title">{fleet.id}</span>
                  <span className="cc-card-item-meta">
                    {fleet.robot_count} robot{fleet.robot_count === 1 ? "" : "s"}
                  </span>
                </li>
              ))}
            </ul>
          )}
        </CcSection>

        <CcSection title="Robots" hint="Physical or simulated platforms in the pool.">
          {loading && robots.length === 0 ? (
            <CcEmptyState title="Loading robots…" />
          ) : robots.length === 0 ? (
            <CcEmptyState
              title="No robots registered"
              description="Robots are defined in your Spanda project config and appear here once the Control Center loads them."
            />
          ) : (
            <ul className="cc-card-list">
              {robots.map((robot) => (
                <li key={robot.id} className="cc-card-item">
                  <span className="cc-card-item-title">{robot.id}</span>
                  <span className="cc-card-item-meta">
                    {robot.hardware_profile ?? robot.model ?? "No profile"}
                  </span>
                </li>
              ))}
            </ul>
          )}
        </CcSection>

        <CcSection
          title="Fleet agents"
          hint="Live runtime endpoints reporting telemetry and readiness."
        >
          {loading && agents.length === 0 ? (
            <CcEmptyState title="Loading agents…" />
          ) : agents.length === 0 ? (
            <CcEmptyState
              title="No agents connected"
              description="Start a robot agent with spanda agent or connect via your deployment pipeline. Agents show their URL here when registered."
            />
          ) : (
            <ul className="cc-card-list">
              {agents.map((agent) => (
                <li key={agent.robot_name} className="cc-card-item">
                  <span className="cc-card-item-title">{agent.robot_name}</span>
                  <a
                    className="cc-card-item-link"
                    href={agent.url}
                    target="_blank"
                    rel="noreferrer"
                  >
                    {agent.url}
                  </a>
                </li>
              ))}
            </ul>
          )}
        </CcSection>
      </div>
    </div>
  );
}

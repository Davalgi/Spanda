import { useCallback, useEffect, useState } from "react";
import type { RbacAction } from "./controlCenterRbac";
import { CcEmptyState, CcMiniStats, CcSection } from "./controlCenterUi";
import { useRegisterTabRefresh } from "./useControlCenterTabRefresh";
import { fetchAgentContinuity, type AgentContinuityResponse } from "./continuity-agent";
import type { FleetAgent } from "./controlCenterTypes";

type Props = {
  baseUrl: string;
  authHeaders: () => HeadersInit;
  can: (action: RbacAction) => boolean;
  hasToken: boolean;
  agents: FleetAgent[];
};

export function ContinuityPanel({ baseUrl, authHeaders, can, hasToken, agents }: Props) {
  const [agentUrl, setAgentUrl] = useState("");
  const [continuity, setContinuity] = useState<AgentContinuityResponse | null>(null);
  const [missions, setMissions] = useState<Record<string, unknown>[]>([]);
  const [busy, setBusy] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const loadMissions = useCallback(async () => {
    if (!hasToken) return;
    try {
      const res = await fetch(`${baseUrl}/v1/operator/missions`, { headers: authHeaders() });
      if (res.ok) {
        const body = await res.json();
        setMissions((body.missions as Record<string, unknown>[]) ?? []);
      }
    } catch {
      setMissions([]);
    }
  }, [baseUrl, authHeaders, hasToken]);

  useEffect(() => {
    void loadMissions();
  }, [loadMissions]);

  useRegisterTabRefresh(loadMissions, { busy });

  const pollAgent = async (url: string) => {
    setBusy(true);
    setError(null);
    try {
      const status = await fetchAgentContinuity(url);
      setContinuity(status);
      setAgentUrl(url);
    } catch (err) {
      setError(String(err));
      setContinuity(null);
    } finally {
      setBusy(false);
    }
  };

  const missionControl = async (missionId: string, action: "pause" | "resume" | "cancel") => {
    if (!hasToken || !can("Operate")) return;
    setBusy(true);
    setError(null);
    try {
      const res = await fetch(`${baseUrl}/v1/operator/mission/${action}`, {
        method: "POST",
        headers: authHeaders(),
        body: JSON.stringify({ mission_id: missionId }),
      });
      if (!res.ok) throw new Error(`mission ${action} ${res.status}`);
      await loadMissions();
    } catch (err) {
      setError(String(err));
    } finally {
      setBusy(false);
    }
  };

  return (
    <div className="cc-panel">
      {error && <div className="error">{error}</div>}

      <CcSection title="Fleet agents" hint="Poll continuity state from connected fleet agents.">
        <CcMiniStats
          items={[
            { label: "Agents", value: agents.length },
            { label: "Active continuity", value: continuity?.continuity_active ?? "—" },
            { label: "Successor", value: continuity?.continuity_successor ?? "—" },
          ]}
        />
        <ul className="cc-card-list">
          {agents.map((agent) => (
            <li key={agent.url} className="cc-card-item">
              <span className="cc-card-item-title">{agent.robot_name}</span>
              <span className="cc-card-item-meta">{agent.url}</span>
              <button type="button" onClick={() => void pollAgent(agent.url)} disabled={busy}>
                Poll continuity
              </button>
            </li>
          ))}
          {agents.length === 0 && (
            <CcEmptyState title="No fleet agents" description="Register agents via spanda fleet agent." />
          )}
        </ul>
      </CcSection>

      {continuity && (
        <CcSection title={`Continuity — ${agentUrl}`}>
          <dl className="cc-detail-grid">
            <div className="cc-detail-row">
              <dt>Active</dt>
              <dd>{continuity.continuity_active ?? "none"}</dd>
            </div>
            <div className="cc-detail-row">
              <dt>Successor</dt>
              <dd>{continuity.continuity_successor ?? "—"}</dd>
            </div>
            <div className="cc-detail-row">
              <dt>Mode</dt>
              <dd>{continuity.continuity_mode ?? "—"}</dd>
            </div>
            <div className="cc-detail-row">
              <dt>Engine</dt>
              <dd>{continuity.continuity_engine ?? "—"}</dd>
            </div>
            <div className="cc-detail-row">
              <dt>Handoff from</dt>
              <dd>{continuity.mission_handoff_from ?? "—"}</dd>
            </div>
          </dl>
          {continuity.last_continuity_commands && continuity.last_continuity_commands.length > 0 && (
            <pre className="cc-action-result">{continuity.last_continuity_commands.join("\n")}</pre>
          )}
        </CcSection>
      )}

      <CcSection title="Mission control" hint="Pause, resume, or cancel missions during continuity events.">
        {missions.length === 0 ? (
          <CcEmptyState title="No active missions" />
        ) : (
          <ul className="cc-card-list">
            {missions.map((mission) => {
              const id = String(mission.mission_id ?? mission.id ?? "unknown");
              return (
                <li key={id} className="cc-card-item">
                  <span className="cc-card-item-title">{id}</span>
                  <span className="cc-card-item-meta">{String(mission.state ?? mission.status ?? "—")}</span>
                  {can("Operate") && hasToken && (
                    <span className="cc-action-bar">
                      <button type="button" onClick={() => void missionControl(id, "pause")} disabled={busy}>
                        Pause
                      </button>
                      <button type="button" onClick={() => void missionControl(id, "resume")} disabled={busy}>
                        Resume
                      </button>
                      <button type="button" onClick={() => void missionControl(id, "cancel")} disabled={busy}>
                        Cancel
                      </button>
                    </span>
                  )}
                </li>
              );
            })}
          </ul>
        )}
      </CcSection>
    </div>
  );
}

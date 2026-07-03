import { useCallback, useEffect, useState } from "react";
import { CollaborationGraph } from "./CollaborationGraph";
import type { RbacAction } from "./controlCenterRbac";
import { CcEmptyState, CcMiniStats, CcSection } from "./controlCenterUi";
import { ControlCenterDataTable } from "./controlCenterDataTable";

type Props = {
  baseUrl: string;
  authHeaders: () => HeadersInit;
  can: (action: RbacAction) => boolean;
  hasToken: boolean;
};

export function HumansPanel({ baseUrl, authHeaders, can, hasToken }: Props) {
  const [humansList, setHumansList] = useState<Record<string, unknown>[]>([]);
  const [wearablesList, setWearablesList] = useState<Record<string, unknown>[]>([]);
  const [hriSessions, setHriSessions] = useState<Record<string, unknown>[]>([]);
  const [humanHealthPolicy, setHumanHealthPolicy] = useState<Record<string, unknown> | null>(null);
  const [teamReadiness, setTeamReadiness] = useState<Record<string, unknown> | null>(null);
  const [collaborationGraph, setCollaborationGraph] = useState<Record<string, unknown> | null>(
    null,
  );
  const [hriContext, setHriContext] = useState<Record<string, unknown> | null>(null);
  const [humanTwins, setHumanTwins] = useState<Record<string, unknown>[]>([]);
  const [missionApprovals, setMissionApprovals] = useState<Record<string, unknown>[]>([]);
  const [annotateSession, setAnnotateSession] = useState("");
  const [annotateText, setAnnotateText] = useState("");
  const [busy, setBusy] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const load = useCallback(async () => {
    setBusy(true);
    setError(null);
    try {
      const [
        humansRes,
        wearablesRes,
        sessionsRes,
        healthRes,
        readinessRes,
        collabRes,
        contextRes,
        twinsRes,
        missionApprovalsRes,
      ] = await Promise.all([
        fetch(`${baseUrl}/v1/humans`),
        fetch(`${baseUrl}/v1/wearables`),
        fetch(`${baseUrl}/v1/hri/sessions`),
        fetch(`${baseUrl}/v1/human-health/policy`),
        fetch(`${baseUrl}/v1/humans/readiness`),
        fetch(`${baseUrl}/v1/hri/collaboration`),
        fetch(`${baseUrl}/v1/hri/context`),
        fetch(`${baseUrl}/v1/humans/twins`),
        fetch(`${baseUrl}/v1/operator/mission/approvals`),
      ]);
      if (!humansRes.ok) throw new Error(`humans ${humansRes.status}`);
      const humansBody = await humansRes.json();
      const wearablesBody = wearablesRes.ok ? await wearablesRes.json() : null;
      const sessionsBody = sessionsRes.ok ? await sessionsRes.json() : null;
      const healthBody = healthRes.ok ? await healthRes.json() : null;
      const readinessBody = readinessRes.ok ? await readinessRes.json() : null;
      const collabBody = collabRes.ok ? await collabRes.json() : null;
      const contextBody = contextRes.ok ? await contextRes.json() : null;
      const twinsBody = twinsRes.ok ? await twinsRes.json() : null;
      const missionApprovalsBody = missionApprovalsRes.ok ? await missionApprovalsRes.json() : null;
      setHumansList((humansBody.humans as Record<string, unknown>[]) ?? []);
      setWearablesList((wearablesBody?.wearables as Record<string, unknown>[]) ?? []);
      setHriSessions((sessionsBody?.sessions as Record<string, unknown>[]) ?? []);
      setHumanHealthPolicy(
        (healthBody?.policy as Record<string, unknown>) ??
          (wearablesBody?.human_health as Record<string, unknown>) ??
          null,
      );
      setTeamReadiness(readinessBody);
      setCollaborationGraph(collabBody);
      setHriContext(contextBody);
      setHumanTwins((twinsBody?.twins as Record<string, unknown>[]) ?? []);
      setMissionApprovals((missionApprovalsBody?.approvals as Record<string, unknown>[]) ?? []);
    } catch (err) {
      setError(String(err));
    } finally {
      setBusy(false);
    }
  }, [baseUrl]);

  useEffect(() => {
    void load();
  }, [load]);

  const resolveMissionApproval = async (
    approvalId: string,
    missionId: string,
    approved: boolean,
  ) => {
    if (!hasToken) return;
    setBusy(true);
    setError(null);
    try {
      const res = await fetch(`${baseUrl}/v1/operator/mission/approve`, {
        method: "POST",
        headers: authHeaders(),
        body: JSON.stringify({
          approval_id: approvalId,
          mission_id: missionId,
          approved,
        }),
      });
      if (!res.ok) throw new Error(`mission approval ${res.status}`);
      await load();
    } catch (err) {
      setError(String(err));
    } finally {
      setBusy(false);
    }
  };

  return (
    <div className="cc-panel">
      {error && <div className="error">{error}</div>}

      <CcSection
        title="Human operations"
        hint="Serve with warehouse-ar/pick_mission.sd or spatial-computing programs."
        actions={
          <button type="button" onClick={() => void load()} disabled={busy}>
            Refresh
          </button>
        }
      >
        <CcMiniStats
          items={[
            { label: "Operators", value: humansList.length },
            { label: "Wearables", value: wearablesList.length },
            { label: "HRI sessions", value: hriSessions.length },
            {
              label: "Health telemetry",
              value: humanHealthPolicy?.active === true ? "opt-in active" : "gated",
            },
          ]}
        />
      </CcSection>

      <CcSection title="Human dashboard">
        <ControlCenterDataTable
          rows={humansList}
          rowKey={(row) => String(row.id)}
          emptyLabel="No humans configured"
          columns={[
            { key: "id", header: "ID", render: (row) => String(row.id) },
            { key: "role", header: "Role", render: (row) => String(row.role ?? "—") },
            { key: "name", header: "Name", render: (row) => String(row.display_name ?? "—") },
            {
              key: "avail",
              header: "Availability",
              render: (row) => String(row.availability ?? "—"),
            },
            {
              key: "caps",
              header: "Capabilities",
              render: (row) => ((row.capabilities as string[]) ?? []).join(", ") || "—",
            },
            {
              key: "wear",
              header: "Wearables",
              render: (row) => String(row.wearable_count ?? 0),
            },
          ]}
        />
      </CcSection>

      <CcSection title="Wearable inventory">
        <ControlCenterDataTable
          rows={wearablesList}
          rowKey={(row) => String(row.id)}
          emptyLabel="No wearables configured"
          columns={[
            { key: "id", header: "ID", render: (row) => String(row.id) },
            { key: "type", header: "Type", render: (row) => String(row.type ?? "—") },
            { key: "human", header: "Human", render: (row) => String(row.human_id ?? "—") },
            { key: "provider", header: "Provider", render: (row) => String(row.provider ?? "—") },
            {
              key: "health",
              header: "Health",
              render: (row) => (row.health_telemetry_allowed === true ? "allowed" : "gated"),
            },
          ]}
        />
      </CcSection>

      <CcSection title="AR sessions & live collaboration">
        <ControlCenterDataTable
          rows={hriSessions}
          rowKey={(row) => String(row.id)}
          emptyLabel="No sessions configured"
          columns={[
            { key: "id", header: "Session", render: (row) => String(row.id) },
            { key: "type", header: "Type", render: (row) => String(row.session_type ?? "—") },
            { key: "status", header: "Status", render: (row) => String(row.status ?? "—") },
            { key: "field", header: "Field", render: (row) => String(row.field_human_id ?? "—") },
            { key: "expert", header: "Expert", render: (row) => String(row.expert_human_id ?? "—") },
          ]}
        />
      </CcSection>

      <CcSection title="Team readiness">
        {teamReadiness?.team_readiness ? (
          <pre className="cc-action-result">
            {JSON.stringify(teamReadiness.team_readiness, null, 2)}
          </pre>
        ) : (
          <CcEmptyState
            title="Team readiness unavailable"
            description="Load with spatial-computing program for mission-scored rollup."
          />
        )}
      </CcSection>

      <CcSection title="Live collaboration graph">
        {collaborationGraph ? (
          <CollaborationGraph graph={collaborationGraph} />
        ) : (
          <CcEmptyState title="Collaboration graph unavailable" />
        )}
      </CcSection>

      <CcSection title="AR annotate">
        <div className="cc-action-bar">
          <input
            type="text"
            placeholder="Session id"
            value={annotateSession}
            onChange={(e) => setAnnotateSession(e.target.value)}
          />
          <input
            type="text"
            placeholder="Annotation text"
            value={annotateText}
            onChange={(e) => setAnnotateText(e.target.value)}
          />
          <button
            type="button"
            disabled={!hasToken || !annotateSession.trim() || busy}
            onClick={() => {
              void (async () => {
                setBusy(true);
                setError(null);
                try {
                  const res = await fetch(`${baseUrl}/v1/hri/annotate`, {
                    method: "POST",
                    headers: authHeaders(),
                    body: JSON.stringify({
                      session_id: annotateSession.trim(),
                      text: annotateText.trim(),
                    }),
                  });
                  if (!res.ok) throw new Error(`annotate ${res.status}`);
                  setAnnotateText("");
                  await load();
                } catch (err) {
                  setError(String(err));
                } finally {
                  setBusy(false);
                }
              })();
            }}
          >
            Post annotation
          </button>
        </div>
      </CcSection>

      <CcSection title="Context awareness (hazard zones)">
        {hriContext ? (
          <pre className="cc-action-result">{JSON.stringify(hriContext, null, 2)}</pre>
        ) : (
          <CcEmptyState title="Context snapshot unavailable" />
        )}
      </CcSection>

      <CcSection title="Human digital twins">
        <ControlCenterDataTable
          rows={humanTwins}
          rowKey={(row) => String(row.id)}
          emptyLabel="No human twins configured"
          columns={[
            { key: "id", header: "Twin", render: (row) => String(row.id) },
            { key: "entity", header: "Entity", render: (row) => String(row.entity_id) },
            {
              key: "mirror",
              header: "Mirror fields",
              render: (row) => ((row.mirror as string[]) ?? []).join(", ") || "—",
            },
            { key: "replay", header: "Replay", render: (row) => (row.replay === true ? "yes" : "no") },
          ]}
        />
      </CcSection>

      <CcSection title="Mission approval queue">
        {missionApprovals.length === 0 ? (
          <CcEmptyState title="No mission approvals" />
        ) : (
          <div className="cc-table-wrap">
            <table className="cc-data-table">
              <thead>
                <tr>
                  <th>ID</th>
                  <th>Mission</th>
                  <th>Requested by</th>
                  <th>Status</th>
                  <th>Action</th>
                </tr>
              </thead>
              <tbody>
                {missionApprovals.map((approval) => (
                  <tr key={String(approval.id)}>
                    <td>{String(approval.id)}</td>
                    <td>{String(approval.mission_id)}</td>
                    <td>{String(approval.requested_by ?? "—")}</td>
                    <td>{String(approval.status)}</td>
                    <td>
                      {approval.status === "pending" && hasToken && can("Approve") && (
                        <div className="cc-action-bar">
                          <button
                            type="button"
                            onClick={() =>
                              void resolveMissionApproval(
                                String(approval.id),
                                String(approval.mission_id),
                                true,
                              )
                            }
                            disabled={busy}
                          >
                            Approve
                          </button>
                          <button
                            type="button"
                            onClick={() =>
                              void resolveMissionApproval(
                                String(approval.id),
                                String(approval.mission_id),
                                false,
                              )
                            }
                            disabled={busy}
                          >
                            Reject
                          </button>
                        </div>
                      )}
                    </td>
                  </tr>
                ))}
              </tbody>
            </table>
          </div>
        )}
      </CcSection>

      <CcSection title="Health opt-in policy">
        {humanHealthPolicy ? (
          <pre className="cc-action-result">{JSON.stringify(humanHealthPolicy, null, 2)}</pre>
        ) : (
          <CcEmptyState
            title="Policy unavailable"
            description="Load with spatial-computing security config."
          />
        )}
      </CcSection>

      <CcSection title="VR training">
        <p className="cc-section-hint">
          Record: <code>spanda sim vr-training/training_mission.sd --record</code> · Replay:{" "}
          <code>spanda replay training_mission.trace --playback</code>
        </p>
        <a
          className="cc-inline-link"
          href={`${baseUrl}/v1/replay/training_mission.trace`}
          target="_blank"
          rel="noreferrer"
        >
          Open latest training replay manifest
        </a>
      </CcSection>
    </div>
  );
}

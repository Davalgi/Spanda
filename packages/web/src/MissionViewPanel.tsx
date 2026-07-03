import { useCallback, useEffect, useState } from "react";
import { DeployGateModal } from "./DeployGateModal";
import type { RbacAction } from "./controlCenterRbac";

type MissionRow = {
  id: string;
  name?: string;
  mission_state?: string;
  lifecycle_state?: string;
  readiness?: string;
};

type ApprovalRow = {
  id: string;
  mission_id: string;
  requested_by?: string;
  status: string;
};

type Props = {
  baseUrl: string;
  authHeaders: () => HeadersInit;
  can: (action: RbacAction) => boolean;
  hasToken: boolean;
};

export function MissionViewPanel({ baseUrl, authHeaders, can, hasToken }: Props) {
  const [missions, setMissions] = useState<MissionRow[]>([]);
  const [approvals, setApprovals] = useState<ApprovalRow[]>([]);
  const [pendingCount, setPendingCount] = useState(0);
  const [busy, setBusy] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [showDeployGate, setShowDeployGate] = useState(false);

  const load = useCallback(async () => {
    setBusy(true);
    setError(null);
    try {
      const [missionsRes, approvalsRes] = await Promise.all([
        fetch(`${baseUrl}/v1/operator/missions`),
        fetch(`${baseUrl}/v1/operator/mission/approvals`),
      ]);
      if (missionsRes.ok) {
        const body = await missionsRes.json();
        setMissions(body.missions ?? []);
        setPendingCount(body.pending_approvals ?? 0);
      }
      if (approvalsRes.ok) {
        const body = await approvalsRes.json();
        setApprovals(body.approvals ?? []);
      }
    } catch (e) {
      setError(String(e));
    } finally {
      setBusy(false);
    }
  }, [baseUrl]);

  useEffect(() => {
    void load();
  }, [load]);

  const approveMission = async (approvalId: string, missionId: string, approved: boolean) => {
    if (!hasToken || !can("Approve")) return;
    setBusy(true);
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
      if (!res.ok) throw new Error(`approve ${res.status}`);
      await load();
    } catch (e) {
      setError(String(e));
    } finally {
      setBusy(false);
    }
  };

  const missionControl = async (path: string, missionId: string) => {
    if (!hasToken) return;
    setBusy(true);
    try {
      const res = await fetch(`${baseUrl}${path}`, {
        method: "POST",
        headers: authHeaders(),
        body: JSON.stringify({ mission_id: missionId }),
      });
      if (!res.ok) throw new Error(`${path} ${res.status}`);
      await load();
    } catch (e) {
      setError(String(e));
    } finally {
      setBusy(false);
    }
  };

  return (
    <section className="cc-mission-panel">
      <header>
        <h3>Mission View</h3>
        <button type="button" onClick={() => void load()} disabled={busy}>
          Refresh
        </button>
        <button type="button" onClick={() => setShowDeployGate(true)}>
          Deploy gate
        </button>
      </header>
      {error && <p className="error">{error}</p>}
      <p className="demo-hint">Pending approvals: {pendingCount}</p>

      <h4>Active missions</h4>
      <table>
        <thead>
          <tr>
            <th>ID</th>
            <th>Name</th>
            <th>State</th>
            <th>Lifecycle</th>
            <th>Actions</th>
          </tr>
        </thead>
        <tbody>
          {missions.map((mission) => {
            const missionKey = mission.name ?? mission.id;
            return (
              <tr key={mission.id}>
                <td>
                  <code>{mission.id}</code>
                </td>
                <td>{mission.name ?? "—"}</td>
                <td>{mission.mission_state ?? "—"}</td>
                <td>{mission.lifecycle_state ?? "—"}</td>
                <td className="cc-action-bar">
                  {can("Operate") && (
                    <>
                      <button
                        type="button"
                        disabled={busy || !hasToken}
                        onClick={() =>
                          void missionControl("/v1/operator/mission/pause", missionKey)
                        }
                      >
                        Pause
                      </button>
                      <button
                        type="button"
                        disabled={busy || !hasToken}
                        onClick={() =>
                          void missionControl("/v1/operator/mission/resume", missionKey)
                        }
                      >
                        Resume
                      </button>
                    </>
                  )}
                  {can("Shutdown") && (
                    <button
                      type="button"
                      disabled={busy || !hasToken}
                      onClick={() =>
                        void missionControl("/v1/operator/mission/cancel", missionKey)
                      }
                    >
                      Cancel
                    </button>
                  )}
                </td>
              </tr>
            );
          })}
          {missions.length === 0 && (
            <tr>
              <td colSpan={5}>No mission entities — load a program with missions.</td>
            </tr>
          )}
        </tbody>
      </table>

      <h4>Approval queue</h4>
      <table>
        <thead>
          <tr>
            <th>ID</th>
            <th>Mission</th>
            <th>Requested by</th>
            <th>Status</th>
            <th>Actions</th>
          </tr>
        </thead>
        <tbody>
          {approvals.map((approval) => (
            <tr key={approval.id}>
              <td>{approval.id}</td>
              <td>{approval.mission_id}</td>
              <td>{approval.requested_by ?? "—"}</td>
              <td>{approval.status}</td>
              <td>
                {approval.status === "pending" && can("Approve") && hasToken && (
                  <>
                    <button
                      type="button"
                      disabled={busy}
                      onClick={() =>
                        void approveMission(approval.id, approval.mission_id, true)
                      }
                    >
                      Approve
                    </button>
                    <button
                      type="button"
                      disabled={busy}
                      onClick={() =>
                        void approveMission(approval.id, approval.mission_id, false)
                      }
                    >
                      Reject
                    </button>
                  </>
                )}
              </td>
            </tr>
          ))}
          {approvals.length === 0 && (
            <tr>
              <td colSpan={5}>No mission approvals</td>
            </tr>
          )}
        </tbody>
      </table>
      <DeployGateModal
        baseUrl={baseUrl}
        open={showDeployGate}
        onClose={() => setShowDeployGate(false)}
      />
    </section>
  );
}

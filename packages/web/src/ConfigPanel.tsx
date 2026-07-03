import { useCallback, useEffect, useState } from "react";
import { DeployGateModal } from "./DeployGateModal";
import type { RbacAction } from "./controlCenterRbac";
import { CcBadge, CcEmptyState, CcMiniStats, CcSection } from "./controlCenterUi";
import { useRegisterTabRefresh } from "./useControlCenterTabRefresh";

type ApprovalRow = {
  id: string;
  snapshot_id: string;
  status: string;
  note?: string;
};

type Props = {
  baseUrl: string;
  authHeaders: () => HeadersInit;
  can: (action: RbacAction) => boolean;
  hasToken: boolean;
};

function approvalTone(status: string): "ok" | "warn" | "danger" | "neutral" {
  const normalized = status.toLowerCase();
  if (normalized === "approved") return "ok";
  if (normalized === "pending") return "warn";
  if (normalized === "rejected") return "danger";
  return "neutral";
}

export function ConfigPanel({ baseUrl, authHeaders, can, hasToken }: Props) {
  const [approvals, setApprovals] = useState<ApprovalRow[]>([]);
  const [history, setHistory] = useState<Record<string, unknown>[]>([]);
  const [deployGate, setDeployGate] = useState<Record<string, unknown> | null>(null);
  const [showDeployGate, setShowDeployGate] = useState(false);
  const [busy, setBusy] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const load = useCallback(async () => {
    setBusy(true);
    setError(null);
    try {
      const [approvalsRes, historyRes, gateRes] = await Promise.all([
        fetch(`${baseUrl}/v1/config/approvals`),
        fetch(`${baseUrl}/v1/config/history`),
        fetch(`${baseUrl}/v1/deploy/gate`),
      ]);
      if (!approvalsRes.ok) throw new Error(`approvals ${approvalsRes.status}`);
      const body = await approvalsRes.json();
      const rows = (body.approvals ?? []) as Record<string, unknown>[];
      setApprovals(
        rows.map((row) => ({
          id: String(row.id ?? ""),
          snapshot_id: String(row.snapshot_id ?? ""),
          status: String(row.status ?? "unknown"),
          note: row.note ? String(row.note) : undefined,
        })),
      );
      if (historyRes.ok) {
        const historyBody = await historyRes.json();
        setHistory((historyBody.history as Record<string, unknown>[]) ?? []);
      }
      if (gateRes.ok) {
        setDeployGate(await gateRes.json());
      }
    } catch (err) {
      setError(String(err));
    } finally {
      setBusy(false);
    }
  }, [baseUrl]);

  useEffect(() => {
    void load();
  }, [load]);

  useRegisterTabRefresh(load, { busy });

  const resolve = async (approvalId: string, approve: boolean) => {
    if (!hasToken || !can("Approve")) return;
    setBusy(true);
    setError(null);
    try {
      const action = approve ? "approve" : "reject";
      const res = await fetch(
        `${baseUrl}/v1/config/approvals/${encodeURIComponent(approvalId)}/${action}`,
        { method: "POST", headers: authHeaders(), body: JSON.stringify({}) },
      );
      if (!res.ok) throw new Error(`approval ${action} ${res.status}`);
      await load();
    } catch (err) {
      setError(String(err));
    } finally {
      setBusy(false);
    }
  };

  const pending = approvals.filter((row) => row.status.toLowerCase() === "pending").length;

  return (
    <div className="cc-panel">
      {error && <div className="error">{error}</div>}

      <CcMiniStats
        items={[
          { label: "Total requests", value: approvals.length },
          { label: "Pending", value: pending, tone: pending > 0 ? "warn" : "ok" },
          {
            label: "Approved",
            value: approvals.filter((row) => row.status.toLowerCase() === "approved").length,
            tone: "ok",
          },
        ]}
      />

      <CcSection
        title="Publish approval queue"
        hint="Review and approve configuration snapshots before they go live."
        actions={
          <button type="button" onClick={() => setShowDeployGate(true)}>
            Deploy gate
          </button>
        }
      >
        {!hasToken && (
          <CcEmptyState
            title="Sign in to manage approvals"
            description="Approving or rejecting publish requests requires a Bearer token with Approve permission."
          />
        )}

        {busy && approvals.length === 0 ? (
          <CcEmptyState title="Loading approvals…" />
        ) : approvals.length === 0 ? (
          <CcEmptyState
            title="No approval requests"
            description="Requests appear when an operator submits a config snapshot for publish review."
          />
        ) : (
          <ul className="cc-approval-list">
            {approvals.map((approval) => (
              <li key={approval.id} className="cc-approval-item">
                <div className="cc-approval-header">
                  <code className="cc-approval-id">{approval.id}</code>
                  <CcBadge tone={approvalTone(approval.status)}>{approval.status}</CcBadge>
                </div>
                <p className="cc-approval-meta">
                  Snapshot <code>{approval.snapshot_id}</code>
                  {approval.note ? ` — ${approval.note}` : ""}
                </p>
                {approval.status.toLowerCase() === "pending" && hasToken && can("Approve") && (
                  <div className="cc-approval-actions">
                    <button
                      type="button"
                      className="primary"
                      onClick={() => void resolve(approval.id, true)}
                      disabled={busy}
                    >
                      Approve
                    </button>
                    <button
                      type="button"
                      onClick={() => void resolve(approval.id, false)}
                      disabled={busy}
                    >
                      Reject
                    </button>
                  </div>
                )}
              </li>
            ))}
          </ul>
        )}
      </CcSection>

      <CcSection title="Config change history" hint="Snapshots and audit-linked config mutations.">
        {history.length === 0 ? (
          <CcEmptyState title="No config history" />
        ) : (
          <ul className="cc-event-log">
            {history.slice(0, 20).map((entry, index) => (
              <li key={`${entry.id ?? index}`}>
                <span className="cc-event-type">{String(entry.action ?? "change")}</span>
                {String(entry.snapshot_id ?? entry.id ?? "")} — {String(entry.timestamp ?? "—")}
              </li>
            ))}
          </ul>
        )}
      </CcSection>

      {deployGate && (
        <CcSection title="Deploy gate summary">
          <pre className="cc-action-result">{JSON.stringify(deployGate, null, 2)}</pre>
        </CcSection>
      )}
      <DeployGateModal
        baseUrl={baseUrl}
        open={showDeployGate}
        onClose={() => setShowDeployGate(false)}
      />
    </div>
  );
}

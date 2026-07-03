import { useCallback, useEffect, useState } from "react";
import type { RbacAction } from "./controlCenterRbac";
import { CcBadge, CcEmptyState, CcMiniStats, CcSection } from "./controlCenterUi";

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
  const [busy, setBusy] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const load = useCallback(async () => {
    setBusy(true);
    setError(null);
    try {
      const res = await fetch(`${baseUrl}/v1/config/approvals`);
      if (!res.ok) throw new Error(`approvals ${res.status}`);
      const body = await res.json();
      const rows = (body.approvals ?? []) as Record<string, unknown>[];
      setApprovals(
        rows.map((row) => ({
          id: String(row.id ?? ""),
          snapshot_id: String(row.snapshot_id ?? ""),
          status: String(row.status ?? "unknown"),
          note: row.note ? String(row.note) : undefined,
        })),
      );
    } catch (err) {
      setError(String(err));
    } finally {
      setBusy(false);
    }
  }, [baseUrl]);

  useEffect(() => {
    void load();
  }, [load]);

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
    </div>
  );
}

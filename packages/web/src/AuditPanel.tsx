import { useCallback, useEffect, useState } from "react";
import type { RbacAction } from "./controlCenterRbac";
import { CcEmptyState, CcMiniStats, CcSection } from "./controlCenterUi";
import { useRegisterTabRefresh } from "./useControlCenterTabRefresh";

type AuditRecord = Record<string, unknown>;

type Props = {
  baseUrl: string;
  authHeaders: () => HeadersInit;
  can: (action: RbacAction) => boolean;
  hasToken: boolean;
};

export function AuditPanel({ baseUrl, authHeaders, can, hasToken }: Props) {
  const [records, setRecords] = useState<AuditRecord[]>([]);
  const [recordCount, setRecordCount] = useState(0);
  const [persistPath, setPersistPath] = useState("");
  const [busy, setBusy] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const load = useCallback(async () => {
    if (!hasToken || !can("Deploy")) return;
    setBusy(true);
    setError(null);
    try {
      const res = await fetch(`${baseUrl}/v1/audit/mutations`, { headers: authHeaders() });
      if (!res.ok) throw new Error(`audit ${res.status}`);
      const body = await res.json();
      const audit = body.audit as Record<string, unknown> | undefined;
      const rows = Array.isArray(audit?.records)
        ? (audit.records as AuditRecord[])
        : Array.isArray(audit)
          ? (audit as AuditRecord[])
          : [];
      setRecords(rows.slice().reverse().slice(0, 50));
      setRecordCount(Number(body.record_count ?? rows.length));
      setPersistPath(String(body.persist_path ?? ""));
    } catch (err) {
      setError(String(err));
    } finally {
      setBusy(false);
    }
  }, [authHeaders, baseUrl, can, hasToken]);

  useEffect(() => {
    void load();
  }, [load]);

  useRegisterTabRefresh(load, { busy });

  return (
    <div className="cc-panel">
      {error && <div className="error">{error}</div>}

      {!hasToken ? (
        <CcEmptyState
          title="Sign in to view audit trail"
          description="Mutation audit requires a Bearer token with Deploy permission."
        />
      ) : (
        <>
          <CcMiniStats
            items={[
              { label: "Total records", value: recordCount },
              { label: "Shown", value: records.length },
            ]}
          />

          {persistPath && (
            <p className="cc-section-hint">
              Persist path: <code>{persistPath}</code>
            </p>
          )}

          <CcSection
            title="Mutation audit trail"
            hint="Platform mutations recorded for compliance and forensics."
          >
            {busy && records.length === 0 ? (
              <CcEmptyState title="Loading audit records…" />
            ) : records.length === 0 ? (
              <CcEmptyState
                title="No mutations recorded"
                description="API mutations with audit logging enabled will appear here."
              />
            ) : (
              <div className="cc-table-wrap">
                <table className="cc-data-table">
                  <thead>
                    <tr>
                      <th>Time</th>
                      <th>Action</th>
                      <th>Actor</th>
                      <th>Target</th>
                    </tr>
                  </thead>
                  <tbody>
                    {records.map((row, index) => (
                      <tr key={index}>
                        <td>{String(row.timestamp ?? row.ts ?? row.time ?? "—")}</td>
                        <td>{String(row.action ?? row.method ?? row.event ?? "—")}</td>
                        <td>{String(row.actor ?? row.key_id ?? row.user ?? "—")}</td>
                        <td>{String(row.target ?? row.path ?? row.resource ?? "—")}</td>
                      </tr>
                    ))}
                  </tbody>
                </table>
              </div>
            )}
          </CcSection>
        </>
      )}
    </div>
  );
}

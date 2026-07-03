import { useCallback, useEffect, useMemo, useState } from "react";
import { CcBadge, CcEmptyState, CcMiniStats, CcSection, trustTone } from "./controlCenterUi";
import { useRegisterTabRefresh } from "./useControlCenterTabRefresh";

type TrustFactor = {
  name?: string;
  score?: number;
  max_score?: number;
  passed?: boolean;
  detail?: string;
};

type Props = {
  baseUrl: string;
};

export function SecurityPanel({ baseUrl }: Props) {
  const [packageName, setPackageName] = useState("spanda-mqtt");
  const [trustReport, setTrustReport] = useState<Record<string, unknown> | null>(null);
  const [rbacMatrix, setRbacMatrix] = useState<Record<string, string[]> | null>(null);
  const [busy, setBusy] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const loadMatrix = useCallback(async () => {
    try {
      const res = await fetch(`${baseUrl}/v1/rbac/matrix`);
      if (!res.ok) throw new Error(`rbac ${res.status}`);
      const body = await res.json();
      setRbacMatrix((body.matrix as Record<string, string[]>) ?? null);
    } catch (err) {
      setError(String(err));
    }
  }, [baseUrl]);

  const evaluateTrust = useCallback(async () => {
    setBusy(true);
    setError(null);
    try {
      const res = await fetch(
        `${baseUrl}/v1/trust/package?name=${encodeURIComponent(packageName)}`,
      );
      if (!res.ok) throw new Error(`trust ${res.status}`);
      setTrustReport(await res.json());
    } catch (err) {
      setError(String(err));
    } finally {
      setBusy(false);
    }
  }, [baseUrl, packageName]);

  const refresh = useCallback(async () => {
    await loadMatrix();
    await evaluateTrust();
  }, [evaluateTrust, loadMatrix]);

  useEffect(() => {
    void loadMatrix();
  }, [loadMatrix]);

  useEffect(() => {
    void evaluateTrust();
  }, [evaluateTrust]);

  useRegisterTabRefresh(refresh, { busy });

  const score = Number(trustReport?.score ?? 0);
  const maxScore = Number(trustReport?.max_score ?? 100);
  const tier = String(trustReport?.tier ?? "unknown");
  const factors = (trustReport?.factors as TrustFactor[] | undefined) ?? [];

  const matrixRows = useMemo(() => {
    if (!rbacMatrix) return [];
    return Object.entries(rbacMatrix).sort(([left], [right]) => left.localeCompare(right));
  }, [rbacMatrix]);

  return (
    <div className="cc-panel">
      {error && <div className="error">{error}</div>}

      <CcSection
        title="Package trust"
        hint="Evaluate registry package trust score and factor breakdown."
        actions={
          <div className="cc-filter-bar">
            <input
              value={packageName}
              onChange={(event) => setPackageName(event.target.value)}
              placeholder="package name"
              aria-label="Package name"
            />
            <button type="button" className="primary" onClick={() => void evaluateTrust()} disabled={busy}>
              {busy ? "Evaluating…" : "Evaluate trust"}
            </button>
          </div>
        }
      >
        {!trustReport ? (
          <CcEmptyState
            title="No trust evaluation yet"
            description="Enter a package name and run evaluation to see score and factors."
          />
        ) : (
          <>
            <CcMiniStats
              items={[
                { label: "Package", value: String(trustReport.package ?? packageName) },
                {
                  label: "Score",
                  value: `${score}/${maxScore}`,
                  tone: score >= maxScore * 0.8 ? "ok" : score >= maxScore * 0.5 ? "warn" : "danger",
                },
                { label: "Tier", value: tier, tone: trustTone(tier) },
              ]}
            />
            {factors.length > 0 && (
              <div className="cc-table-wrap">
                <table className="cc-data-table">
                  <thead>
                    <tr>
                      <th>Factor</th>
                      <th>Score</th>
                      <th>Status</th>
                      <th>Detail</th>
                    </tr>
                  </thead>
                  <tbody>
                    {factors.map((factor) => (
                      <tr key={factor.name ?? factor.detail}>
                        <td>{factor.name ?? "—"}</td>
                        <td>
                          {factor.score ?? 0}/{factor.max_score ?? 0}
                        </td>
                        <td>
                          <CcBadge tone={factor.passed ? "ok" : "danger"}>
                            {factor.passed ? "pass" : "fail"}
                          </CcBadge>
                        </td>
                        <td className="cc-factor-detail">{factor.detail ?? "—"}</td>
                      </tr>
                    ))}
                  </tbody>
                </table>
              </div>
            )}
          </>
        )}
      </CcSection>

      <CcSection title="RBAC permission matrix" hint="Role → allowed mutation actions.">
        {matrixRows.length === 0 ? (
          <CcEmptyState title="Matrix unavailable" />
        ) : (
          <div className="cc-table-wrap">
            <table className="cc-data-table">
              <thead>
                <tr>
                  <th>Role</th>
                  <th>Permissions</th>
                </tr>
              </thead>
              <tbody>
                {matrixRows.map(([role, permissions]) => (
                  <tr key={role}>
                    <td>{role.replace(/^Role::/, "")}</td>
                    <td>
                      <div className="cc-perm-tags">
                        {permissions.map((perm) => (
                          <span key={perm} className="cc-perm-tag ok">
                            {perm.replace(/^RbacAction::/, "")}
                          </span>
                        ))}
                      </div>
                    </td>
                  </tr>
                ))}
              </tbody>
            </table>
          </div>
        )}
      </CcSection>
    </div>
  );
}

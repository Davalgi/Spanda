import { useCallback, useEffect, useState } from "react";
import type { RbacAction } from "./controlCenterRbac";
import { CcBadge, CcEmptyState, CcMiniStats, CcSection } from "./controlCenterUi";
import { useRegisterTabRefresh } from "./useControlCenterTabRefresh";

type ProfileRow = {
  kind?: string;
  display_name?: string;
  default_risk_level?: string;
  max_autonomy_level?: string;
};

type Props = {
  baseUrl: string;
  authHeaders: () => HeadersInit;
  can: (action: RbacAction) => boolean;
  hasToken: boolean;
};

export function GovernancePanel({ baseUrl, authHeaders, can, hasToken }: Props) {
  const [framework, setFramework] = useState<Record<string, unknown> | null>(null);
  const [profiles, setProfiles] = useState<ProfileRow[]>([]);
  const [compliance, setCompliance] = useState<Record<string, unknown> | null>(null);
  const [validation, setValidation] = useState<Record<string, unknown> | null>(null);
  const [certifications, setCertifications] = useState<Record<string, unknown>[]>([]);
  const [risks, setRisks] = useState<Record<string, unknown>[]>([]);
  const [busy, setBusy] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const load = useCallback(async () => {
    try {
      const [govRes, profileRes] = await Promise.all([
        fetch(`${baseUrl}/v1/governance`),
        fetch(`${baseUrl}/v1/deployment-profiles`),
      ]);
      if (!govRes.ok) throw new Error(`governance ${govRes.status}`);
      if (!profileRes.ok) throw new Error(`profiles ${profileRes.status}`);
      const govBody = await govRes.json();
      const profileBody = await profileRes.json();
      setFramework(govBody);
      setProfiles((profileBody.profiles as ProfileRow[]) ?? []);
    } catch (err) {
      setError(String(err));
    }
  }, [baseUrl]);

  const loadAuthenticated = useCallback(async () => {
    if (!hasToken) return;
    try {
      const headers = authHeaders();
      const [compRes, certRes, riskRes] = await Promise.all([
        fetch(`${baseUrl}/v1/compliance`, { headers }),
        fetch(`${baseUrl}/v1/certifications`, { headers }),
        fetch(`${baseUrl}/v1/risk`, { headers }),
      ]);
      if (compRes.ok) setCompliance(await compRes.json());
      if (certRes.ok) {
        const body = await certRes.json();
        setCertifications((body.certifications as Record<string, unknown>[]) ?? []);
      }
      if (riskRes.ok) {
        const body = await riskRes.json();
        setRisks((body.entities as Record<string, unknown>[]) ?? []);
      }
    } catch (err) {
      setError(String(err));
    }
  }, [authHeaders, baseUrl, hasToken]);

  useEffect(() => {
    void load();
    void loadAuthenticated();
  }, [load, loadAuthenticated]);

  useRegisterTabRefresh(async () => {
    await load();
    await loadAuthenticated();
  }, { busy });

  const runValidation = async () => {
    if (!hasToken || !can("Operate")) return;
    setBusy(true);
    setError(null);
    try {
      const res = await fetch(`${baseUrl}/v1/governance/validate`, {
        method: "POST",
        headers: { ...authHeaders(), "Content-Type": "application/json" },
        body: JSON.stringify({}),
      });
      if (!res.ok) throw new Error(`validate ${res.status}`);
      const body = await res.json();
      setValidation(body.validation as Record<string, unknown>);
      setCompliance(body.compliance as Record<string, unknown>);
    } catch (err) {
      setError(String(err));
    } finally {
      setBusy(false);
    }
  };

  const summary = compliance?.summary as Record<string, number> | undefined;
  const validationPassed = validation?.passed as boolean | undefined;

  return (
    <div className="cc-panel-stack">
      <CcSection title="Operational Governance" hint="Deployment profiles, compliance, certification, risk, and human accountability.">
        <p className="hint">
          Spanda provides governance abstractions and validation — not legal or regulatory advice.
        </p>
        <CcMiniStats
          items={[
            { label: "Profiles", value: String(profiles.length) },
            { label: "Certifications", value: String(certifications.length) },
            { label: "Risk entities", value: String(risks.length) },
            {
              label: "Compliance",
              value: summary ? (compliance?.passed ? "PASS" : "FAIL") : "—",
            },
          ]}
        />
      </CcSection>

      {error ? <p className="error">{error}</p> : null}

      <CcSection title="Framework">
        {framework ? (
          <pre className="cc-json">{JSON.stringify(framework.capabilities ?? framework, null, 2)}</pre>
        ) : (
          <CcEmptyState message="Loading governance framework…" />
        )}
      </CcSection>

      <CcSection title="Deployment profiles">
        {profiles.length ? (
          <table className="cc-table">
            <thead>
              <tr>
                <th>Profile</th>
                <th>Risk</th>
                <th>Max autonomy</th>
              </tr>
            </thead>
            <tbody>
              {profiles.map((row) => (
                <tr key={row.kind}>
                  <td>{row.display_name ?? row.kind}</td>
                  <td><CcBadge tone="warn">{row.default_risk_level ?? "—"}</CcBadge></td>
                  <td>{row.max_autonomy_level ?? "—"}</td>
                </tr>
              ))}
            </tbody>
          </table>
        ) : (
          <CcEmptyState message="No deployment profiles loaded." />
        )}
      </CcSection>

      <CcSection title="Validation">
        <button type="button" disabled={busy || !hasToken} onClick={() => void runValidation()}>
          Run governance validate
        </button>
        {validation ? (
          <p>
            Result: <CcBadge tone={validationPassed ? "ok" : "bad"}>{validationPassed ? "PASS" : "FAIL"}</CcBadge>
            {" "}
            Profile: {String(validation.deployment_profile ?? "—")}
            {" · "}
            Maturity: {String(validation.operational_maturity ?? "—")}
          </p>
        ) : null}
      </CcSection>

      <CcSection title="Certification">
        {certifications.length ? (
          <table className="cc-table">
            <thead>
              <tr><th>Entity</th><th>Status</th><th>Profile</th></tr>
            </thead>
            <tbody>
              {certifications.map((row) => (
                <tr key={String(row.entity_id)}>
                  <td>{String(row.entity_id)}</td>
                  <td>{String(row.certification_status ?? "—")}</td>
                  <td>{String(row.deployment_profile ?? "—")}</td>
                </tr>
              ))}
            </tbody>
          </table>
        ) : (
          <CcEmptyState message="No certification records. Configure spanda.governance.toml." />
        )}
      </CcSection>

      <CcSection title="Operational risk">
        {risks.length ? (
          <table className="cc-table">
            <thead>
              <tr><th>Entity</th><th>Risk</th><th>Autonomy</th><th>Health</th></tr>
            </thead>
            <tbody>
              {risks.map((row) => (
                <tr key={String(row.entity_id)}>
                  <td>{String(row.entity_id)}</td>
                  <td><CcBadge tone="warn">{String(row.risk_level ?? "—")}</CcBadge></td>
                  <td>{String(row.autonomy_level ?? "—")}</td>
                  <td>{String(row.health_status ?? "—")}</td>
                </tr>
              ))}
            </tbody>
          </table>
        ) : (
          <CcEmptyState message="No risk metadata on entities." />
        )}
      </CcSection>
    </div>
  );
}

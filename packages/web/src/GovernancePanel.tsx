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
  const [deploymentVerify, setDeploymentVerify] = useState<Record<string, unknown> | null>(null);
  const [certifications, setCertifications] = useState<Record<string, unknown>[]>([]);
  const [risks, setRisks] = useState<Record<string, unknown>[]>([]);
  const [accountability, setAccountability] = useState<Record<string, unknown>[]>([]);
  const [policies, setPolicies] = useState<Record<string, unknown>[]>([]);
  const [audit, setAudit] = useState<Record<string, unknown>[]>([]);
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
      setFramework(await govRes.json());
      const profileBody = await profileRes.json();
      setProfiles((profileBody.profiles as ProfileRow[]) ?? []);
    } catch (err) {
      setError(String(err));
    }
  }, [baseUrl]);

  const loadAuthenticated = useCallback(async () => {
    if (!hasToken) return;
    try {
      const headers = authHeaders();
      const [compRes, certRes, riskRes, acctRes, polRes, auditRes] = await Promise.all([
        fetch(`${baseUrl}/v1/compliance`, { headers }),
        fetch(`${baseUrl}/v1/certifications`, { headers }),
        fetch(`${baseUrl}/v1/risk`, { headers }),
        fetch(`${baseUrl}/v1/governance/accountability`, { headers }),
        fetch(`${baseUrl}/v1/governance/policies`, { headers }),
        fetch(`${baseUrl}/v1/governance/audit`, { headers }),
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
      if (acctRes.ok) {
        const body = await acctRes.json();
        setAccountability((body.entities as Record<string, unknown>[]) ?? []);
      }
      if (polRes.ok) {
        const body = await polRes.json();
        setPolicies((body.assignments as Record<string, unknown>[]) ?? []);
        setAudit((body.audit as Record<string, unknown>[]) ?? []);
      }
      if (auditRes.ok) {
        const body = await auditRes.json();
        if (Array.isArray(body.policy_audit)) {
          setAudit(body.policy_audit as Record<string, unknown>[]);
        }
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

  const runDeploymentVerify = async () => {
    if (!hasToken || !can("Operate")) return;
    setBusy(true);
    setError(null);
    try {
      const res = await fetch(`${baseUrl}/v1/deployment/verify`, {
        method: "POST",
        headers: { ...authHeaders(), "Content-Type": "application/json" },
        body: JSON.stringify({}),
      });
      if (!res.ok) throw new Error(`deployment verify ${res.status}`);
      const body = await res.json();
      setDeploymentVerify(body.report as Record<string, unknown>);
    } catch (err) {
      setError(String(err));
    } finally {
      setBusy(false);
    }
  };

  const summary = compliance?.summary as Record<string, number> | undefined;
  const validationPassed = validation?.passed as boolean | undefined;
  const deployPassed = deploymentVerify?.passed as boolean | undefined;

  return (
    <div className="cc-panel-stack">
      <CcSection title="Operational Governance" hint="Deployment profiles, compliance, certification, risk, policies, and human accountability.">
        <p className="hint">
          Spanda provides governance abstractions and validation — not legal or regulatory advice.
        </p>
        <CcMiniStats
          items={[
            { label: "Profiles", value: String(profiles.length) },
            { label: "Certifications", value: String(certifications.length) },
            { label: "Owners", value: String(accountability.length) },
            {
              label: "Compliance",
              value: summary ? (compliance?.passed ? "PASS" : "FAIL") : "—",
            },
          ]}
        />
      </CcSection>

      {error ? <p className="error">{error}</p> : null}

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

      <CcSection title="Validation & deployment verify">
        <button type="button" disabled={busy || !hasToken} onClick={() => void runValidation()}>
          Run governance validate
        </button>{" "}
        <button type="button" disabled={busy || !hasToken} onClick={() => void runDeploymentVerify()}>
          Run deployment verify
        </button>
        {validation ? (
          <p>
            Governance: <CcBadge tone={validationPassed ? "ok" : "bad"}>{validationPassed ? "PASS" : "FAIL"}</CcBadge>
            {" · "}
            Profile: {String(validation.deployment_profile ?? "—")}
            {" · "}
            Maturity: {String(validation.operational_maturity ?? "—")}
          </p>
        ) : null}
        {deploymentVerify ? (
          <p>
            Deployment: <CcBadge tone={deployPassed ? "ok" : "bad"}>{deployPassed ? "PASS" : "FAIL"}</CcBadge>
            {" · "}
            Profile: {String(deploymentVerify.deployment_profile ?? "—")}
          </p>
        ) : null}
      </CcSection>

      <CcSection title="Operational maturity & certification">
        {certifications.length ? (
          <table className="cc-table">
            <thead>
              <tr><th>Entity</th><th>Maturity</th><th>Certification</th><th>Profile</th></tr>
            </thead>
            <tbody>
              {certifications.map((row) => (
                <tr key={String(row.entity_id)}>
                  <td>{String(row.entity_id)}</td>
                  <td>{String(row.operational_maturity ?? "—")}</td>
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

      <CcSection title="Responsible owners">
        {accountability.length ? (
          <table className="cc-table">
            <thead>
              <tr><th>Entity</th><th>Responsible</th><th>Deployment owner</th><th>Emergency</th><th>Approval chain</th></tr>
            </thead>
            <tbody>
              {accountability.map((row) => (
                <tr key={String(row.entity_id)}>
                  <td>{String(row.entity_id)}</td>
                  <td>{String(row.responsible_person ?? "—")}</td>
                  <td>{String(row.deployment_owner ?? "—")}</td>
                  <td>{String(row.emergency_contact ?? "—")}</td>
                  <td>{Array.isArray(row.approval_chain) ? (row.approval_chain as string[]).join(" → ") : "—"}</td>
                </tr>
              ))}
            </tbody>
          </table>
        ) : (
          <CcEmptyState message="No accountability metadata on entities." />
        )}
      </CcSection>

      <CcSection title="Applicable policies">
        {policies.length ? (
          <table className="cc-table">
            <thead>
              <tr><th>Entity</th><th>Policy</th><th>Version</th><th>Signed</th></tr>
            </thead>
            <tbody>
              {policies.map((row) => {
                const policy = (row.policy as Record<string, unknown> | undefined) ?? {};
                return (
                  <tr key={String(row.id)}>
                    <td>{String(row.entity_id)}</td>
                    <td>{String(policy.name ?? "—")}</td>
                    <td>{String(policy.version ?? "—")}</td>
                    <td>{policy.signature ? "yes" : "no"}</td>
                  </tr>
                );
              })}
            </tbody>
          </table>
        ) : (
          <CcEmptyState message="No policy assignments. Use POST /v1/governance/policies/assign." />
        )}
      </CcSection>

      <CcSection title="Audit history">
        {audit.length ? (
          <table className="cc-table">
            <thead>
              <tr><th>When</th><th>Action</th><th>Entity</th><th>Policy</th><th>Actor</th></tr>
            </thead>
            <tbody>
              {audit.map((row) => (
                <tr key={String(row.id)}>
                  <td>{String(row.at ?? "—")}</td>
                  <td>{String(row.action ?? "—")}</td>
                  <td>{String(row.entity_id ?? "—")}</td>
                  <td>{String(row.policy_name ?? "—")}</td>
                  <td>{String(row.actor ?? "—")}</td>
                </tr>
              ))}
            </tbody>
          </table>
        ) : (
          <CcEmptyState message="No governance policy audit entries yet." />
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

      <CcSection title="Framework">
        {framework ? (
          <pre className="cc-json">{JSON.stringify(framework.capabilities ?? framework, null, 2)}</pre>
        ) : (
          <CcEmptyState message="Loading governance framework…" />
        )}
      </CcSection>
    </div>
  );
}

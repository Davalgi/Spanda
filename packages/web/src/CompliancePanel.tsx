import { useCallback, useEffect, useState } from "react";
import type { RbacAction } from "./controlCenterRbac";
import { CcBadge, CcEmptyState, CcMiniStats, CcSection } from "./controlCenterUi";
import { useRegisterTabRefresh } from "./useControlCenterTabRefresh";

type Profile = {
  name: string;
  description?: string;
  verified?: boolean;
};

type EvidenceRow = Record<string, unknown>;

type Props = {
  baseUrl: string;
  authHeaders: () => HeadersInit;
  can: (action: RbacAction) => boolean;
  hasToken: boolean;
};

export function CompliancePanel({ baseUrl, authHeaders, can, hasToken }: Props) {
  const [profiles, setProfiles] = useState<Profile[]>([]);
  const [profile, setProfile] = useState("defense");
  const [exportData, setExportData] = useState<Record<string, unknown> | null>(null);
  const [evidence, setEvidence] = useState<EvidenceRow[]>([]);
  const [busy, setBusy] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const loadProfiles = useCallback(async () => {
    try {
      const res = await fetch(`${baseUrl}/v1/compliance/profiles`);
      if (!res.ok) throw new Error(`profiles ${res.status}`);
      const body = await res.json();
      const signed = Array.isArray(body.signed_catalog) ? body.signed_catalog : [];
      const builtin = Array.isArray(body.profiles) ? body.profiles : [];
      const merged: Profile[] = signed.length
        ? signed.map((entry: Profile) => ({
            name: entry.name,
            description: entry.description,
            verified: entry.verified,
          }))
        : builtin.map((name: string) => ({ name }));
      setProfiles(merged);
      if (merged.length) {
        setProfile((current) =>
          merged.some((entry) => entry.name === current) ? current : (merged[0]?.name ?? "defense"),
        );
      }
    } catch (err) {
      setError(String(err));
    }
  }, [baseUrl]);

  useEffect(() => {
    void loadProfiles();
  }, [loadProfiles]);

  useRegisterTabRefresh(loadProfiles, { busy });

  const exportProfile = async () => {
    if (!hasToken || !can("Deploy")) return;
    setBusy(true);
    setError(null);
    try {
      const [exportRes, evidenceRes] = await Promise.all([
        fetch(`${baseUrl}/v1/compliance/export?profile=${encodeURIComponent(profile)}`, {
          headers: authHeaders(),
        }),
        fetch(`${baseUrl}/v1/compliance/evidence`, { headers: authHeaders() }),
      ]);
      if (!exportRes.ok) throw new Error(`compliance export ${exportRes.status}`);
      setExportData(await exportRes.json());
      if (evidenceRes.ok) {
        const evidenceBody = await evidenceRes.json();
        setEvidence((evidenceBody.evidence as EvidenceRow[]) ?? []);
      }
    } catch (err) {
      setError(String(err));
    } finally {
      setBusy(false);
    }
  };

  const profileOptions =
    profiles.length > 0
      ? profiles
      : [{ name: "defense" }, { name: "medical" }, { name: "iso26262" }];

  return (
    <div className="cc-panel">
      {error && <div className="error">{error}</div>}

      <CcSection
        title="Accreditation export"
        hint="Generate a signed compliance bundle for the selected profile."
        actions={
          <div className="cc-filter-bar">
            <select
              value={profile}
              onChange={(event) => setProfile(event.target.value)}
              aria-label="Compliance profile"
            >
              {profileOptions.map((entry) => (
                <option key={entry.name} value={entry.name}>
                  {entry.name}
                  {entry.verified ? " (signed)" : ""}
                </option>
              ))}
            </select>
            <button
              type="button"
              className="primary"
              onClick={() => void exportProfile()}
              disabled={busy || !hasToken || !can("Deploy")}
            >
              {busy ? "Exporting…" : "Export profile"}
            </button>
          </div>
        }
      >
        {!hasToken && (
          <CcEmptyState
            title="Sign in to export compliance bundles"
            description="Compliance export requires a Bearer token with Deploy permission."
          />
        )}

        {!exportData ? (
          <CcEmptyState
            title="No export generated"
            description="Select a profile and export to view accreditation evidence."
          />
        ) : (
          <>
            <CcMiniStats
              items={[
                { label: "Profile", value: profile },
                {
                  label: "Status",
                  value: String(exportData.status ?? exportData.profile ?? "exported"),
                  tone: "ok",
                },
                {
                  label: "Evidence records",
                  value: evidence.length,
                },
              ]}
            />
            <dl className="cc-detail-grid">
              {["profile", "format", "generated_at", "checksum", "bundle_id"]
                .filter((key) => exportData[key] !== undefined)
                .map((key) => (
                  <div key={key} className="cc-detail-row">
                    <dt>{key}</dt>
                    <dd>{String(exportData[key])}</dd>
                  </div>
                ))}
            </dl>
          </>
        )}
      </CcSection>

      <CcSection title="Immutable evidence log" hint="Append-only compliance evidence chain.">
        {evidence.length === 0 ? (
          <CcEmptyState title="No evidence records" description="Export a profile to load the evidence log." />
        ) : (
          <ul className="cc-incident-list">
            {evidence.slice(0, 20).map((row, index) => (
              <li key={index} className="cc-incident-item">
                <div className="cc-incident-header">
                  <span className="cc-incident-title">
                    {String(row.kind ?? row.event ?? row.type ?? `Record ${index + 1}`)}
                  </span>
                  {row.verified !== undefined && (
                    <CcBadge tone={row.verified ? "ok" : "warn"}>
                      {row.verified ? "verified" : "unverified"}
                    </CcBadge>
                  )}
                </div>
                <p className="cc-alert-source">
                  {String(row.summary ?? row.message ?? row.hash ?? JSON.stringify(row)).slice(0, 200)}
                </p>
              </li>
            ))}
          </ul>
        )}
      </CcSection>
    </div>
  );
}

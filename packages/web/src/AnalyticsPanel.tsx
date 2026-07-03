import { useCallback, useEffect, useState } from "react";
import { AnalyticsSection } from "./AnalyticsSection";
import { CcEmptyState, CcSection } from "./controlCenterUi";

type AnalyticsData = {
  what_if?: Record<string, unknown>;
  mission_risk?: Record<string, unknown>;
  readiness_forecast?: Record<string, unknown>;
  trust_graph?: Record<string, unknown>;
  mission_twin?: Record<string, unknown>;
  certification_pack?: Record<string, unknown>;
  time_travel?: Record<string, unknown>;
  human_teaming?: Record<string, unknown>;
  governance?: Record<string, unknown>;
};

type Props = {
  baseUrl: string;
};

export function AnalyticsPanel({ baseUrl }: Props) {
  const [analytics, setAnalytics] = useState<AnalyticsData | null>(null);
  const [busy, setBusy] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const load = useCallback(async () => {
    setBusy(true);
    setError(null);
    try {
      const [
        whatIfRes,
        riskRes,
        forecastRes,
        trustRes,
        twinRes,
        certRes,
        travelRes,
        teamRes,
        govRes,
      ] = await Promise.all([
        fetch(`${baseUrl}/v1/analytics/what-if?all=1`),
        fetch(`${baseUrl}/v1/analytics/mission-risk`),
        fetch(`${baseUrl}/v1/analytics/readiness-forecast?all=1`),
        fetch(`${baseUrl}/v1/analytics/trust-graph`),
        fetch(`${baseUrl}/v1/analytics/mission-twin`),
        fetch(`${baseUrl}/v1/analytics/certification-pack`),
        fetch(`${baseUrl}/v1/analytics/time-travel?at=T%2B00%3A01&inspect=decisions`),
        fetch(`${baseUrl}/v1/analytics/human-teaming`),
        fetch(`${baseUrl}/v1/analytics/governance`),
      ]);
      const next: AnalyticsData = {};
      if (whatIfRes.ok) {
        const body = await whatIfRes.json();
        next.what_if = (body.what_if ?? body) as Record<string, unknown>;
      }
      if (riskRes.ok) {
        const body = await riskRes.json();
        next.mission_risk = (body.mission_risk ?? body) as Record<string, unknown>;
      }
      if (forecastRes.ok) {
        const body = await forecastRes.json();
        next.readiness_forecast = (body.readiness_forecast ?? body) as Record<string, unknown>;
      }
      if (trustRes.ok) {
        const body = await trustRes.json();
        next.trust_graph = (body.trust_graph ?? body) as Record<string, unknown>;
      }
      if (twinRes.ok) {
        const body = await twinRes.json();
        next.mission_twin = (body.mission_twin ?? body) as Record<string, unknown>;
      }
      if (certRes.ok) {
        const body = await certRes.json();
        next.certification_pack = (body.certification_pack ?? body) as Record<string, unknown>;
      }
      if (travelRes.ok) {
        const body = await travelRes.json();
        next.time_travel = (body.time_travel ?? body) as Record<string, unknown>;
      }
      if (teamRes.ok) {
        const body = await teamRes.json();
        next.human_teaming = (body.human_teaming ?? body) as Record<string, unknown>;
      }
      if (govRes.ok) {
        const body = await govRes.json();
        next.governance = (body.governance ?? body) as Record<string, unknown>;
      }
      setAnalytics(next);
    } catch (err) {
      setError(String(err));
    } finally {
      setBusy(false);
    }
  }, [baseUrl]);

  useEffect(() => {
    void load();
  }, [load]);

  const hasData = analytics && Object.values(analytics).some(Boolean);

  return (
    <div className="cc-panel">
      {error && <div className="error">{error}</div>}
      <CcSection
        title="Differentiation analytics"
        hint="Requires a loaded program via control-center serve --program."
        actions={
          <button type="button" onClick={() => void load()} disabled={busy}>
            {busy ? "Loading…" : "Refresh"}
          </button>
        }
      >
        {!hasData && !busy ? (
          <CcEmptyState
            title="No analytics data"
            description="Load a program with control-center serve --program to populate analytics endpoints."
          />
        ) : null}
      </CcSection>
      <AnalyticsSection title="What-if analysis" data={analytics?.what_if} />
      <AnalyticsSection title="Mission risk" data={analytics?.mission_risk} />
      <AnalyticsSection title="Readiness forecast" data={analytics?.readiness_forecast} />
      <AnalyticsSection title="Trust graph" data={analytics?.trust_graph} />
      <AnalyticsSection title="Mission twin" data={analytics?.mission_twin} />
      <AnalyticsSection title="Certification pack" data={analytics?.certification_pack} />
      <AnalyticsSection title="Time travel" data={analytics?.time_travel} />
      <AnalyticsSection title="Human teaming" data={analytics?.human_teaming} />
      <AnalyticsSection title="Governance" data={analytics?.governance} />
    </div>
  );
}

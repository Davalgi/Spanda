import { useCallback, useEffect, useState } from "react";
import { AnalyticsSection } from "./AnalyticsSection";
import { CcMiniStats, CcSection } from "./controlCenterUi";
import { useRegisterTabRefresh } from "./useControlCenterTabRefresh";

type DevicePool = {
  total: number;
  healthy: number;
  degraded: number;
};

type ReadinessImpact = {
  mission_ready: boolean;
  impact: {
    blocked_count: number;
    total_devices: number;
  };
};

type Props = {
  baseUrl: string;
  devicePool?: DevicePool | null;
  alertCount?: number;
};

export function AdasPanel({ baseUrl, devicePool, alertCount = 0 }: Props) {
  const [health, setHealth] = useState<Record<string, unknown> | null>(null);
  const [assurance, setAssurance] = useState<Record<string, unknown> | null>(null);
  const [diagnosis, setDiagnosis] = useState<Record<string, unknown> | null>(null);
  const [trust, setTrust] = useState<Record<string, unknown> | null>(null);
  const [otaStatus, setOtaStatus] = useState<Record<string, unknown> | null>(null);
  const [readiness, setReadiness] = useState<ReadinessImpact | null>(null);
  const [busy, setBusy] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const load = useCallback(async () => {
    setBusy(true);
    setError(null);
    try {
      const [healthRes, assuranceRes, diagnosisRes, trustRes, otaRes] = await Promise.all([
        fetch(`${baseUrl}/v1/health/summary`),
        fetch(`${baseUrl}/v1/assurance/summary`),
        fetch(`${baseUrl}/v1/diagnosis/summary`),
        fetch(`${baseUrl}/v1/trust/package?name=spanda-gps`),
        fetch(`${baseUrl}/v1/ota/status`),
      ]);
      if (healthRes.ok) setHealth(await healthRes.json());
      if (assuranceRes.ok) setAssurance(await assuranceRes.json());
      if (diagnosisRes.ok) setDiagnosis(await diagnosisRes.json());
      if (trustRes.ok) setTrust(await trustRes.json());
      if (otaRes.ok) setOtaStatus(await otaRes.json());
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

  const runReadiness = async () => {
    setBusy(true);
    setError(null);
    try {
      const res = await fetch(`${baseUrl}/v1/readiness/run`, { method: "POST" });
      if (!res.ok) throw new Error(`readiness ${res.status}`);
      setReadiness(await res.json());
    } catch (err) {
      setError(String(err));
    } finally {
      setBusy(false);
    }
  };

  const trustScore =
    trust?.trust_score != null ? Math.round(Number(trust.trust_score) * 100) : "—";

  return (
    <div className="cc-panel">
      {error && <div className="error">{error}</div>}

      <CcMiniStats
        items={[
          { label: "Vehicle health", value: String(health?.overall_status ?? "—") },
          { label: "Sensor devices", value: devicePool?.total ?? 0 },
          { label: "Healthy sensors", value: devicePool?.healthy ?? 0 },
          { label: "Degraded", value: devicePool?.degraded ?? 0 },
          { label: "Trust score", value: trustScore },
          { label: "Active alerts", value: alertCount },
        ]}
      />

      <CcSection
        title="Mission readiness"
        actions={
          <button type="button" onClick={() => void runReadiness()} disabled={busy}>
            Run readiness
          </button>
        }
      >
        {readiness ? (
          <pre className="cc-action-result">{JSON.stringify(readiness, null, 2)}</pre>
        ) : (
          <p className="cc-section-hint">
            Monitor driver_takeover and sensor_degradation via the Alerts tab.
          </p>
        )}
      </CcSection>

      <AnalyticsSection title="OTA status" data={otaStatus} />
      <AnalyticsSection title="Assurance summary" data={assurance} />
      <AnalyticsSection title="Diagnosis summary" data={diagnosis} />

      <CcSection title="Replay & compliance">
        <p className="cc-section-hint">
          Record: <code>spanda sim src/highway_drive.sd --record</code> · Replay:{" "}
          <code>spanda replay src/highway_drive.trace --deterministic</code>
        </p>
        <p className="cc-section-hint">
          ISO 26262 export: <code>GET /v1/compliance/export?profile=iso26262</code> — use Compliance
          tab with profile <code>iso26262</code>.
        </p>
      </CcSection>
    </div>
  );
}

/** ADAS vehicle Control Center panel — live health/assurance with honest trust probing. @module */

import { useCallback, useEffect, useState } from "react";
import { AnalyticsSection } from "./AnalyticsSection";
import { CcMiniStats, CcNotice, CcSection } from "./controlCenterUi";
import { useControlCenterDemoMode } from "./useControlCenterDemoMode";
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

const ADAS_TRUST_CANDIDATES = [
  "spanda-gps",
  "spanda-imu",
  "spanda-camera",
  "spanda-lidar",
  "spanda-radar",
];

async function probeTrustPackage(
  baseUrl: string,
  names: string[],
): Promise<{ name: string; body: Record<string, unknown> } | null> {
  // Try configured / ADAS package names until one returns a trust payload.
  //
  // Parameters:
  // - `baseUrl` — API base
  // - `names` — candidate package names
  //
  // Returns:
  // First successful trust package body, or null.
  //
  // Options:
  // None.
  //
  // Example:
  // const hit = await probeTrustPackage(url, ["spanda-gps"]);

  for (const name of names) {
    const res = await fetch(`${baseUrl}/v1/trust/package?name=${encodeURIComponent(name)}`);
    if (!res.ok) continue;
    const body = (await res.json()) as Record<string, unknown>;
    if (body.trust_score != null || body.trust_level != null || body.ok === true) {
      return { name, body };
    }
  }
  return null;
}

export function AdasPanel({ baseUrl, devicePool, alertCount = 0 }: Props) {
  // Load ADAS-oriented health, assurance, diagnosis, OTA, and package trust.
  //
  // Parameters:
  // - `baseUrl` — Control Center API base URL
  // - `devicePool` — optional pool counts from the parent shell
  // - `alertCount` — open alert count
  //
  // Returns:
  // ADAS panel element.
  //
  // Options:
  // None.
  //
  // Example:
  // <AdasPanel baseUrl={url} devicePool={pool} />

  const { demoMode } = useControlCenterDemoMode();
  const [health, setHealth] = useState<Record<string, unknown> | null>(null);
  const [assurance, setAssurance] = useState<Record<string, unknown> | null>(null);
  const [diagnosis, setDiagnosis] = useState<Record<string, unknown> | null>(null);
  const [trust, setTrust] = useState<Record<string, unknown> | null>(null);
  const [trustPackage, setTrustPackage] = useState<string | null>(null);
  const [otaStatus, setOtaStatus] = useState<Record<string, unknown> | null>(null);
  const [readiness, setReadiness] = useState<ReadinessImpact | null>(null);
  const [busy, setBusy] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const load = useCallback(async () => {
    setBusy(true);
    setError(null);
    try {
      const [healthRes, assuranceRes, diagnosisRes, otaRes, dashRes] = await Promise.all([
        fetch(`${baseUrl}/v1/health/summary`),
        fetch(`${baseUrl}/v1/assurance/summary`),
        fetch(`${baseUrl}/v1/diagnosis/summary`),
        fetch(`${baseUrl}/v1/ota/status`),
        fetch(`${baseUrl}/v1/dashboard`),
      ]);
      if (healthRes.ok) setHealth(await healthRes.json());
      if (assuranceRes.ok) setAssurance(await assuranceRes.json());
      if (diagnosisRes.ok) setDiagnosis(await diagnosisRes.json());
      if (otaRes.ok) setOtaStatus(await otaRes.json());

      // Prefer package names discovered from the loaded device pool / dashboard.
      const candidates = [...ADAS_TRUST_CANDIDATES];
      if (dashRes.ok) {
        const dash = (await dashRes.json()) as {
          device_pool?: { devices?: Array<{ package?: string; name?: string }> };
        };
        for (const device of dash.device_pool?.devices ?? []) {
          const pkg = device.package ?? device.name;
          if (typeof pkg === "string" && pkg.length > 0 && !candidates.includes(pkg)) {
            candidates.unshift(pkg);
          }
        }
      }
      const hit = await probeTrustPackage(baseUrl, candidates);
      setTrust(hit?.body ?? null);
      setTrustPackage(hit?.name ?? null);
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

      <CcNotice
        tone={demoMode ? "info" : "warn"}
        title={
          demoMode
            ? "Demo mode — ADAS composite showcase"
            : "ADAS composite — serve the ADAS blueprint for vehicle devices"
        }
      >
        Health, assurance, diagnosis, and OTA are live fleet APIs. Trust probes the first package
        found in the device pool (ADAS sensor candidates otherwise). Example:{" "}
        <code>
          spanda control-center serve --config examples/solutions/adas/spanda.toml --program
          examples/solutions/adas/src/highway_drive.sd
        </code>
        .
      </CcNotice>

      <CcMiniStats
        items={[
          { label: "Vehicle health", value: String(health?.overall_status ?? "—") },
          { label: "Sensor devices", value: devicePool?.total ?? 0 },
          { label: "Healthy sensors", value: devicePool?.healthy ?? 0 },
          { label: "Degraded", value: devicePool?.degraded ?? 0 },
          {
            label: trustPackage ? `Trust (${trustPackage})` : "Trust score",
            value: trustScore,
          },
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

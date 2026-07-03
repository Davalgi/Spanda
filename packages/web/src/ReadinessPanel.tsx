import { useMemo } from "react";
import {
  CcBadge,
  CcEmptyState,
  CcMiniStats,
  CcSection,
  isBlockingLifecycle,
  lifecycleTone,
} from "./controlCenterUi";
import type { DeviceRow } from "./DevicesPanel";

export type ReadinessResult = {
  mission_ready: boolean;
  impact: {
    blocked_count: number;
    total_devices: number;
  };
};

type Props = {
  readiness: ReadinessResult | null;
  devices: DeviceRow[];
  busy?: boolean;
  onRunCheck: () => void;
  onInspectDevice: (id: string) => void;
};

export function ReadinessPanel({
  readiness,
  devices,
  busy,
  onRunCheck,
  onInspectDevice,
}: Props) {
  const attentionDevices = useMemo(
    () => devices.filter((device) => isBlockingLifecycle(device.lifecycle_state)),
    [devices],
  );

  const missionReady = readiness?.mission_ready;
  const blockedCount = readiness?.impact.blocked_count ?? attentionDevices.length;
  const totalDevices = readiness?.impact.total_devices ?? devices.length;

  return (
    <div className="cc-panel">
      <div
        className={`cc-readiness-banner${
          missionReady === undefined
            ? ""
            : missionReady
              ? " ready"
              : " blocked"
        }`}
      >
        <div>
          <p className="cc-readiness-banner-label">Mission readiness</p>
          <p className="cc-readiness-banner-value">
            {missionReady === undefined
              ? "Not checked yet"
              : missionReady
                ? "Ready to deploy"
                : "Not ready — action required"}
          </p>
          {readiness && (
            <p className="cc-readiness-banner-detail">
              {blockedCount} of {totalDevices} devices blocking mission start
            </p>
          )}
        </div>
        <button type="button" className="primary" onClick={onRunCheck} disabled={busy}>
          {busy ? "Running check…" : "Run readiness check"}
        </button>
      </div>

      <CcMiniStats
        items={[
          {
            label: "Status",
            value: missionReady === undefined ? "—" : missionReady ? "Ready" : "Blocked",
            tone: missionReady === undefined ? "neutral" : missionReady ? "ok" : "danger",
          },
          { label: "Blocked", value: blockedCount, tone: blockedCount > 0 ? "danger" : "ok" },
          { label: "Pool size", value: totalDevices },
          {
            label: "Needs attention",
            value: attentionDevices.length,
            tone: attentionDevices.length > 0 ? "warn" : "ok",
          },
        ]}
      />

      <CcSection
        title="Devices affecting readiness"
        hint="Quarantined, failed, discovered, or degraded devices typically block missions."
      >
        {devices.length === 0 ? (
          <CcEmptyState
            title="No devices in pool"
            description="Add devices via discovery and provisioning before running a readiness check."
          />
        ) : attentionDevices.length === 0 ? (
          <CcEmptyState
            title="No blocking devices detected"
            description="All devices in the pool appear ready. Run a check to refresh mission status."
            action={
              <button type="button" onClick={onRunCheck} disabled={busy}>
                Run readiness check
              </button>
            }
          />
        ) : (
          <div className="cc-table-wrap">
            <table className="cc-data-table">
              <thead>
                <tr>
                  <th>Device</th>
                  <th>Lifecycle</th>
                  <th>Trust</th>
                  <th>Robot</th>
                </tr>
              </thead>
              <tbody>
                {attentionDevices.map((device) => (
                  <tr key={device.id} className="cc-row-attention">
                    <td>
                      <button
                        type="button"
                        className="cc-link-btn"
                        onClick={() => onInspectDevice(device.id)}
                      >
                        {device.id}
                      </button>
                    </td>
                    <td>
                      <CcBadge tone={lifecycleTone(device.lifecycle_state)}>
                        {device.lifecycle_state}
                      </CcBadge>
                    </td>
                    <td>{device.trust_level ?? "unknown"}</td>
                    <td>{device.assigned_robot ?? "—"}</td>
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

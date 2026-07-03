import { useMemo } from "react";
import type { RbacAction } from "./controlCenterRbac";
import {
  CcBadge,
  CcEmptyState,
  CcSection,
  CcWizardSteps,
  lifecycleTone,
  trustTone,
  type WizardStep,
} from "./controlCenterUi";
import type { DeviceRow } from "./DevicesPanel";
import type { RobotRow } from "./FleetPanel";

const STEP_DEFS = [
  { key: "discover", label: "Discover" },
  { key: "verify_identity", label: "Verify identity" },
  { key: "trust_validation", label: "Trust validation" },
  { key: "firmware_validation", label: "Firmware check" },
  { key: "health_validation", label: "Health check" },
  { key: "capability_validation", label: "Capabilities" },
  { key: "assign", label: "Assign to robot" },
  { key: "ready", label: "Ready" },
] as const;

export type ProvisionStepResult = {
  step: string;
  passed: boolean;
  message: string;
};

export type ProvisionReport = {
  device_id: string;
  ready: boolean;
  steps: ProvisionStepResult[];
  assigned_robot?: string;
};

type Props = {
  devices: DeviceRow[];
  selectedDevice: string | null;
  deviceDetail: Record<string, unknown> | null;
  provisionReport: ProvisionReport | null;
  robots: RobotRow[];
  robotId: string;
  busy?: boolean;
  hasToken: boolean;
  can: (action: RbacAction) => boolean;
  onSelectDevice: (id: string) => void;
  onRobotChange: (robotId: string) => void;
  onTrust: () => void;
  onProvision: () => void;
  onAssign: () => void;
  onQuarantine: () => void;
  onSignIn?: () => void;
};

function detailField(detail: Record<string, unknown> | null, key: string): string {
  const value = detail?.[key];
  if (value === undefined || value === null || value === "") return "—";
  if (Array.isArray(value)) return value.join(", ");
  return String(value);
}

function buildWizardSteps(
  deviceDetail: Record<string, unknown> | null,
  provisionReport: ProvisionReport | null,
): WizardStep[] {
  if (provisionReport?.steps?.length) {
    const reportByStep = new Map(
      provisionReport.steps.map((step) => [step.step, step]),
    );
    return STEP_DEFS.map((definition) => {
      const result = reportByStep.get(definition.key);
      if (!result) {
        return { id: definition.key, label: definition.label, state: "pending" as const };
      }
      return {
        id: definition.key,
        label: definition.label,
        state: result.passed ? ("done" as const) : ("failed" as const),
        detail: result.message,
      };
    });
  }

  const lifecycle = String(deviceDetail?.lifecycle_state ?? "").toLowerCase();
  const trust = String(deviceDetail?.trust_level ?? "").toLowerCase();
  const assigned = deviceDetail?.assigned_robot ?? deviceDetail?.robot_id;
  const health = String(deviceDetail?.health_status ?? "").toLowerCase();

  const inferred: Record<string, boolean> = {
    discover: Boolean(deviceDetail?.id),
    verify_identity: Boolean(deviceDetail?.id),
    trust_validation: trust === "trusted" || trust === "verified",
    firmware_validation: Boolean(deviceDetail?.firmware_version),
    health_validation: health === "healthy" || health === "ok" || lifecycle === "active",
    capability_validation: Array.isArray(deviceDetail?.capabilities)
      ? (deviceDetail.capabilities as unknown[]).length > 0
      : false,
    assign: Boolean(assigned),
    ready: lifecycle === "active" || lifecycle === "ready",
  };

  let activeReached = false;
  return STEP_DEFS.map((definition) => {
    const passed = inferred[definition.key] ?? false;
    if (passed) {
      return { id: definition.key, label: definition.label, state: "done" as const };
    }
    if (!activeReached) {
      activeReached = true;
      return { id: definition.key, label: definition.label, state: "active" as const };
    }
    return { id: definition.key, label: definition.label, state: "pending" as const };
  });
}

export function ProvisioningPanel({
  devices,
  selectedDevice,
  deviceDetail,
  provisionReport,
  robots,
  robotId,
  busy,
  hasToken,
  can,
  onSelectDevice,
  onRobotChange,
  onTrust,
  onProvision,
  onAssign,
  onQuarantine,
  onSignIn,
}: Props) {
  const wizardSteps = useMemo(
    () => buildWizardSteps(deviceDetail, provisionReport),
    [deviceDetail, provisionReport],
  );

  const lifecycle = String(deviceDetail?.lifecycle_state ?? "unknown");
  const trust = deviceDetail?.trust_level ? String(deviceDetail.trust_level) : "unknown";

  return (
    <div className="cc-panel">
      <CcSection
        title="Select device"
        hint="Pick a device from the pool or arrive here from Discovery / Devices."
        actions={
          <select
            value={selectedDevice ?? ""}
            onChange={(event) => {
              const value = event.target.value;
              if (value) onSelectDevice(value);
            }}
            aria-label="Select device to provision"
          >
            <option value="">Choose a device…</option>
            {devices.map((device) => (
              <option key={device.id} value={device.id}>
                {device.id} ({device.lifecycle_state})
              </option>
            ))}
          </select>
        }
      >
        {!selectedDevice ? (
          <CcEmptyState
            title="No device selected"
            description="Choose a discovered device from the dropdown, or open this tab from the Devices list."
          />
        ) : (
          <div className="cc-provision-summary">
            <div>
              <p className="cc-provision-device-id">{selectedDevice}</p>
              <p className="cc-provision-device-meta">
                {detailField(deviceDetail, "device_type")} ·{" "}
                <CcBadge tone={lifecycleTone(lifecycle)}>{lifecycle}</CcBadge>{" "}
                <CcBadge tone={trustTone(trust)}>{trust}</CcBadge>
              </p>
            </div>
            {provisionReport && (
              <CcBadge tone={provisionReport.ready ? "ok" : "warn"}>
                {provisionReport.ready ? "Provisioned" : "Incomplete"}
              </CcBadge>
            )}
          </div>
        )}
      </CcSection>

      {selectedDevice && (
        <>
          <CcSection title="Provisioning pipeline" hint="Eight gates from discovery to mission-ready.">
            <CcWizardSteps steps={wizardSteps} />
          </CcSection>

          <CcSection
            title="Actions"
            hint="Run steps in order: trust → provision → assign. Quarantine removes a suspect device."
          >
            {!hasToken && (
              <CcEmptyState
                title="Sign in to provision devices"
                description="Trust, provision, assign, and quarantine require a Bearer token."
                action={
                  onSignIn ? (
                    <button type="button" onClick={onSignIn}>
                      Sign in
                    </button>
                  ) : undefined
                }
              />
            )}

            <div className="cc-provision-actions">
              <label className="cc-provision-robot">
                Target robot
                <select
                  value={robotId}
                  onChange={(event) => onRobotChange(event.target.value)}
                  disabled={!hasToken}
                >
                  {robots.map((robot) => (
                    <option key={robot.id} value={robot.id}>
                      {robot.id}
                    </option>
                  ))}
                  {robots.length === 0 && <option value="rover-001">rover-001</option>}
                </select>
              </label>

              <div className="cc-action-bar">
                <button
                  type="button"
                  onClick={onTrust}
                  disabled={busy || !hasToken || !can("Approve")}
                >
                  Trust / Approve
                </button>
                <button
                  type="button"
                  className="primary"
                  onClick={onProvision}
                  disabled={busy || !hasToken || !can("Provision")}
                >
                  Run provision
                </button>
                <button
                  type="button"
                  onClick={onAssign}
                  disabled={busy || !hasToken || !can("Operate")}
                >
                  Assign to fleet
                </button>
                <button
                  type="button"
                  onClick={onQuarantine}
                  disabled={busy || !hasToken || !can("Operate")}
                >
                  Quarantine
                </button>
              </div>
            </div>
          </CcSection>

          {deviceDetail && (
            <CcSection title="Device details" hint="Identity and capability fields from the registry.">
              <dl className="cc-detail-grid">
                <dt>Type</dt>
                <dd>{detailField(deviceDetail, "device_type")}</dd>
                <dt>Logical name</dt>
                <dd>{detailField(deviceDetail, "logical_name")}</dd>
                <dt>Provider</dt>
                <dd>{detailField(deviceDetail, "provider")}</dd>
                <dt>Firmware</dt>
                <dd>{detailField(deviceDetail, "firmware_version")}</dd>
                <dt>Capabilities</dt>
                <dd>{detailField(deviceDetail, "capabilities")}</dd>
                <dt>Assigned robot</dt>
                <dd>{detailField(deviceDetail, "assigned_robot")}</dd>
                <dt>Health</dt>
                <dd>{detailField(deviceDetail, "health_status")}</dd>
              </dl>
            </CcSection>
          )}
        </>
      )}
    </div>
  );
}

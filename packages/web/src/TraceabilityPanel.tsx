import { CcEmptyState, CcSection } from "./controlCenterUi";
import { ControlCenterDataTable } from "./controlCenterDataTable";

type DeviceEntry = {
  id: string;
  trust_level?: string;
  logical_name?: string;
  lifecycle_state?: string;
  device_type?: string;
};

type Props = {
  devices: DeviceEntry[];
};

export function TraceabilityPanel({ devices }: Props) {
  return (
    <div className="cc-panel">
      <CcSection
        title="Device traceability"
        hint="Trust level and logical name mapping for audit and compliance chains."
      >
        {devices.length === 0 ? (
          <CcEmptyState title="No devices in pool" description="Register devices via Discovery." />
        ) : (
          <ControlCenterDataTable
            rows={devices}
            rowKey={(row) => row.id}
            columns={[
              { key: "id", header: "Device ID", render: (row) => row.id },
              { key: "type", header: "Type", render: (row) => String(row.device_type ?? "—") },
              {
                key: "logical",
                header: "Logical name",
                render: (row) => String(row.logical_name ?? "—"),
              },
              {
                key: "trust",
                header: "Trust",
                render: (row) => String(row.trust_level ?? "unknown"),
              },
              {
                key: "lifecycle",
                header: "Lifecycle",
                render: (row) => String(row.lifecycle_state ?? "—"),
              },
            ]}
          />
        )}
      </CcSection>
    </div>
  );
}

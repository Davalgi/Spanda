import { useCallback, useEffect, useState } from "react";
import { CcEmptyState, CcSection } from "./controlCenterUi";
import { ControlCenterDataTable } from "./controlCenterDataTable";

type DeviceEntry = {
  id: string;
  trust_level?: string;
  logical_name?: string;
  lifecycle_state?: string;
  device_type?: string;
};

type TraceabilityRow = {
  capability?: string;
  required_by?: string;
  provided_by?: string;
  hardware?: string;
  status?: string;
};

type Props = {
  baseUrl: string;
  devices: DeviceEntry[];
};

export function TraceabilityPanel({ baseUrl, devices }: Props) {
  const [traceability, setTraceability] = useState<TraceabilityRow[]>([]);
  const [busy, setBusy] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const loadTraceability = useCallback(async () => {
    setBusy(true);
    setError(null);
    try {
      const res = await fetch(`${baseUrl}/v1/programs/verify/capabilities`, {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ traceability: true }),
      });
      if (!res.ok) throw new Error(`capabilities ${res.status}`);
      const body = await res.json();
      const matrix = body.traceability as Record<string, unknown> | undefined;
      const rows = Array.isArray(matrix?.capability_rows)
        ? (matrix.capability_rows as TraceabilityRow[])
        : [];
      setTraceability(rows);
    } catch (err) {
      setError(String(err));
    } finally {
      setBusy(false);
    }
  }, [baseUrl]);

  useEffect(() => {
    void loadTraceability();
  }, [loadTraceability]);

  return (
    <div className="cc-panel">
      {error && <div className="error">{error}</div>}

      <CcSection
        title="Capability traceability"
        hint="Hardware-to-code traceability from the loaded program — POST /v1/programs/verify/capabilities."
        actions={
          <button type="button" onClick={() => void loadTraceability()} disabled={busy}>
            Refresh matrix
          </button>
        }
      >
        {traceability.length === 0 ? (
          <CcEmptyState
            title="No traceability rows"
            description="Start Control Center with --program or run Refresh after loading a program."
          />
        ) : (
          <ControlCenterDataTable
            rows={traceability}
            rowKey={(row, index) => `${row.capability ?? "row"}-${index}`}
            columns={[
              {
                key: "capability",
                header: "Capability",
                render: (row) => String(row.capability ?? "—"),
              },
              {
                key: "required",
                header: "Required by",
                render: (row) => String(row.required_by ?? "—"),
              },
              {
                key: "provided",
                header: "Provided by",
                render: (row) => String(row.provided_by ?? "—"),
              },
              { key: "status", header: "Status", render: (row) => String(row.status ?? "—") },
            ]}
          />
        )}
      </CcSection>

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

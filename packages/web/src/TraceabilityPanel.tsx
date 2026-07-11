/** Control Center capability and device traceability matrix. @module */

import { useCallback, useEffect, useMemo, useState } from "react";
import { CcEmptyState, CcMiniStats, CcSection } from "./controlCenterUi";
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
  package?: string;
  provider?: string;
  safety_rule?: string | null;
  status?: string;
  notes?: string | null;
};

type HardwareRow = {
  hardware_component?: string;
  used_by?: string;
  source_location?: string;
  capability?: string;
  provider?: string;
  verified?: boolean;
  safety_rule?: string | null;
  notes?: string | null;
};

type Props = {
  baseUrl: string;
  devices: DeviceEntry[];
};

function csvEscape(value: unknown): string {
  // Quote CSV cells that contain commas or quotes.
  const text = String(value ?? "");
  if (/[",\n]/.test(text)) return `"${text.replace(/"/g, '""')}"`;
  return text;
}

export function TraceabilityPanel({ baseUrl, devices }: Props) {
  // Hold capability matrix, hardware rows, entity chain, and filter state.
  const [traceability, setTraceability] = useState<TraceabilityRow[]>([]);
  const [hardwareRows, setHardwareRows] = useState<HardwareRow[]>([]);
  const [warnings, setWarnings] = useState<string[]>([]);
  const [errors, setErrors] = useState<string[]>([]);
  const [entityChain, setEntityChain] = useState<string[]>([]);
  const [statusFilter, setStatusFilter] = useState("");
  const [textFilter, setTextFilter] = useState("");
  const [busy, setBusy] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const loadTraceability = useCallback(async () => {
    // Load program capability matrix and unified entity traceability chain.
    setBusy(true);
    setError(null);
    try {
      const [capRes, entityRes] = await Promise.all([
        fetch(`${baseUrl}/v1/programs/verify/capabilities`, {
          method: "POST",
          headers: { "Content-Type": "application/json" },
          body: JSON.stringify({ traceability: true }),
        }),
        fetch(`${baseUrl}/v1/entities/traceability`),
      ]);
      if (!capRes.ok) throw new Error(`capabilities ${capRes.status}`);
      const body = await capRes.json();
      const matrix = body.traceability as Record<string, unknown> | undefined;
      const rows = Array.isArray(matrix?.capability_rows)
        ? (matrix.capability_rows as TraceabilityRow[])
        : [];
      const hw = Array.isArray(matrix?.hardware_rows)
        ? (matrix.hardware_rows as HardwareRow[])
        : [];
      setTraceability(rows);
      setHardwareRows(hw);
      setWarnings(Array.isArray(matrix?.warnings) ? (matrix.warnings as string[]) : []);
      setErrors(Array.isArray(matrix?.errors) ? (matrix.errors as string[]) : []);

      if (entityRes.ok) {
        const entityBody = await entityRes.json();
        const chain = Array.isArray(entityBody.chain_summary)
          ? (entityBody.chain_summary as string[])
          : Array.isArray(entityBody.unified_chain)
            ? (entityBody.unified_chain as string[])
            : Array.isArray(entityBody.digital_thread?.chain_summary)
              ? (entityBody.digital_thread.chain_summary as string[])
              : [];
        setEntityChain(chain.map(String));
      } else {
        setEntityChain([]);
      }
    } catch (err) {
      setError(String(err));
    } finally {
      setBusy(false);
    }
  }, [baseUrl]);

  useEffect(() => {
    void loadTraceability();
  }, [loadTraceability]);

  const filteredRows = useMemo(() => {
    // Apply status and free-text filters to capability rows.
    const status = statusFilter.trim().toLowerCase();
    const text = textFilter.trim().toLowerCase();
    return traceability.filter((row) => {
      if (status && String(row.status ?? "").toLowerCase() !== status) return false;
      if (!text) return true;
      const hay = [
        row.capability,
        row.required_by,
        row.provided_by,
        row.hardware,
        row.package,
        row.provider,
        row.safety_rule,
        row.notes,
        row.status,
      ]
        .map((v) => String(v ?? "").toLowerCase())
        .join(" ");
      return hay.includes(text);
    });
  }, [traceability, statusFilter, textFilter]);

  const exportCsv = () => {
    // Download the filtered capability matrix as CSV.
    const header = [
      "capability",
      "required_by",
      "provided_by",
      "hardware",
      "package",
      "provider",
      "safety_rule",
      "status",
      "notes",
    ];
    const lines = [
      header.join(","),
      ...filteredRows.map((row) =>
        [
          row.capability,
          row.required_by,
          row.provided_by,
          row.hardware,
          row.package,
          row.provider,
          row.safety_rule,
          row.status,
          row.notes,
        ]
          .map(csvEscape)
          .join(","),
      ),
    ];
    const blob = new Blob([lines.join("\n")], { type: "text/csv;charset=utf-8" });
    const url = URL.createObjectURL(blob);
    const anchor = document.createElement("a");
    anchor.href = url;
    anchor.download = "capability-traceability.csv";
    anchor.click();
    URL.revokeObjectURL(url);
  };

  const statuses = useMemo(() => {
    // Collect distinct status values for the filter dropdown.
    return Array.from(
      new Set(traceability.map((row) => String(row.status ?? "").trim()).filter(Boolean)),
    ).sort();
  }, [traceability]);

  return (
    <div className="cc-panel">
      {error && <div className="error">{error}</div>}

      {(errors.length > 0 || warnings.length > 0) && (
        <CcSection title="Matrix diagnostics">
          {errors.map((item) => (
            <p key={item} className="error">
              {item}
            </p>
          ))}
          {warnings.map((item) => (
            <p key={item} className="demo-hint">
              {item}
            </p>
          ))}
        </CcSection>
      )}

      <CcMiniStats
        items={[
          { label: "Capabilities", value: filteredRows.length },
          { label: "Hardware rows", value: hardwareRows.length },
          { label: "Entity chain", value: entityChain.length },
        ]}
      />

      <CcSection
        title="Capability traceability matrix"
        hint="Full hardware-to-code matrix from POST /v1/programs/verify/capabilities — also see Digital Thread for lifecycle graph."
        actions={
          <div className="cc-action-bar">
            <button type="button" onClick={() => void loadTraceability()} disabled={busy}>
              Refresh matrix
            </button>
            <button type="button" onClick={exportCsv} disabled={filteredRows.length === 0}>
              Export CSV
            </button>
          </div>
        }
      >
        <div className="cc-action-bar" style={{ marginBottom: "0.75rem", flexWrap: "wrap" }}>
          <input
            type="search"
            placeholder="Filter capability / hardware / package…"
            value={textFilter}
            onChange={(event) => setTextFilter(event.target.value)}
            style={{ minWidth: "14rem", flex: "1 1 14rem" }}
          />
          <select value={statusFilter} onChange={(event) => setStatusFilter(event.target.value)}>
            <option value="">All statuses</option>
            {statuses.map((status) => (
              <option key={status} value={status}>
                {status}
              </option>
            ))}
          </select>
        </div>
        {filteredRows.length === 0 ? (
          <CcEmptyState
            title="No traceability rows"
            description="Start Control Center with --program or run Refresh after loading a program."
          />
        ) : (
          <ControlCenterDataTable
            rows={filteredRows}
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
              {
                key: "hardware",
                header: "Hardware",
                render: (row) => String(row.hardware ?? "—"),
              },
              {
                key: "package",
                header: "Package",
                render: (row) => String(row.package ?? "—"),
              },
              {
                key: "provider",
                header: "Provider",
                render: (row) => String(row.provider ?? "—"),
              },
              {
                key: "safety",
                header: "Safety rule",
                render: (row) => String(row.safety_rule ?? "—"),
              },
              {
                key: "status",
                header: "Status",
                render: (row) => {
                  const status = String(row.status ?? "—");
                  const tone =
                    status.toLowerCase().includes("ok") || status.toLowerCase() === "satisfied"
                      ? "ok"
                      : status.toLowerCase().includes("miss") ||
                          status.toLowerCase().includes("fail")
                        ? "bad"
                        : "warn";
                  return <span className={`cc-status cc-status-${tone}`}>{status}</span>;
                },
              },
              {
                key: "notes",
                header: "Notes",
                render: (row) => String(row.notes ?? "—"),
              },
            ]}
          />
        )}
      </CcSection>

      <CcSection title="Hardware rows" hint="Hardware components referenced by the capability matrix.">
        {hardwareRows.length === 0 ? (
          <CcEmptyState title="No hardware rows" />
        ) : (
          <ControlCenterDataTable
            rows={hardwareRows}
            rowKey={(row, index) => `${row.hardware_component ?? "hw"}-${index}`}
            columns={[
              {
                key: "component",
                header: "Component",
                render: (row) => String(row.hardware_component ?? "—"),
              },
              { key: "used", header: "Used by", render: (row) => String(row.used_by ?? "—") },
              {
                key: "capability",
                header: "Capability",
                render: (row) => String(row.capability ?? "—"),
              },
              {
                key: "provider",
                header: "Provider",
                render: (row) => String(row.provider ?? "—"),
              },
              {
                key: "verified",
                header: "Verified",
                render: (row) => (row.verified ? "yes" : "no"),
              },
              {
                key: "safety",
                header: "Safety rule",
                render: (row) => String(row.safety_rule ?? "—"),
              },
              {
                key: "location",
                header: "Source",
                render: (row) => String(row.source_location ?? "—"),
              },
            ]}
          />
        )}
      </CcSection>

      <CcSection
        title="Entity traceability chain"
        hint="Unified chain from GET /v1/entities/traceability — open Digital Thread for the interactive lifecycle graph."
      >
        {entityChain.length === 0 ? (
          <CcEmptyState title="No entity chain summary" />
        ) : (
          <ol className="cc-event-log">
            {entityChain.map((step, index) => (
              <li key={`${step}-${index}`}>{step}</li>
            ))}
          </ol>
        )}
      </CcSection>

      <CcSection
        title="Device pool mapping"
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

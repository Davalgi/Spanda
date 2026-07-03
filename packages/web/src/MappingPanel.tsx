import { useCallback, useEffect, useMemo, useState } from "react";
import { CcEmptyState, CcSection } from "./controlCenterUi";
import { useRegisterTabRefresh } from "./useControlCenterTabRefresh";

type Props = {
  baseUrl: string;
};

type MappingRow = {
  logical: string;
  physical: string;
};

function mappingRows(section: unknown): MappingRow[] {
  if (!section || typeof section !== "object") return [];
  return Object.entries(section as Record<string, unknown>).map(([logical, physical]) => ({
    logical,
    physical: String(physical),
  }));
}

export function MappingPanel({ baseUrl }: Props) {
  const [mapping, setMapping] = useState<Record<string, unknown> | null>(null);
  const [hierarchy, setHierarchy] = useState<string[]>([]);
  const [loaded, setLoaded] = useState<boolean | null>(null);
  const [busy, setBusy] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const load = useCallback(async () => {
    setBusy(true);
    setError(null);
    try {
      const res = await fetch(`${baseUrl}/v1/device-tree`);
      if (!res.ok) throw new Error(`device-tree ${res.status}`);
      const body = await res.json();
      setLoaded(Boolean(body.loaded));
      setMapping((body.mapping as Record<string, unknown>) ?? null);
      setHierarchy(Array.isArray(body.hierarchy) ? body.hierarchy.map(String) : []);
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

  const sensors = useMemo(() => mappingRows(mapping?.sensors), [mapping]);
  const actuators = useMemo(() => mappingRows(mapping?.actuators), [mapping]);
  const robots = useMemo(() => mappingRows(mapping?.robots), [mapping]);
  const redundancy = Array.isArray(mapping?.redundancy) ? mapping.redundancy : [];

  const exportReports = async () => {
    setBusy(true);
    try {
      const res = await fetch(`${baseUrl}/v1/device-reports`);
      if (!res.ok) throw new Error(`reports ${res.status}`);
      const body = await res.json();
      const blob = new Blob([JSON.stringify(body.reports ?? body, null, 2)], {
        type: "application/json",
      });
      const url = URL.createObjectURL(blob);
      const anchor = document.createElement("a");
      anchor.href = url;
      anchor.download = "spanda-device-reports.json";
      anchor.click();
      URL.revokeObjectURL(url);
    } catch (err) {
      setError(String(err));
    } finally {
      setBusy(false);
    }
  };

  const renderTable = (rows: MappingRow[], emptyLabel: string) => {
    if (rows.length === 0) {
      return <CcEmptyState title={emptyLabel} />;
    }
    return (
      <div className="cc-table-wrap">
        <table className="cc-data-table">
          <thead>
            <tr>
              <th>Logical name</th>
              <th>Physical device</th>
            </tr>
          </thead>
          <tbody>
            {rows.map((row) => (
              <tr key={`${row.logical}-${row.physical}`}>
                <td>{row.logical}</td>
                <td>
                  <code>{row.physical}</code>
                </td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>
    );
  };

  return (
    <div className="cc-panel">
      {error && <div className="error">{error}</div>}

      <CcSection
        title="Logical ↔ physical mapping"
        hint="How capability names resolve to hardware in the loaded program."
        actions={
          <button type="button" onClick={() => void exportReports()} disabled={busy}>
            Export device reports
          </button>
        }
      >
        {loaded === false ? (
          <CcEmptyState
            title="No program loaded"
            description="Start Control Center with --program to populate logical mapping from your Spanda project."
          />
        ) : busy && !mapping ? (
          <CcEmptyState title="Loading mapping…" />
        ) : !mapping ? (
          <CcEmptyState title="Mapping unavailable" description="Could not load device tree from the API." />
        ) : (
          <div className="cc-panel-grid">
            <div>
              <h4 className="cc-subheading">Sensors</h4>
              {renderTable(sensors, "No sensor mappings")}
            </div>
            <div>
              <h4 className="cc-subheading">Actuators</h4>
              {renderTable(actuators, "No actuator mappings")}
            </div>
            <div>
              <h4 className="cc-subheading">Robots</h4>
              {renderTable(robots, "No robot mappings")}
            </div>
          </div>
        )}
      </CcSection>

      {redundancy.length > 0 && (
        <CcSection title="Redundancy groups" hint="Failover ordering within redundant device groups.">
          <ul className="cc-card-list">
            {(redundancy as Record<string, unknown>[]).map((group, index) => (
              <li key={String(group.group ?? index)} className="cc-card-item">
                <span className="cc-card-item-title">{String(group.group ?? `group-${index}`)}</span>
                <span className="cc-card-item-meta">
                  {Array.isArray(group.members)
                    ? group.members
                        .map((member) =>
                          typeof member === "object" && member
                            ? String((member as Record<string, unknown>).device_id ?? "")
                            : "",
                        )
                        .filter(Boolean)
                        .join(" → ")
                    : "—"}
                </span>
              </li>
            ))}
          </ul>
        </CcSection>
      )}

      {hierarchy.length > 0 && (
        <CcSection title="Device hierarchy" hint="Fleet tree from the loaded configuration.">
          <pre className="cc-hierarchy-lines">{hierarchy.join("\n")}</pre>
        </CcSection>
      )}
    </div>
  );
}

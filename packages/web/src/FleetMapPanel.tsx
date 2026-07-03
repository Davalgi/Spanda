import { useCallback, useEffect, useState } from "react";
import { CcEmptyState, CcMiniStats } from "./controlCenterUi";
import { useRegisterTabRefresh } from "./useControlCenterTabRefresh";

type MapMarker = {
  id: string;
  label: string;
  kind: string;
  x: number;
  y: number;
  status?: string;
  fleet_id?: string;
};

type Props = {
  baseUrl: string;
};

const KIND_COLORS: Record<string, string> = {
  robot: "#3fb950",
  agent: "#58a6ff",
  device: "#d29922",
  entity: "#a371f7",
  human: "#f778ba",
};

export function FleetMapPanel({ baseUrl }: Props) {
  const [markers, setMarkers] = useState<MapMarker[]>([]);
  const [busy, setBusy] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [selected, setSelected] = useState<MapMarker | null>(null);

  const load = useCallback(async () => {
    setBusy(true);
    setError(null);
    try {
      const res = await fetch(`${baseUrl}/v1/fleet/map`);
      if (!res.ok) throw new Error(`fleet map ${res.status}`);
      const body = await res.json();
      setMarkers((body.markers as MapMarker[]) ?? []);
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

  return (
    <div className="cc-panel">
      {error && <div className="error">{error}</div>}

      <CcMiniStats
        items={[
          { label: "Markers", value: markers.length },
          {
            label: "Robots",
            value: markers.filter((marker) => marker.kind === "robot").length,
          },
          {
            label: "Alerts context",
            value: markers.filter((marker) => marker.status === "degraded" || marker.status === "failed").length,
            tone: markers.some((m) => m.status === "failed") ? "danger" : "ok",
          },
        ]}
      />

      {markers.length === 0 ? (
        <CcEmptyState title="No map markers" description="Load a program with robots or fleet agents." />
      ) : (
        <div className="cc-fleet-map">
            <svg viewBox="0 0 100 100" className="cc-fleet-map-svg" role="img" aria-label="Fleet map">
              <rect x="0" y="0" width="100" height="100" className="cc-fleet-map-bg" />
              {markers.map((marker) => (
                <g
                  key={marker.id}
                  transform={`translate(${marker.x}, ${100 - marker.y})`}
                  className="cc-fleet-marker"
                  onClick={() => setSelected(marker)}
                  onKeyDown={(event) => event.key === "Enter" && setSelected(marker)}
                  role="button"
                  tabIndex={0}
                >
                  <circle
                    r="2.2"
                    fill={KIND_COLORS[marker.kind] ?? "#8b949e"}
                    className={selected?.id === marker.id ? "cc-fleet-marker-selected" : undefined}
                  />
                  <title>{`${marker.label} (${marker.kind})`}</title>
                </g>
              ))}
            </svg>
            {selected && (
              <aside className="cc-fleet-map-detail">
                <h4>{selected.label}</h4>
                <dl className="cc-detail-grid">
                  <div className="cc-detail-row">
                    <dt>ID</dt>
                    <dd>{selected.id}</dd>
                  </div>
                  <div className="cc-detail-row">
                    <dt>Kind</dt>
                    <dd>{selected.kind}</dd>
                  </div>
                  <div className="cc-detail-row">
                    <dt>Status</dt>
                    <dd>{selected.status ?? "—"}</dd>
                  </div>
                  <div className="cc-detail-row">
                    <dt>Position</dt>
                    <dd>
                      {selected.x.toFixed(1)}, {selected.y.toFixed(1)}
                    </dd>
                  </div>
                </dl>
              </aside>
            )}
          </div>
        )}
    </div>
  );
}

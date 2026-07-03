import { useCallback, useEffect, useState } from "react";
import { CcEmptyState, CcSection } from "./controlCenterUi";
import { useRegisterTabRefresh } from "./useControlCenterTabRefresh";

type PluginEntry = {
  manifest?: { plugin?: { name?: string; version?: string; description?: string } };
  installed?: { name?: string; version?: string };
  control_center_panels?: { id?: string; title?: string; component?: string }[];
};

type Props = {
  baseUrl: string;
};

export function MarketplacePanel({ baseUrl }: Props) {
  const [plugins, setPlugins] = useState<PluginEntry[]>([]);
  const [ccPlugins, setCcPlugins] = useState<PluginEntry[]>([]);
  const [busy, setBusy] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const load = useCallback(async () => {
    setBusy(true);
    setError(null);
    try {
      const [allRes, ccRes] = await Promise.all([
        fetch(`${baseUrl}/v1/plugins`),
        fetch(`${baseUrl}/v1/plugins/control-center`),
      ]);
      if (allRes.ok) {
        const body = await allRes.json();
        setPlugins((body.plugins as PluginEntry[]) ?? []);
      }
      if (ccRes.ok) {
        const body = await ccRes.json();
        setCcPlugins((body.plugins as PluginEntry[]) ?? []);
      }
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

      {plugins.length === 0 ? (
          <CcEmptyState
            title="No plugins installed"
            description="Install packages with spanda.plugin.toml and control-center-ui panels."
          />
        ) : (
          <ul className="cc-card-list">
            {plugins.map((entry) => {
              const name = entry.installed?.name ?? entry.manifest?.plugin?.name ?? "plugin";
              const version = entry.installed?.version ?? entry.manifest?.plugin?.version ?? "—";
              const panels = entry.control_center_panels ?? [];
              return (
                <li key={name} className="cc-card-item">
                  <span className="cc-card-item-title">{name}</span>
                  <span className="cc-card-item-meta">v{version}</span>
                  {entry.manifest?.plugin?.description && (
                    <p className="cc-section-hint">{entry.manifest.plugin.description}</p>
                  )}
                  {panels.length > 0 && (
                    <ul>
                      {panels.map((panel) => (
                        <li key={panel.id}>
                          Panel: {panel.title ?? panel.id} ({panel.component})
                        </li>
                      ))}
                    </ul>
                  )}
                </li>
              );
            })}
          </ul>
        )}

      <CcSection title="Control Center panels">
        {ccPlugins.length === 0 ? (
          <CcEmptyState title="No Control Center UI plugins" />
        ) : (
          <pre className="cc-action-result">{JSON.stringify(ccPlugins, null, 2)}</pre>
        )}
      </CcSection>
    </div>
  );
}

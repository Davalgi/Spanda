/**
 * Control Center Marketplace — search, install, enable, and disable plugins via REST.
 *
 * @module MarketplacePanel
 */

import { useCallback, useEffect, useState } from "react";
import type { RbacAction } from "./controlCenterRbac";
import { CcEmptyState, CcSection } from "./controlCenterUi";
import { useRegisterTabRefresh } from "./useControlCenterTabRefresh";

type PluginPanel = { id?: string; title?: string; component?: string };

type PluginEntry = {
  name?: string;
  version?: string;
  state?: string;
  plugin_type?: string;
  trust_tier?: string;
  manifest?: {
    plugin?: { name?: string; version?: string; description?: string };
  };
  installed?: { name?: string; version?: string; state?: string };
  control_center_panels?: PluginPanel[];
};

type RegistryEntry = {
  name?: string;
  description?: string;
  plugin_type?: string;
  tier?: string;
  versions?: string[];
};

type Props = {
  baseUrl: string;
  authHeaders: () => HeadersInit;
  can: (action: RbacAction) => boolean;
  hasToken: boolean;
};

export function MarketplacePanel({ baseUrl, authHeaders, can, hasToken }: Props) {
  // Render installed plugins and registry search with install/enable/disable actions.
  //
  // Parameters:
  // - `baseUrl` — Control Center REST base URL
  // - `authHeaders` — bearer / API key header factory
  // - `can` — RBAC capability check
  // - `hasToken` — whether an auth token is present
  //
  // Returns:
  // Marketplace panel React element.
  //
  // Options:
  // None.
  //
  // Example:
  // <MarketplacePanel baseUrl={url} authHeaders={headers} can={can} hasToken />

  const [plugins, setPlugins] = useState<PluginEntry[]>([]);
  const [ccPlugins, setCcPlugins] = useState<PluginEntry[]>([]);
  const [searchQuery, setSearchQuery] = useState("");
  const [searchResults, setSearchResults] = useState<RegistryEntry[]>([]);
  const [busy, setBusy] = useState(false);
  const [actionBusy, setActionBusy] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [notice, setNotice] = useState<string | null>(null);

  const load = useCallback(async () => {
    // Refresh installed plugins and Control Center UI contributions.
    setBusy(true);
    setError(null);
    try {
      const [allRes, ccRes] = await Promise.all([
        fetch(`${baseUrl}/v1/plugins`, { headers: authHeaders() }),
        fetch(`${baseUrl}/v1/plugins/control-center`, { headers: authHeaders() }),
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
  }, [authHeaders, baseUrl]);

  useEffect(() => {
    void load();
  }, [load]);

  useRegisterTabRefresh(load, { busy });

  const runSearch = useCallback(async () => {
    // Query the bundled plugin registry via REST.
    setError(null);
    setNotice(null);
    try {
      const res = await fetch(
        `${baseUrl}/v1/plugins/search?q=${encodeURIComponent(searchQuery)}`,
        { headers: authHeaders() },
      );
      if (!res.ok) throw new Error(`search ${res.status}`);
      const body = await res.json();
      setSearchResults((body.plugins as RegistryEntry[]) ?? []);
    } catch (err) {
      setError(String(err));
    }
  }, [authHeaders, baseUrl, searchQuery]);

  const installPlugin = useCallback(
    async (opts: { name?: string; path?: string }) => {
      // POST install (Provision RBAC) then refresh the installed list.
      if (!hasToken || !can("provision")) {
        setError("Sign in with Provision permission to install plugins.");
        return;
      }
      setActionBusy(true);
      setError(null);
      setNotice(null);
      try {
        const res = await fetch(`${baseUrl}/v1/plugins/install`, {
          method: "POST",
          headers: { "Content-Type": "application/json", ...authHeaders() },
          body: JSON.stringify(opts),
        });
        const body = await res.json().catch(() => ({}));
        if (!res.ok) throw new Error(body.error ?? `install ${res.status}`);
        setNotice(`Installed ${body.plugin?.name ?? opts.name ?? opts.path}`);
        await load();
      } catch (err) {
        setError(String(err));
      } finally {
        setActionBusy(false);
      }
    },
    [authHeaders, baseUrl, can, hasToken, load],
  );

  const setEnabled = useCallback(
    async (name: string, enable: boolean) => {
      // Toggle plugin lifecycle via enable/disable REST (Operate RBAC).
      if (!hasToken || !can("operate")) {
        setError("Sign in with Operate permission to enable or disable plugins.");
        return;
      }
      setActionBusy(true);
      setError(null);
      setNotice(null);
      try {
        const action = enable ? "enable" : "disable";
        const res = await fetch(`${baseUrl}/v1/plugins/${encodeURIComponent(name)}/${action}`, {
          method: "POST",
          headers: authHeaders(),
        });
        const body = await res.json().catch(() => ({}));
        if (!res.ok) throw new Error(body.error ?? `${action} ${res.status}`);
        setNotice(`${enable ? "Enabled" : "Disabled"} ${name}`);
        await load();
      } catch (err) {
        setError(String(err));
      } finally {
        setActionBusy(false);
      }
    },
    [authHeaders, baseUrl, can, hasToken, load],
  );

  return (
    <div className="cc-panel">
      {error && <div className="error">{error}</div>}
      {notice && <p className="demo-hint">{notice}</p>}

      <CcSection title="Search registry">
        <div className="cc-inline-form">
          <input
            type="search"
            value={searchQuery}
            onChange={(e) => setSearchQuery(e.target.value)}
            placeholder="readiness, healthcare, …"
            aria-label="Plugin search"
          />
          <button type="button" onClick={() => void runSearch()} disabled={actionBusy}>
            Search
          </button>
        </div>
        {searchResults.length > 0 && (
          <ul className="cc-card-list">
            {searchResults.map((entry) => {
              const name = entry.name ?? "plugin";
              return (
                <li key={name} className="cc-card-item">
                  <span className="cc-card-item-title">{name}</span>
                  <span className="cc-card-item-meta">
                    {entry.plugin_type ?? "—"} · {entry.tier ?? "community"}
                  </span>
                  {entry.description && <p className="cc-section-hint">{entry.description}</p>}
                  <button
                    type="button"
                    disabled={actionBusy || !hasToken || !can("provision")}
                    onClick={() => void installPlugin({ name })}
                  >
                    Install
                  </button>
                </li>
              );
            })}
          </ul>
        )}
        <p className="cc-section-hint">
          Example Control Center panel:{" "}
          <button
            type="button"
            disabled={actionBusy || !hasToken || !can("provision")}
            onClick={() =>
              void installPlugin({ path: "examples/plugins/control-center-panel" })
            }
          >
            Install example panel plugin
          </button>
        </p>
      </CcSection>

      <CcSection title="Installed plugins">
        {plugins.length === 0 ? (
          <CcEmptyState
            title="No plugins installed"
            description="Search the registry or install packages with spanda.plugin.toml and control-center-ui panels."
          />
        ) : (
          <ul className="cc-card-list">
            {plugins.map((entry) => {
              const name =
                entry.installed?.name ?? entry.manifest?.plugin?.name ?? entry.name ?? "plugin";
              const version =
                entry.installed?.version ?? entry.manifest?.plugin?.version ?? entry.version ?? "—";
              const state = entry.state ?? entry.installed?.state ?? "installed";
              const panels = entry.control_center_panels ?? [];
              const enabled = state === "enabled";
              return (
                <li key={name} className="cc-card-item">
                  <span className="cc-card-item-title">{name}</span>
                  <span className="cc-card-item-meta">
                    v{version} · {state}
                    {entry.trust_tier ? ` · ${entry.trust_tier}` : ""}
                  </span>
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
                  <div className="cc-inline-form">
                    {enabled ? (
                      <button
                        type="button"
                        disabled={actionBusy || !hasToken || !can("operate")}
                        onClick={() => void setEnabled(name, false)}
                      >
                        Disable
                      </button>
                    ) : (
                      <button
                        type="button"
                        disabled={actionBusy || !hasToken || !can("operate")}
                        onClick={() => void setEnabled(name, true)}
                      >
                        Enable
                      </button>
                    )}
                  </div>
                </li>
              );
            })}
          </ul>
        )}
      </CcSection>

      <CcSection title="Control Center panels">
        {ccPlugins.length === 0 ? (
          <CcEmptyState title="No Control Center UI plugins" />
        ) : (
          <ul className="cc-card-list">
            {ccPlugins.map((entry) => {
              const name =
                entry.installed?.name ?? entry.manifest?.plugin?.name ?? entry.name ?? "plugin";
              const panels = entry.control_center_panels ?? [];
              return (
                <li key={name} className="cc-card-item">
                  <span className="cc-card-item-title">{name}</span>
                  {panels.length === 0 ? (
                    <p className="cc-section-hint">No panels declared</p>
                  ) : (
                    <ul>
                      {panels.map((panel) => (
                        <li key={panel.id}>
                          {panel.title ?? panel.id} ({panel.component})
                        </li>
                      ))}
                    </ul>
                  )}
                </li>
              );
            })}
          </ul>
        )}
      </CcSection>
    </div>
  );
}

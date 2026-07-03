import { useEffect, useState } from "react";
import type { PluginPanelEntry } from "./controlCenterTypes";

type Props = {
  panel: PluginPanelEntry;
  baseUrl: string;
};

export function ControlCenterPluginPanel({ panel, baseUrl }: Props) {
  const [bundleStatus, setBundleStatus] = useState<"idle" | "loading" | "loaded" | "missing">(
    "idle",
  );
  const [bundleError, setBundleError] = useState<string | null>(null);

  useEffect(() => {
    let cancelled = false;
    const load = async () => {
      setBundleStatus("loading");
      setBundleError(null);
      try {
        const res = await fetch(
          `${baseUrl}/v1/plugins/control-center/${encodeURIComponent(panel.plugin)}/bundle`,
        );
        const body = await res.json();
        if (!res.ok) throw new Error(`bundle ${res.status}`);
        if (body.available === false || !body.bundle) {
          if (!cancelled) setBundleStatus("missing");
          return;
        }
        const script = document.createElement("script");
        script.type = "text/javascript";
        script.text = String(body.bundle);
        script.dataset.pluginPanel = `${panel.plugin}:${panel.id}`;
        const host = document.querySelector(
          `[data-plugin-host="${panel.plugin}:${panel.id}"]`,
        );
        if (host) {
          host.querySelectorAll("script").forEach((node) => node.remove());
          host.appendChild(script);
        }
        if (!cancelled) setBundleStatus("loaded");
      } catch (error) {
        if (!cancelled) {
          setBundleError(String(error));
          setBundleStatus("missing");
        }
      }
    };
    void load();
    return () => {
      cancelled = true;
    };
  }, [baseUrl, panel.id, panel.plugin]);

  return (
    <section className="cc-panel">
      <h3>{panel.title}</h3>
      <p className="cc-section-hint">
        Plugin <code>{panel.plugin}</code> — component <code>{panel.component}</code>.
        {bundleStatus === "loaded"
          ? " Bundle executed in panel host."
          : bundleStatus === "loading"
            ? " Loading bundle…"
            : " No bundle artifact — build index.js in the plugin directory."}
      </p>
      {bundleError && <p className="demo-hint">{bundleError}</p>}
      <div
        className="cc-plugin-host"
        data-plugin-host={`${panel.plugin}:${panel.id}`}
        data-plugin={panel.plugin}
        data-component={panel.component}
      />
    </section>
  );
}

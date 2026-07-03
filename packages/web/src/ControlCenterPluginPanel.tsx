import type { PluginPanelEntry } from "./controlCenterTypes";

type Props = {
  panel: PluginPanelEntry;
};

export function ControlCenterPluginPanel({ panel }: Props) {
  return (
    <section className="cc-panel">
      <h3>{panel.title}</h3>
      <p className="cc-section-hint">
        Plugin panel from <code>{panel.plugin}</code> — component <code>{panel.component}</code>.
        Host loads TypeScript bundles from the plugin install directory when a{" "}
        <code>index.js</code> artifact is present.
      </p>
    </section>
  );
}

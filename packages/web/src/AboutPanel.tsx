/** About panel — live component versions from the Control Center API. @module */

import { useCallback, useEffect, useState } from "react";
import { CONTROL_CENTER_VERSION } from "./controlCenterVersion";
import { CcEmptyState, CcSection } from "./controlCenterUi";
import { desktopInvoke, isDesktopShell } from "./desktopBridge";
import { useRegisterTabRefresh } from "./useControlCenterTabRefresh";

type VersionInfo = {
  spanda_version?: string;
  control_center_ui_version?: string;
  api_version?: string;
  version?: string;
  grpc?: {
    package?: string;
    proto_semver?: string;
    proto_file?: string;
    rpc_count?: number;
    reflection_enabled?: boolean;
  };
  policy?: string;
};

type DesktopFeatures = {
  app_version?: string;
};

type Props = {
  baseUrl: string;
};

type VersionRow = {
  label: string;
  value: string;
  hint?: string;
};

export function AboutPanel({ baseUrl }: Props) {
  // Render the About tab with live platform, UI, API, and gRPC versions.
  //
  // Parameters:
  // - `baseUrl` — Control Center origin used for `GET /v1/version`
  //
  // Returns:
  // A panel listing component semver rows; desktop shell version when running in Tauri.
  //
  // Options:
  // None.
  //
  // Example:
  // <AboutPanel baseUrl="http://127.0.0.1:8080" />

  const [info, setInfo] = useState<VersionInfo | null>(null);
  const [desktopVersion, setDesktopVersion] = useState<string | null>(null);
  const [busy, setBusy] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const load = useCallback(async () => {
    // Fetch `/v1/version` and, in the desktop shell, Tauri `app_version`.
    setBusy(true);
    setError(null);
    try {
      const res = await fetch(`${baseUrl}/v1/version`);

      // Surface HTTP failures before parsing JSON.
      if (!res.ok) {
        throw new Error(`version ${res.status}`);
      }

      setInfo((await res.json()) as VersionInfo);

      // Only query the Tauri bridge when the webview is inside the desktop shell.
      if (isDesktopShell()) {
        const features = await desktopInvoke<DesktopFeatures>("desktop_features");
        setDesktopVersion(features?.app_version ?? null);
      } else {
        setDesktopVersion(null);
      }
    } catch (err) {
      setError(String(err));
      setInfo(null);
    } finally {
      setBusy(false);
    }
  }, [baseUrl]);

  useEffect(() => {
    void load();
  }, [load]);

  useRegisterTabRefresh(load, { busy });

  const rows: VersionRow[] = [];

  // Prefer live API fields so the table reflects the connected instance.
  if (info?.spanda_version) {
    rows.push({
      label: "Platform (spanda)",
      value: info.spanda_version,
      hint: "Workspace CLI / API crate semver",
    });
  }

  if (info?.control_center_ui_version) {
    rows.push({
      label: "Control Center UI",
      value: info.control_center_ui_version,
      hint: "Embedded UI semver from the server (`packages/web`)",
    });
  }

  rows.push({
    label: "UI bundle (this page)",
    value: CONTROL_CENTER_VERSION,
    hint: "Build-time constant in the loaded SPA; should match Control Center UI",
  });

  // Desktop shell is an independent release stream when present.
  if (desktopVersion) {
    rows.push({
      label: "Desktop shell",
      value: desktopVersion,
      hint: "Tauri app semver (`packages/control-center-desktop`)",
    });
  }

  if (info?.api_version || info?.version) {
    rows.push({
      label: "REST API",
      value: info.api_version ?? info.version ?? "—",
      hint: "Supported API version header / path prefix",
    });
  }

  if (info?.grpc?.proto_semver) {
    rows.push({
      label: "gRPC proto",
      value: info.grpc.proto_semver,
      hint: info.grpc.package
        ? `${info.grpc.package} · ${info.grpc.rpc_count ?? "?"} RPCs`
        : "control_center.proto contract semver",
    });
  }

  if (info?.grpc && typeof info.grpc.reflection_enabled === "boolean") {
    rows.push({
      label: "gRPC reflection",
      value: info.grpc.reflection_enabled ? "enabled" : "disabled",
    });
  }

  return (
    <div className="cc-panel">
      {error && <div className="error">{error}</div>}

      <CcSection
        title="Component versions"
        hint="Live values from GET /v1/version (and the desktop shell when applicable)."
      >
        {rows.length === 0 && !error ? (
          <CcEmptyState
            title={busy ? "Loading versions…" : "No version data"}
            description="Could not read component versions from this Control Center instance."
          />
        ) : (
          <dl className="cc-detail-grid">
            {rows.map((row) => (
              <div key={row.label} className="cc-detail-row">
                <dt title={row.hint}>{row.label}</dt>
                <dd>
                  <code>{row.value}</code>
                </dd>
              </div>
            ))}
          </dl>
        )}
        <p className="cc-section-hint">
          <a href={`${baseUrl}/v1/version`} target="_blank" rel="noreferrer">
            Open raw /v1/version JSON
          </a>
        </p>
      </CcSection>

      {info?.policy && (
        <CcSection title="Compatibility policy">
          <p className="cc-section-hint">{info.policy}</p>
          {info.grpc?.proto_file && (
            <p className="cc-section-hint">
              Proto file: <code>{info.grpc.proto_file}</code>
            </p>
          )}
        </CcSection>
      )}
    </div>
  );
}

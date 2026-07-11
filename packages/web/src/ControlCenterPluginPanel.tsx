/**
 * Sandboxed Control Center plugin panel host.
 *
 * Loads TypeScript/JS panel bundles inside an iframe with a restrictive
 * `sandbox` attribute and a postMessage bridge — never via `script.text`
 * injection into the parent document.
 *
 * @module ControlCenterPluginPanel
 */

import { useEffect, useRef, useState } from "react";
import type { PluginPanelEntry } from "./controlCenterTypes";

type Props = {
  panel: PluginPanelEntry;
  baseUrl: string;
};

/** Message types exchanged between the parent host and the sandboxed iframe. */
const MSG_HOST_READY = "spanda-plugin-host-ready";
const MSG_BUNDLE = "spanda-plugin-bundle";
const MSG_READY = "spanda-plugin-ready";
const MSG_ERROR = "spanda-plugin-error";

/**
 * Minimal HTML document that runs inside the sandboxed iframe and executes
 * plugin bundles received over postMessage.
 */
const IFRAME_SRCDOC = `<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="utf-8" />
  <style>
    html, body { margin: 0; padding: 0; font: 13px/1.4 system-ui, sans-serif; color: #e6edf3; background: transparent; }
    #root { padding: 0.5rem; }
    .hint { opacity: 0.7; }
  </style>
</head>
<body>
  <div id="root"><p class="hint">Waiting for plugin bundle…</p></div>
  <script>
    (function () {
      var root = document.getElementById("root");
      window.addEventListener("message", function (event) {
        var data = event.data;
        if (!data || data.type !== "${MSG_BUNDLE}") return;
        try {
          root.innerHTML = "";
          var mount = document.createElement("div");
          mount.id = "plugin-mount";
          mount.dataset.plugin = data.plugin || "";
          mount.dataset.component = data.component || "";
          mount.dataset.panelId = data.panelId || "";
          root.appendChild(mount);
          var script = document.createElement("script");
          script.type = "text/javascript";
          script.textContent = String(data.bundle || "");
          document.body.appendChild(script);
          parent.postMessage({
            type: "${MSG_READY}",
            plugin: data.plugin,
            panelId: data.panelId
          }, "*");
        } catch (err) {
          parent.postMessage({
            type: "${MSG_ERROR}",
            plugin: data.plugin,
            panelId: data.panelId,
            error: String(err)
          }, "*");
        }
      });
      parent.postMessage({ type: "${MSG_HOST_READY}" }, "*");
    })();
  </script>
</body>
</html>`;

export function ControlCenterPluginPanel({ panel, baseUrl }: Props) {
  // Render a Control Center UI plugin panel inside a sandboxed iframe.
  //
  // Parameters:
  // - `panel` — plugin panel contribution (plugin name, id, component)
  // - `baseUrl` — Control Center REST base URL
  //
  // Returns:
  // React element hosting the sandboxed panel.
  //
  // Options:
  // None.
  //
  // Example:
  // <ControlCenterPluginPanel panel={entry} baseUrl={base} />

  const iframeRef = useRef<HTMLIFrameElement>(null);
  const [bundleStatus, setBundleStatus] = useState<"idle" | "loading" | "loaded" | "missing">(
    "idle",
  );
  const [bundleError, setBundleError] = useState<string | null>(null);
  const [signedHint, setSignedHint] = useState<string | null>(null);
  const pendingBundle = useRef<{
    bundle: string;
    plugin: string;
    component: string;
    panelId: string;
  } | null>(null);
  const hostReady = useRef(false);

  useEffect(() => {
    // Deliver a queued bundle once the iframe host signals readiness.
    const deliver = () => {
      const iframe = iframeRef.current;
      const payload = pendingBundle.current;
      if (!iframe?.contentWindow || !payload || !hostReady.current) return;
      iframe.contentWindow.postMessage(
        {
          type: MSG_BUNDLE,
          bundle: payload.bundle,
          plugin: payload.plugin,
          component: payload.component,
          panelId: payload.panelId,
        },
        "*",
      );
    };

    // Accept ready / error acknowledgements only from our iframe window.
    const onMessage = (event: MessageEvent) => {
      if (event.source !== iframeRef.current?.contentWindow) return;
      const data = event.data;
      if (!data || typeof data !== "object") return;
      if (data.type === MSG_HOST_READY) {
        hostReady.current = true;
        deliver();
        return;
      }
      if (data.type === MSG_READY && data.panelId === panel.id) {
        setBundleStatus("loaded");
        return;
      }
      if (data.type === MSG_ERROR && data.panelId === panel.id) {
        setBundleError(String(data.error ?? "plugin error"));
        setBundleStatus("missing");
      }
    };

    window.addEventListener("message", onMessage);
    let cancelled = false;

    // Fetch the panel bundle from REST and queue it for the iframe.
    const load = async () => {
      setBundleStatus("loading");
      setBundleError(null);
      setSignedHint(null);
      hostReady.current = false;
      pendingBundle.current = null;
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
        // Surface install-time signature policy for operators.
        if (body.signed === true) {
          setSignedHint(
            "Official signed plugin — Ed25519 signature verified at install (server-side).",
          );
        } else if (body.signature_check === "install-time") {
          setSignedHint("Signature verification runs at install time (see plugin-security.md).");
        }
        pendingBundle.current = {
          bundle: String(body.bundle),
          plugin: panel.plugin,
          component: panel.component ?? "",
          panelId: panel.id,
        };
        deliver();
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
      window.removeEventListener("message", onMessage);
    };
  }, [baseUrl, panel.id, panel.plugin, panel.component]);

  return (
    <section className="cc-panel">
      <h3>{panel.title}</h3>
      <p className="cc-section-hint">
        Plugin <code>{panel.plugin}</code> — component <code>{panel.component}</code>.
        {bundleStatus === "loaded"
          ? " Bundle running in sandboxed iframe."
          : bundleStatus === "loading"
            ? " Loading bundle…"
            : " No bundle artifact — build index.js in the plugin directory."}
      </p>
      {signedHint && <p className="demo-hint">{signedHint}</p>}
      {bundleError && <p className="demo-hint">{bundleError}</p>}
      <iframe
        ref={iframeRef}
        className="cc-plugin-host cc-plugin-sandbox"
        title={`Plugin panel ${panel.plugin}:${panel.id}`}
        sandbox="allow-scripts"
        srcDoc={IFRAME_SRCDOC}
        data-plugin-host={`${panel.plugin}:${panel.id}`}
        data-plugin={panel.plugin}
        data-component={panel.component}
      />
    </section>
  );
}

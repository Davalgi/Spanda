import { useState } from "react";
import { RBAC_ACTIONS } from "./controlCenterRbac";

type Props = {
  apiHost: string;
  effectiveRole: string;
  roleMeta: { label: string; summary: string };
  keyId?: string;
  tenantId?: string;
  permissions: string[];
  hasToken: boolean;
  showAuthSetup: boolean;
  envKeyLocked: boolean;
  authError: string | null;
  onVerify: (token: string, persist: boolean) => Promise<void>;
  onForget: () => void;
  onOpenSetup: () => void;
};

export function ControlCenterAuthBanner({
  apiHost,
  effectiveRole,
  roleMeta,
  keyId,
  tenantId,
  permissions,
  hasToken,
  showAuthSetup,
  envKeyLocked,
  authError,
  onVerify,
  onForget,
  onOpenSetup,
}: Props) {
  const [input, setInput] = useState("");
  const [remember, setRemember] = useState(true);
  const [busy, setBusy] = useState(false);
  const [localError, setLocalError] = useState<string | null>(null);

  const submit = async () => {
    setBusy(true);
    setLocalError(null);
    try {
      await onVerify(input, remember);
      setInput("");
    } catch (error) {
      setLocalError(String(error));
    } finally {
      setBusy(false);
    }
  };

  return (
    <>
      {hasToken && !showAuthSetup && (
        <div className="cc-auth-status">
          <span className={`cc-role-badge ${effectiveRole}`}>{roleMeta.label}</span>
          {keyId ? (
            <>
              {" "}
              — <code>{keyId}</code>
            </>
          ) : null}
          {tenantId ? (
            <>
              {" "}
              · tenant <code>{tenantId}</code>
            </>
          ) : null}
          {" on "}
          <code>{apiHost}</code>
          <span className="demo-hint"> — {roleMeta.summary}</span>
          <div className="cc-perm-tags">
            {RBAC_ACTIONS.map((action) => (
              <span
                key={action}
                className={`cc-perm-tag${permissions.includes(action) ? " ok" : ""}`}
              >
                {action}
              </span>
            ))}
          </div>
          {!envKeyLocked && (
            <button type="button" className="secondary cc-forget-btn" onClick={onForget}>
              Forget token
            </button>
          )}
        </div>
      )}

      {!hasToken && !showAuthSetup && (
        <p className="demo-hint">
          Mutations require a Bearer token.{" "}
          <button type="button" onClick={onOpenSetup}>
            Sign in
          </button>
        </p>
      )}

      {showAuthSetup && !envKeyLocked && (
        <section className="cc-auth-setup">
          <h3>Operator API key</h3>
          <p className="demo-hint">
            Mutations require a Bearer token registered on the server (
            <code>SPANDA_API_KEY</code> or <code>SPANDA_API_KEYS_FILE</code>).
          </p>
          <pre>spanda control-center api-key generate --export</pre>
          <label>
            Bearer token
            <input
              type="password"
              autoComplete="off"
              value={input}
              onChange={(event) => setInput(event.target.value)}
              placeholder="paste token"
            />
          </label>
          <label className="cc-remember-row">
            <input
              type="checkbox"
              checked={remember}
              onChange={(event) => setRemember(event.target.checked)}
            />
            Remember on this browser ({apiHost})
          </label>
          <button type="button" onClick={() => void submit()} disabled={busy || !input.trim()}>
            {busy ? "Verifying…" : "Use token"}
          </button>
          {(localError || authError) && (
            <p className="error">{localError ?? authError}</p>
          )}
        </section>
      )}

      {envKeyLocked && (
        <p className="demo-hint">
          API key loaded from <code>VITE_SPANDA_API_KEY</code> (build-time).
        </p>
      )}
    </>
  );
}

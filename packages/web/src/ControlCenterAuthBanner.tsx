import { useState } from "react";
import type { ControlCenterProfile } from "./controlCenterProfiles";
import { RBAC_ACTIONS } from "./controlCenterRbac";

type Props = {
  /** Tighter layout for the sidebar footer (drops redundant host/summary copy). */
  compact?: boolean;
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
  profiles: ControlCenterProfile[];
  activeProfileId?: string;
  onSwitchProfile: (profileId: string) => void;
  onAddConnection: (apiBase: string) => void;
  onVerify: (token: string, persist: boolean) => Promise<void>;
  onSignInWithOidc?: () => Promise<void>;
  oidcLoginEnabled?: boolean;
  onForget: () => void;
  onOpenSetup: () => void;
};

export function ControlCenterAuthBanner({
  compact = false,
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
  profiles,
  activeProfileId,
  onSwitchProfile,
  onAddConnection,
  onVerify,
  onSignInWithOidc,
  oidcLoginEnabled = false,
  onForget,
  onOpenSetup,
}: Props) {
  const [input, setInput] = useState("");
  const [remember, setRemember] = useState(true);
  const [busy, setBusy] = useState(false);
  const [localError, setLocalError] = useState<string | null>(null);
  const [newHost, setNewHost] = useState("");
  const [showAddConnection, setShowAddConnection] = useState(false);

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
      {profiles.length > 0 && (
        <div className={`cc-profile-switcher${compact ? " cc-profile-switcher-compact" : ""}`}>
          <label>
            {compact ? null : "Connection"}
            <select
              aria-label="Connection"
              value={activeProfileId ?? profiles[0]?.id}
              onChange={(event) => onSwitchProfile(event.target.value)}
            >
              {profiles.map((profile) => (
                <option key={profile.id} value={profile.id}>
                  {profile.label}
                  {profile.tenantId ? ` · ${profile.tenantId}` : ""}
                </option>
              ))}
            </select>
          </label>
          <button
            type="button"
            className="secondary cc-profile-add-btn"
            onClick={() => setShowAddConnection((v) => !v)}
            aria-expanded={showAddConnection}
            aria-label="Add connection"
          >
            {compact ? "+" : "Add"}
          </button>
        </div>
      )}

      {showAddConnection && (
        <div className="cc-profile-add">
          <input
            type="url"
            placeholder="https://fleet.example.com:8080"
            value={newHost}
            onChange={(event) => setNewHost(event.target.value)}
          />
          <button
            type="button"
            disabled={!newHost.trim()}
            onClick={() => {
              onAddConnection(newHost.trim());
              setNewHost("");
              setShowAddConnection(false);
            }}
          >
            Save connection
          </button>
        </div>
      )}

      {hasToken && !showAuthSetup && (
        <div className={`cc-auth-status${compact ? " cc-auth-status-compact" : ""}`}>
          {compact ? (
            <>
              <div className="cc-auth-status-row cc-auth-status-row-top">
                <span className={`cc-role-badge ${effectiveRole}`}>{roleMeta.label}</span>
                {!envKeyLocked && (
                  <button type="button" className="secondary cc-forget-btn" onClick={onForget}>
                    Forget
                  </button>
                )}
              </div>
              {(keyId || tenantId) && (
                <div className="cc-auth-status-row cc-auth-status-row-meta">
                  {keyId ? (
                    <span className="cc-auth-identity">
                      <code>{keyId}</code>
                    </span>
                  ) : null}
                  {tenantId ? (
                    <span className="cc-auth-tenant">
                      <code>{tenantId}</code>
                    </span>
                  ) : null}
                </div>
              )}
            </>
          ) : (
            <>
              <div className="cc-auth-status-row">
                <span className={`cc-role-badge ${effectiveRole}`}>{roleMeta.label}</span>
                {keyId ? (
                  <span className="cc-auth-identity">
                    <code>{keyId}</code>
                  </span>
                ) : null}
                {tenantId ? (
                  <span className="cc-auth-tenant">
                    <code>{tenantId}</code>
                  </span>
                ) : null}
                {!envKeyLocked && (
                  <button type="button" className="secondary cc-forget-btn" onClick={onForget}>
                    Forget token
                  </button>
                )}
              </div>
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
            </>
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
          {oidcLoginEnabled && onSignInWithOidc ? (
            <button
              type="button"
              className="secondary"
              disabled={busy}
              onClick={() => {
                setBusy(true);
                setLocalError(null);
                void onSignInWithOidc()
                  .catch((error) => setLocalError(String(error)))
                  .finally(() => setBusy(false));
              }}
            >
              Sign in with SSO
            </button>
          ) : null}
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

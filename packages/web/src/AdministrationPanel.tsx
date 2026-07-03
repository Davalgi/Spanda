import { useCallback, useEffect, useState } from "react";
import type { RbacAction } from "./controlCenterRbac";

type ApiKeyRow = {
  key_id: string;
  role: string;
  label?: string;
  tenant_id?: string;
  source?: string;
};

type UserRow = {
  user_id: string;
  display_name: string;
  email?: string;
  role: string;
  api_key_id?: string;
  enabled: boolean;
};

type ScheduleRow = {
  id: string;
  profile: string;
  format: string;
  destination_url: string;
  interval_hours: number;
  enabled: boolean;
  last_status?: string;
};

type TwinSummary = {
  twin_id: string;
  program: string;
  readiness_score: number;
  mission_ready: boolean;
  history_count?: number;
};

type Props = {
  baseUrl: string;
  authHeaders: () => HeadersInit;
  can: (action: RbacAction) => boolean;
  hasToken: boolean;
};

const ROLES = [
  "administrator",
  "supervisor",
  "developer",
  "operator",
  "safety_officer",
  "auditor",
];

const CHANNEL_TYPES = ["webhook", "email", "pagerduty", "teams", "log"] as const;

export function AdministrationPanel({ baseUrl, authHeaders, can, hasToken }: Props) {
  const [keys, setKeys] = useState<ApiKeyRow[]>([]);
  const [users, setUsers] = useState<UserRow[]>([]);
  const [secrets, setSecrets] = useState<Record<string, unknown>[]>([]);
  const [schedules, setSchedules] = useState<ScheduleRow[]>([]);
  const [integrations, setIntegrations] = useState<Record<string, unknown> | null>(null);
  const [twins, setTwins] = useState<TwinSummary[]>([]);
  const [alertChannelsJson, setAlertChannelsJson] = useState("[]");
  const [persistPath, setPersistPath] = useState("");
  const [usersPath, setUsersPath] = useState("");
  const [newRole, setNewRole] = useState("operator");
  const [newLabel, setNewLabel] = useState("");
  const [newUserId, setNewUserId] = useState("");
  const [newUserName, setNewUserName] = useState("");
  const [newUserEmail, setNewUserEmail] = useState("");
  const [newUserRole, setNewUserRole] = useState("operator");
  const [channelType, setChannelType] = useState<(typeof CHANNEL_TYPES)[number]>("webhook");
  const [channelUrl, setChannelUrl] = useState("");
  const [channelEmail, setChannelEmail] = useState("");
  const [channelRoutingKey, setChannelRoutingKey] = useState("spanda");
  const [createdToken, setCreatedToken] = useState<string | null>(null);
  const [scheduleProfile, setScheduleProfile] = useState("defense");
  const [scheduleUrl, setScheduleUrl] = useState("");
  const [busy, setBusy] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const load = useCallback(async () => {
    if (!hasToken) return;
    setBusy(true);
    setError(null);
    try {
      const headers = authHeaders();
      const [keysRes, usersRes, secretsRes, schedulesRes, integrationsRes, channelsRes, twinsRes] =
        await Promise.all([
          fetch(`${baseUrl}/v1/admin/api-keys`, { headers }),
          fetch(`${baseUrl}/v1/admin/users`, { headers }),
          can("Deploy") ? fetch(`${baseUrl}/v1/secrets`, { headers }) : Promise.resolve(null),
          fetch(`${baseUrl}/v1/reports/schedules`),
          can("Deploy")
            ? fetch(`${baseUrl}/v1/admin/integrations`, { headers })
            : Promise.resolve(null),
          fetch(`${baseUrl}/v1/admin/alert-channels`, { headers }),
          fetch(`${baseUrl}/v1/twins`, { headers }),
        ]);
      if (keysRes.ok) {
        const body = await keysRes.json();
        setKeys(body.keys ?? []);
        setPersistPath(body.persist_path ?? "");
      }
      if (usersRes.ok) {
        const body = await usersRes.json();
        setUsers(body.users ?? []);
        setUsersPath(body.persist_path ?? "");
      }
      if (secretsRes?.ok) {
        const body = await secretsRes.json();
        setSecrets(body.secrets ?? []);
      }
      const schedulesBody = await (schedulesRes.ok ? schedulesRes.json() : { schedules: [] });
      setSchedules(schedulesBody.schedules ?? []);
      if (integrationsRes?.ok) {
        setIntegrations(await integrationsRes.json());
      }
      if (channelsRes?.ok) {
        const body = await channelsRes.json();
        setAlertChannelsJson(JSON.stringify(body.channels ?? [], null, 2));
      }
      if (twinsRes.ok) {
        const body = await twinsRes.json();
        setTwins(body.twins ?? []);
      }
    } catch (e) {
      setError(String(e));
    } finally {
      setBusy(false);
    }
  }, [authHeaders, baseUrl, can, hasToken]);

  const syncTwinCloud = async () => {
    if (!can("Operate")) return;
    setBusy(true);
    try {
      const res = await fetch(`${baseUrl}/v1/twins/sync`, {
        method: "POST",
        headers: { "Content-Type": "application/json", ...authHeaders() },
        body: "{}",
      });
      if (!res.ok) throw new Error(`sync twin ${res.status}`);
      await load();
    } catch (e) {
      setError(String(e));
    } finally {
      setBusy(false);
    }
  };

  useEffect(() => {
    void load();
  }, [load]);

  const createKey = async () => {
    setBusy(true);
    setCreatedToken(null);
    try {
      const res = await fetch(`${baseUrl}/v1/admin/api-keys`, {
        method: "POST",
        headers: authHeaders(),
        body: JSON.stringify({ role: newRole, label: newLabel || undefined }),
      });
      if (!res.ok) throw new Error(`create key ${res.status}`);
      const body = await res.json();
      setCreatedToken(body.token ?? null);
      await load();
    } catch (e) {
      setError(String(e));
    } finally {
      setBusy(false);
    }
  };

  const revokeKey = async (keyId: string) => {
    if (keyId === "env-default") return;
    setBusy(true);
    try {
      const res = await fetch(`${baseUrl}/v1/admin/api-keys/${encodeURIComponent(keyId)}`, {
        method: "DELETE",
        headers: authHeaders(),
      });
      if (!res.ok) throw new Error(`revoke ${res.status}`);
      await load();
    } catch (e) {
      setError(String(e));
    } finally {
      setBusy(false);
    }
  };

  const patchKeyRole = async (keyId: string, role: string) => {
    if (keyId === "env-default") return;
    setBusy(true);
    try {
      const res = await fetch(`${baseUrl}/v1/admin/api-keys/${encodeURIComponent(keyId)}`, {
        method: "PATCH",
        headers: authHeaders(),
        body: JSON.stringify({ role }),
      });
      if (!res.ok) throw new Error(`patch ${res.status}`);
      await load();
    } catch (e) {
      setError(String(e));
    } finally {
      setBusy(false);
    }
  };

  const createUser = async () => {
    if (!newUserId.trim() || !newUserName.trim()) return;
    setBusy(true);
    try {
      const res = await fetch(`${baseUrl}/v1/admin/users`, {
        method: "POST",
        headers: authHeaders(),
        body: JSON.stringify({
          user_id: newUserId,
          display_name: newUserName,
          email: newUserEmail || undefined,
          role: newUserRole,
        }),
      });
      if (!res.ok) throw new Error(`create user ${res.status}`);
      setNewUserId("");
      setNewUserName("");
      setNewUserEmail("");
      await load();
    } catch (e) {
      setError(String(e));
    } finally {
      setBusy(false);
    }
  };

  const toggleUser = async (userId: string, enabled: boolean) => {
    setBusy(true);
    try {
      const res = await fetch(`${baseUrl}/v1/admin/users/${encodeURIComponent(userId)}`, {
        method: "PATCH",
        headers: authHeaders(),
        body: JSON.stringify({ enabled }),
      });
      if (!res.ok) throw new Error(`patch user ${res.status}`);
      await load();
    } catch (e) {
      setError(String(e));
    } finally {
      setBusy(false);
    }
  };

  const deleteUser = async (userId: string) => {
    setBusy(true);
    try {
      const res = await fetch(`${baseUrl}/v1/admin/users/${encodeURIComponent(userId)}`, {
        method: "DELETE",
        headers: authHeaders(),
      });
      if (!res.ok) throw new Error(`delete user ${res.status}`);
      await load();
    } catch (e) {
      setError(String(e));
    } finally {
      setBusy(false);
    }
  };

  const addChannelToJson = () => {
    let channels: unknown[] = [];
    try {
      channels = JSON.parse(alertChannelsJson) as unknown[];
    } catch {
      channels = [];
    }
    let entry: Record<string, unknown>;
    if (channelType === "log") {
      entry = { log: null };
    } else if (channelType === "email") {
      entry = { email: { to: channelEmail } };
    } else if (channelType === "pagerduty") {
      entry = {
        pagerduty: { url: channelUrl, routing_key: channelRoutingKey },
      };
    } else if (channelType === "teams") {
      entry = { teams: { url: channelUrl } };
    } else {
      entry = { webhook: { url: channelUrl } };
    }
    channels.push(entry);
    setAlertChannelsJson(JSON.stringify(channels, null, 2));
  };

  const saveAlertChannels = async () => {
    setBusy(true);
    try {
      const channels = JSON.parse(alertChannelsJson) as unknown[];
      const res = await fetch(`${baseUrl}/v1/admin/alert-channels`, {
        method: "PUT",
        headers: authHeaders(),
        body: JSON.stringify({ channels, use_env_fallback: false }),
      });
      if (!res.ok) throw new Error(`alert channels ${res.status}`);
      await load();
    } catch (e) {
      setError(String(e));
    } finally {
      setBusy(false);
    }
  };

  const createSchedule = async () => {
    if (!can("Deploy") || !scheduleUrl.trim()) return;
    setBusy(true);
    try {
      const res = await fetch(`${baseUrl}/v1/reports/schedules`, {
        method: "POST",
        headers: authHeaders(),
        body: JSON.stringify({
          profile: scheduleProfile,
          destination_url: scheduleUrl,
          format: "markdown",
          interval_hours: 24,
        }),
      });
      if (!res.ok) throw new Error(`schedule ${res.status}`);
      setScheduleUrl("");
      await load();
    } catch (e) {
      setError(String(e));
    } finally {
      setBusy(false);
    }
  };

  if (!hasToken) {
    return <p className="demo-hint">Sign in as administrator to manage users and integrations.</p>;
  }

  if (!can("Delete")) {
    return (
      <p className="demo-hint">
        Administration requires the <strong>administrator</strong> role.
      </p>
    );
  }

  return (
    <section className="cc-admin-panel">
      <header className="cc-section-header">
        <h3>Administration</h3>
        <button type="button" onClick={() => void load()} disabled={busy}>
          Refresh
        </button>
      </header>
      {error && <p className="error">{error}</p>}

      <h4>User directory</h4>
      <p className="demo-hint">Persist path: <code>{usersPath || "—"}</code></p>
      <table>
        <thead>
          <tr>
            <th>User ID</th>
            <th>Name</th>
            <th>Email</th>
            <th>Role</th>
            <th>API key</th>
            <th>Enabled</th>
            <th>Actions</th>
          </tr>
        </thead>
        <tbody>
          {users.map((user) => (
            <tr key={user.user_id}>
              <td><code>{user.user_id}</code></td>
              <td>{user.display_name}</td>
              <td>{user.email ?? "—"}</td>
              <td>{user.role}</td>
              <td>{user.api_key_id ?? "—"}</td>
              <td>{user.enabled ? "yes" : "no"}</td>
              <td className="cc-action-bar">
                <button type="button" onClick={() => void toggleUser(user.user_id, !user.enabled)} disabled={busy}>
                  {user.enabled ? "Disable" : "Enable"}
                </button>
                <button type="button" onClick={() => void deleteUser(user.user_id)} disabled={busy}>
                  Delete
                </button>
              </td>
            </tr>
          ))}
          {users.length === 0 && (
            <tr>
              <td colSpan={7}>No users — add one or load a config with human operators.</td>
            </tr>
          )}
        </tbody>
      </table>
      <div className="digital-thread-filters">
        <label>User ID<input value={newUserId} onChange={(e) => setNewUserId(e.target.value)} /></label>
        <label>Display name<input value={newUserName} onChange={(e) => setNewUserName(e.target.value)} /></label>
        <label>Email<input value={newUserEmail} onChange={(e) => setNewUserEmail(e.target.value)} /></label>
        <label>
          Role
          <select value={newUserRole} onChange={(e) => setNewUserRole(e.target.value)}>
            {ROLES.map((role) => (
              <option key={role} value={role}>{role}</option>
            ))}
          </select>
        </label>
        <button type="button" onClick={() => void createUser()} disabled={busy}>Add user</button>
      </div>

      <h4>API keys</h4>
      <p className="demo-hint">Persist path: <code>{persistPath || "—"}</code></p>
      <table>
        <thead>
          <tr>
            <th>Key ID</th>
            <th>Role</th>
            <th>Label</th>
            <th>Source</th>
            <th>Actions</th>
          </tr>
        </thead>
        <tbody>
          {keys.map((key) => (
            <tr key={key.key_id}>
              <td><code>{key.key_id}</code></td>
              <td>
                {key.key_id === "env-default" ? (
                  key.role
                ) : (
                  <select
                    value={String(key.role).replace(/^Role::/, "").toLowerCase()}
                    onChange={(event) => void patchKeyRole(key.key_id, event.target.value)}
                  >
                    {ROLES.map((role) => (
                      <option key={role} value={role}>{role}</option>
                    ))}
                  </select>
                )}
              </td>
              <td>{key.label ?? "—"}</td>
              <td>{key.source ?? "file"}</td>
              <td>
                {key.key_id !== "env-default" && (
                  <button type="button" onClick={() => void revokeKey(key.key_id)} disabled={busy}>
                    Revoke
                  </button>
                )}
              </td>
            </tr>
          ))}
        </tbody>
      </table>
      <div className="digital-thread-filters">
        <label>
          Role
          <select value={newRole} onChange={(event) => setNewRole(event.target.value)}>
            {ROLES.map((role) => (
              <option key={role} value={role}>{role}</option>
            ))}
          </select>
        </label>
        <label>Label<input value={newLabel} onChange={(event) => setNewLabel(event.target.value)} /></label>
        <button type="button" onClick={() => void createKey()} disabled={busy}>Create API key</button>
      </div>
      {createdToken && (
        <p className="cc-created-token">New token (copy now): <code>{createdToken}</code></p>
      )}

      <h4>Alert channels</h4>
      <div className="digital-thread-filters">
        <label>
          Type
          <select value={channelType} onChange={(e) => setChannelType(e.target.value as typeof channelType)}>
            {CHANNEL_TYPES.map((t) => (
              <option key={t} value={t}>{t}</option>
            ))}
          </select>
        </label>
        {(channelType === "webhook" || channelType === "pagerduty" || channelType === "teams") && (
          <label>URL<input value={channelUrl} onChange={(e) => setChannelUrl(e.target.value)} /></label>
        )}
        {channelType === "email" && (
          <label>To<input value={channelEmail} onChange={(e) => setChannelEmail(e.target.value)} /></label>
        )}
        {channelType === "pagerduty" && (
          <label>Routing key<input value={channelRoutingKey} onChange={(e) => setChannelRoutingKey(e.target.value)} /></label>
        )}
        <button type="button" onClick={addChannelToJson}>Add to list</button>
        <button type="button" onClick={() => void saveAlertChannels()} disabled={busy}>Save channels</button>
      </div>
      <textarea
        className="cc-channels-json"
        rows={8}
        value={alertChannelsJson}
        onChange={(e) => setAlertChannelsJson(e.target.value)}
      />

      <h4>Secrets metadata</h4>
      {can("Deploy") ? (
        secrets.length > 0 ? (
          <pre>{JSON.stringify(secrets, null, 2)}</pre>
        ) : (
          <p className="demo-hint">No secret metadata registered.</p>
        )
      ) : (
        <p className="demo-hint">Deploy permission required to list secrets.</p>
      )}

      <h4>Report schedules</h4>
      <ul>
        {schedules.map((schedule) => (
          <li key={schedule.id}>
            <code>{schedule.id}</code> — {schedule.profile} → {schedule.destination_url} (
            {schedule.interval_hours}h) {schedule.last_status ?? ""}
          </li>
        ))}
        {schedules.length === 0 && <li>No schedules</li>}
      </ul>
      {can("Deploy") && (
        <div className="digital-thread-filters">
          <label>Profile<input value={scheduleProfile} onChange={(e) => setScheduleProfile(e.target.value)} /></label>
          <label>Webhook URL<input value={scheduleUrl} onChange={(e) => setScheduleUrl(e.target.value)} placeholder="https://hooks.example.com/reports" /></label>
          <button type="button" onClick={() => void createSchedule()} disabled={busy}>Add schedule</button>
        </div>
      )}

      <h4>Twin Cloud registry</h4>
      <p className="demo-hint">
        Mission twin snapshots from edge push or sync. Mutations require Operate permission.
      </p>
      <div className="digital-thread-filters">
        <button type="button" onClick={() => void load()} disabled={busy}>
          Refresh twins
        </button>
        {can("Operate") && (
          <button type="button" onClick={() => void syncTwinCloud()} disabled={busy}>
            Sync loaded program
          </button>
        )}
      </div>
      <table>
        <thead>
          <tr>
            <th>Twin</th>
            <th>Program</th>
            <th>Readiness</th>
            <th>Mission ready</th>
            <th>History</th>
          </tr>
        </thead>
        <tbody>
          {twins.map((twin) => (
            <tr key={twin.twin_id}>
              <td><code>{twin.twin_id}</code></td>
              <td>{twin.program}</td>
              <td>{twin.readiness_score}</td>
              <td>{twin.mission_ready ? "yes" : "no"}</td>
              <td>{twin.history_count ?? 1}</td>
            </tr>
          ))}
          {twins.length === 0 && (
            <tr>
              <td colSpan={5}>No twins registered — use Twin Cloud tab or CLI push.</td>
            </tr>
          )}
        </tbody>
      </table>

      <h4>Integrations summary</h4>
      {integrations ? <pre>{JSON.stringify(integrations, null, 2)}</pre> : <p className="demo-hint">Loading…</p>}

      <OidcSlackAdmin baseUrl={baseUrl} authHeaders={authHeaders} can={can} hasToken={hasToken} busy={busy} />
    </section>
  );
}

function OidcSlackAdmin({
  baseUrl,
  authHeaders,
  can,
  hasToken,
  busy,
}: {
  baseUrl: string;
  authHeaders: () => HeadersInit;
  can: (action: import("./controlCenterRbac").RbacAction) => boolean;
  hasToken: boolean;
  busy: boolean;
}) {
  const [oidc, setOidc] = useState<Record<string, unknown> | null>(null);
  const [slack, setSlack] = useState<Record<string, unknown> | null>(null);
  const [issuer, setIssuer] = useState("");
  const [clientId, setClientId] = useState("");
  const [clientSecret, setClientSecret] = useState("");
  const [redirectUri, setRedirectUri] = useState("");
  const [groupRoleMap, setGroupRoleMap] = useState("{}");
  const [oidcCode, setOidcCode] = useState("");
  const [oidcState, setOidcState] = useState("");
  const [slackWebhook, setSlackWebhook] = useState("");
  const [slackClientId, setSlackClientId] = useState("");
  const [slackClientSecret, setSlackClientSecret] = useState("");
  const [slackRedirectUri, setSlackRedirectUri] = useState("");
  const [slackCode, setSlackCode] = useState("");
  const [slackState, setSlackState] = useState("");
  const [status, setStatus] = useState<string | null>(null);

  const loadAdmin = useCallback(async () => {
    if (!hasToken) return;
    const headers = authHeaders();
    const [oidcRes, slackRes] = await Promise.all([
      fetch(`${baseUrl}/v1/admin/oidc`, { headers }),
      fetch(`${baseUrl}/v1/admin/slack`, { headers }),
    ]);
    if (oidcRes.ok) {
      const body = await oidcRes.json();
      setOidc(body);
      setIssuer(String(body.issuer ?? ""));
      setClientId(String(body.client_id ?? ""));
      setRedirectUri(String(body.redirect_uri ?? ""));
      if (body.group_role_map) {
        setGroupRoleMap(JSON.stringify(body.group_role_map, null, 2));
      }
    }
    if (slackRes.ok) {
      const body = await slackRes.json();
      setSlack(body);
      setSlackClientId(String(body.oauth_client_id ?? ""));
      setSlackRedirectUri(String(body.oauth_redirect_uri ?? ""));
    }
  }, [authHeaders, baseUrl, hasToken]);

  useEffect(() => {
    void loadAdmin();
  }, [loadAdmin]);

  const completeOidcOAuth = useCallback(
    async (code?: string, state?: string) => {
      const authCode = (code ?? oidcCode).trim();
      const authState = state ?? oidcState;
      if (!can("Deploy") || !authCode) return;
      const res = await fetch(`${baseUrl}/v1/admin/oidc/oauth/callback`, {
        method: "POST",
        headers: authHeaders(),
        body: JSON.stringify({ code: authCode, state: authState || undefined }),
      });
      const body = await res.json();
      setStatus(
        res.ok
          ? `OAuth import: ${body.users_created ?? 0} created, ${body.users_updated ?? 0} updated.`
          : `OAuth callback failed: ${res.status}`,
      );
      setOidcCode("");
      await loadAdmin();
    },
    [authHeaders, baseUrl, can, loadAdmin, oidcCode, oidcState],
  );

  const completeSlackOAuth = useCallback(
    async (code?: string, state?: string) => {
      const authCode = (code ?? slackCode).trim();
      const authState = state ?? slackState;
      if (!can("Deploy") || !authCode) return;
      const res = await fetch(`${baseUrl}/v1/admin/slack/oauth/callback`, {
        method: "POST",
        headers: authHeaders(),
        body: JSON.stringify({ code: authCode, state: authState || undefined }),
      });
      const body = await res.json();
      setStatus(
        res.ok ? `Slack connected: ${body.team_name ?? "team"}.` : `Slack OAuth failed: ${res.status}`,
      );
      setSlackCode("");
      await loadAdmin();
    },
    [authHeaders, baseUrl, can, loadAdmin, slackCode, slackState],
  );

  useEffect(() => {
    const onMessage = (event: MessageEvent) => {
      if (event.origin !== window.location.origin) return;
      if (event.data?.type === "spanda-oidc-oauth" && event.data.code) {
        const nextCode = String(event.data.code);
        const nextState = event.data.state ? String(event.data.state) : "";
        setOidcCode(nextCode);
        setOidcState(nextState);
        void completeOidcOAuth(nextCode, nextState || undefined);
      }
      if (event.data?.type === "spanda-slack-oauth" && event.data.code) {
        const nextCode = String(event.data.code);
        const nextState = event.data.state ? String(event.data.state) : "";
        setSlackCode(nextCode);
        setSlackState(nextState);
        void completeSlackOAuth(nextCode, nextState || undefined);
      }
    };
    window.addEventListener("message", onMessage);
    return () => window.removeEventListener("message", onMessage);
  }, [completeOidcOAuth, completeSlackOAuth]);

  const parseGroupRoleMap = () => {
    try {
      return JSON.parse(groupRoleMap) as Record<string, string>;
    } catch {
      return {};
    }
  };

  const saveOidc = async () => {
    if (!can("Deploy")) return;
    await fetch(`${baseUrl}/v1/admin/oidc`, {
      method: "PUT",
      headers: authHeaders(),
      body: JSON.stringify({
        enabled: true,
        issuer,
        client_id: clientId,
        client_secret: clientSecret || undefined,
        redirect_uri: redirectUri || undefined,
        group_role_map: parseGroupRoleMap(),
      }),
    });
    setClientSecret("");
    await loadAdmin();
    setStatus("OIDC settings saved.");
  };

  const syncOidc = async () => {
    if (!can("Deploy")) return;
    const res = await fetch(`${baseUrl}/v1/admin/oidc/sync`, {
      method: "POST",
      headers: authHeaders(),
      body: "{}",
    });
    const body = await res.json();
    setStatus(
      res.ok
        ? `Directory sync: ${body.users_created ?? 0} created, ${body.users_updated ?? 0} updated.`
        : `Sync failed: ${res.status}`,
    );
    await loadAdmin();
  };

  const startOidcOAuth = async () => {
    if (!can("Deploy")) return;
    const effectiveRedirect =
      redirectUri.trim() || `${window.location.origin}/admin/oauth/oidc/callback`;
    const res = await fetch(`${baseUrl}/v1/admin/oidc/authorize-url`, {
      method: "POST",
      headers: authHeaders(),
      body: JSON.stringify({ redirect_uri: effectiveRedirect }),
    });
    const body = await res.json();
    if (!res.ok) {
      setStatus(`OIDC authorize failed: ${res.status}`);
      return;
    }
    setOidcState(String(body.state ?? ""));
    const url = String(body.authorize_url ?? "");
    if (url) window.open(url, "_blank", "noopener,noreferrer");
    setStatus("OIDC authorization opened — callback will complete automatically.");
  };

  const submitOidcOAuth = async () => {
    await completeOidcOAuth();
  };

  const saveSlack = async () => {
    if (!can("Deploy")) return;
    await fetch(`${baseUrl}/v1/admin/slack`, {
      method: "POST",
      headers: authHeaders(),
      body: JSON.stringify({
        webhook_url: slackWebhook || undefined,
        oauth_client_id: slackClientId || undefined,
        oauth_client_secret: slackClientSecret || undefined,
        oauth_redirect_uri: slackRedirectUri || undefined,
      }),
    });
    setSlackClientSecret("");
    await loadAdmin();
    setStatus("Slack settings saved.");
  };

  const startSlackOAuth = async () => {
    if (!can("Deploy")) return;
    const effectiveRedirect =
      slackRedirectUri.trim() || `${window.location.origin}/admin/oauth/slack/callback`;
    const res = await fetch(`${baseUrl}/v1/admin/slack/oauth-url`, {
      method: "POST",
      headers: authHeaders(),
      body: JSON.stringify({ redirect_uri: effectiveRedirect }),
    });
    const body = await res.json();
    if (!res.ok) {
      setStatus(`Slack authorize failed: ${res.status}`);
      return;
    }
    setSlackState(String(body.state ?? ""));
    const url = String(body.authorize_url ?? "");
    if (url) window.open(url, "_blank", "noopener,noreferrer");
    setStatus("Slack authorization opened — callback will complete automatically.");
  };

  const submitSlackOAuth = async () => {
    await completeSlackOAuth();
  };

  return (
    <>
      {status && <p className="demo-hint">{status}</p>}

      <h4>OIDC / SSO</h4>
      {oidc ? (
        <p className="demo-hint">
          Enabled: {String(oidc.enabled)} · OAuth ready: {String(oidc.oauth_ready ?? false)} · Last sync:{" "}
          {String(oidc.last_sync_at ?? "never")}
        </p>
      ) : (
        <p className="demo-hint">Sign in to configure OIDC.</p>
      )}
      {can("Deploy") && hasToken && (
        <div className="digital-thread-filters">
          <label>
            Issuer URL
            <input value={issuer} onChange={(e) => setIssuer(e.target.value)} placeholder="https://idp.example.com" />
          </label>
          <label>
            Client ID
            <input value={clientId} onChange={(e) => setClientId(e.target.value)} />
          </label>
          <label>
            Client secret
            <input
              type="password"
              value={clientSecret}
              onChange={(e) => setClientSecret(e.target.value)}
              placeholder={oidc?.client_secret_set ? "saved (enter to replace)" : "required for OAuth"}
            />
          </label>
          <label>
            Redirect URI
            <input
              value={redirectUri}
              onChange={(e) => setRedirectUri(e.target.value)}
              placeholder="http://127.0.0.1:8080/admin/oauth/oidc/callback"
            />
          </label>
          <label>
            Group → role map (JSON)
            <textarea value={groupRoleMap} onChange={(e) => setGroupRoleMap(e.target.value)} rows={4} />
          </label>
          <button type="button" onClick={() => void saveOidc()} disabled={busy}>
            Save OIDC
          </button>
          <button type="button" onClick={() => void syncOidc()} disabled={busy}>
            Sync directory
          </button>
          <button type="button" onClick={() => void startOidcOAuth()} disabled={busy}>
            Start OIDC OAuth
          </button>
          <label>
            Authorization code
            <input value={oidcCode} onChange={(e) => setOidcCode(e.target.value)} placeholder="paste code" />
          </label>
          <button type="button" onClick={() => void submitOidcOAuth()} disabled={busy || !oidcCode.trim()}>
            Complete OIDC OAuth
          </button>
        </div>
      )}

      <h4>Slack setup wizard</h4>
      {slack ? (
        <p className="demo-hint">
          Configured: {String(slack.configured)} · Team: {String(slack.team_name ?? "—")} · Webhook:{" "}
          {String(slack.webhook_url_set ?? false)}
        </p>
      ) : null}
      {can("Deploy") && hasToken && (
        <div className="digital-thread-filters">
          <label>
            Webhook URL
            <input
              value={slackWebhook}
              onChange={(e) => setSlackWebhook(e.target.value)}
              placeholder="https://hooks.slack.com/..."
            />
          </label>
          <label>
            OAuth client ID
            <input value={slackClientId} onChange={(e) => setSlackClientId(e.target.value)} />
          </label>
          <label>
            OAuth client secret
            <input
              type="password"
              value={slackClientSecret}
              onChange={(e) => setSlackClientSecret(e.target.value)}
              placeholder={slack?.oauth_client_secret_set ? "saved (enter to replace)" : ""}
            />
          </label>
          <label>
            OAuth redirect URI
            <input
              value={slackRedirectUri}
              onChange={(e) => setSlackRedirectUri(e.target.value)}
              placeholder="http://127.0.0.1:8080/admin/oauth/slack/callback"
            />
          </label>
          <button type="button" onClick={() => void saveSlack()} disabled={busy}>
            Save Slack
          </button>
          <button type="button" onClick={() => void startSlackOAuth()} disabled={busy}>
            Start Slack OAuth
          </button>
          <label>
            Authorization code
            <input value={slackCode} onChange={(e) => setSlackCode(e.target.value)} placeholder="paste code" />
          </label>
          <button type="button" onClick={() => void submitSlackOAuth()} disabled={busy || !slackCode.trim()}>
            Complete Slack OAuth
          </button>
        </div>
      )}
    </>
  );
}

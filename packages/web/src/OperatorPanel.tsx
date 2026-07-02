import type { RbacAction } from "./controlCenterRbac";

type Props = {
  baseUrl: string;
  authHeaders: () => HeadersInit;
  can: (action: RbacAction) => boolean;
  hasToken: boolean;
  selectedDeviceId?: string;
  onQuarantine?: () => void;
};

export function OperatorPanel({
  baseUrl,
  authHeaders,
  can,
  hasToken,
  selectedDeviceId,
  onQuarantine,
}: Props) {
  const testAlert = async () => {
    if (!hasToken || !can("Operate")) return;
    await fetch(`${baseUrl}/v1/alerts/test`, {
      method: "POST",
      headers: authHeaders(),
    });
  };

  const quarantine = async () => {
    if (!hasToken || !can("Operate") || !selectedDeviceId) return;
    await fetch(`${baseUrl}/v1/operator/quarantine`, {
      method: "POST",
      headers: authHeaders(),
      body: JSON.stringify({ device_id: selectedDeviceId, reason: "operator panel" }),
    });
    onQuarantine?.();
  };

  return (
    <section>
      <h3>Operator workflows</h3>
      <p className="demo-hint">
        Mission approvals live on the Mission tab. Use quarantine for safety holds.
      </p>
      <div className="cc-action-bar">
        <button
          type="button"
          disabled={!hasToken || !can("Operate")}
          onClick={() => void testAlert()}
          title={!can("Operate") ? "Requires Operate permission" : undefined}
        >
          Test alert
        </button>
        <button
          type="button"
          disabled={!hasToken || !can("Operate") || !selectedDeviceId}
          onClick={() => void quarantine()}
          title={!selectedDeviceId ? "Select a device first" : undefined}
        >
          Quarantine selected device
        </button>
      </div>
      <ul>
        <li>
          <code>POST /v1/operator/mission/approve</code> — mission approval (Approve role)
        </li>
        <li>
          <code>POST /v1/operator/quarantine</code> — quarantine device (Operate role)
        </li>
        <li>
          <code>POST /v1/rpc</code> — gRPC-compatible JSON gateway
        </li>
      </ul>
    </section>
  );
}

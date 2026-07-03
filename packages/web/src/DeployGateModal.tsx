import { useCallback, useEffect, useState } from "react";
import { CcBadge, CcSection } from "./controlCenterUi";

type Props = {
  baseUrl: string;
  open: boolean;
  onClose: () => void;
};

export function DeployGateModal({ baseUrl, open, onClose }: Props) {
  const [gate, setGate] = useState<Record<string, unknown> | null>(null);
  const [busy, setBusy] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const load = useCallback(async () => {
    if (!open) return;
    setBusy(true);
    setError(null);
    try {
      const res = await fetch(`${baseUrl}/v1/deploy/gate`);
      if (!res.ok) throw new Error(`deploy gate ${res.status}`);
      setGate(await res.json());
    } catch (err) {
      setError(String(err));
    } finally {
      setBusy(false);
    }
  }, [baseUrl, open]);

  useEffect(() => {
    void load();
  }, [load]);

  if (!open) return null;

  const checks = (gate?.checks as Record<string, unknown>[]) ?? [];
  const ready = gate?.mission_ready === true || gate?.ready === true;

  return (
    <div className="cc-modal-backdrop" role="presentation" onClick={onClose}>
      <div
        className="cc-modal"
        role="dialog"
        aria-modal="true"
        aria-label="Deploy gate checklist"
        onClick={(event) => event.stopPropagation()}
      >
        <header className="cc-modal-header">
          <h3>Deploy gate</h3>
          <button type="button" onClick={onClose}>
            Close
          </button>
        </header>
        {error && <div className="error">{error}</div>}
        <CcSection title="Readiness checklist" hint="Pre-deploy gates from the loaded program.">
          <p>
            Status:{" "}
            <CcBadge tone={ready ? "ok" : "danger"}>{ready ? "Ready" : "Blocked"}</CcBadge>
          </p>
          {busy ? (
            <p>Loading…</p>
          ) : checks.length > 0 ? (
            <ul className="cc-checklist">
              {checks.map((check, index) => (
                <li key={`${check.name ?? index}`}>
                  <strong>{String(check.name ?? check.id ?? "check")}</strong> —{" "}
                  {String(check.status ?? check.result ?? "—")}
                </li>
              ))}
            </ul>
          ) : (
            <pre className="cc-action-result">{JSON.stringify(gate, null, 2)}</pre>
          )}
        </CcSection>
      </div>
    </div>
  );
}

import { useCallback, useEffect, useState } from "react";
import type { RbacAction } from "./controlCenterRbac";
import { CcEmptyState, CcSection } from "./controlCenterUi";
import { useRegisterTabRefresh } from "./useControlCenterTabRefresh";

type Props = {
  baseUrl: string;
  authHeaders: () => HeadersInit;
  can: (action: RbacAction) => boolean;
  hasToken: boolean;
};

export function ChaosPanel({ baseUrl, authHeaders, can, hasToken }: Props) {
  const [injections, setInjections] = useState<string[]>([]);
  const [selected, setSelected] = useState<string[]>([]);
  const [result, setResult] = useState<string | null>(null);
  const [busy, setBusy] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const load = useCallback(async () => {
    setBusy(true);
    setError(null);
    try {
      const res = await fetch(`${baseUrl}/v1/chaos/injections`);
      if (!res.ok) throw new Error(`chaos catalog ${res.status}`);
      const body = await res.json();
      const list = (body.injections as string[]) ?? [];
      setInjections(list);
      setSelected(list.slice(0, 2));
    } catch (err) {
      setError(String(err));
    } finally {
      setBusy(false);
    }
  }, [baseUrl]);

  useEffect(() => {
    void load();
  }, [load]);

  useRegisterTabRefresh(load, { busy });

  const toggle = (injection: string) => {
    setSelected((current) =>
      current.includes(injection)
        ? current.filter((item) => item !== injection)
        : [...current, injection],
    );
  };

  const simulate = async () => {
    if (!hasToken || !can("Deploy")) return;
    setBusy(true);
    setError(null);
    try {
      const res = await fetch(`${baseUrl}/v1/chaos/simulate`, {
        method: "POST",
        headers: authHeaders(),
        body: JSON.stringify({ injections: selected }),
      });
      const text = await res.text();
      setResult(text);
    } catch (err) {
      setError(String(err));
    } finally {
      setBusy(false);
    }
  };

  return (
    <div className="cc-panel">
      {error && <div className="error">{error}</div>}

      {injections.length === 0 ? (
          <CcEmptyState title="No injections available" />
        ) : (
          <ul className="cc-checkbox-list">
            {injections.map((injection) => (
              <li key={injection}>
                <label>
                  <input
                    type="checkbox"
                    checked={selected.includes(injection)}
                    onChange={() => toggle(injection)}
                  />
                  {injection}
                </label>
              </li>
            ))}
          </ul>
        )}
      {can("Deploy") && hasToken && (
        <button type="button" onClick={() => void simulate()} disabled={busy || selected.length === 0}>
          Run chaos simulation
        </button>
      )}

      {result && (
        <CcSection title="Simulation result">
          <pre className="cc-action-result">{result}</pre>
        </CcSection>
      )}
    </div>
  );
}

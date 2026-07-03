import { useCallback, useEffect, useState } from "react";
import type { RbacAction } from "./controlCenterRbac";
import { CcEmptyState, CcSection } from "./controlCenterUi";
import { useRegisterTabRefresh } from "./useControlCenterTabRefresh";

type ScheduleRow = {
  id: string;
  profile: string;
  format: string;
  destination_url: string;
  interval_hours: number;
  enabled: boolean;
  last_status?: string;
};

type Props = {
  baseUrl: string;
  authHeaders: () => HeadersInit;
  can: (action: RbacAction) => boolean;
  hasToken: boolean;
};

export function ReportsPanel({ baseUrl, authHeaders, can, hasToken }: Props) {
  const [schedules, setSchedules] = useState<ScheduleRow[]>([]);
  const [preview, setPreview] = useState<string | null>(null);
  const [profile, setProfile] = useState("defense");
  const [format, setFormat] = useState("markdown");
  const [destinationUrl, setDestinationUrl] = useState("");
  const [intervalHours, setIntervalHours] = useState(24);
  const [busy, setBusy] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const load = useCallback(async () => {
    setBusy(true);
    setError(null);
    try {
      const res = await fetch(`${baseUrl}/v1/reports/schedules`);
      if (!res.ok) throw new Error(`schedules ${res.status}`);
      const body = await res.json();
      setSchedules((body.schedules as ScheduleRow[]) ?? []);
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

  const createSchedule = async () => {
    if (!hasToken || !can("Deploy")) return;
    setBusy(true);
    setError(null);
    try {
      const res = await fetch(`${baseUrl}/v1/reports/schedules`, {
        method: "POST",
        headers: authHeaders(),
        body: JSON.stringify({
          profile,
          format,
          destination_url: destinationUrl,
          interval_hours: intervalHours,
          enabled: true,
        }),
      });
      if (!res.ok) throw new Error(`create schedule ${res.status}`);
      await load();
    } catch (err) {
      setError(String(err));
    } finally {
      setBusy(false);
    }
  };

  const previewReport = async () => {
    setBusy(true);
    setError(null);
    try {
      const res = await fetch(`${baseUrl}/v1/compliance/export?profile=${encodeURIComponent(profile)}&format=${format}`);
      const text = await res.text();
      setPreview(text.slice(0, 8000));
    } catch (err) {
      setError(String(err));
    } finally {
      setBusy(false);
    }
  };

  return (
    <div className="cc-panel">
      {error && <div className="error">{error}</div>}

      <CcSection
        title="Report schedules"
        hint="Scheduled Markdown, JSON, or PDF delivery via webhook."
      >
        {schedules.length === 0 ? (
          <CcEmptyState title="No schedules" description="Add a webhook schedule for automated compliance reports." />
        ) : (
          <table className="cc-table">
            <thead>
              <tr>
                <th>ID</th>
                <th>Profile</th>
                <th>Format</th>
                <th>Interval</th>
                <th>Destination</th>
                <th>Last status</th>
              </tr>
            </thead>
            <tbody>
              {schedules.map((schedule) => (
                <tr key={schedule.id}>
                  <td><code>{schedule.id}</code></td>
                  <td>{schedule.profile}</td>
                  <td>{schedule.format}</td>
                  <td>{schedule.interval_hours}h</td>
                  <td>{schedule.destination_url}</td>
                  <td>{schedule.last_status ?? "—"}</td>
                </tr>
              ))}
            </tbody>
          </table>
        )}
      </CcSection>

      {can("Deploy") && hasToken && (
        <CcSection title="Add schedule">
          <div className="cc-filter-bar">
            <label className="cc-field">
              Profile
              <input value={profile} onChange={(event) => setProfile(event.target.value)} />
            </label>
            <label className="cc-field">
              Format
              <select value={format} onChange={(event) => setFormat(event.target.value)}>
                <option value="markdown">markdown</option>
                <option value="json">json</option>
                <option value="pdf">pdf</option>
              </select>
            </label>
            <label className="cc-field">
              Interval (hours)
              <input
                type="number"
                value={intervalHours}
                onChange={(event) => setIntervalHours(Number(event.target.value))}
              />
            </label>
            <label className="cc-field">
              Webhook URL
              <input value={destinationUrl} onChange={(event) => setDestinationUrl(event.target.value)} />
            </label>
          </div>
          <button type="button" onClick={() => void createSchedule()} disabled={busy}>
            Create schedule
          </button>
        </CcSection>
      )}

      <CcSection title="Preview" actions={<button type="button" onClick={() => void previewReport()} disabled={busy}>Generate preview</button>}>
        {preview ? <pre className="cc-action-result">{preview}</pre> : <CcEmptyState title="No preview yet" />}
      </CcSection>
    </div>
  );
}

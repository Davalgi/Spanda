import { CcSection } from "./controlCenterUi";
import { scalarEntries } from "./controlCenterDataTable";

type Props = {
  title: string;
  data: Record<string, unknown> | null | undefined;
};

export function AnalyticsSection({ title, data }: Props) {
  if (!data) return null;
  const scalars = scalarEntries(data);
  return (
    <CcSection title={title}>
      {scalars.length > 0 ? (
        <dl className="cc-detail-grid">
          {scalars.map(([key, value]) => (
            <div key={key} className="cc-detail-row">
              <dt>{key}</dt>
              <dd>{value}</dd>
            </div>
          ))}
        </dl>
      ) : null}
      <details className="cc-json-details">
        <summary>Raw JSON</summary>
        <pre className="cc-action-result">{JSON.stringify(data, null, 2)}</pre>
      </details>
    </CcSection>
  );
}

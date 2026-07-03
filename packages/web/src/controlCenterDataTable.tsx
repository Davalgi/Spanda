import { CcEmptyState } from "./controlCenterUi";

type Column<Row> = {
  key: string;
  header: string;
  render: (row: Row) => string;
};

type Props<Row> = {
  columns: Column<Row>[];
  rows: Row[];
  emptyLabel?: string;
  rowKey: (row: Row, index: number) => string;
  rowClassName?: (row: Row) => string | undefined;
};

export function ControlCenterDataTable<Row>({
  columns,
  rows,
  emptyLabel = "No rows",
  rowKey,
  rowClassName,
}: Props<Row>) {
  if (rows.length === 0) {
    return <CcEmptyState title={emptyLabel} />;
  }
  return (
    <div className="cc-table-wrap">
      <table className="cc-data-table">
        <thead>
          <tr>
            {columns.map((column) => (
              <th key={column.key}>{column.header}</th>
            ))}
          </tr>
        </thead>
        <tbody>
          {rows.map((row, index) => (
            <tr key={rowKey(row, index)} className={rowClassName?.(row)}>
              {columns.map((column) => (
                <td key={column.key}>{column.render(row)}</td>
              ))}
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  );
}

export function scalarEntries(
  data: Record<string, unknown> | null | undefined,
): [string, string][] {
  if (!data) return [];
  return Object.entries(data)
    .filter(([, value]) => value === null || typeof value !== "object")
    .map(([key, value]) => [key, String(value)]);
}

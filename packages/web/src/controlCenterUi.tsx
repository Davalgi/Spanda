import type { ReactNode } from "react";

export type CcTone = "ok" | "warn" | "danger" | "neutral" | "info";

export function lifecycleTone(state: string): CcTone {
  const normalized = state.toLowerCase();
  if (normalized === "active" || normalized === "ready") return "ok";
  if (normalized === "discovered" || normalized === "degraded") return "warn";
  if (normalized === "quarantined" || normalized === "failed") return "danger";
  return "neutral";
}

export function trustTone(level: string | undefined): CcTone {
  const normalized = (level ?? "unknown").toLowerCase();
  if (normalized === "trusted" || normalized === "verified") return "ok";
  if (normalized === "pending" || normalized === "unknown") return "warn";
  if (normalized === "untrusted" || normalized === "revoked") return "danger";
  return "neutral";
}

export function severityTone(severity: string): CcTone {
  const normalized = severity.toLowerCase();
  if (normalized === "critical" || normalized === "high") return "danger";
  if (normalized === "medium" || normalized === "warning") return "warn";
  if (normalized === "low" || normalized === "info") return "info";
  return "neutral";
}

export function isBlockingLifecycle(state: string): boolean {
  const normalized = state.toLowerCase();
  return (
    normalized === "quarantined" ||
    normalized === "failed" ||
    normalized === "discovered" ||
    normalized === "degraded"
  );
}

type BadgeProps = {
  tone?: CcTone;
  children: ReactNode;
};

export function CcBadge({ tone = "neutral", children }: BadgeProps) {
  return <span className={`cc-badge tone-${tone}`}>{children}</span>;
}

type NoticeProps = {
  tone?: CcTone;
  title: string;
  children?: ReactNode;
};

/** Inline honesty / provenance banner for composite or simulated data. */
export function CcNotice({ tone = "info", title, children }: NoticeProps) {
  // Render a compact provenance banner so simulated/composite views stay honest.
  //
  // Parameters:
  // - `tone` — visual severity (`info` | `warn` | …)
  // - `title` — short headline
  // - `children` — optional detail copy
  //
  // Returns:
  // Banner element.
  //
  // Options:
  // None.
  //
  // Example:
  // <CcNotice tone="warn" title="Simulated">Seeded occupancy</CcNotice>

  return (
    <div className={`cc-notice tone-${tone}`} role="note">
      <strong className="cc-notice-title">{title}</strong>
      {children ? <div className="cc-notice-body">{children}</div> : null}
    </div>
  );
}

type EmptyStateProps = {
  title: string;
  description?: string;
  action?: ReactNode;
};

export function CcEmptyState({ title, description, action }: EmptyStateProps) {
  return (
    <div className="cc-empty-state">
      <p className="cc-empty-title">{title}</p>
      {description && <p className="cc-empty-desc">{description}</p>}
      {action && <div className="cc-empty-action">{action}</div>}
    </div>
  );
}

type SectionProps = {
  title?: string;
  hint?: string;
  actions?: ReactNode;
  children: ReactNode;
};

export function CcSection({ title, hint, actions, children }: SectionProps) {
  const hasHeader = Boolean(title || hint || actions);
  return (
    <section className="cc-section">
      {hasHeader && (
        <div
          className={
            title || hint ? "cc-section-header" : "cc-section-header cc-section-header--actions-only"
          }
        >
          {(title || hint) && (
            <div>
              {title && <h3 className="cc-section-heading">{title}</h3>}
              {hint && <p className="cc-section-hint">{hint}</p>}
            </div>
          )}
          {actions && <div className="cc-section-actions">{actions}</div>}
        </div>
      )}
      {children}
    </section>
  );
}

export function CcPanelToolbar({ children }: { children: ReactNode }) {
  return <div className="cc-panel-toolbar">{children}</div>;
}

type MiniStat = {
  label: string;
  value: number | string;
  tone?: CcTone;
};

export function CcMiniStats({ items }: { items: MiniStat[] }) {
  return (
    <div className="cc-mini-stats">
      {items.map((item) => (
        <div key={item.label} className={`cc-mini-stat${item.tone ? ` tone-${item.tone}` : ""}`}>
          <span className="cc-mini-stat-label" title={item.label}>
            {item.label}
          </span>
          <span className="cc-mini-stat-value" title={String(item.value)}>
            {item.value}
          </span>
        </div>
      ))}
    </div>
  );
}

export function formatTimestamp(ms: number | undefined): string {
  if (!ms || Number.isNaN(ms)) return "—";
  return new Date(ms).toLocaleString();
}

export type WizardStepState = "done" | "active" | "pending" | "failed";

export type WizardStep = {
  id: string;
  label: string;
  state: WizardStepState;
  detail?: string;
};

export function CcWizardSteps({ steps }: { steps: WizardStep[] }) {
  return (
    <ol className="cc-wizard-steps">
      {steps.map((step, index) => (
        <li key={step.id} className={`cc-wizard-step state-${step.state}`}>
          <span className="cc-wizard-step-index">{index + 1}</span>
          <div className="cc-wizard-step-body">
            <span className="cc-wizard-step-label">{step.label}</span>
            {step.detail && <span className="cc-wizard-step-detail">{step.detail}</span>}
          </div>
        </li>
      ))}
    </ol>
  );
}

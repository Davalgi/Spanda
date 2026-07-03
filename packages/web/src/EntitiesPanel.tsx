import { useCallback, useEffect, useState } from "react";
import type { RbacAction } from "./controlCenterRbac";
import {
  EntityGraphPanel,
  type EntityGraphPayload,
  type EntityRelationship,
  type EntitySummary,
  type RegisterEntityInput,
} from "./EntityGraphPanel";

type Props = {
  baseUrl: string;
  authHeaders: () => HeadersInit;
  can: (action: RbacAction) => boolean;
  hasToken: boolean;
};

export function EntitiesPanel({ baseUrl, authHeaders, can, hasToken }: Props) {
  const [entityList, setEntityList] = useState<EntitySummary[]>([]);
  const [entityGraph, setEntityGraph] = useState<EntityGraphPayload | null>(null);
  const [selectedEntity, setSelectedEntity] = useState<string | null>(null);
  const [entityDetail, setEntityDetail] = useState<Record<string, unknown> | null>(null);
  const [entityRelationships, setEntityRelationships] = useState<EntityRelationship[]>([]);
  const [entityKindFilter, setEntityKindFilter] = useState("");
  const [entitySearch, setEntitySearch] = useState("");
  const [busy, setBusy] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const loadEntities = useCallback(async () => {
    setBusy(true);
    setError(null);
    try {
      const params = new URLSearchParams();
      if (entityKindFilter) params.set("kind", entityKindFilter);
      if (entitySearch.trim()) params.set("search", entitySearch.trim());
      const query = params.toString();
      const [listRes, graphRes] = await Promise.all([
        fetch(`${baseUrl}/v1/entities${query ? `?${query}` : ""}`),
        fetch(`${baseUrl}/v1/entities/graph`),
      ]);
      if (!listRes.ok) throw new Error(`entities ${listRes.status}`);
      const listBody = await listRes.json();
      setEntityList((listBody.entities as EntitySummary[]) ?? []);
      if (graphRes.ok) {
        const graphBody = await graphRes.json();
        setEntityGraph((graphBody.graph as EntityGraphPayload) ?? null);
      }
    } catch (err) {
      setError(String(err));
    } finally {
      setBusy(false);
    }
  }, [baseUrl, entityKindFilter, entitySearch]);

  useEffect(() => {
    void loadEntities();
  }, [baseUrl, entityKindFilter]);

  const loadEntityDetail = useCallback(
    async (entityId: string) => {
      setBusy(true);
      setError(null);
      try {
        const encoded = encodeURIComponent(entityId);
        const [detailRes, relRes, healthRes, readinessRes, trustRes] = await Promise.all([
          fetch(`${baseUrl}/v1/entities/${encoded}`),
          fetch(`${baseUrl}/v1/entities/${encoded}/relationships`),
          fetch(`${baseUrl}/v1/entities/${encoded}/health`),
          fetch(`${baseUrl}/v1/entities/${encoded}/readiness`),
          fetch(`${baseUrl}/v1/entities/${encoded}/trust`),
        ]);
        if (!detailRes.ok) throw new Error(`entity ${detailRes.status}`);
        const detailBody = await detailRes.json();
        const healthBody = healthRes.ok ? await healthRes.json() : {};
        const readinessBody = readinessRes.ok ? await readinessRes.json() : {};
        const trustBody = trustRes.ok ? await trustRes.json() : {};
        const entity = (detailBody.entity as Record<string, unknown>) ?? {};
        setEntityDetail({
          ...entity,
          health_report: healthBody.report,
          readiness_report: readinessBody.report,
          trust_report: trustBody.report,
          mission_ready: readinessBody.mission_ready,
        });
        if (relRes.ok) {
          const relBody = await relRes.json();
          setEntityRelationships((relBody.relationships as EntityRelationship[]) ?? []);
        } else {
          setEntityRelationships([]);
        }
      } catch (err) {
        setError(String(err));
      } finally {
        setBusy(false);
      }
    },
    [baseUrl],
  );

  const selectEntity = useCallback(
    (entityId: string | null) => {
      setSelectedEntity(entityId);
      setEntityDetail(null);
      setEntityRelationships([]);
      if (entityId) void loadEntityDetail(entityId);
    },
    [loadEntityDetail],
  );

  const registerEntity = async (input: RegisterEntityInput) => {
    if (!hasToken) return;
    setBusy(true);
    setError(null);
    try {
      const res = await fetch(`${baseUrl}/v1/entities/register`, {
        method: "POST",
        headers: authHeaders(),
        body: JSON.stringify(input),
      });
      if (!res.ok) throw new Error(`register entity ${res.status}`);
      await loadEntities();
      if (input.id) selectEntity(input.id);
    } catch (err) {
      setError(String(err));
    } finally {
      setBusy(false);
    }
  };

  const tagEntity = async (entityId: string, tags: string[]) => {
    if (!hasToken) return;
    setBusy(true);
    setError(null);
    try {
      const res = await fetch(`${baseUrl}/v1/entities/${encodeURIComponent(entityId)}/tags`, {
        method: "POST",
        headers: authHeaders(),
        body: JSON.stringify({ add: tags }),
      });
      if (!res.ok) throw new Error(`tag entity ${res.status}`);
      await loadEntities();
      if (selectedEntity === entityId) await loadEntityDetail(entityId);
    } catch (err) {
      setError(String(err));
    } finally {
      setBusy(false);
    }
  };

  const relateEntities = async (
    fromId: string,
    toId: string,
    kind: string,
    label?: string,
  ) => {
    if (!hasToken) return;
    setBusy(true);
    setError(null);
    try {
      const body: Record<string, string> = { from_id: fromId, to_id: toId, kind };
      if (label) body.label = label;
      const res = await fetch(`${baseUrl}/v1/entities/relationships`, {
        method: "POST",
        headers: authHeaders(),
        body: JSON.stringify(body),
      });
      if (!res.ok) throw new Error(`relate entities ${res.status}`);
      await loadEntities();
      if (selectedEntity) await loadEntityDetail(selectedEntity);
    } catch (err) {
      setError(String(err));
    } finally {
      setBusy(false);
    }
  };

  const syncEntities = async () => {
    if (!hasToken) return;
    setBusy(true);
    setError(null);
    try {
      const res = await fetch(`${baseUrl}/v1/entities/sync`, {
        method: "POST",
        headers: authHeaders(),
        body: JSON.stringify({}),
      });
      if (!res.ok) throw new Error(`sync entities ${res.status}`);
      await loadEntities();
      if (selectedEntity) await loadEntityDetail(selectedEntity);
    } catch (err) {
      setError(String(err));
    } finally {
      setBusy(false);
    }
  };

  return (
    <div className="cc-panel">
      {error && <div className="error">{error}</div>}
      <EntityGraphPanel
        entities={entityList}
        graph={entityGraph}
        selectedId={selectedEntity}
        onSelect={selectEntity}
        kindFilter={entityKindFilter}
        onKindFilterChange={(value) => {
          setEntityKindFilter(value);
        }}
        search={entitySearch}
        onSearchChange={(value) => {
          setEntitySearch(value);
        }}
        detail={entityDetail}
        relationships={entityRelationships}
        loading={busy}
        write={{
          canWrite: hasToken && can("Provision"),
          busy,
          onRegister: registerEntity,
          onTag: tagEntity,
          onRelate: relateEntities,
          onSync: syncEntities,
        }}
      />
    </div>
  );
}

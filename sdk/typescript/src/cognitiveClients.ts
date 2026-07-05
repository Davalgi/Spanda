/** Cognitive & Resilience Architecture domain SDK clients. */

import type { JsonValue, SpandaClient } from "./client.js";

export class ReflexClient {
  constructor(private readonly client: SpandaClient) {}

  async list(): Promise<JsonValue> {
    return this.client.listAutonomyReflex();
  }

  async traces(): Promise<JsonValue> {
    return this.client.listAutonomyReflexTraces();
  }
}

export class HomeostasisClient {
  constructor(private readonly client: SpandaClient) {}

  async summary(): Promise<JsonValue> {
    return this.client.getAutonomyHomeostasis();
  }
}

export class ImmunityClient {
  constructor(private readonly client: SpandaClient) {}

  async scan(): Promise<JsonValue> {
    return this.client.scanAutonomyImmunity();
  }
}

export class AttentionClient {
  constructor(private readonly client: SpandaClient) {}

  async queue(): Promise<JsonValue> {
    return this.client.getAutonomyAttention();
  }
}

export class FusionClient {
  constructor(private readonly client: SpandaClient) {}

  async summary(): Promise<JsonValue> {
    return this.client.getAutonomyFusion();
  }
}

export class MemoryClient {
  constructor(private readonly client: SpandaClient) {}

  async summary(): Promise<JsonValue> {
    return this.client.getAutonomyMemory();
  }

  async entityRefs(entityId: string): Promise<JsonValue> {
    return this.client.getEntityAutonomy(entityId);
  }
}

/**
 * Native gRPC client for Spanda Control Center (`spanda.v1.ControlCenter`).
 * @module
 */
import * as grpc from "@grpc/grpc-js";
import * as protoLoader from "@grpc/proto-loader";
import path from "node:path";
import { fileURLToPath } from "node:url";
import { ConnectionError, SpandaError } from "./errors.js";
import type { JsonValue } from "./types.js";

const moduleDir = path.dirname(fileURLToPath(import.meta.url));

export type GrpcClientOptions = {
  /** gRPC endpoint, for example `127.0.0.1:50051`. */
  address?: string;
  /** Bearer token for mutation RPCs (defaults to `SPANDA_API_KEY`). */
  apiKey?: string;
};

type ControlCenterClient = grpc.Client;

type JsonResponse = {
  json?: string;
};

type ControlCenterConstructor = new (
  address: string,
  creds: grpc.ChannelCredentials,
) => ControlCenterClient;

let clientFactoryPromise: Promise<ControlCenterConstructor> | null = null;

/** Parse JSON payload from Control Center `JsonResponse`. */
export function parseGrpcJson(raw: string): JsonValue {
  try {
    return JSON.parse(raw) as JsonValue;
  } catch (error) {
    throw new SpandaError(
      error instanceof Error ? error.message : "invalid JSON from gRPC response",
    );
  }
}

async function loadControlCenterConstructor(): Promise<ControlCenterConstructor> {
  if (!clientFactoryPromise) {
    clientFactoryPromise = (async () => {
      const protoPath = path.join(moduleDir, "../proto/spanda/v1/control_center.proto");
      const includeDir = path.join(moduleDir, "../proto");
      const definition = await protoLoader.load(protoPath, {
        keepCase: false,
        longs: String,
        enums: String,
        defaults: true,
        oneofs: true,
        includeDirs: [includeDir],
      });
      const bundle = grpc.loadPackageDefinition(definition) as unknown as {
        spanda: {
          v1: {
            ControlCenter: ControlCenterConstructor;
          };
        };
      };
      return bundle.spanda.v1.ControlCenter;
    })();
  }
  const ctor = await clientFactoryPromise;
  if (!ctor) {
    throw new ConnectionError("failed to load Control Center gRPC client");
  }
  return ctor;
}

function unaryJson(
  client: ControlCenterClient,
  method: string,
  request: unknown,
  metadata: grpc.Metadata,
): Promise<JsonValue> {
  return new Promise((resolve, reject) => {
    const rpc = (client as unknown as Record<string, unknown>)[method];
    if (typeof rpc !== "function") {
      reject(new ConnectionError(`gRPC method not found: ${method}`));
      return;
    }
    (rpc as (req: unknown, meta: grpc.Metadata, cb: grpc.requestCallback<JsonResponse>) => void).call(
      client,
      request,
      metadata,
      (error: grpc.ServiceError | null, response?: JsonResponse) => {
        if (error) {
          reject(new ConnectionError(error.message));
          return;
        }
        resolve(parseGrpcJson(response?.json ?? "{}"));
      },
    );
  });
}

/** Async gRPC client for Control Center mesh and entity RPCs. */
export class GrpcClient {
  private readonly inner: ControlCenterClient;
  private readonly metadata: grpc.Metadata;

  private constructor(inner: ControlCenterClient, apiKey?: string) {
    this.inner = inner;
    this.metadata = new grpc.Metadata();
    if (apiKey) {
      this.metadata.add("authorization", `Bearer ${apiKey}`);
    }
  }

  /** Connect to a gRPC endpoint. */
  static async connect(options: GrpcClientOptions = {}): Promise<GrpcClient> {
    const address =
      options.address ??
      process.env.SPANDA_GRPC_URL ??
      process.env.SPANDA_CONTROL_CENTER_GRPC ??
      "127.0.0.1:50051";
    const apiKey = options.apiKey ?? process.env.SPANDA_API_KEY;
    const ControlCenter = await loadControlCenterConstructor();
    const inner = new ControlCenter(address, grpc.credentials.createInsecure());
    return new GrpcClient(inner, apiKey);
  }

  /** Default local Control Center gRPC endpoint. */
  static async local(): Promise<GrpcClient> {
    return GrpcClient.connect();
  }

  /** Close the underlying channel. */
  close(): void {
    this.inner.close();
  }

  /** Mesh topology via `GetMeshTopology`. */
  getMeshTopology(): Promise<JsonValue> {
    return unaryJson(this.inner, "getMeshTopology", {}, this.metadata);
  }

  /** Mesh nodes via `GetMeshNodes`. */
  getMeshNodes(): Promise<JsonValue> {
    return unaryJson(this.inner, "getMeshNodes", {}, this.metadata);
  }

  /** Mesh routes via `GetMeshRoutes`. */
  getMeshRoutes(query = ""): Promise<JsonValue> {
    return unaryJson(this.inner, "getMeshRoutes", { query }, this.metadata);
  }

  /** Mesh partitions via `GetMeshPartitions`. */
  getMeshPartitions(): Promise<JsonValue> {
    return unaryJson(this.inner, "getMeshPartitions", {}, this.metadata);
  }

  /** Mesh health via `GetMeshHealth`. */
  getMeshHealth(): Promise<JsonValue> {
    return unaryJson(this.inner, "getMeshHealth", {}, this.metadata);
  }

  /** Mesh graph via `GetMeshGraph`. */
  getMeshGraph(): Promise<JsonValue> {
    return unaryJson(this.inner, "getMeshGraph", {}, this.metadata);
  }

  /** Mesh merge report via `GetMeshMergeReport`. */
  getMeshMergeReport(): Promise<JsonValue> {
    return unaryJson(this.inner, "getMeshMergeReport", {}, this.metadata);
  }

  /** Run mesh discovery via `DiscoverMesh`. */
  discoverMesh(body: JsonValue = {}): Promise<JsonValue> {
    return unaryJson(this.inner, "discoverMesh", { bodyJson: JSON.stringify(body) }, this.metadata);
  }

  /** Find mesh capability via `FindMeshCapability`. */
  findMeshCapability(capability: string): Promise<JsonValue> {
    return unaryJson(
      this.inner,
      "findMeshCapability",
      { bodyJson: JSON.stringify({ capability }) },
      this.metadata,
    );
  }

  /** Simulate mesh partition via `SimulateMeshPartition`. */
  simulateMeshPartition(entityIds: string[]): Promise<JsonValue> {
    return unaryJson(
      this.inner,
      "simulateMeshPartition",
      { bodyJson: JSON.stringify({ entity_ids: entityIds }) },
      this.metadata,
    );
  }
}

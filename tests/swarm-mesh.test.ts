import { createServer } from "node:http";
import { join } from "node:path";
import { describe, expect, it } from "vitest";
import { compileFile } from "../src/compile.js";
import { registerFleetAgent } from "../src/fleet-remote.js";
import {
  coordinateSwarmsMesh,
  emptySwarmState,
} from "../src/swarm-coordinator.js";

const swarmExample = join(import.meta.dirname, "..", "examples/robotics/swarm_coordination.sd");

function startMeshServer(): Promise<{
  meshPort: number;
  close: () => void;
}> {
  return new Promise((resolveListen, rejectListen) => {
    const server = createServer((req, res) => {
      if (req.method === "POST" && req.url === "/v1/mesh/relay") {
        let body = "";
        req.on("data", (chunk) => {
          body += chunk;
        });
        req.on("end", () => {
          const payload = JSON.parse(body) as { deliveries: unknown[] };
          res.writeHead(200, { "Content-Type": "application/json" });
          res.end(JSON.stringify({
            ok: true,
            relayed: payload.deliveries.length,
            failed: 0,
          }));
        });
        return;
      }
      if (req.method === "GET" && req.url === "/v1/health") {
        res.writeHead(200, { "Content-Type": "application/json" });
        res.end(JSON.stringify({ ok: true, agent: "mock-mesh" }));
        return;
      }
      res.writeHead(404);
      res.end();
    });
    server.listen(0, () => {
      const meshPort = (server.address() as { port: number }).port;
      resolveListen({
        meshPort,
        close: () => server.close(),
      });
    });
    server.on("error", rejectListen);
  });
}

describe("swarm mesh (TS mirror)", () => {
  it("relays leader-follow peer deliveries through mesh coordinator", async () => {
    const agentServer = createServer((req, res) => {
      if (req.method === "GET" && req.url === "/v1/health") {
        res.writeHead(200, { "Content-Type": "application/json" });
        res.end(JSON.stringify({ ok: true }));
        return;
      }
      if (req.method === "POST" && req.url === "/v1/peer") {
        res.writeHead(200, { "Content-Type": "application/json" });
        res.end(JSON.stringify({ ok: true, to_robot: "ScoutB", topic: "mission_step", step: "navigate" }));
        return;
      }
      res.writeHead(404);
      res.end();
    });
    await new Promise<void>((resolveListen) => agentServer.listen(0, resolveListen));
    const agentPort = (agentServer.address() as { port: number }).port;
    let registry = registerFleetAgent({ agents: [] }, "ScoutB", `http://127.0.0.1:${agentPort}`);
    registry = registerFleetAgent(registry, "ScoutC", `http://127.0.0.1:${agentPort}`);
    const { meshPort, close } = await startMeshServer();
    const { program } = compileFile(swarmExample, "typescript");
    const state = emptySwarmState();
    const result = await coordinateSwarmsMesh(
      program,
      "swarm_coordination.sd",
      state,
      `http://127.0.0.1:${meshPort}`,
    );
    const leader = result.swarms.find((swarm) => swarm.policy === "leader_follow");
    expect(result.success).toBe(true);
    expect(leader?.remoteRelayed).toBeGreaterThan(0);
    expect(leader?.coordinationMode.endsWith("_mesh")).toBe(true);
    close();
    agentServer.close();
    void registry;
  });

  it("relays round-robin peer links through mesh coordinator", async () => {
    const agentServer = createServer((req, res) => {
      if (req.method === "GET" && req.url === "/v1/health") {
        res.writeHead(200, { "Content-Type": "application/json" });
        res.end(JSON.stringify({ ok: true }));
        return;
      }
      if (req.method === "POST" && req.url === "/v1/peer") {
        res.writeHead(200, { "Content-Type": "application/json" });
        res.end(JSON.stringify({ ok: true, to_robot: "ScoutB", topic: "mission_step", step: "navigate" }));
        return;
      }
      res.writeHead(404);
      res.end();
    });
    await new Promise<void>((resolveListen) => agentServer.listen(0, resolveListen));
    const agentPort = (agentServer.address() as { port: number }).port;
    registerFleetAgent({ agents: [] }, "ScoutB", `http://127.0.0.1:${agentPort}`);
    const { meshPort, close } = await startMeshServer();
    const { program } = compileFile(swarmExample, "typescript");
    const state = emptySwarmState();
    const result = await coordinateSwarmsMesh(
      program,
      "swarm_coordination.sd",
      state,
      `http://127.0.0.1:${meshPort}`,
    );
    const roundRobin = result.swarms.find((swarm) => swarm.policy === "round_robin");
    expect(result.success).toBe(true);
    expect(roundRobin?.remoteRelayed).toBeGreaterThan(0);
    close();
    agentServer.close();
  });
});

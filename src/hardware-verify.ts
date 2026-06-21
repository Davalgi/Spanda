/**
 * TypeScript hardware verification fallback when the native CLI is unavailable.
 * @module
 */

import type { Program, RobotDecl } from "./ast/nodes.js";
import type {
  RequiresHardwareDecl,
  RequiresNetworkDecl,
  RequiresConnectivityDecl,
  ResourceBudgetDecl,
  MissionDecl,
} from "./foundations.js";
import type { CompatItem, VerifyResult } from "./rust-bridge.js";
import {
  validateConnectivityPolicy,
  validateGeofence,
  verifyRequiresConnectivity,
} from "./connectivity-positioning.js";
import {
  applyFault,
  buildProfileRegistry,
  type HardwareProfile,
} from "./hardware-profile.js";

const ESTIMATED_TASK_COST_MS = 5;

export type VerifyHardwareTsOptions = {
  target?: string;
  allTargets?: boolean;
  simulate?: boolean;
};

function compat(
  category: string,
  message: string,
  severity: CompatItem["severity"],
  line: number,
  column: number,
): CompatItem {
  return { category, message, severity, line, column };
}

function verifyRequiresHardware(
  req: RequiresHardwareDecl,
  profile: HardwareProfile,
): CompatItem[] {
  const items: CompatItem[] = [];
  const line = req.span.start.line;
  const column = req.span.start.column;

  if (req.memoryMbMin != null) {
    const min = req.memoryMbMin;
    if (profile.memoryMb == null) {
      items.push(compat("memory", "Target memory unknown — cannot verify memory requirement", "warning", line, column));
    } else if (profile.memoryMb >= min) {
      items.push(compat("memory", `Memory ${profile.memoryMb} MB meets requirement >= ${min} MB`, "pass", line, column));
    } else {
      items.push(compat("memory", `Memory requirement ${min} MB exceeds target ${profile.memoryMb} MB`, "error", line, column));
    }
  }

  if (req.storageMbMin != null) {
    const min = req.storageMbMin;
    if (profile.storageMb == null) {
      items.push(compat("storage", "Target storage unknown — cannot verify storage requirement", "warning", line, column));
    } else if (profile.storageMb >= min) {
      items.push(compat("storage", `Storage ${profile.storageMb} MB meets requirement >= ${min} MB`, "pass", line, column));
    } else {
      items.push(compat("storage", `Storage requirement ${min} MB exceeds target ${profile.storageMb} MB`, "error", line, column));
    }
  }

  if (req.gpuRequired && !profile.gpuRequired && profile.gpuTops == null) {
    items.push(compat("gpu", `GPU required but hardware profile '${profile.name}' has no GPU`, "error", line, column));
  }

  if (req.gpuTopsMin != null) {
    const min = req.gpuTopsMin;
    if (profile.gpuTops == null) {
      items.push(compat("gpu", `GPU requirement ${min} TOPS but target has no GPU`, "error", line, column));
    } else if (profile.gpuTops >= min) {
      items.push(compat("gpu", `GPU ${profile.gpuTops} TOPS meets requirement >= ${min} TOPS`, "pass", line, column));
    } else {
      items.push(compat("gpu", `GPU requirement ${min} TOPS exceeds target ${profile.gpuTops} TOPS`, "error", line, column));
    }
  }

  const sensorSet = new Set(profile.sensors);
  for (const sensorType of req.sensors) {
    if (sensorSet.has(sensorType)) {
      items.push(compat("sensors", `Required sensor '${sensorType}' available on ${profile.name}`, "pass", line, column));
    } else {
      items.push(compat("sensors", `Required sensor '${sensorType}' not on '${profile.name}' [${profile.sensors.join(", ")}]`, "error", line, column));
    }
  }

  const actuatorSet = new Set(profile.actuators);
  for (const actuatorType of req.actuators) {
    if (actuatorSet.has(actuatorType)) {
      items.push(compat("actuators", `Required actuator '${actuatorType}' available on ${profile.name}`, "pass", line, column));
    } else {
      items.push(compat("actuators", `Required actuator '${actuatorType}' not on '${profile.name}' [${profile.actuators.join(", ")}]`, "error", line, column));
    }
  }

  return items;
}

function verifyRequiresNetwork(req: RequiresNetworkDecl, profile: HardwareProfile): CompatItem[] {
  const items: CompatItem[] = [];
  const line = req.span.start.line;
  const column = req.span.start.column;

  if (req.bandwidthMbpsMin != null) {
    const min = req.bandwidthMbpsMin;
    if (profile.networkBandwidthMbps == null) {
      items.push(compat("network", "Target bandwidth unknown — cannot verify bandwidth requirement", "warning", line, column));
    } else if (profile.networkBandwidthMbps >= min) {
      items.push(compat("network", `Bandwidth ${profile.networkBandwidthMbps} Mbps meets requirement >= ${min} Mbps`, "pass", line, column));
    } else {
      items.push(compat("network", `Bandwidth requirement ${min} Mbps exceeds target ${profile.networkBandwidthMbps} Mbps`, "error", line, column));
    }
  }

  if (req.latencyMsMax != null) {
    const max = req.latencyMsMax;
    if (profile.networkLatencyMs == null) {
      items.push(compat("network", "Target latency unknown — cannot verify latency requirement", "warning", line, column));
    } else if (profile.networkLatencyMs <= max) {
      items.push(compat("network", `Latency ${profile.networkLatencyMs} ms meets requirement <= ${max} ms`, "pass", line, column));
    } else {
      items.push(compat("network", `Latency requirement ${max} ms violated by target ${profile.networkLatencyMs} ms`, "error", line, column));
    }
  }

  return items;
}

function verifyResourceBudget(
  budget: ResourceBudgetDecl,
  profile: HardwareProfile,
  taskIntervalMs: number,
): CompatItem[] {
  const items: CompatItem[] = [];
  const line = budget.span.start.line;
  const column = budget.span.start.column;

  if (budget.memoryMbMax != null && profile.memoryMb != null) {
    if (budget.memoryMbMax <= profile.memoryMb) {
      items.push(compat("memory", `Task memory budget ${budget.memoryMbMax} MB within target ${profile.memoryMb} MB`, "pass", line, column));
    } else {
      items.push(compat("memory", `Task memory budget ${budget.memoryMbMax} MB exceeds target ${profile.memoryMb} MB`, "error", line, column));
    }
  }

  if (budget.cpuPctMax != null) {
    const duty = (ESTIMATED_TASK_COST_MS / Math.max(taskIntervalMs, 1)) * 100;
    if (duty <= budget.cpuPctMax) {
      items.push(compat("timing", `Task CPU duty ${duty.toFixed(1)}% within budget ${budget.cpuPctMax}%`, "pass", line, column));
    } else {
      items.push(compat("timing", `Task CPU duty ${duty.toFixed(1)}% exceeds budget ${budget.cpuPctMax}%`, "error", line, column));
    }
  }

  return items;
}

function verifyBatteryMission(mission: MissionDecl, profile: HardwareProfile): CompatItem[] {
  const line = mission.span.start.line;
  const column = mission.span.start.column;
  if (profile.batteryWh == null) {
    return [compat("battery", "Target battery capacity unknown — cannot verify mission duration", "warning", line, column)];
  }
  const hours = mission.durationHours;
  const estimatedWh = hours * profile.powerDrawW;
  if (estimatedWh <= profile.batteryWh) {
    return [compat("battery", `Mission ${hours}h (~${estimatedWh.toFixed(0)} Wh) within ${profile.batteryWh} Wh capacity`, "pass", line, column)];
  }
  return [compat("battery", `Mission ${hours}h (~${estimatedWh.toFixed(0)} Wh) exceeds ${profile.batteryWh} Wh capacity`, "error", line, column)];
}

function verifyTiming(robot: RobotDecl, profile: HardwareProfile): CompatItem[] {
  const items: CompatItem[] = [];
  const line = robot.span.start.line;
  const column = robot.span.start.column;
  let totalCpuPct = 0;

  for (const task of robot.tasks) {
    if (task.intervalMs < profile.minControlPeriodMs) {
      items.push(compat("timing", `Task '${task.name}' interval ${task.intervalMs}ms below min_period ${profile.minControlPeriodMs}ms on '${profile.name}'`, "error", task.span.start.line, task.span.start.column));
    } else {
      items.push(compat("timing", `Task '${task.name}' interval ${task.intervalMs}ms schedulable on '${profile.name}'`, "pass", task.span.start.line, task.span.start.column));
    }
    totalCpuPct += (ESTIMATED_TASK_COST_MS / Math.max(task.intervalMs, 1)) * 100;
  }

  if (totalCpuPct > 100) {
    items.push(compat("timing", `Aggregate CPU load ${totalCpuPct.toFixed(1)}% exceeds 100% on '${profile.name}'`, "error", line, column));
  } else if (totalCpuPct > 0) {
    items.push(compat("timing", `Aggregate CPU load ${totalCpuPct.toFixed(1)}% within scheduling budget on '${profile.name}'`, "pass", line, column));
  }

  return items;
}

function verifyRobotAgainstProfile(
  robot: RobotDecl,
  profile: HardwareProfile,
  programRequiresHw: RequiresHardwareDecl | null,
  programRequiresNet: RequiresNetworkDecl | null,
  programRequiresConn: RequiresConnectivityDecl | null,
  spanLine: number,
  spanColumn: number,
): CompatItem[] {
  const items: CompatItem[] = [];
  const sensorSet = new Set(profile.sensors);
  const actuatorSet = new Set(profile.actuators);

  for (const sensor of robot.sensors) {
    if (sensorSet.has(sensor.sensorType)) {
      items.push(compat("sensors", `Sensor '${sensor.name}' (${sensor.sensorType}) available on ${profile.name}`, "pass", sensor.span.start.line, sensor.span.start.column));
    } else {
      items.push(compat("sensors", `Sensor '${sensor.name}' requires type '${sensor.sensorType}' but hardware profile '${profile.name}' provides [${profile.sensors.join(", ")}]`, "error", sensor.span.start.line, sensor.span.start.column));
    }
  }

  if (robot.observe) {
    for (const sensorName of robot.observe.sensors) {
      const declared = robot.sensors.find((s) => s.name === sensorName);
      if (declared && !sensorSet.has(declared.sensorType)) {
        items.push(compat("sensors", `observe fuses sensor '${sensorName}' (${declared.sensorType}) not supported on '${profile.name}'`, "error", robot.observe.span.start.line, robot.observe.span.start.column));
      }
    }
  }

  for (const actuator of robot.actuators) {
    if (actuatorSet.has(actuator.actuatorType)) {
      items.push(compat("actuators", `Actuator '${actuator.name}' (${actuator.actuatorType}) available on ${profile.name}`, "pass", actuator.span.start.line, actuator.span.start.column));
    } else {
      items.push(compat("actuators", `Actuator '${actuator.name}' requires type '${actuator.actuatorType}' but hardware profile '${profile.name}' provides [${profile.actuators.join(", ")}]`, "error", actuator.span.start.line, actuator.span.start.column));
    }
  }

  if (robot.sensors.length === 0 && robot.actuators.length === 0) {
    items.push(compat("robot", `Robot '${robot.name}' has no sensor/actuator requirements`, "pass", spanLine, spanColumn));
  }

  if (programRequiresHw) {
    items.push(...verifyRequiresHardware(programRequiresHw, profile));
  }
  if (programRequiresNet) {
    items.push(...verifyRequiresNetwork(programRequiresNet, profile));
  }
  const reqConn = robot.requiresConnectivity ?? programRequiresConn;
  if (reqConn) {
    items.push(...verifyRequiresConnectivity(reqConn, profile));
  }

  for (const task of robot.tasks) {
    if (task.budget) items.push(...verifyResourceBudget(task.budget, profile, task.intervalMs));
  }
  if (robot.mission) items.push(...verifyBatteryMission(robot.mission, profile));
  items.push(...verifyTiming(robot, profile));

  return items;
}

function resolveTargets(
  program: Program,
  options: VerifyHardwareTsOptions,
  registry: Map<string, HardwareProfile>,
): Array<[string, string, number, number]> {
  if (options.allTargets) {
    const pairs: Array<[string, string, number, number]> = [];
    for (const robot of program.robots) {
      for (const target of registry.keys()) {
        pairs.push([robot.name, target, robot.span.start.line, robot.span.start.column]);
      }
    }
    return pairs;
  }
  if (options.target) {
    return program.robots.map((r) => [r.name, options.target!, r.span.start.line, r.span.start.column]);
  }
  const pairs: Array<[string, string, number, number]> = [];
  for (const deploy of program.deployments) {
    for (const target of deploy.targets) {
      pairs.push([deploy.robotName, target, deploy.span.start.line, deploy.span.start.column]);
    }
  }
  return pairs;
}

export function verifyHardwareProgram(
  program: Program,
  options: VerifyHardwareTsOptions = {},
): VerifyResult {
  // Run hardware compatibility verification in TypeScript.
  //
  // Parameters:
  // - `program` — parsed Spanda program
  // - `options` — optional target and simulation flags
  //
  // Returns:
  // Verify result compatible with the native CLI JSON shape.
  //
  // Options:
  // - `options.target` — hardware profile name
  // - `options.allTargets` — verify against every known profile
  // - `options.simulate` — apply simulate_compatibility faults
  //
  // Example:
  // const result = verifyHardwareProgram(program, { target: "RoverV2" });

  const registry = buildProfileRegistry(program);
  const items: CompatItem[] = [];

  for (const geofence of program.geofences) {
    items.push(...validateGeofence(geofence));
  }
  for (const policy of program.connectivityPolicies) {
    items.push(...validateConnectivityPolicy(policy));
  }

  const targets = resolveTargets(program, options, registry);
  const runSimulation = options.simulate || program.simulateCompatibility != null;

  if (targets.length === 0 && !options.target && !options.allTargets) {
    return {
      ok: true,
      compatible: true,
      target: undefined,
      items: [compat("deploy", "No deployment targets declared — hardware compatibility not required", "pass", 1, 1)],
    };
  }

  const primaryTarget = targets[0]?.[1];
  const matrixCells: Array<{ robot: string; target: string; compatible: boolean }> = [];

  for (const [robotName, targetName, line, column] of targets) {
    let profile = registry.get(targetName);
    if (!profile) {
      items.push(compat("deploy", `Unknown hardware profile '${targetName}'`, "error", line, column));
      matrixCells.push({ robot: robotName, target: targetName, compatible: false });
      continue;
    }

    if (runSimulation && program.simulateCompatibility) {
      for (const fault of program.simulateCompatibility.faults) {
        profile = applyFault(profile, fault.faultType);
        items.push(compat("simulate", `Applied fault '${fault.faultType}' against '${targetName}'`, "pass", program.simulateCompatibility.span.start.line, program.simulateCompatibility.span.start.column));
      }
    }

    const robot = program.robots.find((r) => r.name === robotName);
    if (!robot) {
      items.push(compat("deploy", `deploy references unknown robot '${robotName}'`, "error", line, column));
      matrixCells.push({ robot: robotName, target: targetName, compatible: false });
      continue;
    }

    items.push(compat("deploy", `Verifying robot '${robotName}' against hardware '${targetName}'`, "pass", line, column));
    const pairItems = verifyRobotAgainstProfile(
      robot,
      profile,
      program.requiresHardware,
      program.requiresNetwork,
      program.requiresConnectivity,
      line,
      column,
    );
    const pairOk = !pairItems.some((i) => i.severity === "error");
    items.push(...pairItems);
    matrixCells.push({ robot: robotName, target: targetName, compatible: pairOk });
  }

  const ok = !items.some((i) => i.severity === "error");
  return {
    ok,
    compatible: ok,
    target: options.target ?? primaryTarget,
    items,
    matrix: options.allTargets ? { cells: matrixCells } : undefined,
  };
}

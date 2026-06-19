import type { UnitKind, RoboType } from "../ast/nodes.js";

export type TypeError = {
  message: string;
  line: number;
  column: number;
};

export class TypeCheckError extends Error {
  constructor(public errors: TypeError[]) {
    super(errors.map((e) => e.message).join("\n"));
    this.name = "TypeCheckError";
  }
}

export function unitsCompatible(a: UnitKind, b: UnitKind): boolean {
  if (a === b) return true;
  if (a === "none" || b === "none") return true;
  if ((a === "deg" && b === "rad") || (a === "rad" && b === "deg")) return true;
  return false;
}

export function normalizeUnit(unit: UnitKind): UnitKind {
  return unit;
}

export function resultUnitForBinary(
  op: string,
  left: RoboType,
  right: RoboType,
): RoboType | null {
  if (op === "and" || op === "or") {
    if (left.kind === "bool" && right.kind === "bool") return { kind: "bool" };
    return null;
  }

  if (["<", "<=", ">", ">=", "==", "!="].includes(op)) {
    if (left.kind === "number" && right.kind === "number") {
      if (unitsCompatible(left.unit, right.unit)) return { kind: "bool" };
    }
    if (left.kind === "bool" && right.kind === "bool") return { kind: "bool" };
    return null;
  }

  if (op === "+" || op === "-") {
    if (left.kind === "number" && right.kind === "number") {
      if (unitsCompatible(left.unit, right.unit)) {
        return { kind: "number", unit: left.unit !== "none" ? left.unit : right.unit };
      }
    }
    return null;
  }

  if (op === "*" || op === "/") {
    if (left.kind === "number" && right.kind === "number") {
      return { kind: "number", unit: "none" };
    }
    return null;
  }

  return null;
}

export const SENSOR_TYPES: Record<string, RoboType> = {
  Lidar: { kind: "named", name: "Lidar" },
  IMU: { kind: "named", name: "IMU" },
  GPS: { kind: "named", name: "GPS" },
  Camera: { kind: "named", name: "Camera" },
  AltitudeSensor: { kind: "named", name: "AltitudeSensor" },
  ForceTorque: { kind: "named", name: "ForceTorque" },
};

export const ACTUATOR_TYPES: Record<string, RoboType> = {
  DifferentialDrive: { kind: "named", name: "DifferentialDrive" },
  RoboticArm: { kind: "named", name: "RoboticArm" },
  DroneRotors: { kind: "named", name: "DroneRotors" },
  Gripper: { kind: "named", name: "Gripper" },
};

export const BUILTIN_METHODS: Record<
  string,
  Record<string, { params: RoboType[]; namedParams?: Record<string, RoboType>; returns: RoboType }>
> = {
  Lidar: {
    read: { params: [], returns: { kind: "scan" } },
  },
  IMU: {
    read: { params: [], returns: { kind: "named", name: "IMUReading" } },
  },
  AltitudeSensor: {
    read: { params: [], returns: { kind: "number", unit: "m" } },
  },
  ForceTorque: {
    read: { params: [], returns: { kind: "named", name: "ForceTorqueReading" } },
  },
  DifferentialDrive: {
    drive: {
      params: [],
      namedParams: {
        linear: { kind: "number", unit: "m/s" },
        angular: { kind: "number", unit: "rad/s" },
      },
      returns: { kind: "void" },
    },
    stop: { params: [], returns: { kind: "void" } },
  },
  RoboticArm: {
    move_to: {
      params: [],
      namedParams: {
        x: { kind: "number", unit: "m" },
        y: { kind: "number", unit: "m" },
        z: { kind: "number", unit: "m" },
      },
      returns: { kind: "void" },
    },
    grip: { params: [], returns: { kind: "void" } },
    release: { params: [], returns: { kind: "void" } },
  },
  DroneRotors: {
    set_thrust: {
      params: [],
      namedParams: {
        thrust: { kind: "number", unit: "none" },
      },
      returns: { kind: "void" },
    },
    hover: { params: [], returns: { kind: "void" } },
  },
  Gripper: {
    close: { params: [], returns: { kind: "void" } },
    open: { params: [], returns: { kind: "void" } },
  },
  Scan: {
    nearest_distance: { params: [], returns: { kind: "number", unit: "m" } },
  },
};

export const SCAN_PROPERTIES: Record<string, RoboType> = {
  nearest_distance: { kind: "number", unit: "m" },
};

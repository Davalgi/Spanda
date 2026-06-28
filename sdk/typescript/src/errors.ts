/** Structured errors for the Spanda TypeScript SDK. */

export class SpandaError extends Error {
  constructor(
    message: string,
    readonly status?: number,
  ) {
    super(message);
    this.name = "SpandaError";
  }

  static fromStatus(status: number, message: string): SpandaError {
    if (status === 401 || status === 403) {
      return new PermissionError(message, status);
    }
    if (status === 400) {
      return new ValidationError(message, status);
    }
    return new SpandaError(message, status);
  }
}

export class ValidationError extends SpandaError {
  constructor(message: string, status?: number) {
    super(message, status);
    this.name = "ValidationError";
  }
}

export class ReadinessError extends SpandaError {
  constructor(message: string, status?: number) {
    super(message, status);
    this.name = "ReadinessError";
  }
}

export class VerificationError extends SpandaError {
  constructor(message: string, status?: number) {
    super(message, status);
    this.name = "VerificationError";
  }
}

export class SecurityError extends SpandaError {
  constructor(message: string, status?: number) {
    super(message, status);
    this.name = "SecurityError";
  }
}

export class ConnectionError extends SpandaError {
  constructor(message: string) {
    super(message);
    this.name = "ConnectionError";
  }
}

export class PermissionError extends SpandaError {
  constructor(message: string, status?: number) {
    super(message, status);
    this.name = "PermissionError";
  }
}

export const DEPRECATED_SYN_EXTENSION_WARNING =
  ".syn files are deprecated. Use .sd files instead.";

/** Warn when a path uses the legacy `.syn` extension (still accepted). */
export function warnDeprecatedSourceExtension(filePath: string): void {
  if (filePath.endsWith(".syn")) {
    console.warn(DEPRECATED_SYN_EXTENSION_WARNING);
  }
}

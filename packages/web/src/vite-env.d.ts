/**
 * vite env.d module (vite-env.d.ts).
 * @module
 */
/// <reference types="vite/client" />

declare module "*.wasm?url" {
  const url: string;
  export default url;
}

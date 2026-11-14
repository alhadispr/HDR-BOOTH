// Type declaration fallback for source-only checks. The actual implementation is provided by @tauri-apps/api at build time.
declare module '@tauri-apps/api/core' {
  export function invoke<T = unknown>(command: string, args?: Record<string, unknown>): Promise<T>;
}

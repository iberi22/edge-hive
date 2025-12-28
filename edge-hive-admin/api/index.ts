
import { mockApi as mockImplementation } from './mockClient';
import { tauriApi } from './tauriClient';

// Simple detection for now. 'window.__TAURI__' might be available or we check import.meta.env
// In Tauri v2, we can check core availability conceptually, or just try to use it.
// However, 'window.__TAURI_INTERNALS__' is often present.

const isTauri = typeof window !== 'undefined' && '__TAURI_INTERNALS__' in window;

export const api = isTauri ? tauriApi : mockImplementation;
export { tauriApi };

// Export mockApi for backward compatibility during refactor, but deprecated
export const mockApi = api;

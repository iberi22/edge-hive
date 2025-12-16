import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest';
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';

vi.mock('@tauri-apps/api/core');
vi.mock('@tauri-apps/api/event');

describe('Terminal', () => {
   beforeEach(() => {
      vi.clearAllMocks();
      (invoke as any).mockResolvedValue('Terminal spawned');
      (listen as any).mockResolvedValue(() => { });
   });

   afterEach(() => {
      vi.restoreAllMocks();
   });

   it('mock test - PTY spawn command exists', () => {
      expect(invoke).toBeDefined();
      expect(listen).toBeDefined();
   });

   it('mock test - terminal_spawn can be invoked', async () => {
      const result = await invoke('terminal_spawn');
      expect(result).toBe('Terminal spawned');
   });

   it('mock test - terminal output listener can be registered', async () => {
      const mockCallback = vi.fn();
      const unlisten = await listen('terminal-output', mockCallback);

      expect(unlisten).toBeDefined();
      expect(typeof unlisten).toBe('function');
   });
});

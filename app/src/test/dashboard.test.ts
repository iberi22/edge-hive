import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest';
import { get } from 'svelte/store';
import { nodes, stats, dashboardActions } from '../stores/dashboard';
import { mcpClient } from '../lib/mcp-client';
import { invoke } from '@tauri-apps/api/core';

vi.mock('../lib/mcp-client', () => ({
  mcpClient: {
    updateStats: vi.fn(),
    updateNodes: vi.fn(),
  },
}));

vi.mock('@tauri-apps/api/core', () => ({
  invoke: vi.fn(),
}));

describe('Dashboard Store', () => {
  beforeEach(() => {
    vi.clearAllMocks();
    vi.useFakeTimers();
  });

  afterEach(() => {
    vi.useRealTimers();
  });

  describe('initial state', () => {
    it('has default stats values', () => {
      const state = get(stats);
      
      expect(state).toHaveProperty('cpu');
      expect(state).toHaveProperty('ram');
      expect(state).toHaveProperty('storage');
      expect(state).toHaveProperty('network');
    });

    it('has initial nodes', () => {
      const nodesList = get(nodes);
      
      expect(Array.isArray(nodesList)).toBe(true);
      expect(nodesList.length).toBeGreaterThan(0);
    });
  });

  describe('refresh', () => {
    it('fetches system stats and updates store', async () => {
      const mockSysStats = {
        cpu_usage: 42.5,
        total_memory: 16000000000,
        used_memory: 8000000000,
        total_swap: 4000000000,
        used_swap: 1000000000,
      };

      (invoke as any).mockResolvedValue(mockSysStats);
      (mcpClient.updateStats as any).mockResolvedValue(undefined);

      await dashboardActions.refresh();

      expect(invoke).toHaveBeenCalledWith('get_system_stats');
      expect(mcpClient.updateStats).toHaveBeenCalledWith(
        expect.objectContaining({
          cpu_usage: 42.5,
        })
      );
    });

    it('handles errors gracefully', async () => {
      const mockError = new Error('Backend error');
      (invoke as any).mockRejectedValue(mockError);

      // Should not throw
      await expect(dashboardActions.refresh()).resolves.not.toThrow();
    });
  });

  describe('updateNodeStatus', () => {
    it('updates node status in store', async () => {
      (mcpClient.updateNodes as any).mockResolvedValue(undefined);

      await dashboardActions.updateNodeStatus('n1', 'offline');

      const nodesList = get(nodes);
      const node1 = nodesList.find(n => n.id === 'n1');
      
      expect(node1?.status).toBe('offline');
    });

    it('syncs with MCP server', async () => {
      await dashboardActions.updateNodeStatus('n2', 'online');

      expect(mcpClient.updateNodes).toHaveBeenCalled();
      });
   });
});

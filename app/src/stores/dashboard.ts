import { writable } from 'svelte/store';
import { invoke } from '@tauri-apps/api/core';
import { mcpClient } from '$lib/mcp-client';
import type { DashboardStats as MCPDashboardStats, Node as MCPNode } from '$lib/mcp-client';

// Check environment
const isTauri = typeof window !== 'undefined' && '__TAURI_INTERNALS__' in window;

export interface Node {
   id: string;
   name: string;
   ip: string;
   status: 'online' | 'offline' | 'syncing';
   cpu: string;
   ram: string;
}

export interface DashboardStats {
   cpu: number;
   ram: number;
   storage: number;
   network: number;
}

interface SystemStats {
   cpu_usage: number;
   total_memory: number;
   used_memory: number;
   total_swap: number;
   used_swap: number;
}

// Initial State
const initialNodes: Node[] = [
   { id: 'n1', name: 'Edge-Node-01', ip: '192.168.1.10', status: 'online', cpu: '12%', ram: '2.4GB' },
   { id: 'n2', name: 'Edge-Node-02', ip: '192.168.1.11', status: 'syncing', cpu: '45%', ram: '4.1GB' },
   { id: 'n3', name: 'Edge-Node-03', ip: '192.168.1.12', status: 'offline', cpu: '-', ram: '-' },
   { id: 'n4', name: 'Termux-Mobile', ip: '10.0.0.5', status: 'online', cpu: '5%', ram: '1.2GB' },
];

const initialStats: DashboardStats = {
   cpu: 0,
   ram: 0,
   storage: 128,
   network: 1.2,
};

export const nodes = writable<Node[]>(initialNodes);
export const stats = writable<DashboardStats>(initialStats);

// Actions (Simulating API calls that Agents might trigger)
export const dashboardActions = {
   refresh: async () => {
      try {
         let sysStats: SystemStats;

         if (isTauri) {
            // Fetch real system stats from Rust backend via Tauri IPC
            sysStats = await invoke<SystemStats>('get_system_stats');
         } else {
            // Fallback for browser dev
            // Try to fetch from local backend if available, else mock
            try {
               const res = await fetch('http://localhost:8080/api/v1/info');
               if (res.ok) {
                  // The info endpoint doesn't return exact stats structure, so we mock it for now
                  // or we could add a stats endpoint to the backend.
                  // For now, let's return dynamic mock data
                  sysStats = {
                     cpu_usage: Math.random() * 20 + 10,
                     total_memory: 16 * 1024 * 1024 * 1024,
                     used_memory: (Math.random() * 4 + 4) * 1024 * 1024 * 1024,
                     total_swap: 0,
                     used_swap: 0
                  };
               } else {
                  throw new Error('Backend not reachable');
               }
            } catch (e) {
               // Pure mock
               sysStats = {
                  cpu_usage: 15,
                  total_memory: 16 * 1024 * 1024 * 1024,
                  used_memory: 8 * 1024 * 1024 * 1024,
                  total_swap: 0,
                  used_swap: 0
               };
            }
         }

         stats.update(s => ({
            ...s,
            cpu: parseFloat(sysStats.cpu_usage.toFixed(1)),
            ram: parseFloat((sysStats.used_memory / 1024 / 1024 / 1024).toFixed(1)), // Convert bytes to GB
         }));

         // Sync stats with MCP server for agent access
         await mcpClient.updateStats({
            cpu_usage: sysStats.cpu_usage,
            total_memory: sysStats.total_memory,
            used_memory: sysStats.used_memory,
            active_nodes: initialNodes.filter(n => n.status === 'online').length,
            total_tunnels: 0, // TODO: Get from tunnel service
         });

         console.log('Dashboard refreshed with data:', sysStats);
      } catch (error) {
         console.error('Failed to fetch system stats:', error);
         // Fallback or error handling
      }
   },
   updateNodeStatus: async (id: string, status: Node['status']) => {
      nodes.update(n => n.map(node => node.id === id ? { ...node, status } : node));

      // Sync updated nodes with MCP server
      try {
         const currentNodes = await new Promise<Node[]>(resolve => {
            let unsub: (() => void) | undefined;
            unsub = nodes.subscribe(value => {
               resolve(value);
               if (unsub) unsub();
            });
         });

         await mcpClient.updateNodes(currentNodes.map(node => ({
            id: node.id,
            name: node.name,
            status: node.status,
            cpu: parseFloat(node.cpu) || 0,
            memory: parseFloat(node.ram) * 1024 * 1024 * 1024, // GB to bytes
            ip: node.ip,
         })));
      } catch (error) {
         console.error('Failed to sync nodes with MCP:', error);
      }
   },
   startAutoRefresh: (intervalMs = 5000) => {
      dashboardActions.refresh(); // Initial fetch
      const interval = setInterval(dashboardActions.refresh, intervalMs);
      return () => clearInterval(interval); // Return cleanup function
   }
};

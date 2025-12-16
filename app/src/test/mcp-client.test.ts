import { describe, it, expect, vi } from 'vitest';
import { mcpClient } from '../lib/mcp-client';
import { invoke } from '@tauri-apps/api/core';

vi.mock('@tauri-apps/api/core');

describe('MCP Client', () => {
   beforeEach(() => {
      vi.clearAllMocks();
   });

   describe('getDashboardStats', () => {
      it('calls correct MCP tool', async () => {
         const mockStats = {
            cpu_usage: 45.5,
            total_memory: 16000000,
            used_memory: 8000000,
            active_nodes: 3,
            total_tunnels: 5,
         };

         (invoke as any).mockResolvedValue({
            jsonrpc: '2.0',
            id: 1,
            result: mockStats,
         });

         const result = await mcpClient.getDashboardStats();

         expect(invoke).toHaveBeenCalledWith('mcp_handle_request', {
            request: expect.objectContaining({
               method: 'tools/call',
               params: {
                  name: 'admin_get_dashboard_stats',
                  arguments: {},
               },
            }),
         });

         expect(result).toEqual(mockStats);
      });
   });

   describe('listNodes', () => {
      it('lists all nodes without filter', async () => {
         const mockNodes = [
            { id: 'n1', name: 'Node 1', status: 'active', cpu: 30, memory: 4000000, ip: '192.168.1.10' },
            { id: 'n2', name: 'Node 2', status: 'idle', cpu: 5, memory: 2000000, ip: '192.168.1.11' },
         ];

         (invoke as any).mockResolvedValue({
            jsonrpc: '2.0',
            id: 2,
            result: mockNodes,
         });

         const result = await mcpClient.listNodes();

         expect(invoke).toHaveBeenCalledWith('mcp_handle_request', {
            request: expect.objectContaining({
               params: {
                  name: 'admin_list_nodes',
                  arguments: {},
               },
            }),
         });

         expect(result).toEqual(mockNodes);
      });

      it('filters nodes by status', async () => {
         const mockActiveNodes = [
            { id: 'n1', name: 'Node 1', status: 'active', cpu: 30, memory: 4000000, ip: '192.168.1.10' },
         ];

         (invoke as any).mockResolvedValue({
            jsonrpc: '2.0',
            id: 3,
            result: mockActiveNodes,
         });

         await mcpClient.listNodes('active');

         expect(invoke).toHaveBeenCalledWith('mcp_handle_request', {
            request: expect.objectContaining({
               params: {
                  name: 'admin_list_nodes',
                  arguments: { status_filter: 'active' },
               },
            }),
         });
      });
   });

   describe('restartNode', () => {
      it('sends restart command', async () => {
         (invoke as any).mockResolvedValue({
            jsonrpc: '2.0',
            id: 4,
            result: { success: true, message: 'Node node-1 restart initiated' },
         });

         const result = await mcpClient.restartNode('node-1');

         expect(invoke).toHaveBeenCalledWith('mcp_handle_request', {
            request: expect.objectContaining({
               params: {
                  name: 'admin_restart_node',
                  arguments: { node_id: 'node-1' },
               },
            }),
         });

         expect(result.success).toBe(true);
      });
   });

   describe('updateNodeStatus', () => {
      it('updates node status', async () => {
         (invoke as any).mockResolvedValue({
            jsonrpc: '2.0',
            id: 5,
            result: { success: true, node_id: 'node-1', status: 'maintenance' },
         });

         const result = await mcpClient.updateNodeStatus('node-1', 'maintenance');

         expect(invoke).toHaveBeenCalledWith('mcp_handle_request', {
            request: expect.objectContaining({
               params: {
                  name: 'admin_update_node_status',
                  arguments: { node_id: 'node-1', status: 'maintenance' },
               },
            }),
         });

         expect(result.status).toBe('maintenance');
      });
   });

   describe('error handling', () => {
      it('throws error when MCP returns error', async () => {
         (invoke as any).mockResolvedValue({
            jsonrpc: '2.0',
            id: 6,
            error: {
               code: -32602,
               message: 'Invalid params',
            },
         });

         await expect(mcpClient.getDashboardStats()).rejects.toThrow('MCP Error -32602: Invalid params');
      });
   });

   describe('listTools', () => {
      it('retrieves available tools', async () => {
         const mockTools = [
            { name: 'admin_get_dashboard_stats', description: 'Get stats' },
            { name: 'admin_list_nodes', description: 'List nodes' },
         ];

         (invoke as any).mockResolvedValue({
            jsonrpc: '2.0',
            id: 7,
            result: mockTools,
         });

         const result = await mcpClient.listTools();

         expect(invoke).toHaveBeenCalledWith('mcp_handle_request', {
            request: expect.objectContaining({
               method: 'tools/list',
            }),
         });

         expect(result).toEqual(mockTools);
      });
   });
});

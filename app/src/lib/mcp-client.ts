import { invoke } from '@tauri-apps/api/core';

// Check if running in Tauri environment
const isTauri = typeof window !== 'undefined' && '__TAURI_INTERNALS__' in window;
const API_BASE_URL = 'http://localhost:8080/api/v1';

export interface MCPRequest {
   jsonrpc: string;
   id?: number | string;
   method: string;
   params?: Record<string, unknown>;
}

export interface MCPResponse {
   jsonrpc: string;
   id?: number | string;
   result?: unknown;
   error?: {
      code: number;
      message: string;
      data?: unknown;
   };
}

export interface DashboardStats {
   cpu_usage: number;
   total_memory: number;
   used_memory: number;
   active_nodes: number;
   total_tunnels: number;
}

export interface Node {
   id: string;
   name: string;
   status: string;
   cpu: number;
   memory: number;
   ip: string;
}

/**
 * MCP Client for AI agent control of the dashboard
 */
export class MCPClient {
   private requestId = 0;

   /**
    * Call an MCP tool
    */
   async callTool(
      toolName: string,
      args: Record<string, unknown> = {}
   ): Promise<unknown> {
      const request: MCPRequest = {
         jsonrpc: '2.0',
         id: ++this.requestId,
         method: 'tools/call',
         params: {
            name: toolName,
            arguments: args,
         },
      };

      if (isTauri) {
         const response = await invoke<MCPResponse>('mcp_handle_request', {
            request,
         });

         if (response.error) {
            throw new Error(
               `MCP Error ${response.error.code}: ${response.error.message}`
            );
         }
         return response.result;
      } else {
         // Fallback to HTTP for browser dev
         try {
            const res = await fetch(`${API_BASE_URL}/mcp`, {
               method: 'POST',
               headers: { 'Content-Type': 'application/json' },
               body: JSON.stringify(request),
            });

            if (!res.ok) {
               throw new Error(`HTTP Error: ${res.status} ${res.statusText}`);
            }

            const response = await res.json() as MCPResponse;
            if (response.error) {
               throw new Error(
                  `MCP Error ${response.error.code}: ${response.error.message}`
               );
            }
            return response.result;
         } catch (error) {
            console.warn('MCP HTTP fallback failed:', error);
            // Return mock data for UI development if backend is offline
            if (toolName === 'admin_get_dashboard_stats') {
               return {
                  cpu_usage: 15.5,
                  total_memory: 16 * 1024 * 1024 * 1024,
                  used_memory: 8 * 1024 * 1024 * 1024,
                  active_nodes: 3,
                  total_tunnels: 1
               };
            }
            if (toolName === 'admin_list_nodes') {
               return [
                  { id: 'n1', name: 'Edge-Node-01', status: 'active', cpu: 12, memory: 2.4 * 1024 * 1024 * 1024, ip: '192.168.1.10' },
                  { id: 'n2', name: 'Edge-Node-02', status: 'idle', cpu: 5, memory: 1.1 * 1024 * 1024 * 1024, ip: '192.168.1.11' }
               ];
            }
            throw error;
         }
      }
   }

   /**
    * List available MCP tools
    */
   async listTools(): Promise<unknown> {
      const request: MCPRequest = {
         jsonrpc: '2.0',
         id: ++this.requestId,
         method: 'tools/list',
      };

      if (isTauri) {
         const response = await invoke<MCPResponse>('mcp_handle_request', {
            request,
         });

         if (response.error) {
            throw new Error(
               `MCP Error ${response.error.code}: ${response.error.message}`
            );
         }
         return response.result;
      } else {
         // Fallback to HTTP
         const res = await fetch(`${API_BASE_URL}/mcp`, {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify(request),
         });
         const response = await res.json() as MCPResponse;
         return response.result;
      }
   }

   /**
    * Get dashboard statistics
    */
   async getDashboardStats(): Promise<DashboardStats> {
      return (await this.callTool('admin_get_dashboard_stats')) as DashboardStats;
   }

   /**
    * List all nodes
    */
   async listNodes(statusFilter?: 'active' | 'idle' | 'error' | 'all'): Promise<Node[]> {
      const args = statusFilter ? { status_filter: statusFilter } : {};
      return (await this.callTool('admin_list_nodes', args)) as Node[];
   }

   /**
    * Restart a node
    */
   async restartNode(nodeId: string): Promise<{ success: boolean; message: string }> {
      return (await this.callTool('admin_restart_node', {
         node_id: nodeId,
      })) as { success: boolean; message: string };
   }

   /**
    * Update node status
    */
   async updateNodeStatus(
      nodeId: string,
      status: 'active' | 'idle' | 'error' | 'maintenance'
   ): Promise<{ success: boolean; message: string }> {
      return (await this.callTool('admin_update_node_status', {
         node_id: nodeId,
         status,
      })) as { success: boolean; message: string };
   }

   /**
    * Update stats in the MCP server (called from dashboard store)
    */
   async updateStats(stats: DashboardStats): Promise<void> {
      if (isTauri) {
         await invoke('mcp_update_stats', { stats });
      } else {
         console.log('Mock updateStats:', stats);
      }
   }

   /**
    * Update nodes in the MCP server (called from dashboard store)
    */
   async updateNodes(nodes: Node[]): Promise<void> {
      if (isTauri) {
         await invoke('mcp_update_nodes', { nodes });
      } else {
         console.log('Mock updateNodes:', nodes);
      }
   }
}

// Export singleton instance
export const mcpClient = new MCPClient();

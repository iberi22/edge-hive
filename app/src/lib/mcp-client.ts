import { invoke } from '@tauri-apps/api/core';

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

    const response = await invoke<MCPResponse>('mcp_handle_request', {
      request,
    });

    if (response.error) {
      throw new Error(
        `MCP Error ${response.error.code}: ${response.error.message}`
      );
    }

    return response.result;
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

    const response = await invoke<MCPResponse>('mcp_handle_request', {
      request,
    });

    if (response.error) {
      throw new Error(
        `MCP Error ${response.error.code}: ${response.error.message}`
      );
    }

    return response.result;
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
    await invoke('mcp_update_stats', { stats });
  }

  /**
   * Update nodes in the MCP server (called from dashboard store)
   */
  async updateNodes(nodes: Node[]): Promise<void> {
    await invoke('mcp_update_nodes', { nodes });
  }
}

// Export singleton instance
export const mcpClient = new MCPClient();

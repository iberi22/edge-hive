
import { invoke } from '@tauri-apps/api/core';
import { listen, UnlistenFn } from '@tauri-apps/api/event';
import { SystemMetric, LogEntry, DatabaseTable, EdgeFunction, User, StorageBucket, StorageFile, ApiKey, TopPath, RLSPolicy, OAuthProvider, AccessLogEntry, EmailTemplate, StoragePolicy, Backup, CacheMetrics, GraphNode, GraphEdge, LiveQuery, QueryResult, VPNPeer, ChaosExperiment } from '../types';

export const tauriApi = {
   // Metrics & Logs
   getMetrics: async (): Promise<SystemMetric[]> => {
      try {
         const stats = await invoke<any>('get_system_stats');
         // Convert backend stats to SystemMetric array
         // For now, return a single point or adapt logic.
         // The backend 'get_system_stats' might return a snapshot.
         // We might need to accumulate it on frontend or backend.
         // Assuming snapshot for now:
         return [{
            time: new Date().toLocaleTimeString(),
            cpu: stats.cpu_usage || 0,
            memory: stats.memory_usage || 0,
            latency: 0 // Not provided by system stats yet
         }];
      } catch (e) {
         console.error("Failed to get metrics", e);
         return [];
      }
   },

   getLogs: async (): Promise<LogEntry[]> => {
      try {
         const logs = await invoke<any[]>('get_logs');
         return logs.map((l: any) => ({
            id: l.id,
            timestamp: l.timestamp,
            level: l.level as 'INFO' | 'WARN' | 'ERROR' | 'DEBUG',
            service: l.service,
            message: l.message
         }));
      } catch (e) {
         console.error("Failed to fetch logs", e);
         return [];
      }
   },

   // Database interactions
   getTables: async (): Promise<DatabaseTable[]> => {
      try {
         // 'INFO FOR DB' returns structure in SurrealDB.
         const result = await invoke<any[]>('db_query', { sql: 'INFO FOR DB;' });
         if (result && result[0] && result[0].tables) {
            const tablesObj = result[0].tables;
            return Object.keys(tablesObj).map(name => ({
               name,
               rows: 0, // Need 'count(SELECT * FROM table)' to get real count, expensive
               size: '0 B',
               columns: [], // Parsing 'DEFINE TABLE' string is complex, leaving empty for now
               is_graph: false
            }));
         }
         return [];
      } catch (e) {
         console.error("Failed to get tables", e);
         return [];
      }
   },

   runQuery: async (query: string): Promise<QueryResult> => {
      const start = performance.now();
      try {
         const result = await invoke<any[]>('db_query', { sql: query });
         // Wrapper result from Surreal usually is an array of results for each statement
         // If we send one statement, we take the first one?
         // But db_query implementation returns the whole thing.
         // Let's assume result is the direct array of records for a single query for simplicity,
         // or handle the documented 'Vec<Value>' behavior more robustly.

         // If generic query, let's just assume it's an array of objects
         const rows = Array.isArray(result) ? result : [result];
         // If it's a wrapper like [{ result: [...], status: "OK" }] from HTTP, we'd parse.
         // Using SDK/embedded it might return the data directly.

         const columns = rows.length > 0 && typeof rows[0] === 'object' ? Object.keys(rows[0]) : [];

         return {
            columns,
            rows,
            duration_ms: performance.now() - start,
            affected_rows: rows.length
         };
      } catch (e: any) {
         throw new Error(e.toString());
      }
   },

   // Graph Data
   getGraphData: async (): Promise<{ nodes: GraphNode[], edges: GraphEdge[] }> => {
      try {
         // Attempt to fetch nodes and edges if they exist
         const nodesP = invoke<any[]>('db_query', { sql: 'SELECT * FROM node' });
         const edgesP = invoke<any[]>('db_query', { sql: 'SELECT * FROM edge' });
         const [nodesRaw, edgesRaw] = await Promise.all([nodesP, edgesP]);

         return {
            nodes: Array.isArray(nodesRaw) ? nodesRaw.map((n: any) => ({ id: n.id, label: n.label || n.id, data: n })) : [],
            edges: Array.isArray(edgesRaw) ? edgesRaw.map((e: any) => ({ id: e.id, out: e.out, in: e.in, label: e.label || 'edge' })) : []
         };
      } catch (e) {
         console.warn("Graph tables not present");
         return { nodes: [], edges: [] };
      }
   },

   // Live Queries
   getLiveQueries: async (): Promise<LiveQuery[]> => [],

   // Auth
   signIn: async (email, password) => {
      try {
         return await invoke('login', { email, password });
      } catch (e) {
         console.error("Login failed", e);
         throw e;
      }
   },
   signUp: async (email, password) => {
      try {
         return await invoke('register', { email, password });
      } catch (e) {
         console.error("Register failed", e);
         throw e;
      }
   },
   getCurrentUser: async () => {
      try {
         return await invoke('get_current_user');
      } catch (e) {
         return null;
      }
   },
   getUsers: async () => [], // Admin only, stub for now
   getOAuthProviders: async () => [],
   getEmailTemplates: async () => [],

   // Billing
   getSubscriptionStatus: async (): Promise<any> => {
      try {
         return await invoke('get_subscription_status');
      } catch (e) {
         console.error("Failed to get subscription", e);
         return null;
      }
   },
   getUsageMetrics: async (): Promise<any> => {
      try {
         return await invoke('get_usage_metrics');
      } catch (e) {
         console.error("Failed to get usage", e);
         return { storage_bytes: 0, egress_bytes: 0, api_requests: 0, active_nodes: 0 };
      }
   },
   createCheckoutSession: async (plan: string): Promise<string> => {
      try {
         return await invoke('create_checkout_session', { plan });
      } catch (e) {
         console.error("Checkout failed", e);
         throw e;
      }
   },

   // Cache
   getCacheMetrics: async (): Promise<CacheMetrics> => {
      try {
         const stats: any = await invoke('get_cache_stats');
         return {
            total_keys: stats.l1_entry_count,
            memory_used_mb: 0, // Not exposed yet
            hits_per_sec: stats.total_hits, // Cumulative for now
            misses_per_sec: stats.total_misses,
            avg_latency_us: 0,
            uptime_secs: 0,
            engine_type: stats.l2_enabled ? 'Redis + Moka' : 'Moka (L1 Only)',
            active_transactions: stats.total_writes
         };
      } catch (e) {
         return { total_keys: 0, memory_used_mb: 0, hits_per_sec: 0, misses_per_sec: 0, avg_latency_us: 0, uptime_secs: 0, engine_type: 'unknown', active_transactions: 0 };
      }
   },
   getCacheKeys: async () => {
      try {
         return await invoke('get_cache_keys', { pattern: '*' });
      } catch (e) {
         return [];
      }
   },
   deleteCacheKey: async (key: string) => {
      // TODO: expose delete
   },
   flushCache: async () => {
      try {
         await invoke('clear_cache');
      } catch (e) {
         console.error("Failed to clear cache", e);
      }
   },

   // Tunnel / Network
   startTunnel: async (port: number) => {
      try {
         return await invoke('start_tunnel', { port });
      } catch (e) {
         console.error("Failed to start tunnel", e);
         throw e;
      }
   },
   stopTunnel: async () => {
      try {
         await invoke('stop_tunnel');
      } catch (e) {
         console.error("Failed to stop tunnel", e);
      }
   },
   getTunnelStatus: async (): Promise<{ is_running: boolean; public_url: string | null }> => {
      try {
         return await invoke('get_tunnel_status');
      } catch (e) {
         return { is_running: false, public_url: null };
      }
   },

   // Functions implementation
   getFunctions: async (): Promise<EdgeFunction[]> => {
      try {
         const fns = await invoke<any[]>('list_functions');
         return fns.map((f: any) => ({
            id: f.id,
            name: f.name,
            status: f.status,
            invocations: f.invocations || 0,
            lastRun: f.last_run || 'never',
            source_code: f.description || '',
            env_vars: {}
         }));
      } catch (e) {
         console.error("Failed to list functions", e);
         return [];
      }
   },
   getFunctionVersions: async () => [],
   rollbackFunction: async () => { },

   // Storage implementation
   getBuckets: async (): Promise<StorageBucket[]> => {
      try {
         const buckets = await invoke<any[]>('list_buckets');
         return buckets.map((b: any) => ({
            id: b.id,
            name: b.name,
            public: b.public,
            size: b.size,
            files_count: b.files_count
         }));
      } catch (e) {
         console.error("Failed to list buckets", e);
         return [];
      }
   },
   getFiles: async (bucketId: string): Promise<StorageFile[]> => {
      try {
         const files = await invoke<any[]>('list_files', { bucketId });
         return files.map((f: any) => ({
            id: f.id,
            name: f.name,
            size: f.size,
            type: f.type_ || 'unknown',
            lastModified: f.last_modified
         }));
      } catch (e) {
         console.error("Failed to list files", e);
         return [];
      }
   },

   getStoragePolicies: async () => [],
   getApiKeys: async () => [],
   getBackups: async () => [],
   getTopPaths: async () => [],
   getAccessLogs: async () => [],
   getTasks: async () => [],
   invokeFunction: async (fnId: string, payload: any) => {
      try {
         const result = await invoke<any>('invoke_function', { id: fnId, payload });
         return {
            status: result.status || 200,
            time_ms: 0,
            body: result.result || {},
            logs: []
         };
      } catch (e: any) {
         console.error("Function invocation failed", e);
         return { status: 500, time_ms: 0, body: { error: e.toString() }, logs: [] };
      }
   },
   getMigrations: async () => [],
   getRLSPolicies: async () => [],

   // Real-time Subscriptions
   subscribeToMetrics: async (callback: (metric: SystemMetric) => void): Promise<UnlistenFn> => {
      try {
         return await listen<SystemMetric>('system_metrics', (event) => {
            callback(event.payload);
         });
      } catch (e) {
         console.error("Failed to subscribe to metrics", e);
         return () => { };
      }
   },
   subscribeToLogs: async (callback: (log: LogEntry) => void): Promise<UnlistenFn> => {
      try {
         return await listen<LogEntry>('log_event', (event) => {
            callback({
               ...event.payload,
               level: (event.payload.level as any) || 'INFO'
            });
         });
      } catch (e) {
         console.error("Failed to subscribe to logs", e);
         return () => { };
      }
   },
   subscribeToChaos: async (callback: (exp: ChaosExperiment) => void): Promise<UnlistenFn> => {
      try {
         return await listen<ChaosExperiment>('chaos_update', (event) => {
            callback(event.payload);
         });
      } catch (e) {
         console.error("Failed to subscribe to chaos events", e);
         return () => { };
      }
   },

   // VPN Mesh
   getVPNPeers: async (): Promise<VPNPeer[]> => {
      try {
         return await invoke('get_vpn_peers');
      } catch (e) {
         console.error("Failed to get VPN peers", e);
         return [];
      }
   },
   generateVPNConfig: async (): Promise<string> => {
      try {
         return await invoke('generate_vpn_config');
      } catch (e) {
         console.error("Failed to generate VPN config", e);
         return "";
      }
   },

   // Chaos Lab
   getExperiments: async (): Promise<ChaosExperiment[]> => {
      try {
         return await invoke('get_experiments');
      } catch (e) {
         console.error("Failed to get experiments", e);
         return [];
      }
   },
   runExperiment: async (id: string): Promise<void> => {
      try {
         await invoke('run_experiment', { id });
      } catch (e) {
         console.error("Failed to run experiment", e);
         throw e;
      }
   },
};

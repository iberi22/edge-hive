
import { SystemMetric, LogEntry, DatabaseTable, EdgeFunction, User, StorageBucket, StorageFile, ApiKey, TopPath, RLSPolicy, OAuthProvider, AccessLogEntry, EmailTemplate, StoragePolicy, Backup, TableColumn, SystemTask, EdgeFunctionVersion, Migration, QueryResult, CacheKey, CacheMetrics, GraphNode, GraphEdge, LiveQuery } from '../types';

// Generators
const generateMetrics = (): SystemMetric[] => Array.from({ length: 20 }).map((_, i) => ({
    time: `${10 + i}:00:00`,
    cpu: 20 + Math.random() * 60,
    memory: 40 + Math.random() * 40,
    latency: 1 + Math.random() * 5, // Rust speed: sub-ms
}));

const generateLogs = (): LogEntry[] => Array.from({ length: 20 }).map((_, i) => ({
    id: `log-${i}`,
    timestamp: new Date().toISOString(),
    level: 'INFO',
    service: 'surreal-engine',
    message: `Statement executed: SELECT * FROM person:${Math.floor(Math.random()*100)}`,
}));

// Mock SurrealDB Tables
const MOCK_TABLES: DatabaseTable[] = [
  { name: 'person', rows: 450, size: '12 MB', columns: [
    { name: 'id', type: 'record', is_primary: true, is_nullable: false },
    { name: 'name', type: 'string', is_primary: false, is_nullable: false },
    { name: 'age', type: 'int', is_primary: false, is_nullable: true }
  ] },
  { name: 'post', rows: 1200, size: '45 MB', columns: [
    { name: 'id', type: 'record', is_primary: true, is_nullable: false },
    { name: 'title', type: 'string', is_primary: false, is_nullable: false }
  ] },
  { name: 'wrote', rows: 1200, size: '5 MB', is_graph: true, columns: [
    { name: 'id', type: 'record', is_primary: true, is_nullable: false },
    { name: 'in', type: 'record', is_primary: false, is_nullable: false }, // Post
    { name: 'out', type: 'record', is_primary: false, is_nullable: false } // Person
  ] }
];

const MOCK_GRAPH: { nodes: GraphNode[], edges: GraphEdge[] } = {
    nodes: [
        { id: 'person:tobie', label: 'Tobie', data: { age: 32 } },
        { id: 'person:jaime', label: 'Jaime', data: { age: 28 } },
        { id: 'post:hello', label: 'Hello Surreal', data: { tags: ['rust'] } },
    ],
    edges: [
        { id: 'wrote:1', out: 'person:tobie', in: 'post:hello', label: 'wrote' },
        { id: 'wrote:2', out: 'person:jaime', in: 'post:hello', label: 'wrote' },
    ]
};

const MOCK_LIVE_QUERIES: LiveQuery[] = [
    { id: 'lq_123', table: 'person', query: 'LIVE SELECT * FROM person WHERE age > 18', clients: 4, uptime: '2h 15m' },
    { id: 'lq_456', table: 'post', query: 'LIVE SELECT title FROM post', clients: 125, uptime: '45m' },
];

export const mockApi = {
  getMetrics: async () => generateMetrics(),
  getLogs: async () => generateLogs(),
  getTables: async () => MOCK_TABLES,
  getGraphData: async () => MOCK_GRAPH,
  getLiveQueries: async () => MOCK_LIVE_QUERIES,
  
  // Storage Engine Metrics (Surreal style)
  getCacheMetrics: async () => ({
      total_keys: 154000,
      memory_used_mb: 1024.5,
      hits_per_sec: 45000,
      misses_per_sec: 12,
      avg_latency_us: 45, // 0.045ms
      uptime_secs: 86400,
      engine_type: 'tikv',
      active_transactions: 4
  } as CacheMetrics),

  getCacheKeys: async (prefix?: string) => [],
  deleteCacheKey: async (key: string) => {},
  flushCache: async () => {},
  getRLSPolicies: async (table: string) => [],
  getFunctions: async () => [],
  getFunctionVersions: async (fnId: string) => [],
  rollbackFunction: async (fnId: string, versionId: string) => {},
  getUsers: async () => [],
  getOAuthProviders: async () => [],
  getEmailTemplates: async () => [],
  getBuckets: async () => [],
  getFiles: async (bucketId: string) => [],
  getStoragePolicies: async (bucketId: string) => [],
  getApiKeys: async () => [],
  getBackups: async () => [],
  getTopPaths: async () => [],
  getAccessLogs: async () => [],
  getTasks: async () => [],
  invokeFunction: async (fnId: string, payload: any) => ({ status: 200, time_ms: 0, body: {}, logs: [] }),
  signIn: async (email: string) => ({ token: '', user: {} as any }),
  getMigrations: async () => [],
  runQuery: async (query: string) => ({ columns: ['id', 'val'], rows: [], duration_ms: 1, affected_rows: 0 }),
};

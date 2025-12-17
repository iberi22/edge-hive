
export interface SystemMetric {
  time: string;
  cpu: number;
  memory: number;
  latency: number;
}

export interface LogEntry {
  id: string;
  timestamp: string;
  level: 'INFO' | 'WARN' | 'ERROR' | 'DEBUG';
  service: string;
  message: string;
}

export interface CloudProvider {
  id: 'cloudflare' | 'aws' | 'gcp' | 'azure';
  name: string;
  status: 'connected' | 'offline' | 'linking';
  resources: number;
  cost_mtd: number;
  icon_color: string;
}

export type ViewState = 'dashboard' | 'data' | 'auth' | 'storage' | 'functions' | 'settings' | 'tasks' | 'cache' | 'sharding' | 'quantum' | 'observability' | 'integrations' | 'billing' | 'governance' | 'federation' | 'deep-edge' | 'onion' | 'vpn' | 'chaos-lab' | 'ledger';

export interface ChaosExperiment {
  id: string;
  type: 'latency' | 'node_failure' | 'network_partition' | 'storage_corruption';
  target: string;
  intensity: number;
  status: 'idle' | 'running' | 'completed' | 'healing';
  impact_score: number;
}

export interface LedgerBlock {
  id: string;
  timestamp: string;
  operation: string;
  record_id: string;
  hash: string;
  prev_hash: string;
  verified_by: string[];
}

export interface OnionService {
  id: string;
  address: string;
  port: number;
  target_node: string;
  uptime: string;
  status: 'active' | 'rotating' | 'offline';
}

export interface VPNPeer {
  id: string;
  public_key: string;
  endpoint: string;
  allowed_ips: string[];
  last_handshake: string;
  transfer_rx: string;
  transfer_tx: string;
  status: 'connected' | 'idle' | 'failed';
}

export interface PhysicalNode {
  id: string;
  type: 'gateway' | 'sensor' | 'compute_unit';
  location: string;
  coords: { x: number, y: number };
  status: 'online' | 'warning' | 'critical';
  metrics: {
    temp: number;
    power: number;
    signal: number;
  };
}

export interface TableColumn {
  name: string;
  type: string;
  is_primary: boolean;
  is_nullable: boolean;
}

export interface DatabaseTable {
  name: string;
  rows: number;
  size: string;
  columns: TableColumn[];
  is_graph?: boolean;
}

export interface EdgeFunction {
  id: string;
  name: string;
  status: string;
  invocations: number;
  lastRun: string;
  source_code: string;
  env_vars: any;
}

export interface User {
  id: string;
  email: string;
  provider: string;
  created_at: string;
  last_sign_in: string;
  status: string;
}

export interface StorageBucket {
  id: string;
  name: string;
  public: boolean;
  size: string;
  files_count: number;
}

export interface StorageFile {
  id: string;
  name: string;
  size: string;
  type: string;
  lastModified?: string;
}

export interface ApiKey {
  id: string;
  name: string;
  prefix: string;
  created_at: string;
  role: string;
}

export interface Backup {
  id: string;
  name: string;
  size: string;
  created_at: string;
  status: string;
  type: string;
}

export interface SystemTask {
  id: string;
  title: string;
  description: string;
  status: string;
  priority: string;
  due_date: string;
  created_at: string;
  assignee?: string;
}

export interface CacheMetrics {
  total_keys: number;
  memory_used_mb: number;
  hits_per_sec: number;
  misses_per_sec: number;
  avg_latency_us: number;
  uptime_secs: number;
  engine_type: string;
  active_transactions: number;
}

export interface LiveQuery {
  id: string;
  table: string;
  query: string;
  clients: number;
  uptime: string;
}

export interface GraphNode {
  id: string;
  label: string;
  data: any;
}

export interface GraphEdge {
  id: string;
  out: string;
  in: string;
  label: string;
}

export interface TopPath {
  path: string;
  requests: number;
  latency: number;
}

export interface RLSPolicy {
  id: string;
  name: string;
  table: string;
  action: string;
  check: string;
}

export interface OAuthProvider {
  id: string;
  name: string;
  enabled: boolean;
}

export interface AccessLogEntry {
  id: string;
  timestamp: string;
  method: string;
  path: string;
  status_code: number;
  duration_ms: number;
  ip_address: string;
}

export interface EmailTemplate {
  id: string;
  name: string;
  subject: string;
  content: string;
}

export interface StoragePolicy {
  id: string;
  name: string;
  bucket: string;
  definition: string;
}

export interface EdgeFunctionVersion {
  id: string;
  function_id: string;
  version: string;
  created_at: string;
  status: string;
  author: string;
  commit_message: string;
}

export interface Migration {
  id: string;
  name: string;
  executed_at: string;
}

export interface QueryResult {
  columns: string[];
  rows: any[];
  duration_ms: number;
  affected_rows: number;
}

export interface CacheKey {
  key: string;
  value: string;
  ttl: number;
}

export interface QueryHistoryItem {
  id: string;
  query: string;
  timestamp: string;
}

export interface FunctionInvocationResult {
  status: number;
  time_ms: number;
  body: any;
  logs: string[];
}

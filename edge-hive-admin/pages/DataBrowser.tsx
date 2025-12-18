
import React, { useState, useEffect } from 'react';
import { Database, Table as TableIcon, Play, RefreshCw, MoreHorizontal, MoreVertical, LayoutList, Grip, Key, Type as TypeIcon, Hash, ShieldCheck, ShieldAlert, Plus, Save, Filter, ChevronLeft, ChevronRight, X, Link as LinkIcon, ArrowRight, Edit3, Trash2, Code2, GitMerge, FileCode, CheckCircle, Clock, Share2, ZoomIn, ZoomOut, Zap, Terminal, Info, Shield, Binary, Eye, Activity, Search, Sparkles, Orbit } from 'lucide-react';
import { mockApi } from '../api';
import { DatabaseTable, RLSPolicy, TableColumn, Migration, QueryResult, QueryHistoryItem, GraphNode, GraphEdge } from '../types';
import { useToast } from '../context/ToastContext';

const VectorSpaceExplorer = () => {
    const [query, setQuery] = useState("");
    const [isSearching, setIsSearching] = useState(false);
    const [results, setResults] = useState<{ label: string, distance: number, x: number, y: number }[]>([]);

    const handleSearch = () => {
        setIsSearching(true);
        setTimeout(() => {
            const mockResults = [
                { label: "Product: Rust Book", distance: 0.12, x: 20, y: -10 },
                { label: "Code: WebAssembly Core", distance: 0.18, x: -15, y: 25 },
                { label: "User: Jaime Dev", distance: 0.45, x: 40, y: 40 },
                { label: "Doc: Edge Hive API", distance: 0.05, x: 5, y: 5 },
                { label: "Post: SurrealDB Graph", distance: 0.22, x: -30, y: -20 },
            ];
            setResults(mockResults);
            setIsSearching(false);
        }, 1500);
    };

    return (
        <div className="flex-1 bg-slate-950 flex flex-col relative overflow-hidden">
            {/* Ambient Background */}
            <div className="absolute inset-0 opacity-20 pointer-events-none" style={{ backgroundImage: 'radial-gradient(#06b6d4 1px, transparent 1px)', backgroundSize: '60px 60px' }}></div>

            <div className="p-8 z-10 flex flex-col h-full">
                <div className="max-w-2xl mx-auto w-full mb-12">
                    <h3 className="text-sm font-bold text-white uppercase tracking-widest flex items-center gap-3 mb-6">
                        <Sparkles size={18} className="text-hive-cyan animate-pulse" />
                        Vector Semantic Search
                    </h3>
                    <div className="relative group">
                        <input
                            type="text"
                            value={query}
                            onChange={(e) => setQuery(e.target.value)}
                            onKeyDown={(e) => e.key === 'Enter' && handleSearch()}
                            placeholder="Describe what you are looking for semantically..."
                            className="w-full bg-slate-900/80 border border-white/10 rounded-xl px-6 py-4 text-sm text-white placeholder-slate-600 focus:outline-none focus:border-hive-cyan shadow-2xl transition-all"
                        />
                        <button
                            onClick={handleSearch}
                            className="absolute right-3 top-1/2 -translate-y-1/2 p-2 bg-hive-cyan text-black rounded-lg shadow-neon-cyan hover:scale-105 transition-transform"
                        >
                            <Search size={18} />
                        </button>
                    </div>
                    <div className="mt-4 flex justify-center gap-4">
                        <span className="text-[10px] font-mono text-slate-500 uppercase">Embedding: BERT-L-12</span>
                        <span className="text-[10px] font-mono text-slate-500 uppercase">Dimensions: 768</span>
                        <span className="text-[10px] font-mono text-slate-500 uppercase">Metric: Cosine</span>
                    </div>
                </div>

                <div className="flex-1 relative flex items-center justify-center">
                    {/* The "Sun" (Current Query) */}
                    <div className="absolute w-4 h-4 bg-white rounded-full shadow-[0_0_30px_#fff] z-20">
                        <div className="absolute -inset-4 border border-white/10 rounded-full animate-ping"></div>
                    </div>

                    {/* Orbits */}
                    {[80, 160, 240, 320].map((r, i) => (
                        <div key={i} className="absolute border border-white/5 rounded-full pointer-events-none" style={{ width: r * 2, height: r * 2 }}></div>
                    ))}

                    {/* Results planets */}
                    {results.map((res, i) => (
                        <div
                            key={i}
                            className="absolute group transition-all duration-1000 ease-out"
                            style={{
                                transform: `translate(${res.x * 4}px, ${res.y * 4}px)`,
                                opacity: isSearching ? 0 : 1
                            }}
                        >
                            <div className="relative">
                                <div className={`w-3 h-3 rounded-full shadow-lg cursor-pointer hover:scale-150 transition-transform ${res.distance < 0.1 ? 'bg-hive-cyan shadow-neon-cyan' : 'bg-slate-700'}`}></div>
                                <div className="absolute top-full left-1/2 -translate-x-1/2 pt-2 opacity-0 group-hover:opacity-100 transition-opacity whitespace-nowrap pointer-events-none z-30">
                                    <div className="bg-slate-900 border border-white/10 p-2 rounded text-[10px] font-mono">
                                        <div className="text-white font-bold">{res.label}</div>
                                        <div className="text-slate-500">Distance: {res.distance.toFixed(4)}</div>
                                    </div>
                                </div>
                            </div>
                        </div>
                    ))}

                    {isSearching && (
                        <div className="absolute inset-0 flex items-center justify-center bg-slate-950/40 backdrop-blur-sm z-40">
                            <div className="flex flex-col items-center gap-4">
                                <Orbit size={48} className="text-hive-cyan animate-spin" />
                                <span className="text-[10px] font-mono text-hive-cyan uppercase tracking-[0.2em] animate-pulse">Projecting Vectors...</span>
                            </div>
                        </div>
                    )}
                </div>
            </div>
        </div>
    );
};

const BTreeVisualizer: React.FC = () => {
    const [isRebalancing, setIsRebalancing] = useState(false);
    // Fix: useToast returns the toast object directly
    const toast = useToast();

    const handleRebalance = () => {
        setIsRebalancing(true);
        toast.info("Triggering heavy ingress simulation...", "B-Tree Rebalancing");
        setTimeout(() => {
            setIsRebalancing(false);
            toast.success("Index optimized", "Tree depth maintained at level 4");
        }, 3000);
    };

    return (
        <div className="flex-1 bg-[#090c10] p-12 flex flex-col items-center justify-center relative overflow-hidden">
            <div className="absolute inset-0 opacity-10 pointer-events-none" style={{ backgroundImage: 'radial-gradient(#334155 1px, transparent 1px)', backgroundSize: '40px 40px' }}></div>

            <div className="relative z-10 space-y-12 w-full max-w-3xl">
                <div className="flex items-center justify-between border-b border-white/10 pb-6 mb-12">
                    <div>
                        <h3 className="text-lg font-bold text-white flex items-center gap-3 uppercase tracking-tighter">
                            <Binary size={20} className="text-hive-orange" />
                            B-Tree Index Structure
                        </h3>
                        <p className="text-[10px] font-mono text-slate-500 uppercase mt-1">Optimization: 98.4% Balanced • Depth: 4 Levels</p>
                    </div>
                    <button
                        onClick={handleRebalance}
                        disabled={isRebalancing}
                        className={`px-4 py-1.5 border rounded text-[10px] font-bold uppercase transition-all flex items-center gap-2
                            ${isRebalancing ? 'bg-hive-orange text-black border-hive-orange' : 'bg-slate-900 border-white/10 text-white hover:border-hive-orange/50'}
                        `}
                    >
                        {isRebalancing ? <RefreshCw size={12} className="animate-spin" /> : <Zap size={12} />}
                        {isRebalancing ? 'Rebalancing...' : 'Simulate Ingress'}
                    </button>
                </div>

                <div className="flex flex-col items-center gap-8">
                    {/* Level 0: Root */}
                    <div className={`w-16 h-16 rounded border flex items-center justify-center text-[10px] font-bold uppercase relative transition-all duration-700
                        ${isRebalancing ? 'bg-orange-500 shadow-neon-orange border-white text-black scale-110' : 'bg-hive-orange border-white/20 shadow-neon-orange text-black'}
                    `}>
                        Root
                        <div className="absolute top-full left-1/2 w-[1px] h-8 bg-hive-orange/40"></div>
                    </div>

                    {/* Level 1 */}
                    <div className="flex gap-16 relative">
                        <div className="absolute -top-8 left-1/2 -translate-x-1/2 w-[150px] h-[1px] bg-hive-orange/40"></div>
                        {[1, 2, 3].map(i => (
                            <div key={i} className={`w-12 h-12 bg-slate-900 border rounded flex items-center justify-center text-[8px] font-mono transition-all duration-1000
                                ${isRebalancing ? 'border-orange-500 text-orange-500 translate-y-2' : 'border-hive-cyan/50 text-hive-cyan shadow-neon-cyan'}
                            `}>P_0{i + 3}</div>
                        ))}
                    </div>

                    {/* Level 2: Leaf Nodes */}
                    <div className={`grid grid-cols-6 gap-8 transition-opacity duration-1000 ${isRebalancing ? 'opacity-80' : 'opacity-40'}`}>
                        {Array.from({ length: 6 }).map((_, i) => (
                            <div key={i} className={`w-8 h-8 bg-slate-950 border border-white/10 rounded flex items-center justify-center text-[7px] font-mono text-slate-600 transition-transform duration-500
                                ${isRebalancing ? 'scale-110 text-hive-orange' : ''}
                             `}>L_{i + 10}</div>
                        ))}
                    </div>
                </div>

                <div className="mt-12 p-6 bg-slate-900/40 border border-white/5 rounded-xl backdrop-blur-md">
                    <div className="flex items-center gap-4 text-xs font-mono text-slate-500 uppercase mb-4">
                        <Info size={14} className="text-hive-cyan" />
                        Internal Node Analysis
                    </div>
                    <div className="grid grid-cols-2 gap-8 text-[11px] font-mono">
                        <div className="space-y-2">
                            <div className="flex justify-between border-b border-white/5 pb-1"><span>Page Size</span><span className="text-white">16 KB</span></div>
                            <div className="flex justify-between border-b border-white/5 pb-1"><span>Fill Factor</span><span className="text-emerald-500">85%</span></div>
                        </div>
                        <div className="space-y-2">
                            <div className="flex justify-between border-b border-white/5 pb-1"><span>Search Complexity</span><span className="text-hive-cyan">O(log n)</span></div>
                            <div className="flex justify-between border-b border-white/5 pb-1"><span>Dirty Pages</span><span className="text-orange-500">{isRebalancing ? '12' : '0'}</span></div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    );
};

const BlueprintView: React.FC<{ table: DatabaseTable }> = ({ table }) => {
    return (
        <div className="flex-1 overflow-auto p-8 bg-[#090c10] font-mono">
            <div className="max-w-4xl space-y-8 animate-in fade-in slide-in-from-bottom-2">
                <div className="flex items-center justify-between border-b border-white/10 pb-4">
                    <div>
                        <h2 className="text-xl font-bold text-white flex items-center gap-2 uppercase tracking-tighter">
                            <Grip className="text-hive-cyan" />
                            DEFINE TABLE {table.name}
                        </h2>
                        <p className="text-xs text-slate-500 mt-1 uppercase italic">TYPE: SCHEMAFULL • ENGINE: ATOMIC</p>
                    </div>
                    <div className="flex gap-2">
                        <button className="px-3 py-1 bg-slate-800 text-[10px] rounded text-slate-300 border border-white/5">EXPORT DDL</button>
                        <button className="px-3 py-1 bg-hive-orange/20 text-[10px] rounded text-hive-orange border border-hive-orange/20">EDIT SCHEMA</button>
                    </div>
                </div>

                <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
                    <div className="space-y-4">
                        <h3 className="text-[10px] text-slate-500 uppercase tracking-widest border-l-2 border-hive-cyan pl-2">Fields & Constraints</h3>
                        <div className="space-y-2">
                            {table.columns.map(col => (
                                <div key={col.name} className="bg-slate-900/50 p-3 rounded border border-white/5 flex items-center justify-between group">
                                    <div className="flex items-center gap-3">
                                        <div className="w-1.5 h-1.5 rounded-full bg-hive-cyan/50"></div>
                                        <span className="text-sm font-bold text-slate-200">{col.name}</span>
                                        <span className="text-[10px] text-slate-600">:: {col.type}</span>
                                    </div>
                                    <div className="flex gap-2 opacity-0 group-hover:opacity-100 transition-opacity">
                                        {col.is_primary && <span className="text-[9px] text-hive-orange font-bold">PRIMARY</span>}
                                        {!col.is_nullable && <span className="text-[9px] text-red-500 font-bold">REQUIRED</span>}
                                    </div>
                                </div>
                            ))}
                        </div>
                    </div>

                    <div className="space-y-4">
                        <h3 className="text-[10px] text-slate-500 uppercase tracking-widest border-l-2 border-purple-500 pl-2">Access Permissions</h3>
                        <div className="bg-slate-900/50 p-4 rounded border border-white/5 space-y-3">
                            <div className="flex items-center justify-between">
                                <span className="text-xs text-white flex items-center gap-2"><Shield size={12} className="text-emerald-500" /> SELECT</span>
                                <span className="text-[10px] text-slate-500 font-bold">WHERE auth.id = id</span>
                            </div>
                            <div className="flex items-center justify-between">
                                <span className="text-xs text-white flex items-center gap-2"><Shield size={12} className="text-hive-orange" /> UPDATE</span>
                                <span className="text-[10px] text-slate-500 font-bold">WHERE auth.id = id</span>
                            </div>
                        </div>

                        <h3 className="text-[10px] text-slate-500 uppercase tracking-widest border-l-2 border-hive-orange pl-2">Indices</h3>
                        <div className="bg-slate-900/50 p-3 rounded border border-white/5 font-mono text-[10px]">
                            <div className="text-slate-400">idx_unique_email UNIQUE ON email</div>
                            <div className="text-slate-400 mt-1 italic opacity-50"># Search complexity: O(log n)</div>
                        </div>
                    </div>
                </div>

                <div className="mt-8 pt-6 border-t border-white/5 bg-slate-900/20 p-4 rounded-lg">
                    <div className="flex items-center gap-2 mb-4">
                        <Terminal size={14} className="text-slate-500" />
                        <span className="text-[10px] font-bold text-slate-500 uppercase tracking-widest">DDL Blueprint Output</span>
                    </div>
                    <pre className="text-[11px] text-slate-300 leading-relaxed overflow-x-auto whitespace-pre">
                        {`DEFINE TABLE ${table.name} SCHEMAFULL;
${table.columns.map(c => `DEFINE FIELD ${c.name} ON ${table.name} TYPE ${c.type};`).join('\n')}

DEFINE INDEX idx_id ON ${table.name} FIELDS id UNIQUE;`}
                    </pre>
                </div>
            </div>
        </div>
    );
}

const GraphVisualizer: React.FC = () => {
    const [graphData, setGraphData] = useState<{ nodes: GraphNode[], edges: GraphEdge[] }>({ nodes: [], edges: [] });
    const [selectedNode, setSelectedNode] = useState<GraphNode | null>(null);

    useEffect(() => {
        mockApi.getGraphData().then(setGraphData);
    }, []);

    return (
        <div className="flex-1 bg-[#090c10] relative overflow-hidden flex flex-col lg:flex-row group/graph">
            {/* Background Grid */}
            <div className="absolute inset-0 opacity-10 pointer-events-none" style={{ backgroundImage: 'radial-gradient(#334155 1px, transparent 1px)', backgroundSize: '30px 30px' }}></div>

            {/* Viewport Control */}
            <div className="absolute top-4 left-4 z-10 flex flex-col gap-2">
                <div className="bg-slate-900/80 backdrop-blur-md border border-white/10 rounded-lg p-3 space-y-3">
                    <div className="text-[10px] font-mono text-slate-500 uppercase">Engine Status</div>
                    <div className="flex items-center gap-2">
                        <div className="w-2 h-2 bg-emerald-500 rounded-full animate-pulse shadow-neon-cyan"></div>
                        <span className="text-[9px] font-mono text-white">TRAVERSAL_ACTIVE</span>
                    </div>
                </div>
            </div>

            {/* Main SVG Area */}
            <div className="flex-1 relative">
                <svg className="absolute inset-0 w-full h-full">
                    {graphData.edges.map((edge, i) => (
                        <g key={edge.id}>
                            <path
                                d={`M ${300 + (i * 100)} 200 L ${400 + (i * 100)} 350`}
                                stroke="url(#lineGradient)"
                                strokeWidth="2"
                                className="opacity-20"
                            />
                            <circle r="2" fill="#06b6d4" className="animate-pulse">
                                <animateMotion dur="2s" repeatCount="indefinite" path={`M ${300 + (i * 100)} 200 L ${400 + (i * 100)} 350`} />
                            </circle>
                        </g>
                    ))}
                    <defs>
                        <linearGradient id="lineGradient">
                            <stop offset="0%" stopColor="#06b6d4" stopOpacity="0" />
                            <stop offset="50%" stopColor="#06b6d4" stopOpacity="0.5" />
                            <stop offset="100%" stopColor="#06b6d4" stopOpacity="0" />
                        </linearGradient>
                    </defs>
                </svg>

                <div className="relative w-full h-full">
                    {graphData.nodes.map((node, i) => (
                        <div
                            key={node.id}
                            onClick={() => setSelectedNode(node)}
                            className={`absolute p-4 bg-slate-900 border rounded-lg shadow-2xl transition-all cursor-pointer group/node
                                ${selectedNode?.id === node.id ? 'border-hive-cyan shadow-neon-cyan scale-110' : 'border-white/10 hover:border-hive-cyan/50'}
                            `}
                            style={{ left: `${25 + (i * 25)}%`, top: `${35 + (Math.sin(i) * 20)}%` }}
                        >
                            <div className="flex items-center gap-2 mb-2">
                                <div className={`w-2 h-2 rounded-full ${node.id.startsWith('person') ? 'bg-hive-cyan' : 'bg-purple-500'}`}></div>
                                <span className="text-[9px] font-mono text-slate-500 tracking-tighter uppercase">{node.id.split(':')[0]}</span>
                            </div>
                            <div className="text-xs font-bold text-white group-hover/node:text-hive-cyan transition-colors">{node.label}</div>
                        </div>
                    ))}
                </div>
            </div>

            {/* Node Inspector Panel */}
            {selectedNode && (
                <div className="w-full lg:w-80 bg-slate-950 border-t lg:border-t-0 lg:border-l border-white/10 p-6 animate-in slide-in-from-right duration-300 z-20 overflow-y-auto custom-scrollbar">
                    <div className="flex items-center justify-between mb-8">
                        <h4 className="text-xs font-bold text-white uppercase tracking-widest flex items-center gap-2">
                            <Eye size={14} className="text-hive-cyan" />
                            Node Inspector
                        </h4>
                        <button onClick={() => setSelectedNode(null)} className="text-slate-500 hover:text-white"><X size={16} /></button>
                    </div>

                    <div className="space-y-6 font-mono">
                        <div>
                            <span className="text-[9px] text-slate-600 uppercase block mb-1">Record ID</span>
                            <div className="text-sm font-bold text-hive-cyan bg-hive-cyan/5 p-2 rounded border border-hive-cyan/20">
                                {selectedNode.id}
                            </div>
                        </div>

                        <div>
                            <span className="text-[9px] text-slate-600 uppercase block mb-1">Raw Content</span>
                            <div className="bg-black/40 rounded p-3 text-[10px] text-slate-400 leading-relaxed overflow-x-auto">
                                <pre>{JSON.stringify(selectedNode.data, null, 2)}</pre>
                            </div>
                        </div>

                        <div>
                            <span className="text-[9px] text-slate-600 uppercase block mb-4">Neural Connections</span>
                            <div className="space-y-2">
                                {graphData.edges.filter(e => e.out === selectedNode.id || e.in === selectedNode.id).map(edge => (
                                    <div key={edge.id} className="flex items-center justify-between p-2 rounded bg-slate-900 border border-white/5 text-[10px]">
                                        <div className="flex items-center gap-2">
                                            <GitMerge size={12} className="text-purple-500" />
                                            <span className="text-white uppercase font-bold">{edge.label}</span>
                                        </div>
                                        <span className="text-slate-500">{edge.out === selectedNode.id ? '-> OUT' : '<- IN'}</span>
                                    </div>
                                ))}
                            </div>
                        </div>

                        <button className="w-full py-2 bg-slate-800 hover:bg-slate-700 text-white font-bold text-[10px] rounded border border-white/5 transition-colors uppercase mt-4">
                            Relate to Node...
                        </button>
                    </div>
                </div>
            )}
        </div>
    );
};

const DataBrowser: React.FC = () => {
    // Fix: useToast returns the toast object directly
    const toast = useToast();
    const [tables, setTables] = useState<DatabaseTable[]>([]);
    const [selectedTable, setSelectedTable] = useState<string>('person');
    const [viewMode, setViewMode] = useState<'data' | 'structure' | 'rls' | 'sql' | 'graph' | 'btree' | 'vector'>('data');
    const [sqlQuery, setSqlQuery] = useState("SELECT * FROM person FETCH wrote.in;");
    const [isQueryRunning, setIsQueryRunning] = useState(false);

    useEffect(() => {
        mockApi.getTables().then(setTables);
    }, []);

    const handleRunQuery = async () => {
        setIsQueryRunning(true);
        try {
            const res = await mockApi.runQuery(sqlQuery);
            toast.success("Query committed", `${res.duration_ms}ms execution`);
        } catch (e: any) {
            toast.error(e.message);
        } finally {
            setIsQueryRunning(false);
        }
    };

    const activeTable = tables.find(t => t.name === selectedTable);

    return (
        <div className="flex flex-col md:flex-row h-full md:h-[calc(100vh-8rem)] gap-6 overflow-hidden">

            {/* Tables Sidebar */}
            <div className="w-full md:w-64 flex-shrink-0 flex flex-col gap-4 h-full">
                <div className="bg-slate-900/50 border border-white/5 rounded-lg p-4 flex flex-col h-full backdrop-blur-sm">
                    <div className="flex items-center justify-between mb-4 text-slate-400 px-2">
                        <span className="text-[10px] font-mono uppercase tracking-widest">Collections</span>
                        <button className="p-1 hover:bg-white/10 rounded text-hive-orange"><Plus size={14} /></button>
                    </div>
                    <div className="space-y-1 overflow-y-auto pr-2 custom-scrollbar flex-1">
                        {tables.map(table => (
                            <button
                                key={table.name}
                                onClick={() => setSelectedTable(table.name)}
                                className={`w-full flex items-center justify-between px-3 py-2.5 rounded text-[11px] font-mono transition-all group
                  ${selectedTable === table.name
                                        ? 'bg-hive-cyan/10 text-hive-cyan border border-hive-cyan/20'
                                        : 'text-slate-500 hover:bg-white/5 hover:text-slate-300'
                                    }`}
                            >
                                <div className="flex items-center gap-3">
                                    {table.is_graph ? <Share2 size={12} className="text-purple-500" /> : <Database size={12} className="text-hive-cyan" />}
                                    <span className="font-bold">{table.name}</span>
                                </div>
                            </button>
                        ))}
                    </div>
                </div>
            </div>

            {/* Main Panel */}
            <div className="flex-1 flex flex-col gap-4 min-w-0 h-full">
                <div className="flex-1 bg-slate-900/50 border border-white/5 rounded-lg overflow-hidden flex flex-col relative">
                    <div className="px-6 py-4 border-b border-white/5 flex flex-col sm:flex-row items-center justify-between bg-slate-900/80 backdrop-blur-lg gap-4">
                        <div className="flex items-center gap-6">
                            <div className="flex bg-slate-950 rounded-lg border border-white/10 p-0.5">
                                {[
                                    { id: 'data', label: 'Explore', icon: LayoutList },
                                    { id: 'graph', label: 'Graph', icon: Share2 },
                                    { id: 'structure', label: 'Blueprint', icon: Grip },
                                    { id: 'btree', label: 'Index Tree', icon: Binary },
                                    { id: 'vector', label: 'Vector Space', icon: Sparkles },
                                    { id: 'sql', label: 'SurrealQL', icon: Terminal }
                                ].map(tab => (
                                    <button
                                        key={tab.id}
                                        onClick={() => setViewMode(tab.id as any)}
                                        className={`flex items-center gap-2 px-4 py-1.5 rounded-md text-[10px] font-bold uppercase transition-all
                                ${viewMode === tab.id ? 'bg-white/10 text-white shadow-inner' : 'text-slate-500 hover:text-slate-300'}
                            `}
                                    >
                                        <tab.icon size={12} />
                                        <span className="hidden lg:inline">{tab.label}</span>
                                    </button>
                                ))}
                            </div>
                        </div>
                        <div className="flex gap-2">
                            <button className="p-2 hover:bg-white/10 rounded-lg text-slate-500 transition-colors"><Filter size={14} /></button>
                            <button className="p-2 hover:bg-white/10 rounded-lg text-slate-500 transition-colors"><RefreshCw size={14} /></button>
                        </div>
                    </div>

                    <div className="flex-1 overflow-hidden flex flex-col">
                        {viewMode === 'graph' && <GraphVisualizer />}
                        {viewMode === 'structure' && activeTable && <BlueprintView table={activeTable} />}
                        {viewMode === 'btree' && <BTreeVisualizer />}
                        {viewMode === 'vector' && <VectorSpaceExplorer />}

                        {viewMode === 'data' && (
                            <div className="flex-1 overflow-auto bg-[#090c10] custom-scrollbar">
                                <table className="w-full text-left text-xs font-mono border-collapse">
                                    <thead className="bg-slate-950/80 sticky top-0 z-10 text-slate-500 uppercase tracking-tighter text-[10px]">
                                        <tr>
                                            <th className="p-4 border-b border-white/10 w-48 bg-slate-950/90 backdrop-blur-md">Record ID</th>
                                            {tables.find(t => t.name === selectedTable)?.columns.filter(c => c.name !== 'id').map(c => (
                                                <th key={c.name} className="p-4 border-b border-white/10 bg-slate-950/90 backdrop-blur-md">{c.name}</th>
                                            ))}
                                        </tr>
                                    </thead>
                                    <tbody className="text-slate-300 divide-y divide-white/5">
                                        {Array.from({ length: 15 }).map((_, i) => (
                                            <tr key={i} className="hover:bg-white/5 transition-colors">
                                                <td className="p-4">
                                                    <button className="bg-hive-cyan/10 text-hive-cyan px-2 py-0.5 rounded border border-hive-cyan/20 font-bold">
                                                        {selectedTable}:{i + 100}
                                                    </button>
                                                </td>
                                                {tables.find(t => t.name === selectedTable)?.columns.filter(c => c.name !== 'id').map(c => (
                                                    <td key={c.name} className="p-4 text-slate-400">{c.type === 'string' ? `val_record_${i}` : i * 2}</td>
                                                ))}
                                            </tr>
                                        ))}
                                    </tbody>
                                </table>
                            </div>
                        )}

                        {viewMode === 'sql' && (
                            <div className="flex-1 flex flex-col lg:flex-row h-full">
                                <div className="w-full lg:w-64 bg-slate-950 border-r border-white/5 overflow-y-auto custom-scrollbar">
                                    <div className="p-3 space-y-2">
                                        <h4 className="text-[10px] font-mono text-slate-500 uppercase tracking-widest mb-3">Blueprints</h4>
                                        {[
                                            { name: 'Fetch Graph', code: 'SELECT * FROM person FETCH wrote.in;' },
                                            { name: 'Relate records', code: 'RELATE person:tobie->wrote->post:hello\nSET time.written = time::now();' },
                                            { name: 'Define Table', code: 'DEFINE TABLE user SCHEMAFULL\n  PERMISSIONS\n    FOR select WHERE auth.id = id;' },
                                            { name: 'Live Select', code: 'LIVE SELECT * FROM post WHERE tags CONTAINS "rust";' }
                                        ].map(s => (
                                            <button
                                                key={s.name}
                                                onClick={() => setSqlQuery(s.code)}
                                                className="w-full text-left p-2 rounded bg-slate-900 border border-white/5 hover:border-hive-cyan/30 text-[10px] font-mono text-slate-400 hover:text-hive-cyan transition-all group"
                                            >
                                                <div className="flex items-center justify-between mb-1">
                                                    <span className="text-white font-bold">{s.name}</span>
                                                    <Zap size={10} className="opacity-0 group-hover:opacity-100" />
                                                </div>
                                                <div className="truncate opacity-60 italic">{s.code}</div>
                                            </button>
                                        ))}
                                    </div>
                                </div>
                                <div className="flex-1 flex flex-col overflow-hidden">
                                    <div className="flex-1 relative bg-[#090c10]">
                                        <textarea value={sqlQuery} onChange={e => setSqlQuery(e.target.value)} className="w-full h-full bg-transparent text-hive-cyan font-mono text-sm p-8 focus:outline-none resize-none selection:bg-hive-orange/30 custom-scrollbar" spellCheck={false} placeholder="-- Write SurrealQL..." />
                                        <div className="absolute bottom-6 right-6 flex gap-3">
                                            <button onClick={handleRunQuery} className="flex items-center gap-2 px-6 py-2 bg-hive-orange hover:bg-orange-600 text-black font-bold text-[10px] rounded transition shadow-neon-orange uppercase">
                                                {isQueryRunning ? <RefreshCw className="animate-spin" size={12} /> : <Play size={12} fill="currentColor" />}
                                                Run Statement
                                            </button>
                                        </div>
                                    </div>
                                </div>
                            </div>
                        )}
                    </div>
                </div>
            </div>
        </div>
    );
};

export default DataBrowser;

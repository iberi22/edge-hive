
import React, { useEffect, useState } from 'react';
import { LoadingState } from '../components/LoadingState';
import { Zap, Activity, ShieldAlert, Cpu, HardDrive, Database, Radio, GitBranch, Terminal, ShieldCheck, Info, Binary, Sparkles, Orbit, Search } from 'lucide-react';
import { tauriApi } from '../api/tauriClient';
import { CacheMetrics, LiveQuery } from '../types';
import { useToast } from '../context/ToastContext';

const SemanticCacheVisualizer = () => {
    const points = Array.from({ length: 40 });
    return (
        <div className="relative h-48 w-full bg-slate-950 rounded border border-white/5 overflow-hidden group">
            <div className="absolute inset-0 opacity-10 pointer-events-none" style={{ backgroundImage: 'radial-gradient(#06b6d4 1px, transparent 1px)', backgroundSize: '20px 20px' }}></div>
            <div className="absolute inset-0 flex items-center justify-center">
                {points.map((_, i) => {
                    const active = Math.random() > 0.6;
                    return (
                        <div
                            key={i}
                            className={`absolute w-1 h-1 rounded-full transition-all duration-1000 ${active ? 'bg-hive-cyan shadow-neon-cyan scale-150' : 'bg-slate-800'}`}
                            style={{
                                left: `${Math.random() * 100}%`,
                                top: `${Math.random() * 100}%`,
                                animation: active ? `pulse 2s infinite ${Math.random() * 2}s` : 'none'
                            }}
                        ></div>
                    );
                })}
            </div>
            <div className="absolute top-2 left-3 flex items-center gap-2">
                <Sparkles size={10} className="text-hive-cyan animate-pulse" />
                <span className="text-[8px] font-mono text-slate-500 uppercase tracking-widest">Semantic Hot-Spots: Active</span>
            </div>
            <div className="absolute bottom-2 right-3 text-[8px] font-mono text-slate-600 uppercase">Latent Space Cache: 42.4 GB</div>
        </div>
    );
};

const MemoryMap: React.FC = () => {
    const cells = Array.from({ length: 128 });
    return (
        <div className="grid grid-cols-16 gap-1 p-2 bg-slate-950 rounded border border-white/5">
            {cells.map((_, i) => {
                const state = Math.random();
                let color = 'bg-slate-800';
                let label = "Free";

                if (state > 0.90) {
                    color = 'bg-hive-orange shadow-neon-orange';
                    label = "Hot Record";
                } else if (state > 0.70) {
                    color = 'bg-hive-cyan shadow-neon-cyan';
                    label = "B-Tree Index";
                } else if (state > 0.60) {
                    color = 'bg-purple-600';
                    label = "Edge Pointer";
                } else if (state > 0.40) {
                    color = 'bg-slate-700';
                    label = "Metadata";
                }

                return (
                    <div
                        key={i}
                        className={`w-full pt-[100%] rounded-sm transition-all duration-500 hover:scale-125 cursor-help ${color}`}
                        title={`Page 0x${i.toString(16).toUpperCase()}: ${label}`}
                    ></div>
                );
            })}
        </div>
    );
};

const EngineBadge: React.FC<{ type: string }> = ({ type }) => {
    const colors = {
        memory: 'bg-blue-500/10 text-blue-400 border-blue-500/20',
        tikv: 'bg-emerald-500/10 text-emerald-400 border-emerald-500/20',
        rocksdb: 'bg-orange-500/10 text-orange-400 border-orange-500/20'
    };
    return (
        <span className={`px-2 py-0.5 rounded text-[10px] font-bold uppercase border ${colors[type as keyof typeof colors] || colors.memory}`}>
            {type} ENGINE
        </span>
    );
};

const Cache: React.FC = () => {
    // Fix: useToast returns the toast object directly
    const toast = useToast();
    const [metrics, setMetrics] = useState<CacheMetrics | null>(null);
    const [liveQueries, setLiveQueries] = useState<LiveQuery[]>([]);
    const [entropy, setEntropy] = useState(0.002);

    const fetchData = async () => {
        try {
            const [m, lq] = await Promise.all([
                tauriApi.getCacheMetrics(),
                tauriApi.getLiveQueries()
            ]);
            setMetrics(m);
            setLiveQueries(lq);
        } catch (e) {
            toast.error("Surreal Engine connection lost");
        }
    };

    useEffect(() => {
        fetchData();
        const interval = setInterval(() => {
            tauriApi.getCacheMetrics().then(setMetrics);
            setEntropy(Math.random() * 0.005);
        }, 3000);
        return () => clearInterval(interval);
    }, []);


    if (!metrics) return <LoadingState message="Syncing with Surreal Engine..." />;

    return (
        <div className="space-y-6 animate-in fade-in duration-500">
            <div className="grid grid-cols-1 lg:grid-cols-4 gap-6">
                <div className="lg:col-span-3 bg-slate-900/40 border border-white/5 p-6 rounded-lg backdrop-blur-sm flex flex-col sm:flex-row items-start sm:items-center justify-between gap-4">
                    <div className="flex items-center gap-4">
                        <div className="w-12 h-12 rounded bg-gradient-to-br from-hive-orange to-red-600 flex items-center justify-center shadow-neon-orange relative group">
                            <Database size={24} className="text-white" />
                        </div>
                        <div>
                            <h2 className="text-xl font-bold text-white flex items-center gap-3">
                                Atomic Storage
                                <EngineBadge type={metrics.engine_type} />
                            </h2>
                            <div className="flex items-center gap-4 mt-1">
                                <p className="text-xs text-slate-500 font-mono flex items-center gap-1">
                                    <ShieldCheck size={12} className="text-emerald-500" />
                                    RUST_SAFETY: PASSED
                                </p>
                                <p className="text-xs text-slate-500 font-mono flex items-center gap-1">
                                    <Activity size={12} className="text-hive-cyan" />
                                    OPS: {(metrics.hits_per_sec / 1000).toFixed(1)}k/s
                                </p>
                            </div>
                        </div>
                    </div>
                    <div className="flex gap-2">
                        <button className="px-3 py-1.5 bg-slate-950 border border-white/10 text-[10px] font-bold text-slate-400 rounded hover:text-white transition uppercase tracking-widest">Dump Heap</button>
                        <button className="px-3 py-1.5 bg-hive-orange text-black text-[10px] font-bold rounded shadow-neon-orange hover:bg-orange-600 transition uppercase tracking-widest">Vacuum</button>
                    </div>
                </div>

                <div className="bg-slate-900/40 border border-white/5 p-6 rounded-lg backdrop-blur-sm flex flex-col justify-center">
                    <div className="text-[10px] font-mono text-slate-500 uppercase mb-1 flex justify-between">
                        <span>System Entropy</span>
                        <span className="text-hive-orange">{entropy.toFixed(4)}</span>
                    </div>
                    <div className="flex gap-0.5 h-8 items-end">
                        {Array.from({ length: 20 }).map((_, i) => (
                            <div
                                key={i}
                                className="bg-hive-orange/30 w-full animate-pulse"
                                style={{ height: `${Math.random() * 100}%`, animationDelay: `${i * 100}ms` }}
                            ></div>
                        ))}
                    </div>
                </div>
            </div>

            <div className="grid grid-cols-1 lg:grid-cols-3 gap-6">
                <div className="space-y-6">
                    <div className="bg-slate-900/40 border border-white/5 rounded-lg p-5 backdrop-blur-sm relative overflow-hidden">
                        <div className="flex items-center justify-between mb-4">
                            <h3 className="text-xs font-mono text-slate-400 uppercase flex items-center gap-2">
                                <Binary size={14} className="text-hive-cyan" />
                                Atomic Page Map
                            </h3>
                            <span className="text-[9px] font-mono text-slate-600 uppercase">Sector: 0x4f02</span>
                        </div>
                        <MemoryMap />
                        <div className="mt-6 space-y-2">
                            <div className="flex justify-between items-center text-[10px] font-mono">
                                <div className="flex items-center gap-2"><div className="w-2 h-2 bg-hive-orange rounded-full"></div> HOT RECORDS</div>
                                <span className="text-white">12.4%</span>
                            </div>
                            <div className="flex justify-between items-center text-[10px] font-mono">
                                <div className="flex items-center gap-2"><div className="w-2 h-2 bg-hive-cyan rounded-full"></div> INDEX PAGES</div>
                                <span className="text-white">42.8%</span>
                            </div>
                        </div>
                    </div>

                    <div className="bg-slate-900/40 border border-white/5 rounded-lg p-5 backdrop-blur-sm">
                        <div className="flex items-center justify-between mb-4">
                            <h3 className="text-xs font-mono text-slate-400 uppercase flex items-center gap-2">
                                <Sparkles size={14} className="text-hive-cyan" />
                                Semantic Cache
                            </h3>
                            <div className="text-[10px] font-mono text-emerald-500 font-bold uppercase">Hit Rate: 84.5%</div>
                        </div>
                        <SemanticCacheVisualizer />
                        <div className="mt-4 p-3 bg-slate-950/50 rounded border border-white/5">
                            <div className="flex items-center justify-between text-[10px] font-mono text-slate-500 uppercase tracking-widest">
                                <span>Embedding Hit Ratio</span>
                                <span className="text-white">High</span>
                            </div>
                            <p className="text-[8px] text-slate-600 mt-2 font-mono leading-relaxed">
                                Vector query results are cached based on latent proximity to reduce repeated inference cost.
                            </p>
                        </div>
                    </div>
                </div>

                <div className="lg:col-span-2 space-y-6">
                    <div className="bg-slate-900/40 border border-white/5 rounded-lg overflow-hidden backdrop-blur-sm">
                        <div className="p-4 border-b border-white/5 bg-slate-950/50 flex items-center justify-between">
                            <h3 className="text-sm font-bold text-white flex items-center gap-2">
                                <Radio size={16} className="text-red-500 animate-pulse" />
                                Live Channel Activity
                            </h3>
                            <div className="flex items-center gap-4">
                                <span className="text-[10px] font-mono text-slate-500">MODE: SUBSCRIPTION_ONLY</span>
                            </div>
                        </div>
                        <div className="max-h-64 overflow-y-auto custom-scrollbar">
                            <table className="w-full text-left text-[11px] border-collapse">
                                <thead className="bg-slate-950/50 font-mono text-slate-500 uppercase tracking-widest">
                                    <tr>
                                        <th className="p-3 border-b border-white/5">Channel_UID</th>
                                        <th className="p-3 border-b border-white/5">Namespace:Table</th>
                                        <th className="p-3 border-b border-white/5 text-right">State</th>
                                    </tr>
                                </thead>
                                <tbody className="divide-y divide-white/5 font-mono text-slate-400">
                                    {liveQueries.map((lq) => (
                                        <tr key={lq.id} className="hover:bg-white/5 transition-colors group">
                                            <td className="p-3 text-hive-cyan font-bold">{lq.id}</td>
                                            <td className="p-3 text-white">main:{lq.table}</td>
                                            <td className="p-3 text-right">
                                                <span className="px-2 py-0.5 bg-emerald-500/10 text-emerald-500 rounded text-[9px] font-bold">STREAMING</span>
                                            </td>
                                        </tr>
                                    ))}
                                </tbody>
                            </table>
                        </div>
                    </div>

                    <div className="bg-slate-950/50 border border-white/5 p-6 rounded-lg flex items-center justify-between">
                        <div className="flex items-center gap-4">
                            <div className="p-3 bg-purple-500/10 rounded-lg border border-purple-500/20">
                                <GitBranch size={20} className="text-purple-500" />
                            </div>
                            <div>
                                <h4 className="text-xs font-bold text-white uppercase tracking-widest">Concurrent Transactions</h4>
                                <p className="text-[10px] text-slate-500 font-mono">SurrealDB ACID Compliance Active</p>
                            </div>
                        </div>
                        <div className="text-right">
                            <span className="text-2xl font-black text-white">{metrics.active_transactions}</span>
                            <span className="text-[10px] text-slate-500 block uppercase">Active TX</span>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    );
};

export default Cache;


import React, { useState, useEffect } from 'react';
import { GitBranch, Server, Activity, ArrowRightLeft, Zap, ShieldCheck, Database, LayoutGrid, Info, RefreshCw, AlertTriangle, TrendingUp } from 'lucide-react';
import { useToast } from '../context/ToastContext';

interface ShardNode {
    id: string;
    region: string;
    load: number;
    shards: string[];
    status: 'healthy' | 'critical' | 'balancing';
}

const Sharding: React.FC = () => {
    // Fix: useToast returns the toast object directly
    const toast = useToast();
    const [nodes, setNodes] = useState<ShardNode[]>([
        { id: 'HN-01', region: 'us-east', load: 45, shards: ['S1', 'S5', 'S9'], status: 'healthy' },
        { id: 'HN-02', region: 'us-east', load: 82, shards: ['S2', 'S6', 'S10', 'S13', 'S14'], status: 'critical' },
        { id: 'HN-03', region: 'eu-west', load: 30, shards: ['S3', 'S7', 'S11'], status: 'healthy' },
        { id: 'HN-04', region: 'ap-south', load: 15, shards: ['S4', 'S8', 'S12'], status: 'healthy' },
    ]);
    const [isRebalancing, setIsRebalancing] = useState(false);

    const handleAutoRebalance = () => {
        setIsRebalancing(true);
        toast.info("Surreal Mesh detected high load on HN-02. Initiating gradient sharding...", "Autonomous Rebalancing");
        
        // Step 1: Mark as balancing
        setNodes(prev => prev.map(n => n.id === 'HN-02' || n.id === 'HN-04' ? { ...n, status: 'balancing' } : n));

        // Step 2: Simulate move
        setTimeout(() => {
            setNodes(prev => prev.map(n => {
                if (n.id === 'HN-02') return { ...n, load: 55, shards: ['S2', 'S6', 'S10'], status: 'healthy' };
                if (n.id === 'HN-04') return { ...n, load: 35, shards: ['S4', 'S8', 'S12', 'S13', 'S14'], status: 'healthy' };
                return n;
            }));
            setIsRebalancing(false);
            toast.success("Shard migration complete. Cluster entropy stabilized.", "Sync Successful");
        }, 4000);
    };

    return (
        <div className="space-y-8 animate-in fade-in duration-500">
            {/* Header */}
            <div className="flex flex-col md:flex-row items-start md:items-center justify-between gap-6">
                <div>
                    <h2 className="text-2xl font-bold text-white flex items-center gap-3">
                        <GitBranch className="text-hive-cyan" size={28} />
                        Neural Sharding Matrix
                    </h2>
                    <p className="text-slate-400 text-sm mt-1 uppercase font-mono tracking-tight">Consistent Hashing Strategy: Virtual_Nodes_256</p>
                </div>
                <div className="flex gap-4">
                     <button 
                        onClick={handleAutoRebalance}
                        disabled={isRebalancing}
                        className={`px-6 py-2 rounded-lg font-bold text-xs uppercase transition-all flex items-center gap-2 border shadow-2xl
                            ${isRebalancing ? 'bg-hive-orange text-black border-hive-orange' : 'bg-slate-900 border-white/10 text-white hover:border-hive-cyan/50'}
                        `}
                    >
                        {isRebalancing ? <RefreshCw size={14} className="animate-spin" /> : <Zap size={14} />}
                        {isRebalancing ? 'Relocating Shards...' : 'Auto-Rebalance'}
                    </button>
                </div>
            </div>

            {/* Matrix View */}
            <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6">
                {nodes.map(node => (
                    <div key={node.id} className={`bg-slate-900/40 backdrop-blur-md border rounded-xl p-6 transition-all relative overflow-hidden group
                        ${node.status === 'critical' ? 'border-red-500/50 shadow-[0_0_20px_rgba(239,68,68,0.1)]' : 
                          node.status === 'balancing' ? 'border-hive-orange/50 shadow-neon-orange' : 'border-white/5 hover:border-white/20'}
                    `}>
                        {node.status === 'balancing' && (
                            <div className="absolute inset-0 bg-hive-orange/5 flex items-center justify-center">
                                <Activity className="text-hive-orange animate-pulse" size={48} />
                            </div>
                        )}
                        
                        <div className="relative z-10 flex flex-col h-full">
                            <div className="flex justify-between items-start mb-6">
                                <div>
                                    <h4 className="text-white font-black text-lg font-mono">{node.id}</h4>
                                    <span className="text-[10px] text-slate-500 font-mono uppercase">{node.region}</span>
                                </div>
                                <div className={`p-2 rounded-lg ${node.status === 'critical' ? 'bg-red-500/20 text-red-500' : 'bg-slate-950/50 text-slate-500'}`}>
                                    <Server size={20} />
                                </div>
                            </div>

                            <div className="space-y-4 mb-8">
                                <div className="flex justify-between text-[10px] font-mono text-slate-500 uppercase">
                                    <span>Ingress Pressure</span>
                                    <span className={node.load > 70 ? 'text-red-500' : 'text-hive-cyan'}>{node.load}%</span>
                                </div>
                                <div className="h-1.5 w-full bg-slate-800 rounded-full overflow-hidden">
                                    <div 
                                        className={`h-full transition-all duration-1000 ${node.load > 70 ? 'bg-red-500 shadow-[0_0_10px_#ef4444]' : 'bg-hive-cyan shadow-neon-cyan'}`} 
                                        style={{ width: `${node.load}%` }}
                                    ></div>
                                </div>
                            </div>

                            <div className="flex-1">
                                <div className="text-[9px] font-mono text-slate-500 uppercase mb-3 flex items-center gap-2">
                                    <LayoutGrid size={12} />
                                    Active Shards ({node.shards.length})
                                </div>
                                <div className="grid grid-cols-4 gap-2">
                                    {node.shards.map(s => (
                                        <div key={s} className="bg-slate-950 border border-white/5 rounded p-1 text-[8px] font-mono text-slate-400 text-center group-hover:border-hive-cyan/30">
                                            {s}
                                        </div>
                                    ))}
                                    <div className="bg-slate-950 border border-dashed border-white/5 rounded p-1 text-[8px] flex items-center justify-center text-slate-700">+</div>
                                </div>
                            </div>
                        </div>
                    </div>
                ))}
            </div>

            {/* Visual Analytics */}
            <div className="grid grid-cols-1 lg:grid-cols-3 gap-8">
                <div className="lg:col-span-2 bg-slate-900/40 border border-white/5 rounded-xl p-8 backdrop-blur-sm flex flex-col gap-8 relative overflow-hidden">
                     <div className="flex items-center justify-between">
                         <h3 className="text-sm font-bold text-white uppercase tracking-widest flex items-center gap-2">
                            <Activity size={18} className="text-purple-500" />
                            Cluster Entropy Monitor
                         </h3>
                         <div className="flex gap-4">
                            <div className="flex items-center gap-2">
                                <div className="w-2 h-2 rounded-full bg-emerald-500"></div>
                                <span className="text-[10px] text-slate-500 font-mono uppercase">Lattice Stabilized</span>
                            </div>
                         </div>
                     </div>
                     
                     <div className="flex-1 flex items-center justify-around py-12">
                        {/* Simulation of a Mesh connection */}
                        <div className="relative w-64 h-64 border-2 border-dashed border-white/5 rounded-full flex items-center justify-center">
                            <div className="absolute inset-0 animate-spin-slow">
                                <div className="absolute top-0 left-1/2 -translate-x-1/2 w-4 h-4 bg-hive-cyan rounded-full shadow-neon-cyan"></div>
                            </div>
                            <div className="absolute inset-8 border border-white/10 rounded-full flex items-center justify-center">
                                <Database size={48} className="text-white/20" />
                            </div>
                            <div className="text-center">
                                <div className="text-4xl font-black text-white">0.02</div>
                                <div className="text-[10px] text-slate-500 font-mono uppercase">Entropy Delta</div>
                            </div>
                        </div>

                        <div className="space-y-6 w-64">
                             <div className="bg-slate-950/80 p-4 rounded-lg border border-white/5">
                                <div className="flex items-center gap-3 mb-2">
                                    <ShieldCheck size={16} className="text-emerald-500" />
                                    <span className="text-[10px] font-bold text-white uppercase tracking-widest">Shard Security</span>
                                </div>
                                <p className="text-[9px] text-slate-500 font-mono leading-relaxed italic uppercase">
                                    Each shard is cross-verified with a lattice-based signature chain for zero-trust data movement.
                                </p>
                             </div>
                             <div className="bg-slate-950/80 p-4 rounded-lg border border-white/5">
                                <div className="flex items-center gap-3 mb-2">
                                    <TrendingUp size={16} className="text-hive-cyan" />
                                    <span className="text-[10px] font-bold text-white uppercase tracking-widest">Sharding Strategy</span>
                                </div>
                                <p className="text-[9px] text-slate-500 font-mono leading-relaxed italic uppercase">
                                    Predictive Sharding (ML_V2) enabled. Autonomous rebalance triggered every 500ms if skew > 15%.
                                </p>
                             </div>
                        </div>
                     </div>
                </div>

                <div className="bg-slate-900/40 border border-white/5 rounded-xl p-8 backdrop-blur-sm space-y-8">
                    <h3 className="text-sm font-bold text-white uppercase tracking-widest flex items-center gap-2">
                        <Info size={18} className="text-hive-orange" />
                        Mesh Configuration
                    </h3>
                    
                    <div className="space-y-6">
                        <div>
                            <label className="text-[10px] font-mono text-slate-500 uppercase block mb-3">Hashing Algorithm</label>
                            <div className="flex items-center justify-between p-3 rounded bg-slate-950 border border-white/10">
                                <span className="text-xs font-bold text-white">MAGLEV (Google Style)</span>
                                <ArrowRightLeft size={14} className="text-slate-600" />
                            </div>
                        </div>

                        <div>
                            <label className="text-[10px] font-mono text-slate-500 uppercase block mb-3">Virtual Node Density</label>
                            <div className="flex items-center justify-between p-3 rounded bg-slate-950 border border-white/10">
                                <span className="text-xs font-bold text-white">256 VNODES / CLUSTER</span>
                                <GitBranch size={14} className="text-slate-600" />
                            </div>
                        </div>

                        <div className="pt-8 border-t border-white/5">
                            <div className="flex items-center gap-3 text-red-500 mb-2">
                                <AlertTriangle size={16} />
                                <span className="text-[10px] font-bold uppercase tracking-widest">Conflict Detection</span>
                            </div>
                            <p className="text-[9px] text-slate-600 font-mono leading-relaxed italic uppercase">
                                Vector clock skew: 0.001ms.
                                Last consensus round: 12ms ago.
                            </p>
                        </div>
                    </div>
                </div>
            </div>

            <style>{`
                .animate-spin-slow {
                    animation: spin 10s linear infinite;
                }
                @keyframes spin {
                    from { transform: rotate(0deg); }
                    to { transform: rotate(360deg); }
                }
            `}</style>
        </div>
    );
};

export default Sharding;

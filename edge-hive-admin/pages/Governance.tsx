
import React, { useState, useEffect } from 'react';
import { 
  Scale, ShieldCheck, Activity, BrainCircuit, Zap, AlertTriangle, 
  ChevronRight, Network, Server, Cpu, Clock, RefreshCw, BarChart, 
  ShieldAlert, Globe, ArrowRight
} from 'lucide-react';
import { useToast } from '../context/ToastContext';

const DecisionTree = () => {
    const decisions = [
      { id: 1, type: 'trigger', label: 'Traffic Anomaly detected in EU-WEST', time: '2m ago' },
      { id: 2, type: 'analysis', label: 'Inference: 140% spike projected in 15min', time: '1m ago' },
      { id: 3, type: 'action', label: 'Auto-Scaling: Spawning HN-08 in GCP Frankfurt', time: '30s ago', status: 'completed' },
      { id: 4, type: 'check', label: 'Mesh Integrity re-verified post-sync', time: 'Just now', status: 'success' }
    ];

    return (
        <div className="space-y-4">
            {decisions.map((d, i) => (
                <div key={d.id} className="relative pl-8 group">
                    {i < decisions.length - 1 && (
                        <div className="absolute left-3.5 top-7 bottom-0 w-[1px] bg-white/10 group-hover:bg-hive-orange/30 transition-colors"></div>
                    )}
                    <div className="absolute left-0 top-1.5 w-7 h-7 rounded-full bg-slate-950 border border-white/10 flex items-center justify-center z-10">
                        {d.type === 'trigger' && <Activity size={12} className="text-red-500" />}
                        {d.type === 'analysis' && <BrainCircuit size={12} className="text-purple-500" />}
                        {d.type === 'action' && <Zap size={12} className="text-hive-orange" />}
                        {d.type === 'check' && <ShieldCheck size={12} className="text-emerald-500" />}
                    </div>
                    <div className="bg-slate-900/50 border border-white/5 p-4 rounded-xl group-hover:border-white/10 transition-all">
                        <div className="flex justify-between items-start mb-1">
                            <h4 className="text-xs font-bold text-white uppercase tracking-tighter">{d.label}</h4>
                            <span className="text-[9px] font-mono text-slate-600 uppercase">{d.time}</span>
                        </div>
                        {d.status && (
                            <div className="flex items-center gap-2 mt-2">
                                <div className={`w-1 h-1 rounded-full ${d.status === 'success' ? 'bg-emerald-500' : 'bg-hive-orange'}`}></div>
                                <span className={`text-[8px] font-bold uppercase tracking-widest ${d.status === 'success' ? 'text-emerald-500' : 'text-hive-orange'}`}>
                                    {d.status}
                                </span>
                            </div>
                        )}
                    </div>
                </div>
            ))}
        </div>
    );
};

const Governance: React.FC = () => {
    // Fix: useToast returns the toast object directly
    const toast = useToast();
    const [isAuditActive, setIsAuditActive] = useState(false);
    const [complianceScore, setComplianceScore] = useState(94.2);

    const handleRunAudit = () => {
        setIsAuditActive(true);
        toast.info("Initializing Quantum Vulnerability Scan...", "Mesh Audit");
        setTimeout(() => {
            setIsAuditActive(false);
            setComplianceScore(98.8);
            toast.success("Mesh Audit complete. Safety Score optimized to 98.8%", "Audit Success");
        }, 3500);
    };

    return (
        <div className="space-y-8 animate-in fade-in duration-500 pb-12">
            <div className="flex flex-col md:flex-row items-start md:items-end justify-between gap-4 border-b border-white/5 pb-6">
                <div>
                    <div className="flex items-center gap-2 mb-2">
                        <div className="w-2 h-2 rounded-full bg-emerald-500 shadow-neon-cyan animate-pulse"></div>
                        <span className="text-[10px] font-mono text-emerald-500 font-bold uppercase tracking-widest">Autonomous Core: Online</span>
                    </div>
                    <h1 className="text-4xl font-black text-white tracking-tighter uppercase">Governance</h1>
                </div>
                <div className="flex gap-4">
                    <div className="text-right">
                        <div className="text-[9px] font-mono text-slate-500 uppercase">Policy Version</div>
                        <div className="text-sm font-bold text-hive-cyan flex items-center gap-2 justify-end uppercase">
                            SURREAL_AUTO_v0.4
                        </div>
                    </div>
                </div>
            </div>

            <div className="grid grid-cols-1 lg:grid-cols-3 gap-8">
                <div className="lg:col-span-2 space-y-8">
                    <div className="bg-slate-900/40 border border-white/5 rounded-xl p-8 backdrop-blur-md relative overflow-hidden group">
                        <div className="absolute top-0 right-0 p-8 text-hive-orange opacity-5 pointer-events-none group-hover:rotate-12 transition-transform">
                            <BrainCircuit size={200} />
                        </div>
                        <div className="flex items-center justify-between mb-8">
                            <h3 className="text-xs font-bold text-white uppercase tracking-widest flex items-center gap-2">
                                <Network size={16} className="text-hive-orange" />
                                Neural Decision Logic
                            </h3>
                            <div className="flex items-center gap-2 text-[10px] font-mono text-slate-500 uppercase">
                                <Clock size={12} /> Last decision: 45s ago
                            </div>
                        </div>
                        <DecisionTree />
                    </div>

                    <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
                        <div className="bg-slate-900/40 border border-white/5 p-6 rounded-xl backdrop-blur-sm">
                            <h4 className="text-[10px] font-bold text-white uppercase tracking-widest mb-6 flex items-center gap-2">
                                <Cpu size={14} className="text-purple-500" />
                                Resource Elasticity
                            </h4>
                            <div className="flex items-center justify-between mb-2">
                                <span className="text-2xl font-black text-white">42 / 64</span>
                                <span className="text-[10px] font-mono text-slate-500 uppercase">Nodes Active</span>
                            </div>
                            <div className="h-1.5 w-full bg-slate-850 rounded-full overflow-hidden">
                                <div className="h-full bg-purple-500 shadow-[0_0_10px_#a855f7] w-[65%]"></div>
                            </div>
                            <div className="mt-4 flex justify-between text-[8px] font-mono text-slate-600 uppercase">
                                <span>Reserve: 22 Nodes</span>
                                <span className="text-emerald-500">Auto-Scaling: Enabled</span>
                            </div>
                        </div>

                        <div className="bg-slate-900/40 border border-white/5 p-6 rounded-xl backdrop-blur-sm">
                            <h4 className="text-[10px] font-bold text-white uppercase tracking-widest mb-6 flex items-center gap-2">
                                <Scale size={14} className="text-hive-cyan" />
                                Compliance Mesh Score
                            </h4>
                            <div className="flex items-center justify-between mb-2">
                                <span className="text-3xl font-black text-emerald-500">{complianceScore}%</span>
                                <div className="text-right">
                                    <div className="text-[9px] font-mono text-slate-500 uppercase">Lattice Density</div>
                                    <div className="text-xs font-bold text-white uppercase">99.2% Robust</div>
                                </div>
                            </div>
                            <div className="h-1.5 w-full bg-slate-850 rounded-full overflow-hidden">
                                <div className="h-full bg-emerald-500 shadow-neon-cyan" style={{ width: `${complianceScore}%` }}></div>
                            </div>
                        </div>
                    </div>
                </div>

                <div className="space-y-6">
                    <div className="bg-slate-900/40 border border-white/5 p-8 rounded-xl backdrop-blur-md relative overflow-hidden group">
                        <h4 className="text-xs font-bold text-white uppercase tracking-widest mb-6 flex items-center gap-2 border-b border-white/5 pb-4">
                            <ShieldAlert size={16} className="text-red-500" />
                            Post-Quantum Audit
                        </h4>
                        <div className="space-y-6">
                            <div className="p-4 bg-slate-950/50 border border-white/5 rounded-lg">
                                <div className="text-[10px] font-bold text-slate-500 uppercase mb-2">Last Threat detected</div>
                                <div className="text-xs font-mono text-red-400">Brute-force attempt on Node:CF-EU-01</div>
                                <div className="text-[9px] text-slate-600 mt-1 uppercase">Action: IP Quarantined in 0.002ms</div>
                            </div>
                            
                            <button 
                                onClick={handleRunAudit}
                                disabled={isAuditActive}
                                className="w-full py-3 bg-slate-950 border border-white/10 text-white font-bold text-[10px] rounded uppercase tracking-widest hover:border-hive-cyan transition-all flex items-center justify-center gap-3"
                            >
                                {isAuditActive ? <RefreshCw size={14} className="animate-spin text-hive-orange" /> : <ShieldCheck size={14} className="text-emerald-500" />}
                                {isAuditActive ? 'Scanning mesh...' : 'Full Security Scan'}
                            </button>
                        </div>
                    </div>

                    <div className="bg-slate-900 border border-white/5 p-6 rounded-xl relative group shadow-2xl">
                        <div className="absolute top-0 right-0 p-4 opacity-10 text-hive-orange pointer-events-none">
                            <Globe size={80} />
                        </div>
                        <h4 className="text-xs font-bold text-white uppercase tracking-widest mb-4 flex items-center gap-2">
                           <BarChart size={16} className="text-hive-cyan" />
                           Economic Guard
                        </h4>
                        <p className="text-[10px] text-slate-400 font-mono leading-relaxed mb-6 uppercase italic">
                           "IA detected 12% unused reservation in Azure. Suggested release to save ~$4.20/day."
                        </p>
                        <div className="flex gap-2">
                           <button className="flex-1 py-2 bg-hive-orange text-black font-bold text-[9px] rounded shadow-neon-orange uppercase">Execute</button>
                           <button className="flex-1 py-2 bg-slate-950 border border-white/10 text-slate-500 font-bold text-[9px] rounded uppercase">Ignore</button>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    );
};

export default Governance;

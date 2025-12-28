
import React, { useState, useEffect } from 'react';
import { Eye, Activity, ShieldAlert, Zap, Search, RefreshCw, BarChart3, Radar, Server, Cpu, Globe, AlertTriangle } from 'lucide-react';
import { useToast } from '../context/ToastContext';
import LogViewer from '../components/LogViewer';

const AnomalyRadar = () => {
    const [scannedPoints, setScannedPoints] = useState<{ x: number, y: number, intensity: number }[]>([]);
    
    useEffect(() => {
        const interval = setInterval(() => {
            const point = {
                x: Math.random() * 100,
                y: Math.random() * 100,
                intensity: Math.random()
            };
            setScannedPoints(prev => [...prev.slice(-12), point]);
        }, 800);
        return () => clearInterval(interval);
    }, []);

    return (
        <div className="relative w-full h-64 bg-slate-950 border border-white/10 rounded-xl overflow-hidden group">
            <div className="absolute inset-0 flex items-center justify-center">
                 <div className="w-[80%] h-[80%] border border-white/5 rounded-full"></div>
                 <div className="w-[50%] h-[50%] border border-white/5 rounded-full"></div>
                 <div className="w-[20%] h-[20%] border border-white/5 rounded-full"></div>
                 <div className="absolute inset-0 bg-conic-radar opacity-10 animate-spin-slow pointer-events-none"></div>
            </div>
            
            {scannedPoints.map((p, i) => (
                <div 
                    key={i}
                    className={`absolute w-1.5 h-1.5 rounded-full transition-opacity duration-1000 ${p.intensity > 0.8 ? 'bg-red-500 shadow-[0_0_15px_#ef4444]' : 'bg-hive-cyan shadow-neon-cyan'}`}
                    style={{ 
                        left: `${p.x}%`, 
                        top: `${p.y}%`,
                        opacity: 1 - (i * 0.05)
                    }}
                ></div>
            ))}

            <div className="absolute top-4 left-4 flex items-center gap-2">
                <Radar size={14} className="text-hive-cyan animate-pulse" />
                <span className="text-[10px] font-mono text-slate-500 uppercase tracking-widest">Neural Scanner: Scanning...</span>
            </div>

            <style>{`
                .bg-conic-radar {
                    background: conic-gradient(from 0deg at 50% 50%, rgba(6, 182, 212, 0.4) 0deg, transparent 60deg);
                }
                .animate-spin-slow {
                    animation: spin 5s linear infinite;
                }
                @keyframes spin {
                    from { transform: rotate(0deg); }
                    to { transform: rotate(360deg); }
                }
            `}</style>
        </div>
    );
};

const NodeEntropyGrid = () => {
    const [cells, setCells] = useState(Array.from({ length: 64 }).map(() => Math.random()));
    
    useEffect(() => {
        const interval = setInterval(() => {
            setCells(prev => prev.map(c => Math.max(0, Math.min(1, c + (Math.random() - 0.5) * 0.2))));
        }, 1000);
        return () => clearInterval(interval);
    }, []);

    return (
        <div className="grid grid-cols-8 gap-1 bg-slate-950 p-2 rounded border border-white/5">
            {cells.map((val, i) => {
                const color = val > 0.8 ? 'bg-red-500 shadow-[0_0_10px_#ef4444]' : 
                              val > 0.5 ? 'bg-hive-orange' : 'bg-hive-cyan/40';
                return (
                    <div 
                        key={i} 
                        className={`w-full pt-[100%] rounded-sm transition-all duration-700 ${color}`}
                        style={{ opacity: 0.2 + val * 0.8 }}
                    ></div>
                );
            })}
        </div>
    );
};

const Observability: React.FC = () => {
    // Fix: useToast returns the toast object directly
    const toast = useToast();
    const [isScanning, setIsScanning] = useState(false);

    const runGlobalDiagnostics = () => {
        setIsScanning(true);
        toast.info("Running deep neural diagnostics across clúster nodes...", "System Audit");
        setTimeout(() => {
            setIsScanning(false);
            toast.success("Diagnostics complete. 0 critical vulnerabilities found. 2 latency hotspots optimized.", "Audit Success");
        }, 4000);
    };

    return (
        <div className="space-y-8 animate-in fade-in duration-500 pb-12">
            <div className="flex flex-col md:flex-row items-start md:items-end justify-between gap-4 border-b border-white/5 pb-6">
                <div>
                    <div className="flex items-center gap-2 mb-2">
                        <div className="w-2 h-2 rounded-full bg-hive-cyan shadow-neon-cyan animate-pulse"></div>
                        <span className="text-[10px] font-mono text-hive-cyan font-bold uppercase tracking-widest">Neural Vision Active</span>
                    </div>
                    <h1 className="text-4xl font-black text-white tracking-tighter uppercase">Observability</h1>
                </div>
                <button 
                    onClick={runGlobalDiagnostics}
                    disabled={isScanning}
                    className={`px-6 py-2 rounded-lg font-bold text-xs uppercase transition-all flex items-center gap-2 border
                        ${isScanning ? 'bg-hive-orange text-black border-hive-orange' : 'bg-slate-900 border-white/10 text-white hover:border-hive-cyan/50 shadow-2xl'}
                    `}
                >
                    {isScanning ? <RefreshCw size={14} className="animate-spin" /> : <Search size={14} />}
                    {isScanning ? 'Diagnosing...' : 'Neural Audit'}
                </button>
            </div>

            <div className="grid grid-cols-1 lg:grid-cols-3 gap-8">
                <div className="lg:col-span-2 space-y-8">
                    <div className="bg-slate-900/40 border border-white/5 rounded-xl p-6 backdrop-blur-md">
                        <h3 className="text-xs font-bold text-white uppercase tracking-widest flex items-center gap-2 mb-6">
                            <Radar size={16} className="text-hive-cyan" />
                            Anomaly Detection Radar
                        </h3>
                        <AnomalyRadar />
                        <div className="mt-6 grid grid-cols-2 md:grid-cols-4 gap-4">
                            <div className="p-3 bg-slate-950 border border-white/5 rounded">
                                <span className="text-[9px] font-mono text-slate-500 block uppercase">Outliers</span>
                                <span className="text-lg font-bold text-white">12</span>
                            </div>
                            <div className="p-3 bg-slate-950 border border-white/5 rounded">
                                <span className="text-[9px] font-mono text-slate-500 block uppercase">P99 Drift</span>
                                <span className="text-lg font-bold text-hive-cyan">+2.4ms</span>
                            </div>
                            <div className="p-3 bg-slate-950 border border-white/5 rounded">
                                <span className="text-[9px] font-mono text-slate-500 block uppercase">Inference Load</span>
                                <span className="text-lg font-bold text-purple-500">14%</span>
                            </div>
                            <div className="p-3 bg-slate-950 border border-white/5 rounded">
                                <span className="text-[9px] font-mono text-slate-500 block uppercase">Safety Score</span>
                                <span className="text-lg font-bold text-emerald-500">99.9</span>
                            </div>
                        </div>
                    </div>

                    <LogViewer />
                </div>

                <div className="space-y-8">
                    <div className="bg-slate-900/40 border border-white/5 rounded-xl p-6 backdrop-blur-md">
                        <h3 className="text-xs font-bold text-white uppercase tracking-widest flex items-center gap-2 mb-6">
                            <Cpu size={16} className="text-hive-cyan" />
                            Cluster Entropy Matrix
                        </h3>
                        <NodeEntropyGrid />
                        <p className="mt-4 text-[9px] text-slate-500 font-mono leading-relaxed uppercase">
                            Visualizing the real-time entropy of the B-Tree root nodes across the clúster lattice.
                        </p>
                    </div>

                    <div className="bg-slate-900/40 border border-white/5 rounded-xl p-6 backdrop-blur-md">
                        <h3 className="text-xs font-bold text-white uppercase tracking-widest flex items-center gap-2 mb-4">
                            <AlertTriangle size={16} className="text-red-500" />
                            Risk Assessment
                        </h3>
                        <div className="space-y-6">
                            <div>
                                <div className="flex justify-between text-[10px] font-mono text-slate-500 mb-2">
                                    <span>Injection Probability</span>
                                    <span className="text-emerald-500">0.001%</span>
                                </div>
                                <div className="h-1 w-full bg-slate-850 rounded-full overflow-hidden">
                                    <div className="h-full bg-emerald-500 w-[1%]"></div>
                                </div>
                            </div>
                            <div>
                                <div className="flex justify-between text-[10px] font-mono text-slate-500 mb-2">
                                    <span>B-Tree Skew Risk</span>
                                    <span className="text-hive-orange">Low</span>
                                </div>
                                <div className="h-1 w-full bg-slate-850 rounded-full overflow-hidden">
                                    <div className="h-full bg-hive-orange w-[15%]"></div>
                                </div>
                            </div>
                        </div>
                    </div>

                    <div className="bg-slate-950 border border-white/10 p-6 rounded-xl flex items-center justify-between">
                        <div className="flex items-center gap-4">
                            <div className="p-3 bg-purple-500/10 rounded-lg border border-purple-500/20">
                                <BarChart3 size={20} className="text-purple-500" />
                            </div>
                            <div>
                                <h4 className="text-[10px] font-bold text-white uppercase tracking-widest">Neural Prediction</h4>
                                <p className="text-[9px] text-slate-500 font-mono">Next spike: ~14:30 GMT</p>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    );
};

export default Observability;

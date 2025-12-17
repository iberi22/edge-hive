
import React, { useEffect, useState } from 'react';
import {
    LineChart,
    Line,
    XAxis,
    YAxis,
    CartesianGrid,
    Tooltip,
    ResponsiveContainer,
    AreaChart,
    Area,
    Legend
} from 'recharts';
import { Activity, Server, Zap, Globe, ArrowUpRight, ShieldCheck, Cpu, HardDrive, GitCommit, Search, Map as MapIcon, Wifi, Share2, Radio, Binary, Box, BrainCircuit, TrendingUp } from 'lucide-react';
import { mockApi } from '../api';
import { SystemMetric, TopPath } from '../types';

const MetricCard = ({ title, value, unit, trend, icon: Icon, color, detail }: any) => (
    <div className="bg-slate-900/40 backdrop-blur-md border border-white/5 rounded-lg p-5 relative overflow-hidden group hover:border-white/20 transition-all">
        <div className={`absolute top-0 right-0 p-3 opacity-10 group-hover:opacity-30 transition-opacity ${color}`}>
            <Icon size={56} />
        </div>
        <div className="relative z-10">
            <h3 className="text-slate-500 text-[10px] font-mono uppercase tracking-widest mb-1">{title}</h3>
            <div className="flex items-baseline gap-2">
                <span className="text-3xl font-bold text-white font-mono tracking-tighter">{value}</span>
                <span className="text-xs text-slate-500 uppercase">{unit}</span>
            </div>
            <div className="mt-4 flex items-center justify-between">
                <div className="text-[10px] font-mono flex items-center gap-1.5">
                    <span className="text-emerald-500">▲ {trend}</span>
                    <span className="text-slate-600">OFFSET_LATEST</span>
                </div>
                <span className="text-[9px] text-slate-700 font-mono">{detail}</span>
            </div>
        </div>
    </div>
);

const NeuralLoadBalancer = () => {
    const [data, setData] = useState<{ t: string, actual: number, predicted: number }[]>([]);

    useEffect(() => {
        const generatePoint = () => {
            const now = new Date();
            const base = 40 + Math.sin(now.getTime() / 5000) * 20;
            return {
                t: now.toLocaleTimeString([], { hour12: false, hour: '2-digit', minute: '2-digit', second: '2-digit' }),
                actual: Math.max(0, base + (Math.random() - 0.5) * 15),
                predicted: Math.max(0, base + (Math.random() - 0.5) * 5)
            };
        };

        setData(Array.from({ length: 15 }).map(generatePoint));

        const interval = setInterval(() => {
            setData(prev => [...prev.slice(1), generatePoint()]);
        }, 3000);
        return () => clearInterval(interval);
    }, []);

    return (
        <div className="h-[200px] w-full">
            <ResponsiveContainer width="100%" height="100%">
                <AreaChart data={data}>
                    <defs>
                        <linearGradient id="actualGrad" x1="0" y1="0" x2="0" y2="1">
                            <stop offset="5%" stopColor="#06b6d4" stopOpacity={0.3} />
                            <stop offset="95%" stopColor="#06b6d4" stopOpacity={0} />
                        </linearGradient>
                    </defs>
                    <CartesianGrid strokeDasharray="3 3" stroke="#1e293b" vertical={false} strokeOpacity={0.1} />
                    <XAxis dataKey="t" hide />
                    <YAxis hide domain={[0, 100]} />
                    <Tooltip
                        contentStyle={{ backgroundColor: '#020617', border: '1px solid rgba(255,255,255,0.1)', fontSize: '10px', fontFamily: 'monospace' }}
                        itemStyle={{ padding: '0px' }}
                    />
                    <Area type="monotone" dataKey="actual" stroke="#06b6d4" fill="url(#actualGrad)" strokeWidth={2} dot={false} isAnimationActive={false} />
                    <Line type="monotone" dataKey="predicted" stroke="#f97316" strokeWidth={1} strokeDasharray="5 5" dot={false} isAnimationActive={false} />
                </AreaChart>
            </ResponsiveContainer>
        </div>
    );
};

const ThreadMonitor = () => {
    const [threads, setThreads] = useState(Array.from({ length: 16 }).map(() => Math.random() * 100));

    useEffect(() => {
        const interval = setInterval(() => {
            setThreads(prev => prev.map(t => Math.max(0, Math.min(100, t + (Math.random() - 0.5) * 40))));
        }, 800);
        return () => clearInterval(interval);
    }, []);

    return (
        <div className="grid grid-cols-4 sm:grid-cols-8 gap-2">
            {threads.map((load, i) => (
                <div key={i} className="bg-slate-950/50 border border-white/5 p-2 rounded flex flex-col gap-1.5">
                    <div className="flex justify-between text-[7px] font-mono text-slate-600 uppercase">
                        <span>TH_{i}</span>
                        <span className={load > 80 ? 'text-hive-orange' : 'text-hive-cyan'}>{Math.round(load)}%</span>
                    </div>
                    <div className="h-1 bg-slate-900 rounded-full overflow-hidden">
                        <div
                            className={`h-full transition-all duration-500 ${load > 80 ? 'bg-hive-orange shadow-neon-orange' : 'bg-hive-cyan shadow-neon-cyan'}`}
                            style={{ width: `${load}%` }}
                        ></div>
                    </div>
                </div>
            ))}
        </div>
    );
};

const NeuralTrafficGrid = () => {
    const nodes = [
        { id: 'NYC-01', x: 20, y: 35, color: 'text-hive-cyan' },
        { id: 'LDN-02', x: 45, y: 25, color: 'text-hive-orange' },
        { id: 'TKY-05', x: 85, y: 40, color: 'text-hive-cyan' },
        { id: 'SGP-03', x: 75, y: 65, color: 'text-purple-500' },
        { id: 'SAO-01', x: 35, y: 75, color: 'text-hive-cyan' },
    ];

    const [activePulse, setActivePulse] = useState(0);

    useEffect(() => {
        const interval = setInterval(() => {
            setActivePulse(prev => (prev + 1) % nodes.length);
        }, 3000);
        return () => clearInterval(interval);
    }, []);

    return (
        <div className="relative w-full h-64 bg-slate-950/50 rounded-lg border border-white/5 overflow-hidden">
            <div className="absolute inset-0 opacity-10 pointer-events-none" style={{ backgroundImage: 'linear-gradient(#475569 1px, transparent 1px), linear-gradient(90deg, #475569 1px, transparent 1px)', backgroundSize: '40px 40px' }}></div>

            <svg className="absolute inset-0 w-full h-full pointer-events-none">
                {nodes.map((node, i) => (
                    nodes.slice(i + 1).map((target, j) => (
                        <line
                            key={`${i}-${j}`}
                            x1={`${node.x}%`} y1={`${node.y}%`}
                            x2={`${target.x}%`} y2={`${target.y}%`}
                            stroke="currentColor"
                            strokeWidth="0.5"
                            className="text-slate-800 opacity-20"
                        />
                    ))
                ))}

                <circle
                    cx={`${nodes[activePulse].x}%`}
                    cy={`${nodes[activePulse].y}%`}
                    r="40"
                    fill="url(#pulseGradient)"
                    className="animate-ping opacity-20"
                />

                <defs>
                    <radialGradient id="pulseGradient">
                        <stop offset="0%" stopColor="#f97316" stopOpacity="0.5" />
                        <stop offset="100%" stopColor="#f97316" stopOpacity="0" />
                    </radialGradient>
                </defs>
            </svg>

            {nodes.map((node, i) => (
                <div
                    key={node.id}
                    className="absolute group"
                    style={{ left: `${node.x}%`, top: `${node.y}%` }}
                >
                    <div className="relative -translate-x-1/2 -translate-y-1/2">
                        <div className={`w-3 h-3 rounded-full border-2 border-slate-900 bg-current ${node.color} shadow-lg transition-transform group-hover:scale-150`}></div>
                        <div className="absolute top-4 left-1/2 -translate-x-1/2 bg-slate-900/90 backdrop-blur-md border border-white/10 px-2 py-1 rounded text-[8px] font-mono text-white opacity-0 group-hover:opacity-100 transition-opacity whitespace-nowrap z-20 pointer-events-none">
                            <span className="block font-bold">{node.id}</span>
                            <span className="text-slate-500 uppercase">Latency: {Math.floor(Math.random() * 20) + 1}ms</span>
                        </div>
                    </div>
                </div>
            ))}

            <div className="absolute bottom-4 left-4 flex items-center gap-6">
                <div className="flex items-center gap-2">
                    <Radio size={12} className="text-hive-orange animate-pulse" />
                    <span className="text-[9px] font-mono text-slate-500 uppercase tracking-widest">Raft Consensus: Synced</span>
                </div>
                <div className="flex items-center gap-2">
                    <Share2 size={12} className="text-hive-cyan" />
                    <span className="text-[9px] font-mono text-slate-500 uppercase tracking-widest">Node Propagation: 0.12ms</span>
                </div>
            </div>
        </div>
    );
};

const Overview: React.FC = () => {
    const [data, setData] = useState<SystemMetric[]>([]);
    const [topPaths, setTopPaths] = useState<TopPath[]>([]);

    useEffect(() => {
        // Initial fetch
        mockApi.getMetrics().then(setData);
        mockApi.getTopPaths().then(setTopPaths);

        // Real-time subscription
        let unlisten: () => void;

        const setupSubscription = async () => {
            // @ts-ignore - Dynamic import or direct usage
            const { tauriApi } = await import('./tauriClient'); // Lazy load to avoid cycle if any
            unlisten = await tauriApi.subscribeToMetrics((metric: SystemMetric) => {
                setData(prev => {
                    const newPoint = {
                        ...metric,
                        // Ensure time is formatted if backend sends raw ISO or similar
                        time: metric.time || new Date().toLocaleTimeString(),
                    };
                    // Keep window of 20 points
                    const newData = [...prev, newPoint];
                    if (newData.length > 20) newData.shift();
                    return newData;
                });
            });
        };

        setupSubscription();

        return () => {
            if (unlisten) unlisten();
        };
    }, []);

    if (data.length === 0) return <div className="p-8 text-center text-slate-500 font-mono animate-pulse">BOOTING_SENSORS...</div>;

    const lastMetric = data[data.length - 1];

    return (
        <div className="space-y-8 animate-in fade-in duration-700">

            <div className="flex flex-col md:flex-row items-start md:items-end justify-between gap-4 border-b border-white/5 pb-6">
                <div>
                    <div className="flex items-center gap-2 mb-2">
                        <div className="w-2 h-2 rounded-full bg-emerald-500 shadow-[0_0_10px_#10b981]"></div>
                        <span className="text-[10px] font-mono text-emerald-500 font-bold uppercase tracking-widest">Global Cluster Health: Optimal</span>
                    </div>
                    <h1 className="text-4xl font-black text-white tracking-tighter">COCKPIT <span className="text-hive-orange text-2xl">v0.4.2</span></h1>
                </div>
                <div className="flex gap-4">
                    <div className="text-right">
                        <div className="text-[9px] font-mono text-slate-500 uppercase">Rust Safety Audit</div>
                        <div className="text-sm font-bold text-emerald-400 flex items-center gap-2">
                            <ShieldCheck size={14} /> 100% MEM_SAFE
                        </div>
                    </div>
                    <div className="text-right border-l border-white/10 pl-4">
                        <div className="text-[9px] font-mono text-slate-500 uppercase">SIMD Vectorization</div>
                        <div className="text-sm font-bold text-hive-cyan flex items-center gap-2">
                            <Cpu size={14} /> AVX-512 ACTIVE
                        </div>
                    </div>
                </div>
            </div>

            <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6">
                <MetricCard title="Processing Load" value={lastMetric.cpu.toFixed(1)} unit="%" trend="1.2%" icon={Activity} color="text-hive-orange" detail="CORE_POOL_8" />
                <MetricCard title="Memory Residency" value={lastMetric.memory.toFixed(1)} unit="MB" trend="0.4%" icon={HardDrive} color="text-hive-cyan" detail="HEAP_NON_GC" />
                <MetricCard title="Atomic Latency" value={lastMetric.latency.toFixed(0)} unit="μs" trend="-4" icon={Zap} color="text-purple-500" detail="SUB_MILLI_P99" />
                <MetricCard title="Global Ingress" value="42.5" unit="k/s" trend="12%" icon={Globe} color="text-emerald-500" detail="NET_IO_SYNC" />
            </div>

            <div className="grid grid-cols-1 lg:grid-cols-3 gap-8">
                <div className="lg:col-span-2 space-y-6">
                    <div className="bg-slate-900/40 border border-white/5 rounded-lg p-8 backdrop-blur-sm relative overflow-hidden group">
                        <div className="absolute top-0 left-0 w-1 h-full bg-hive-orange opacity-50"></div>
                        <div className="flex items-center justify-between mb-8">
                            <div>
                                <h3 className="text-sm font-bold text-white flex items-center gap-2 uppercase tracking-widest">
                                    <Activity size={16} className="text-hive-orange" />
                                    Engine Telemetry
                                </h3>
                                <p className="text-[10px] text-slate-500 font-mono mt-1">SAMPLING_RATE: 500ms • BUFFER: CIRCULAR</p>
                            </div>
                        </div>

                        <div className="h-[280px] w-full">
                            <ResponsiveContainer width="100%" height="100%">
                                <AreaChart data={data}>
                                    <defs>
                                        <linearGradient id="colorCpu" x1="0" y1="0" x2="0" y2="1">
                                            <stop offset="5%" stopColor="#f97316" stopOpacity={0.2} />
                                            <stop offset="95%" stopColor="#f97316" stopOpacity={0} />
                                        </linearGradient>
                                        <linearGradient id="colorMem" x1="0" y1="0" x2="0" y2="1">
                                            <stop offset="5%" stopColor="#06b6d4" stopOpacity={0.2} />
                                            <stop offset="95%" stopColor="#06b6d4" stopOpacity={0} />
                                        </linearGradient>
                                    </defs>
                                    <CartesianGrid strokeDasharray="3 3" stroke="#1e293b" vertical={false} strokeOpacity={0.2} />
                                    <XAxis dataKey="time" stroke="#475569" fontSize={9} tickMargin={10} fontFamily='monospace' axisLine={false} />
                                    <YAxis stroke="#475569" fontSize={9} fontFamily='monospace' axisLine={false} tickLine={false} />
                                    <Tooltip
                                        contentStyle={{ backgroundColor: '#020617', border: '1px solid rgba(255,255,255,0.1)', borderRadius: '4px', fontSize: '10px', fontFamily: 'monospace' }}
                                        itemStyle={{ padding: '2px 0' }}
                                    />
                                    <Area type="monotone" dataKey="cpu" stroke="#f97316" strokeWidth={3} fill="url(#colorCpu)" animationDuration={1000} />
                                    <Area type="monotone" dataKey="memory" stroke="#06b6d4" strokeWidth={3} fill="url(#colorMem)" animationDuration={1000} />
                                </AreaChart>
                            </ResponsiveContainer>
                        </div>
                    </div>

                    <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
                        <div className="bg-slate-900/40 border border-white/5 rounded-lg p-6 backdrop-blur-sm">
                            <h3 className="text-xs font-bold text-white mb-6 uppercase tracking-widest flex items-center gap-2">
                                <MapIcon size={14} className="text-hive-cyan" />
                                Global Traffic Grid
                            </h3>
                            <NeuralTrafficGrid />
                        </div>
                        <div className="bg-slate-900/40 border border-white/5 rounded-lg p-6 backdrop-blur-sm flex flex-col">
                            <div className="flex items-center justify-between mb-6">
                                <h3 className="text-xs font-bold text-white uppercase tracking-widest flex items-center gap-2">
                                    <BrainCircuit size={14} className="text-hive-orange" />
                                    Neural Load Balancer
                                </h3>
                                <div className="flex items-center gap-2">
                                    <span className="text-[10px] font-mono text-emerald-500 uppercase font-bold">Inference: 99.2%</span>
                                </div>
                            </div>
                            <NeuralLoadBalancer />
                            <div className="mt-auto pt-4 flex items-center justify-between text-[10px] font-mono text-slate-500 uppercase tracking-widest">
                                <span>Actual Ingress</span>
                                <div className="flex items-center gap-4">
                                    <div className="flex items-center gap-1.5"><div className="w-1.5 h-1.5 rounded-full bg-hive-cyan"></div> Real</div>
                                    <div className="flex items-center gap-1.5"><div className="w-1.5 h-1.5 bg-hive-orange rounded-full opacity-50 border border-dashed border-white"></div> ML Model</div>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>

                <div className="space-y-6">
                    <div className="bg-slate-900/40 border border-white/5 rounded-lg p-6 backdrop-blur-sm">
                        <h3 className="text-xs font-bold text-white mb-6 uppercase tracking-widest flex items-center gap-2">
                            <GitCommit size={14} className="text-purple-500" />
                            Storage Entropy
                        </h3>
                        <div className="space-y-6">
                            <div>
                                <div className="flex justify-between text-[10px] font-mono text-slate-500 mb-2">
                                    <span>PAGE_FRAGMENTATION</span>
                                    <span className="text-emerald-500">0.02%</span>
                                </div>
                                <div className="h-1.5 w-full bg-slate-850 rounded-full overflow-hidden">
                                    <div className="h-full bg-emerald-500 w-[2%] shadow-[0_0_5px_#10b981]"></div>
                                </div>
                            </div>
                            <div>
                                <div className="flex justify-between text-[10px] font-mono text-slate-500 mb-2">
                                    <span>POINTER_DENSITY</span>
                                    <span className="text-purple-500">88.4%</span>
                                </div>
                                <div className="h-1.5 w-full bg-slate-850 rounded-full overflow-hidden">
                                    <div className="h-full bg-purple-500 w-[88%] shadow-[0_0_5px_#a855f7]"></div>
                                </div>
                            </div>
                        </div>
                    </div>

                    <div className="bg-slate-900/40 border border-white/5 rounded-lg p-6 backdrop-blur-sm">
                        <h3 className="text-xs font-bold text-white mb-6 uppercase tracking-widest flex items-center gap-2">
                            <Binary size={14} className="text-hive-cyan" />
                            Rust Thread Concurrency
                        </h3>
                        <ThreadMonitor />
                        <div className="mt-4 p-3 bg-slate-950/50 border border-white/5 rounded">
                            <p className="text-[9px] font-mono text-slate-500 leading-relaxed uppercase tracking-widest">
                                Parallelism: Work-Stealing Scheduler (Tokio)
                                Current Load Balancing: Symmetric
                            </p>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    );
};

export default Overview;

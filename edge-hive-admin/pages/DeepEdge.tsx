
import React, { useState, useEffect } from 'react';
import { 
  Radio, Cpu, Thermometer, Zap, ShieldAlert, MapPin, 
  Activity, RefreshCw, Layers, HardDrive, Server, Signal,
  ArrowUpRight, Eye, ShieldCheck, ChevronRight
} from 'lucide-react';
import { useToast } from '../context/ToastContext';
import { PhysicalNode } from '../types';

const TacticalMap = ({ nodes, onSelect }: { nodes: PhysicalNode[], onSelect: (node: PhysicalNode) => void }) => {
    return (
        <div className="relative w-full h-[500px] bg-slate-950/80 border border-white/10 rounded-xl overflow-hidden group shadow-2xl">
            {/* Grid Background */}
            <div className="absolute inset-0 opacity-10 pointer-events-none" style={{ backgroundImage: 'linear-gradient(#334155 1px, transparent 1px), linear-gradient(90deg, #334155 1px, transparent 1px)', backgroundSize: '40px 40px' }}></div>
            
            <svg className="absolute inset-0 w-full h-full">
                {/* Connecting Lines */}
                {nodes.map((node, i) => (
                    nodes.slice(i + 1).map((target, j) => (
                        <line 
                            key={`${i}-${j}`}
                            x1={`${node.coords.x}%`} y1={`${node.coords.y}%`} 
                            x2={`${target.coords.x}%`} y2={`${target.coords.y}%`}
                            stroke="rgba(6, 182, 212, 0.1)" 
                            strokeWidth="1"
                            strokeDasharray="4,4"
                        />
                    ))
                ))}
            </svg>

            {nodes.map((node) => (
                <button 
                    key={node.id} 
                    onClick={() => onSelect(node)}
                    className="absolute group/node -translate-x-1/2 -translate-y-1/2"
                    style={{ left: `${node.coords.x}%`, top: `${node.coords.y}%` }}
                >
                    <div className="relative">
                        {/* Ping Circle */}
                        <div className={`absolute -inset-4 rounded-full animate-ping opacity-20 ${node.status === 'critical' ? 'bg-red-500' : 'bg-hive-cyan'}`}></div>
                        
                        {/* Node Marker */}
                        <div className={`w-4 h-4 rounded-full border-2 border-slate-950 shadow-xl transition-all group-hover/node:scale-150 z-10 relative
                            ${node.status === 'online' ? 'bg-emerald-500 shadow-neon-cyan' : 
                              node.status === 'warning' ? 'bg-hive-orange shadow-neon-orange' : 'bg-red-500 shadow-[0_0_15px_#ef4444]'}
                        `}></div>

                        {/* Node Label */}
                        <div className="absolute top-6 left-1/2 -translate-x-1/2 bg-slate-900/90 border border-white/10 px-2 py-1 rounded text-[8px] font-mono text-white opacity-0 group-hover/node:opacity-100 transition-opacity whitespace-nowrap z-20">
                            {node.id} :: {node.location}
                        </div>
                    </div>
                </button>
            ))}

            <div className="absolute bottom-4 right-4 bg-slate-900/60 p-4 border border-white/5 rounded backdrop-blur-md">
                <div className="flex items-center gap-4 text-[9px] font-mono text-slate-500 uppercase">
                    <div className="flex items-center gap-1.5"><div className="w-2 h-2 rounded-full bg-emerald-500"></div> Optimal</div>
                    <div className="flex items-center gap-1.5"><div className="w-2 h-2 rounded-full bg-hive-orange"></div> Drift</div>
                    <div className="flex items-center gap-1.5"><div className="w-2 h-2 rounded-full bg-red-500"></div> Critical</div>
                </div>
            </div>
        </div>
    );
};

const HardwareTelemetry = ({ node }: { node: PhysicalNode }) => {
    return (
        <div className="bg-slate-900/40 border border-white/5 rounded-xl p-6 backdrop-blur-md animate-in slide-in-from-right-4 duration-300">
            <div className="flex items-center justify-between mb-8 border-b border-white/5 pb-4">
                <div className="flex items-center gap-3">
                    <div className="p-2 bg-slate-950 border border-white/10 rounded-lg text-hive-cyan">
                        <Cpu size={20} />
                    </div>
                    <div>
                        <h4 className="text-sm font-bold text-white uppercase tracking-tighter">{node.id}</h4>
                        <span className="text-[9px] font-mono text-slate-500 uppercase">{node.type} // ARM64_V8</span>
                    </div>
                </div>
                <div className="text-right">
                    <div className="text-[9px] font-mono text-slate-500 uppercase">Lattice ID</div>
                    <div className="text-[10px] font-bold text-white font-mono">0x{Math.random().toString(16).slice(2, 10).toUpperCase()}</div>
                </div>
            </div>

            <div className="grid grid-cols-2 gap-4">
                <div className="bg-slate-950 p-4 rounded-lg border border-white/5 space-y-3">
                    <div className="flex items-center justify-between text-[10px] font-mono text-slate-500 uppercase">
                        <div className="flex items-center gap-2"><Thermometer size={12} className="text-hive-orange" /> Core Temp</div>
                        <span className={node.metrics.temp > 70 ? 'text-red-500' : 'text-white'}>{node.metrics.temp}°C</span>
                    </div>
                    <div className="h-1 w-full bg-slate-900 rounded-full overflow-hidden">
                        <div className={`h-full transition-all duration-1000 ${node.metrics.temp > 70 ? 'bg-red-500 shadow-[0_0_10px_#ef4444]' : 'bg-hive-orange'}`} style={{ width: `${node.metrics.temp}%` }}></div>
                    </div>
                </div>

                <div className="bg-slate-950 p-4 rounded-lg border border-white/5 space-y-3">
                    <div className="flex items-center justify-between text-[10px] font-mono text-slate-500 uppercase">
                        <div className="flex items-center gap-2"><Zap size={12} className="text-hive-cyan" /> Power Draw</div>
                        <span className="text-white">{node.metrics.power}W</span>
                    </div>
                    <div className="h-1 w-full bg-slate-900 rounded-full overflow-hidden">
                        <div className="h-full bg-hive-cyan shadow-neon-cyan" style={{ width: `${(node.metrics.power / 20) * 100}%` }}></div>
                    </div>
                </div>

                <div className="col-span-2 bg-slate-950 p-4 rounded-lg border border-white/5 space-y-3">
                    <div className="flex items-center justify-between text-[10px] font-mono text-slate-500 uppercase">
                        <div className="flex items-center gap-2"><Signal size={12} className="text-purple-500" /> Mesh Signal</div>
                        <span className="text-white">{node.metrics.signal}% Integrity</span>
                    </div>
                    <div className="h-1 w-full bg-slate-950 rounded-full overflow-hidden">
                        <div className="h-full bg-purple-500 shadow-[0_0_10px_#a855f7]" style={{ width: `${node.metrics.signal}%` }}></div>
                    </div>
                </div>
            </div>

            <div className="mt-8 space-y-4">
                <button className="w-full py-2 bg-slate-950 border border-white/10 rounded text-[10px] font-bold text-white uppercase hover:border-hive-cyan transition-all flex items-center justify-center gap-2">
                    <RefreshCw size={14} /> Cold Boot HW
                </button>
                <button className="w-full py-2 bg-slate-950 border border-white/10 rounded text-[10px] font-bold text-slate-500 hover:text-white uppercase transition-all">
                    Access Local TTY
                </button>
            </div>
        </div>
    );
};

const DeepEdge: React.FC = () => {
    // Fix: useToast returns the toast object directly
    const toast = useToast();
    const [nodes] = useState<PhysicalNode[]>([
        { id: 'RX-400', type: 'compute_unit', location: 'NYC_FACILITY_A', coords: { x: 20, y: 35 }, status: 'online', metrics: { temp: 42, power: 12.5, signal: 98 } },
        { id: 'RX-401', type: 'gateway', location: 'LON_HUB_01', coords: { x: 45, y: 25 }, status: 'warning', metrics: { temp: 68, power: 18.2, signal: 84 } },
        { id: 'RX-402', type: 'sensor', location: 'TKY_CORE_B', coords: { x: 85, y: 45 }, status: 'online', metrics: { temp: 38, power: 4.2, signal: 99 } },
        { id: 'RX-405', type: 'compute_unit', location: 'SGP_NODE_03', coords: { x: 75, y: 75 }, status: 'critical', metrics: { temp: 84, power: 19.8, signal: 12 } },
    ]);
    const [selectedNode, setSelectedNode] = useState<PhysicalNode | null>(nodes[0]);
    const [isScanning, setIsScanning] = useState(false);

    const handlePing = () => {
        setIsScanning(true);
        toast.info("Sending global physical ping to last-mile devices...", "Network Ingress");
        setTimeout(() => {
            setIsScanning(false);
            toast.success("Mesh topography verified. 4 devices responding in < 4ms.");
        }, 2500);
    };

    return (
        <div className="space-y-8 animate-in fade-in duration-500 pb-12">
            <div className="flex flex-col md:flex-row items-start md:items-end justify-between gap-4 border-b border-white/5 pb-6">
                <div>
                    <div className="flex items-center gap-2 mb-2">
                        <Radio className="text-hive-orange animate-pulse" size={14} />
                        <span className="text-[10px] font-mono text-slate-500 font-bold uppercase tracking-widest">Hardware Orchestration Active</span>
                    </div>
                    <h1 className="text-4xl font-black text-white tracking-tighter uppercase">Deep Edge HW</h1>
                </div>
                <div className="flex gap-4">
                    <button 
                        onClick={handlePing}
                        disabled={isScanning}
                        className={`px-4 py-2 rounded border text-[10px] font-bold uppercase transition-all flex items-center gap-2
                            ${isScanning ? 'bg-hive-orange text-black border-hive-orange shadow-neon-orange' : 'bg-slate-900 border-white/10 text-white hover:border-hive-cyan/50'}
                        `}
                    >
                        {isScanning ? <RefreshCw size={14} className="animate-spin" /> : <Signal size={14} />}
                        Physical Ping
                    </button>
                    <button className="px-4 py-2 bg-hive-cyan text-black font-bold text-[10px] rounded shadow-neon-cyan uppercase">
                        Provision HW Node
                    </button>
                </div>
            </div>

            <div className="grid grid-cols-1 xl:grid-cols-3 gap-8">
                <div className="xl:col-span-2">
                    <TacticalMap nodes={nodes} onSelect={setSelectedNode} />
                </div>
                <div className="space-y-6">
                    {selectedNode ? (
                        <HardwareTelemetry node={selectedNode} />
                    ) : (
                        <div className="h-full bg-slate-900/40 border border-white/5 rounded-xl p-12 flex flex-col items-center justify-center text-center">
                            <Eye size={48} className="text-slate-700 mb-4" />
                            <p className="text-[10px] font-mono text-slate-500 uppercase tracking-widest">Select a physical node for telemetry analysis</p>
                        </div>
                    )}

                    <div className="bg-slate-950 border border-white/10 p-6 rounded-xl relative overflow-hidden group">
                        <div className="absolute top-0 right-0 p-4 opacity-10 text-emerald-500">
                            <ShieldCheck size={64} />
                        </div>
                        <h4 className="text-[10px] font-bold text-white uppercase tracking-widest mb-4 flex items-center gap-2">
                            <Layers size={14} className="text-emerald-500" />
                            Firmware Integrity
                        </h4>
                        <div className="space-y-3">
                             <div className="flex items-center justify-between text-[9px] font-mono border-b border-white/5 pb-2">
                                <span className="text-slate-500">Kernel Version</span>
                                <span className="text-white">6.6.15-hive-rt</span>
                             </div>
                             <div className="flex items-center justify-between text-[9px] font-mono border-b border-white/5 pb-2">
                                <span className="text-slate-500">Surreal binary</span>
                                <span className="text-emerald-500 font-bold uppercase">SECURE</span>
                             </div>
                             <div className="flex items-center justify-between text-[9px] font-mono">
                                <span className="text-slate-500">Update Channel</span>
                                <span className="text-hive-orange font-bold uppercase">Canary</span>
                             </div>
                        </div>
                    </div>
                </div>
            </div>

            <div className="grid grid-cols-1 md:grid-cols-2 xl:grid-cols-4 gap-6">
                {[
                    { label: 'Ingress Jitter', val: '0.42ms', icon: Activity, color: 'text-hive-cyan' },
                    { label: 'HVAC Load', val: 'Low', icon: Thermometer, color: 'text-emerald-500' },
                    { label: 'Uptime (Mesh)', val: '99.999%', icon: ShieldCheck, color: 'text-purple-500' },
                    { label: 'HW Failures', val: '0', icon: ShieldAlert, color: 'text-slate-500' },
                ].map((stat, i) => (
                    <div key={i} className="bg-slate-900/40 border border-white/5 p-4 rounded-lg backdrop-blur-sm flex items-center gap-4">
                        <div className={`p-2 rounded bg-slate-950 border border-white/5 ${stat.color}`}>
                            <stat.icon size={18} />
                        </div>
                        <div>
                            <div className="text-[9px] font-mono text-slate-500 uppercase">{stat.label}</div>
                            <div className="text-sm font-bold text-white font-mono">{stat.val}</div>
                        </div>
                    </div>
                ))}
            </div>
        </div>
    );
};

export default DeepEdge;

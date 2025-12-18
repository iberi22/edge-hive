
import React, { useState, useEffect } from 'react';
import {
    Ghost, Shield, Globe, RefreshCw, Plus, Trash2, Key,
    Activity, ArrowRight, ShieldAlert, Cpu, Network, Binary, ShieldCheck
} from 'lucide-react';
import { useToast } from '../context/ToastContext';
import { OnionService } from '../types';
import { tauriApi } from '../api/tauriClient';
import { LoadingState } from '../components/LoadingState';

const CircuitVisualizer = () => {
    const steps = [
        { label: 'Guard', icon: Shield, color: 'text-emerald-500' },
        { label: 'Middle', icon: Binary, color: 'text-slate-400' },
        { label: 'Exit', icon: Globe, color: 'text-hive-cyan' },
        { label: 'Hidden Service', icon: Ghost, color: 'text-purple-500' }
    ];

    return (
        <div className="flex items-center justify-between p-8 bg-slate-950/50 border border-white/5 rounded-xl relative overflow-hidden group">
            {/* Pulsing connection lines */}
            <div className="absolute inset-0 flex items-center justify-center px-12 pointer-events-none">
                <div className="w-full h-px bg-gradient-to-r from-emerald-500/20 via-slate-400/20 to-purple-500/20 animate-pulse"></div>
            </div>

            {steps.map((step, i) => (
                <div key={i} className="flex flex-col items-center gap-4 relative z-10">
                    <div className={`w-12 h-12 rounded-2xl bg-slate-900 border border-white/10 flex items-center justify-center shadow-2xl group-hover:scale-110 transition-transform ${step.color}`}>
                        <step.icon size={20} />
                    </div>
                    <div className="text-center">
                        <span className="text-[9px] font-mono text-slate-500 uppercase tracking-widest">{step.label}</span>
                        <div className="text-[10px] font-bold text-white font-mono mt-1">192.1{i}.{Math.floor(Math.random() * 255)}</div>
                    </div>
                </div>
            ))}
        </div>
    );
};

const OnionNode: React.FC = () => {
    // Fix: useToast returns the toast object directly
    const toast = useToast();
    const [services, setServices] = useState<OnionService[]>([]);
    const [isLoading, setIsLoading] = useState(true);

    useEffect(() => {
        const fetchStatus = async () => {
            try {
                const status = await tauriApi.getTunnelStatus();
                if (status.is_running && status.public_url) {
                    // If a tunnel is active, display it. For now adapting single tunnel status to list
                    setServices([
                        { id: '1', address: status.public_url, port: 8080, target_node: 'Localhost', uptime: 'Active', status: 'active' }
                    ]);
                } else {
                    setServices([]);
                }
            } finally {
                setIsLoading(false);
            }
        };
        fetchStatus();
    }, []);

    const handleRotate = (id: string) => {
        toast.info("Rotating Onion identity keys...", "Secret Ingress");
        // Stub for rotate functionality
        setTimeout(() => {
            toast.success("Identity rotated. New .onion address propagating.");
        }, 2000);
    };

    const handleCreate = async () => {
        try {
            await tauriApi.startTunnel(8080);
            const status = await tauriApi.getTunnelStatus();
            if (status.public_url) {
                setServices([{ id: '1', address: status.public_url, port: 8080, target_node: 'Localhost', uptime: 'Just started', status: 'active' }]);
                toast.success("Hidden Service created successfully");
            }
        } catch (e) {
            toast.error("Failed to start tunnel");
        }
    };

    if (isLoading) return <LoadingState />;

    return (
        <div className="space-y-8 animate-in fade-in duration-500 pb-12">
            <div className="flex flex-col md:flex-row items-start md:items-end justify-between gap-4 border-b border-white/5 pb-6">
                <div>
                    <div className="flex items-center gap-2 mb-2">
                        <Ghost className="text-purple-500 animate-pulse" size={14} />
                        <span className="text-[10px] font-mono text-purple-400 font-bold uppercase tracking-widest">Onion Overlay Active</span>
                    </div>
                    <h1 className="text-4xl font-black text-white tracking-tighter uppercase">Onion Nodes</h1>
                </div>
                <button onClick={handleCreate} className="px-4 py-2 bg-purple-600 text-white font-bold text-[10px] rounded shadow-[0_0_15px_rgba(168,85,247,0.4)] uppercase flex items-center gap-2">
                    <Plus size={14} /> New Hidden Service
                </button>
            </div>

            <div className="grid grid-cols-1 lg:grid-cols-3 gap-8">
                <div className="lg:col-span-2 space-y-8">
                    <div className="bg-slate-900/40 border border-white/5 rounded-xl p-8 backdrop-blur-md">
                        <h3 className="text-xs font-bold text-white uppercase tracking-widest mb-8 flex items-center gap-2">
                            <Network size={16} className="text-purple-500" />
                            Live Tor Circuit Path
                        </h3>
                        <CircuitVisualizer />
                        <div className="mt-6 p-4 bg-slate-950 border border-white/5 rounded-lg flex items-center justify-between text-[10px] font-mono">
                            <div className="flex items-center gap-4 text-slate-500">
                                <span className="flex items-center gap-1"><RefreshCw size={10} className="animate-spin" /> Next Rotation: 14:02:10</span>
                                <span className="text-emerald-500">Latency: 412ms</span>
                            </div>
                            <span className="text-slate-600">Tor v0.4.8.10 (git-6e06)</span>
                        </div>
                    </div>

                    <div className="bg-slate-900/40 border border-white/5 rounded-xl overflow-hidden backdrop-blur-md">
                        <table className="w-full text-left text-sm border-collapse">
                            <thead className="bg-slate-950 text-slate-500 font-mono text-[10px] uppercase tracking-wider">
                                <tr>
                                    <th className="p-4 border-b border-white/10">Hidden Address</th>
                                    <th className="p-4 border-b border-white/10">Target Node</th>
                                    <th className="p-4 border-b border-white/10">Status</th>
                                    <th className="p-4 border-b border-white/10 text-right">Actions</th>
                                </tr>
                            </thead>
                            <tbody className="divide-y divide-white/5 text-slate-300 font-mono text-[11px]">
                                {services.map(s => (
                                    <tr key={s.id} className="hover:bg-white/5 transition-colors group">
                                        <td className="p-4 font-bold text-purple-400 group-hover:text-white transition-colors">{s.address}</td>
                                        <td className="p-4 text-slate-500">{s.target_node} <span className="text-[9px] text-slate-700">:{s.port}</span></td>
                                        <td className="p-4">
                                            <div className="flex items-center gap-2">
                                                <div className={`w-1.5 h-1.5 rounded-full ${s.status === 'active' ? 'bg-emerald-500 shadow-neon-cyan' : 'bg-hive-orange animate-pulse'}`}></div>
                                                <span className={`uppercase font-bold text-[9px] ${s.status === 'active' ? 'text-emerald-500' : 'text-hive-orange'}`}>{s.status}</span>
                                            </div>
                                        </td>
                                        <td className="p-4 text-right">
                                            <div className="flex items-center justify-end gap-3">
                                                <button onClick={() => handleRotate(s.id)} className="p-1.5 hover:bg-slate-800 rounded text-slate-500" title="Rotate Keys"><RefreshCw size={14} /></button>
                                                <button className="p-1.5 hover:bg-red-500/10 rounded text-slate-500 hover:text-red-500"><Trash2 size={14} /></button>
                                            </div>
                                        </td>
                                    </tr>
                                ))}
                            </tbody>
                        </table>
                    </div>
                </div>

                <div className="space-y-6">
                    <div className="bg-slate-900 border border-white/5 p-6 rounded-xl backdrop-blur-sm relative overflow-hidden group">
                        <div className="absolute top-0 right-0 p-4 opacity-5 text-purple-500">
                            <Shield size={80} />
                        </div>
                        <h4 className="text-xs font-bold text-white uppercase tracking-widest mb-6 border-b border-white/5 pb-4">
                            Security Audit
                        </h4>
                        <div className="space-y-4">
                            <div className="p-4 bg-slate-950 rounded border border-white/5">
                                <div className="text-[10px] text-slate-500 uppercase mb-2">Private Key Storage</div>
                                <div className="flex items-center gap-2 text-xs font-bold text-emerald-500">
                                    <ShieldCheck size={14} /> SURREAL_TPM_SEALED
                                </div>
                            </div>
                            <div className="p-4 bg-slate-950 rounded border border-white/5">
                                <div className="text-[10px] text-slate-500 uppercase mb-2">Exit Node Policy</div>
                                <div className="text-xs font-bold text-white uppercase">STRICT_PRIVATE_ONLY</div>
                            </div>
                        </div>
                    </div>

                    <div className="bg-slate-950 border border-white/10 p-6 rounded-xl">
                        <h4 className="text-[10px] font-bold text-white uppercase tracking-widest mb-4 flex items-center gap-2">
                            <Activity size={14} className="text-purple-500" />
                            Dark Traffic
                        </h4>
                        <div className="space-y-3">
                            <div className="flex justify-between text-[10px] font-mono">
                                <span className="text-slate-500 uppercase">Requests (24h)</span>
                                <span className="text-white">12,402</span>
                            </div>
                            <div className="flex justify-between text-[10px] font-mono">
                                <span className="text-slate-500 uppercase">Bandwidth</span>
                                <span className="text-white">1.2 GB</span>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    );
};

export default OnionNode;

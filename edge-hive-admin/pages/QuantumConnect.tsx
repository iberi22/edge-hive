
import React, { useState, useEffect } from 'react';
import { Shield, Lock, Radio, Activity, Cpu, Cloud, Smartphone, Globe, RefreshCw, Key, Zap, CheckCircle, Fingerprint, Network } from 'lucide-react';
import { useToast } from '../context/ToastContext';

const EphemeralVault = () => {
    const [tokens, setTokens] = useState(Array.from({ length: 9 }).map((_, i) => ({
        id: `EP-${Math.random().toString(36).substring(7).toUpperCase()}`,
        expires: Math.random() * 60,
        type: 'TPM_SEALED'
    })));

    useEffect(() => {
        const interval = setInterval(() => {
            setTokens(prev => prev.map(t => {
                const nextExpires = t.expires - 1;
                return nextExpires <= 0 ? {
                    id: `EP-${Math.random().toString(36).substring(7).toUpperCase()}`,
                    expires: 60,
                    type: 'TPM_SEALED'
                } : { ...t, expires: nextExpires };
            }));
        }, 1000);
        return () => clearInterval(interval);
    }, []);

    return (
        <div className="bg-slate-900/40 border border-white/5 rounded-xl p-6 backdrop-blur-sm">
            <div className="flex items-center justify-between mb-6">
                <h3 className="text-xs font-bold text-white uppercase tracking-widest flex items-center gap-2">
                    <Fingerprint size={16} className="text-hive-orange" />
                    Ephemeral Identity Vault
                </h3>
                <span className="text-[9px] font-mono text-slate-500 uppercase">Hardware-Locked (TPM 2.0)</span>
            </div>
            
            <div className="grid grid-cols-3 gap-3">
                {tokens.map((token) => (
                    <div key={token.id} className="bg-slate-950/80 border border-white/5 rounded p-3 group hover:border-hive-orange/40 transition-all">
                        <div className="flex flex-col gap-1">
                            <span className="text-[8px] font-mono text-slate-600 uppercase">ID_PULSE</span>
                            <span className="text-[10px] font-mono text-white font-bold">{token.id}</span>
                        </div>
                        <div className="mt-3 h-0.5 bg-slate-900 rounded-full overflow-hidden">
                            <div 
                                className="h-full bg-hive-orange shadow-neon-orange transition-all duration-1000 linear" 
                                style={{ width: `${(token.expires / 60) * 100}%` }}
                            ></div>
                        </div>
                    </div>
                ))}
            </div>
            
            <div className="mt-6 flex items-center justify-between p-3 bg-hive-orange/5 border border-hive-orange/10 rounded-lg">
                <p className="text-[9px] text-hive-orange/80 font-mono italic">
                    "Tokens are rotated every 60s via hardware-entropy seed."
                </p>
                <button className="p-1 hover:bg-hive-orange/10 rounded transition">
                    <RefreshCw size={12} className="text-hive-orange" />
                </button>
            </div>
        </div>
    );
};

const QuantumTunnelMonitor = () => {
    const [tunnels, setTunnels] = useState([
        { id: 'TNL-842', client: 'iOS_SDK_v4', strength: 98, node: 'us-east-04', algo: 'Kyber-1024' },
        { id: 'TNL-119', client: 'Rust_Core_CLI', strength: 100, node: 'eu-west-01', algo: 'Dilithium-5' },
        { id: 'TNL-032', client: 'Web_WASM_Client', strength: 74, node: 'ap-south-02', algo: 'Kyber-768' },
    ]);

    return (
        <div className="bg-slate-900/40 border border-white/5 rounded-xl p-6 backdrop-blur-sm">
             <div className="flex items-center justify-between mb-8">
                <h3 className="text-xs font-bold text-white uppercase tracking-widest flex items-center gap-2">
                    <Shield size={16} className="text-hive-cyan" />
                    PQC SDK Tunneling
                </h3>
                <div className="flex items-center gap-2">
                    <div className="w-2 h-2 rounded-full bg-emerald-500 shadow-neon-cyan"></div>
                    <span className="text-[10px] font-mono text-emerald-500 font-bold uppercase">Safe Ingress</span>
                </div>
            </div>

            <div className="space-y-4">
                {tunnels.map((t) => (
                    <div key={t.id} className="p-4 bg-slate-950/50 border border-white/5 rounded-lg flex items-center justify-between group hover:bg-slate-900/50 transition-colors">
                        <div className="flex items-center gap-4">
                            <div className="p-2 bg-slate-900 rounded border border-white/10">
                                {t.client.includes('iOS') ? <Smartphone size={16} className="text-slate-400" /> : <Globe size={16} className="text-slate-400" />}
                            </div>
                            <div>
                                <div className="flex items-center gap-2">
                                    <span className="text-xs font-bold text-white">{t.client}</span>
                                    <span className="text-[8px] px-1.5 py-0.5 bg-hive-cyan/10 text-hive-cyan rounded-full border border-hive-cyan/20 font-bold">{t.algo}</span>
                                </div>
                                <span className="text-[9px] font-mono text-slate-600 uppercase">Node: {t.node} • ID: {t.id}</span>
                            </div>
                        </div>
                        <div className="flex items-center gap-6">
                            <div className="text-right">
                                <span className="text-[9px] font-mono text-slate-500 uppercase block mb-1">Entanglement</span>
                                <span className="text-xs font-bold text-white font-mono">{t.strength}%</span>
                            </div>
                            <div className="w-12 h-1.5 bg-slate-800 rounded-full overflow-hidden">
                                <div className="h-full bg-hive-cyan shadow-neon-cyan" style={{ width: `${t.strength}%` }}></div>
                            </div>
                        </div>
                    </div>
                ))}
            </div>
        </div>
    );
};

const CrossCloudMesh = () => {
    const clouds = [
        { id: 'AWS', icon: Cloud, color: 'text-orange-500', nodes: 24, load: 42 },
        { id: 'GCP', icon: Globe, color: 'text-blue-500', nodes: 12, load: 15 },
        { id: 'AZURE', icon: Cpu, color: 'text-blue-400', nodes: 8, load: 68 },
        { id: 'ON_PREM', icon: Lock, color: 'text-slate-400', nodes: 4, load: 10 },
    ];

    return (
        <div className="bg-slate-900/40 border border-white/5 rounded-xl p-8 backdrop-blur-sm relative overflow-hidden">
             <div className="flex items-center justify-between mb-12">
                <h3 className="text-sm font-bold text-white uppercase tracking-widest flex items-center gap-2">
                    <Network size={20} className="text-purple-500" />
                    Cross-Cloud Mesh Federation
                </h3>
                <span className="text-[10px] font-mono text-slate-500 uppercase tracking-widest">Topology: Zero-Trust-Grid</span>
            </div>

            <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-8">
                {clouds.map((cloud) => (
                    <div key={cloud.id} className="relative group flex flex-col items-center">
                        <div className="p-6 bg-slate-950 rounded-2xl border border-white/5 mb-4 group-hover:border-purple-500/50 transition-all shadow-2xl relative">
                            <cloud.icon size={32} className={`${cloud.color} transition-transform group-hover:scale-110`} />
                            <div className="absolute -top-1 -right-1">
                                <div className="w-3 h-3 bg-emerald-500 rounded-full border-2 border-slate-950 shadow-neon-cyan animate-pulse"></div>
                            </div>
                        </div>
                        <h4 className="text-white font-black text-sm uppercase tracking-tighter mb-1">{cloud.id} Cluster</h4>
                        <div className="flex items-center gap-2 text-[9px] font-mono text-slate-500 uppercase">
                            <span>{cloud.nodes} Active Nodes</span>
                            <span>•</span>
                            <span className={cloud.load > 60 ? 'text-red-500' : 'text-slate-500'}>{cloud.load}% Load</span>
                        </div>
                        
                        <div className="mt-4 w-full h-1 bg-slate-900 rounded-full overflow-hidden">
                             <div className="h-full bg-purple-500 shadow-[0_0_10px_#a855f7]" style={{ width: `${cloud.load}%` }}></div>
                        </div>
                    </div>
                ))}
            </div>

            {/* Sync Lines Simulation */}
            <div className="absolute inset-0 pointer-events-none opacity-20 z-0">
                <svg className="w-full h-full">
                    <path d="M 20% 50% Q 50% 10% 80% 50%" stroke="#a855f7" strokeWidth="1" fill="none" strokeDasharray="5,5" />
                    <path d="M 20% 50% Q 50% 90% 80% 50%" stroke="#a855f7" strokeWidth="1" fill="none" strokeDasharray="5,5" />
                </svg>
            </div>
        </div>
    );
};

const QuantumConnect: React.FC = () => {
    return (
        <div className="space-y-8 animate-in fade-in duration-500 pb-12">
            <div className="flex flex-col md:flex-row items-start md:items-end justify-between gap-4 border-b border-white/5 pb-6">
                <div>
                    <div className="flex items-center gap-2 mb-2">
                        <div className="w-2 h-2 rounded-full bg-purple-500 shadow-[0_0_10px_#a855f7]"></div>
                        <span className="text-[10px] font-mono text-purple-400 font-bold uppercase tracking-widest">NIST PQ Standards Active</span>
                    </div>
                    <h1 className="text-4xl font-black text-white tracking-tighter uppercase">Quantum Connectivity</h1>
                </div>
                <div className="flex gap-4">
                     <div className="text-right">
                        <div className="text-[9px] font-mono text-slate-500 uppercase">Encryption Level</div>
                        <div className="text-sm font-bold text-white flex items-center gap-2 justify-end">
                            AES-256-GCM + CRYSTALS-Kyber
                        </div>
                    </div>
                </div>
            </div>

            <div className="grid grid-cols-1 lg:grid-cols-2 gap-8">
                <QuantumTunnelMonitor />
                <EphemeralVault />
            </div>

            <CrossCloudMesh />

            <div className="bg-slate-950/80 border border-white/5 p-6 rounded-xl flex items-center justify-between">
                <div className="flex items-center gap-4">
                    <div className="p-3 bg-emerald-500/10 rounded-lg border border-emerald-500/20">
                        <CheckCircle size={24} className="text-emerald-500" />
                    </div>
                    <div>
                        <h4 className="text-sm font-bold text-white uppercase tracking-widest">Zero-Trust Verified</h4>
                        <p className="text-[10px] text-slate-500 font-mono">Continuous posture assessment enabled for all clúster connections.</p>
                    </div>
                </div>
                <button className="px-4 py-2 bg-slate-900 border border-white/10 rounded text-[10px] font-bold text-slate-400 hover:text-white transition uppercase tracking-widest">
                    Run Mesh Audit
                </button>
            </div>
        </div>
    );
};

export default QuantumConnect;


import React, { useState } from 'react';
import {
    Lock, Shield, Zap, Plus, Smartphone, Server, Globe,
    Activity, ArrowRightLeft, MoreVertical, Copy, Key,
    Signal, RefreshCw, Terminal, CheckCircle2
} from 'lucide-react';
import { useToast } from '../context/ToastContext';
import { VPNPeer } from '../types';

const PeerCard = ({ peer }: { peer: VPNPeer }) => {
    return (
        <div className="bg-slate-900/40 border border-white/5 rounded-xl p-5 group hover:border-hive-cyan/30 transition-all backdrop-blur-sm">
            <div className="flex justify-between items-start mb-6">
                <div className="flex items-center gap-4">
                    <div className={`p-3 rounded-lg bg-slate-950 border border-white/5 transition-colors ${peer.status === 'connected' ? 'text-hive-cyan shadow-neon-cyan' : 'text-slate-600'}`}>
                        {peer.endpoint.includes('Mobile') ? <Smartphone size={20} /> : <Server size={20} />}
                    </div>
                    <div>
                        <h4 className="text-sm font-bold text-white uppercase tracking-tighter">{peer.id}</h4>
                        <span className="text-[9px] font-mono text-slate-500 uppercase tracking-widest">{peer.endpoint}</span>
                    </div>
                </div>
                <div className={`px-2 py-0.5 rounded-full text-[8px] font-bold uppercase tracking-widest border
                    ${peer.status === 'connected' ? 'bg-emerald-500/10 text-emerald-500 border-emerald-500/20' : 'bg-slate-800 text-slate-500 border-white/5'}`}>
                    {peer.status}
                </div>
            </div>

            <div className="space-y-3 font-mono">
                <div className="flex items-center justify-between text-[9px]">
                    <span className="text-slate-600 uppercase">Allowed IPs</span>
                    <span className="text-slate-300">{peer.allowed_ips.join(', ')}</span>
                </div>
                <div className="flex items-center justify-between text-[9px]">
                    <span className="text-slate-600 uppercase">Transfer</span>
                    <span className="text-slate-300 flex items-center gap-2">
                        <ArrowRightLeft size={10} className="text-hive-cyan" /> {peer.transfer_rx} / {peer.transfer_tx}
                    </span>
                </div>
                <div className="flex items-center justify-between text-[9px] pt-2 border-t border-white/5">
                    <span className="text-slate-600 uppercase">Last Handshake</span>
                    <span className="text-emerald-500">{peer.last_handshake}</span>
                </div>
            </div>

            <div className="mt-6 flex gap-2 opacity-0 group-hover:opacity-100 transition-opacity">
                <button className="flex-1 py-1.5 bg-slate-950 border border-white/5 rounded text-[9px] font-bold text-slate-400 hover:text-white uppercase transition-all">Configure</button>
                <button className="px-2 py-1.5 bg-slate-950 border border-white/5 rounded text-slate-400 hover:text-white transition-all"><MoreVertical size={12} /></button>
            </div>
        </div>
    );
};

const VPNMesh: React.FC = () => {
    // Fix: useToast returns the toast object directly
    const toast = useToast();
    const [peers, setPeers] = useState<VPNPeer[]>([]);

    useEffect(() => {
        const loadPeers = async () => {
            const { tauriApi } = await import('../api/tauriClient');
            try {
                const data = await tauriApi.getVPNPeers();
                setPeers(data);
            } catch (e) {
                console.error("Failed to load peers", e);
            }
        };
        loadPeers();
    }, []);

    const handleGenerateConfig = async () => {
        const { tauriApi } = await import('../api/tauriClient');
        const config = await tauriApi.generateVPNConfig();
        navigator.clipboard.writeText(config);
        toast.success("Client configuration generated & copied", "Mesh Access");
    };

    return (
        <div className="space-y-8 animate-in fade-in duration-500 pb-12">
            <div className="flex flex-col md:flex-row items-start md:items-end justify-between gap-4 border-b border-white/5 pb-6">
                <div>
                    <div className="flex items-center gap-2 mb-2">
                        <Lock className="text-hive-cyan animate-pulse" size={14} />
                        <span className="text-[10px] font-mono text-hive-cyan font-bold uppercase tracking-widest">WireGuard Mesh Active</span>
                    </div>
                    <h1 className="text-4xl font-black text-white tracking-tighter uppercase">VPN Mesh</h1>
                </div>
                <div className="flex gap-4">
                    <button onClick={handleGenerateConfig} className="px-4 py-2 bg-slate-900 border border-white/10 rounded text-[10px] font-bold text-slate-400 uppercase hover:text-white hover:border-hive-cyan transition-all flex items-center gap-2">
                        <Key size={14} /> Global PubKey
                    </button>
                    <button className="px-4 py-2 bg-hive-cyan text-black font-bold text-[10px] rounded shadow-neon-cyan uppercase flex items-center gap-2">
                        <Plus size={14} /> Add Peer
                    </button>
                </div>
            </div>

            <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
                {peers.map(p => <PeerCard key={p.id} peer={p} />)}
                <button className="border-2 border-dashed border-white/5 rounded-xl p-8 flex flex-col items-center justify-center text-slate-600 hover:text-hive-cyan hover:border-hive-cyan/30 hover:bg-white/5 transition-all group">
                    <Signal size={32} className="mb-4 group-hover:scale-125 transition-transform" />
                    <span className="text-[10px] font-bold uppercase tracking-widest">Awaiting Peers...</span>
                </button>
            </div>

            <div className="grid grid-cols-1 lg:grid-cols-3 gap-8">
                <div className="lg:col-span-2 bg-slate-900/40 border border-white/5 rounded-xl p-8 backdrop-blur-md relative overflow-hidden group">
                    <div className="absolute top-0 right-0 p-12 text-hive-cyan opacity-5 group-hover:rotate-12 transition-transform">
                        <Globe size={200} />
                    </div>
                    <h3 className="text-sm font-bold text-white uppercase tracking-widest mb-8 flex items-center gap-2">
                        <Activity size={18} className="text-hive-cyan" />
                        Encrypted Tunnel Traffic
                    </h3>
                    <div className="h-48 flex items-end gap-1 px-4">
                        {Array.from({ length: 48 }).map((_, i) => (
                            <div
                                key={i}
                                className="bg-hive-cyan/30 w-full hover:bg-hive-cyan transition-colors"
                                style={{ height: `${20 + Math.random() * 80}%` }}
                                title={`Traffic @ T-${48 - i}m: ${Math.floor(Math.random() * 100)} Mbps`}
                            ></div>
                        ))}
                    </div>
                    <div className="mt-8 grid grid-cols-3 gap-4 border-t border-white/5 pt-8">
                        <div>
                            <div className="text-[10px] text-slate-600 uppercase mb-1">MTU Optimized</div>
                            <div className="text-sm font-bold text-white">1420 Bytes</div>
                        </div>
                        <div>
                            <div className="text-[10px] text-slate-600 uppercase mb-1">Ciphersuite</div>
                            <div className="text-sm font-bold text-white">ChaCha20-Poly1305</div>
                        </div>
                        <div>
                            <div className="text-[10px] text-slate-600 uppercase mb-1">Mesh Topology</div>
                            <div className="text-sm font-bold text-emerald-500 uppercase">FULL_CONNECT</div>
                        </div>
                    </div>
                </div>

                <div className="bg-slate-950 border border-white/10 rounded-xl p-6 flex flex-col relative overflow-hidden">
                    <h4 className="text-[10px] font-bold text-slate-500 uppercase tracking-widest mb-6 flex items-center gap-2">
                        <Terminal size={14} />
                        SurrealDB State Sink
                    </h4>
                    <div className="flex-1 space-y-4 font-mono text-[9px]">
                        <p className="text-slate-500 italic">"VPN state is persisted as record streams in the 'vpn_state' table."</p>
                        <div className="p-3 bg-slate-900 border border-white/5 rounded text-slate-400">
                            <span className="text-hive-cyan">SELECT</span> * <span className="text-hive-cyan">FROM</span> vpn_state <span className="text-hive-cyan">WHERE</span> peer = <span className="text-purple-400">"HN-01"</span>;
                        </div>
                        <div className="space-y-2 mt-6">
                            <div className="flex items-center gap-3">
                                <CheckCircle2 size={12} className="text-emerald-500" />
                                <span className="text-slate-400">Atomic peer rotation</span>
                            </div>
                            <div className="flex items-center gap-3">
                                <CheckCircle2 size={12} className="text-emerald-500" />
                                <span className="text-slate-400">Zero-leak routing (NS)</span>
                            </div>
                            <div className="flex items-center gap-3 opacity-50">
                                <Activity size={12} className="text-slate-600" />
                                <span className="text-slate-600">IPsec Fallback (Disabled)</span>
                            </div>
                        </div>
                    </div>
                    <button className="w-full mt-8 py-2 bg-slate-900 border border-white/5 rounded text-[9px] font-bold text-slate-500 hover:text-white uppercase transition-all">Export Peer List</button>
                </div>
            </div>
        </div>
    );
};

export default VPNMesh;

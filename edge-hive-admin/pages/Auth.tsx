
import React, { useEffect, useState } from 'react';
import { LoadingState } from '../components/LoadingState';
import {
    Users, Search, Mail, Shield, MoreVertical, UserPlus, FileCode, FlaskConical,
    CheckCircle2, XCircle, Radio, BrainCircuit, Key, Lock, ShieldCheck, Zap,
    MessageSquare, Send, RefreshCw
} from 'lucide-react';
import { tauriApi } from '../api/tauriClient';
import { User } from '../types';
import { StatusBadge } from '../components/StatusBadge';
import { useToast } from '../context/ToastContext';

const NeuralRBACEditor = () => {
    // Fix: useToast returns the toast object directly
    const toast = useToast();
    const [input, setInput] = useState("");
    const [isProcessing, setIsProcessing] = useState(false);
    const [history, setHistory] = useState([
        { role: 'ai', text: 'Neural RBAC Studio Online. How can I assist with permission architecture? (e.g. "Grant billing access to the engineering group but only for cloud-workers")' }
    ]);

    const handleApply = () => {
        if (!input.trim() || isProcessing) return;
        setIsProcessing(true);
        setHistory(prev => [...prev, { role: 'user', text: input }]);
        setInput("");

        setTimeout(() => {
            setHistory(prev => [...prev, { role: 'ai', text: 'Analyzing request... Lattice Policy synthesized. [UPDATE] Added condition: group="eng" AND resource.type="worker_node". Apply changes?' }]);
            setIsProcessing(false);
        }, 1500);
    };

    return (
        <div className="bg-slate-900/40 border border-purple-500/20 rounded-xl p-6 backdrop-blur-sm h-full flex flex-col">
            <div className="flex items-center justify-between mb-6">
                <h3 className="text-sm font-bold text-white flex items-center gap-3">
                    <BrainCircuit size={18} className="text-purple-500" />
                    Neural RBAC Studio
                </h3>
                <span className="text-[9px] font-mono text-slate-500 uppercase">Decision Logic: Lattice_v3</span>
            </div>

            <div className="flex-1 overflow-y-auto mb-6 space-y-4 custom-scrollbar pr-2">
                {history.map((h, i) => (
                    <div key={i} className={`flex ${h.role === 'user' ? 'justify-end' : 'justify-start'}`}>
                        <div className={`max-w-[85%] p-3 rounded-xl text-[10px] font-mono leading-relaxed
                            ${h.role === 'user' ? 'bg-hive-orange text-black font-bold' : 'bg-slate-950 border border-white/5 text-slate-300'}
                        `}>
                            {h.text}
                            {h.role === 'ai' && i > 0 && (
                                <div className="mt-2 pt-2 border-t border-white/5 flex gap-2">
                                    <button onClick={() => toast.success("Policy Applied Successfully")} className="text-[8px] font-bold text-emerald-500 uppercase hover:underline">Apply Policy</button>
                                    <button className="text-[8px] font-bold text-red-500 uppercase hover:underline">Discard</button>
                                </div>
                            )}
                        </div>
                    </div>
                ))}
            </div>

            <div className="relative mt-auto">
                <input
                    type="text"
                    value={input}
                    onChange={e => setInput(e.target.value)}
                    onKeyDown={e => e.key === 'Enter' && handleApply()}
                    placeholder="Describe permission change..."
                    className="w-full bg-slate-950 border border-white/10 rounded-lg pl-4 pr-12 py-3 text-[10px] text-white focus:outline-none focus:border-purple-500/50"
                />
                <button
                    onClick={handleApply}
                    disabled={isProcessing}
                    className="absolute right-2 top-1/2 -translate-y-1/2 p-1.5 bg-purple-600 text-white rounded hover:scale-105 transition-transform"
                >
                    {isProcessing ? <RefreshCw size={14} className="animate-spin" /> : <Send size={14} />}
                </button>
            </div>
        </div>
    );
};

const Auth: React.FC = () => {
    const [users, setUsers] = useState<User[]>([]);
    const [activeTab, setActiveTab] = useState<'users' | 'rbac' | 'lab'>('users');
    const [isLoading, setIsLoading] = useState(true);

    useEffect(() => {
        tauriApi.getUsers().then(u => {
            setUsers(u);
            setIsLoading(false);
        });
    }, []);

    if (isLoading) return <LoadingState />;

    return (
        <div className="space-y-8 animate-in fade-in duration-500 pb-12">
            <div className="flex flex-col md:flex-row items-start md:items-end justify-between gap-4 border-b border-white/5 pb-6">
                <div>
                    <div className="flex items-center gap-2 mb-2">
                        <ShieldCheck className="text-hive-cyan" size={14} />
                        <span className="text-[10px] font-mono text-slate-500 font-bold uppercase tracking-widest">Identity & Access Governance</span>
                    </div>
                    <h1 className="text-4xl font-black text-white tracking-tighter uppercase">Authentication</h1>
                </div>
                <div className="flex bg-slate-900 p-1 rounded-lg border border-white/10">
                    {[
                        { id: 'users', label: 'Identity Grid', icon: Users },
                        { id: 'rbac', label: 'Neural RBAC', icon: BrainCircuit },
                        { id: 'lab', label: 'Policy Lab', icon: FlaskConical }
                    ].map(tab => (
                        <button
                            key={tab.id}
                            onClick={() => setActiveTab(tab.id as any)}
                            className={`px-4 py-1.5 rounded-md text-[10px] font-bold uppercase flex items-center gap-2 transition-all
                            ${activeTab === tab.id ? 'bg-white/10 text-white shadow-inner' : 'text-slate-500 hover:text-slate-300'}
                        `}
                        >
                            <tab.icon size={12} /> {tab.label}
                        </button>
                    ))}
                </div>
            </div>

            {activeTab === 'users' && (
                <div className="grid grid-cols-1 xl:grid-cols-4 gap-8">
                    <div className="xl:col-span-3 bg-slate-900/40 border border-white/5 rounded-xl overflow-hidden flex flex-col relative h-[600px]">
                        <div className="p-4 border-b border-white/5 bg-slate-950/50 flex items-center justify-between">
                            <div className="relative w-64">
                                <Search className="absolute left-3 top-1/2 -translate-y-1/2 text-slate-600" size={14} />
                                <input type="text" placeholder="Filter identities..." className="w-full bg-slate-900 border border-white/5 rounded pl-10 pr-4 py-1.5 text-[10px] text-white focus:outline-none" />
                            </div>
                            <button className="px-3 py-1 bg-hive-cyan text-black font-bold text-[10px] rounded shadow-neon-cyan uppercase flex items-center gap-2">
                                <UserPlus size={12} /> Invite Identity
                            </button>
                        </div>
                        <div className="flex-1 overflow-auto custom-scrollbar">
                            <table className="w-full text-left text-sm border-collapse min-w-[800px]">
                                <thead className="bg-slate-950 sticky top-0 z-10 font-mono text-[10px] uppercase text-slate-500">
                                    <tr>
                                        <th className="p-4 border-b border-white/10">User ID</th>
                                        <th className="p-4 border-b border-white/10">Email</th>
                                        <th className="p-4 border-b border-white/10">Role Map</th>
                                        <th className="p-4 border-b border-white/10 text-center">Status</th>
                                    </tr>
                                </thead>
                                <tbody className="divide-y divide-white/5 text-slate-300 font-mono text-[11px]">
                                    {users.map((user) => (
                                        <tr key={user.id} className="hover:bg-white/5 transition-colors group">
                                            <td className="p-4 text-hive-cyan font-bold">{user.id}</td>
                                            <td className="p-4">{user.email}</td>
                                            <td className="p-4">
                                                <span className="px-2 py-0.5 bg-slate-950 border border-white/10 rounded-full text-[9px] uppercase">
                                                    {user.provider === 'github' ? 'Engineering' : 'Platform_Admin'}
                                                </span>
                                            </td>
                                            <td className="p-4 text-center">
                                                <div className="flex justify-center">
                                                    <StatusBadge status={user.status === 'active' ? 'healthy' : 'error'} pulse={false} />
                                                </div>
                                            </td>
                                        </tr>
                                    ))}
                                </tbody>
                            </table>
                        </div>
                    </div>
                    <div className="space-y-6">
                        <div className="bg-slate-900/40 border border-white/5 p-6 rounded-xl backdrop-blur-sm">
                            <h4 className="text-[10px] font-bold text-white uppercase tracking-widest mb-4 flex items-center gap-2">
                                <Shield size={14} className="text-hive-orange" />
                                Session Lattice
                            </h4>
                            <div className="space-y-4">
                                <div className="flex items-center justify-between text-[10px] font-mono">
                                    <span className="text-slate-500">Active Sessions</span>
                                    <span className="text-white">142</span>
                                </div>
                                <div className="flex items-center justify-between text-[10px] font-mono">
                                    <span className="text-slate-500">Average TTL</span>
                                    <span className="text-white">4h 12m</span>
                                </div>
                                <button className="w-full mt-4 py-2 bg-slate-950 border border-white/5 rounded text-[10px] font-bold text-red-500 uppercase hover:bg-red-500/10 transition-all">
                                    Revoke Global
                                </button>
                            </div>
                        </div>
                    </div>
                </div>
            )}

            {activeTab === 'rbac' && (
                <div className="grid grid-cols-1 lg:grid-cols-2 gap-8 h-[600px]">
                    <NeuralRBACEditor />
                    <div className="bg-slate-950 border border-white/10 rounded-xl p-8 flex flex-col relative overflow-hidden group">
                        <div className="absolute top-0 right-0 p-8 text-hive-cyan opacity-5 group-hover:rotate-6 transition-transform">
                            <ShieldCheck size={200} />
                        </div>
                        <h4 className="text-[10px] font-bold text-slate-500 uppercase tracking-widest mb-8 border-b border-white/5 pb-4">
                            Policy Visualization
                        </h4>
                        <div className="flex-1 flex flex-col items-center justify-center space-y-8">
                            {/* Policy Visual Graph */}
                            <div className="relative">
                                <div className="w-16 h-16 bg-purple-600 rounded-2xl flex items-center justify-center shadow-neon-orange z-10 relative">
                                    <Lock size={32} className="text-white" />
                                </div>
                                <div className="absolute top-1/2 left-full w-24 h-[1px] bg-white/10 flex items-center justify-end">
                                    <div className="w-2 h-2 rounded-full bg-hive-cyan shadow-neon-cyan"></div>
                                </div>
                            </div>
                            <div className="grid grid-cols-3 gap-12 w-full">
                                {['Ingress', 'Database', 'Storage'].map(role => (
                                    <div key={role} className="flex flex-col items-center">
                                        <div className="w-10 h-10 bg-slate-900 border border-white/10 rounded-lg flex items-center justify-center mb-2">
                                            <Key size={18} className="text-slate-500" />
                                        </div>
                                        <span className="text-[9px] font-mono text-slate-600 uppercase tracking-widest">{role}</span>
                                    </div>
                                ))}
                            </div>
                        </div>
                    </div>
                </div>
            )}

            {activeTab === 'lab' && (
                <div className="bg-slate-900/40 border border-white/5 rounded-xl p-12 text-center text-slate-700 uppercase tracking-widest font-mono italic">
                    Policy Simulation Sandbox - Restricted Access
                </div>
            )}
        </div>
    );
};

export default Auth;

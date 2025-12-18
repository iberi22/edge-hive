
import React, { useEffect, useState, useRef } from 'react';
import { HardDrive, FolderOpen, File, MoreVertical, UploadCloud, Globe, Lock, Search, Grid, List, Shield, ShieldCheck, Thermometer, Clock, ArrowDownCircle, Zap, TrendingDown, DollarSign, Settings2 } from 'lucide-react';
import { mockApi } from '../api';
import { StorageBucket, StorageFile, StoragePolicy } from '../types';
import { useToast } from '../context/ToastContext';

const EntropyBar: React.FC<{ temperature: number }> = ({ temperature }) => {
    // temperature 0-1 (0 = cold, 1 = hot)
    const colorClass = temperature > 0.7 ? 'bg-orange-500 shadow-neon-orange' : temperature > 0.3 ? 'bg-hive-cyan' : 'bg-blue-600';
    return (
        <div className="flex flex-col gap-1 w-full">
            <div className="h-1 bg-slate-800 rounded-full overflow-hidden">
                <div className={`h-full ${colorClass} transition-all duration-1000`} style={{ width: `${temperature * 100}%` }}></div>
            </div>
        </div>
    );
};

const Storage: React.FC = () => {
    // Fix: useToast returns the toast object directly
    const toast = useToast();
    const [buckets, setBuckets] = useState<StorageBucket[]>([]);
    const [selectedBucket, setSelectedBucket] = useState<string | null>(null);
    const [files, setFiles] = useState<StorageFile[]>([]);
    const [policies, setPolicies] = useState<StoragePolicy[]>([]);
    const [activeTab, setActiveTab] = useState<'files' | 'policies' | 'lifecycle'>('files');
    const [isDragging, setIsDragging] = useState(false);

    // Tiering Config
    const [tieringRules, setTieringRules] = useState({
        move_to_warm: 30, // days
        move_to_cold: 90, // days
        auto_delete: 365, // days
    });

    useEffect(() => {
        mockApi.getBuckets().then(data => {
            setBuckets(data);
            if (data.length > 0) setSelectedBucket(bucketId => bucketId || data[0].id);
        });
    }, []);

    useEffect(() => {
        if (selectedBucket) {
            if (activeTab === 'files' || activeTab === 'lifecycle') {
                mockApi.getFiles(selectedBucket).then(setFiles);
            } else if (activeTab === 'policies') {
                mockApi.getStoragePolicies(selectedBucket).then(setPolicies);
            }
        }
    }, [selectedBucket, activeTab]);

    const activeBucket = buckets.find(b => b.id === selectedBucket);

    const handleDrop = (e: React.DragEvent) => {
        e.preventDefault();
        setIsDragging(false);
        if (e.dataTransfer.files.length > 0) {
            toast.success(`${e.dataTransfer.files.length} files uploaded to ${activeBucket?.name}`);
        }
    };

    const saveTieringRules = () => {
        toast.success("Archival rules updated", "Hive Engine is recalculating entropy...");
    };

    return (
        <div className="flex flex-col md:flex-row h-full md:h-[calc(100vh-8rem)] gap-6"
            onDragEnter={() => setIsDragging(true)}
            onDragOver={(e) => e.preventDefault()}
            onDragLeave={() => setIsDragging(false)}
            onDrop={handleDrop}>

            {/* Bucket List */}
            <div className="w-full md:w-64 flex-shrink-0 flex flex-col gap-4">
                <div className="bg-slate-900/50 border border-white/5 rounded-lg p-4 h-full flex flex-col">
                    <div className="flex items-center justify-between mb-4">
                        <h3 className="text-[10px] font-mono text-slate-500 uppercase tracking-widest">Buckets</h3>
                        <Zap size={12} className="text-hive-orange" />
                    </div>
                    <div className="space-y-2 overflow-y-auto custom-scrollbar flex-1">
                        {buckets.map(bucket => (
                            <button
                                key={bucket.id}
                                onClick={() => setSelectedBucket(bucket.id)}
                                className={`w-full text-left p-3 rounded border transition-all
                            ${selectedBucket === bucket.id ? 'bg-slate-900 border-hive-cyan/30 shadow-neon-cyan' : 'bg-transparent border-transparent hover:bg-white/5'}
                        `}
                            >
                                <div className="flex items-center justify-between mb-1">
                                    <span className={`text-xs font-bold ${selectedBucket === bucket.id ? 'text-white' : 'text-slate-400'}`}>{bucket.name}</span>
                                    {bucket.public ? <Globe size={10} className="text-emerald-500" /> : <Lock size={10} className="text-slate-600" />}
                                </div>
                                <div className="flex justify-between text-[9px] font-mono text-slate-600 uppercase">
                                    <span>{bucket.size}</span>
                                    <span>{bucket.files_count} OBJ</span>
                                </div>
                            </button>
                        ))}
                    </div>
                </div>
            </div>

            {/* Explorer Area */}
            <div className="flex-1 flex flex-col gap-4 min-w-0 h-full relative">
                <div className="bg-slate-900/40 border border-white/5 rounded-lg flex flex-col h-full overflow-hidden backdrop-blur-sm">
                    {/* Nav Header */}
                    <div className="px-6 py-4 border-b border-white/5 flex flex-col sm:flex-row items-center justify-between gap-4 bg-slate-950/40">
                        <div className="flex items-center gap-4">
                            <div className="bg-slate-800 p-2 rounded border border-white/10">
                                <HardDrive size={18} className="text-hive-cyan" />
                            </div>
                            <div>
                                <h2 className="text-sm font-bold text-white uppercase tracking-wider">{activeBucket?.name || '---'}</h2>
                                <span className="text-[10px] font-mono text-slate-500">{activeBucket?.id}</span>
                            </div>
                        </div>
                        <div className="flex bg-slate-950 p-1 rounded-lg border border-white/10">
                            {[
                                { id: 'files', label: 'Objects', icon: Grid },
                                { id: 'lifecycle', label: 'Lifecycle', icon: Thermometer },
                                { id: 'policies', label: 'Security', icon: ShieldCheck }
                            ].map(tab => (
                                <button
                                    key={tab.id}
                                    onClick={() => setActiveTab(tab.id as any)}
                                    className={`px-4 py-1.5 rounded-md text-[10px] font-bold uppercase flex items-center gap-2 transition-all
                                ${activeTab === tab.id ? 'bg-white/10 text-white' : 'text-slate-500 hover:text-slate-300'}
                            `}
                                >
                                    <tab.icon size={12} /> {tab.label}
                                </button>
                            ))}
                        </div>
                    </div>

                    {/* Content Display */}
                    <div className="flex-1 overflow-auto bg-[#090c10] custom-scrollbar relative p-6">
                        {isDragging && (
                            <div className="absolute inset-4 z-50 bg-hive-cyan/5 border-2 border-dashed border-hive-cyan/30 rounded-xl flex flex-col items-center justify-center animate-in fade-in">
                                <UploadCloud size={48} className="text-hive-cyan animate-bounce mb-4" />
                                <span className="text-white font-bold uppercase tracking-widest text-sm">Drop to ingress to Hive</span>
                            </div>
                        )}

                        {activeTab === 'files' && (
                            <div className="grid grid-cols-2 sm:grid-cols-3 lg:grid-cols-5 gap-4">
                                {files.map(file => (
                                    <div key={file.id} className="bg-slate-900/50 border border-white/5 rounded-lg p-4 group hover:border-hive-cyan/50 transition-all cursor-pointer">
                                        <div className="aspect-square bg-slate-950 rounded mb-3 flex items-center justify-center relative overflow-hidden">
                                            <File size={24} className="text-slate-600 group-hover:text-hive-cyan transition-colors" />
                                            <div className="absolute bottom-0 left-0 w-full h-1 bg-hive-cyan/20"></div>
                                        </div>
                                        <div className="text-center">
                                            <p className="text-[11px] font-bold text-slate-300 truncate mb-1" title={file.name}>{file.name}</p>
                                            <p className="text-[9px] font-mono text-slate-600 uppercase">{file.size} • {file.type.split('/')[1]}</p>
                                        </div>
                                    </div>
                                ))}
                            </div>
                        )}

                        {activeTab === 'lifecycle' && (
                            <div className="space-y-8 max-w-5xl mx-auto">
                                <div className="grid grid-cols-1 lg:grid-cols-3 gap-6">
                                    <div className="lg:col-span-2 space-y-6">
                                        <div className="bg-slate-900/50 border border-white/5 rounded-lg p-6 backdrop-blur-sm">
                                            <div className="flex items-center justify-between mb-6">
                                                <h3 className="text-sm font-bold text-white flex items-center gap-2 uppercase tracking-widest">
                                                    <Settings2 size={16} className="text-hive-orange" />
                                                    Auto-Tiering Engine
                                                </h3>
                                            </div>
                                            <div className="space-y-8">
                                                <div className="space-y-4">
                                                    <div className="flex justify-between items-end">
                                                        <div>
                                                            <span className="text-[10px] font-mono text-slate-500 uppercase block mb-1">L1 (Fast) {'->'} L2 (Warm) Archival</span>
                                                            <p className="text-[9px] text-slate-600">Move to SSD if unaccessed for</p>
                                                        </div>
                                                        <span className="text-sm font-bold text-hive-cyan font-mono">{tieringRules.move_to_warm} Days</span>
                                                    </div>
                                                    <input type="range" min="1" max="60" value={tieringRules.move_to_warm} onChange={e => setTieringRules({ ...tieringRules, move_to_warm: parseInt(e.target.value) })} className="w-full h-1 bg-slate-800 rounded-lg appearance-none cursor-pointer accent-hive-cyan" />
                                                </div>

                                                <div className="space-y-4">
                                                    <div className="flex justify-between items-end">
                                                        <div>
                                                            <span className="text-[10px] font-mono text-slate-500 uppercase block mb-1">L2 (Warm) {'->'} L3 (Cold) Archival</span>
                                                            <p className="text-[9px] text-slate-600">Move to S3 Glacier if unaccessed for</p>
                                                        </div>
                                                        <span className="text-sm font-bold text-blue-500 font-mono">{tieringRules.move_to_cold} Days</span>
                                                    </div>
                                                    <input type="range" min="30" max="365" value={tieringRules.move_to_cold} onChange={e => setTieringRules({ ...tieringRules, move_to_cold: parseInt(e.target.value) })} className="w-full h-1 bg-slate-800 rounded-lg appearance-none cursor-pointer accent-blue-500" />
                                                </div>

                                                <button onClick={saveTieringRules} className="w-full py-2 bg-slate-800 hover:bg-slate-700 text-white font-bold text-[10px] rounded border border-white/10 transition-colors uppercase tracking-widest">
                                                    Commit Archival Policy
                                                </button>
                                            </div>
                                        </div>

                                        <div className="bg-slate-900/50 rounded-lg border border-white/5 overflow-hidden">
                                            <table className="w-full text-left text-[10px] border-collapse">
                                                <thead className="bg-slate-950 text-slate-500 font-mono uppercase">
                                                    <tr>
                                                        <th className="p-4 border-b border-white/10">Reference</th>
                                                        <th className="p-4 border-b border-white/10">Status</th>
                                                        <th className="p-4 border-b border-white/10">Temperature</th>
                                                        <th className="p-4 border-b border-white/10 text-right">Proj. Savings</th>
                                                    </tr>
                                                </thead>
                                                <tbody className="text-slate-400 font-mono">
                                                    {files.map((file, i) => {
                                                        const temp = Math.random();
                                                        return (
                                                            <tr key={file.id} className="hover:bg-white/5 border-b border-white/5 last:border-0 transition-colors group">
                                                                <td className="p-4"><span className="text-white font-bold">{file.name}</span></td>
                                                                <td className="p-4 uppercase">
                                                                    <span className={`px-2 py-0.5 rounded-full border text-[8px] ${temp > 0.7 ? 'border-orange-500/30 text-orange-500' : temp > 0.3 ? 'border-cyan-500/30 text-cyan-500' : 'border-blue-500/30 text-blue-500'}`}>
                                                                        {temp > 0.7 ? 'L1_HOT' : temp > 0.3 ? 'L2_WARM' : 'L3_COLD'}
                                                                    </span>
                                                                </td>
                                                                <td className="p-4"><EntropyBar temperature={temp} /></td>
                                                                <td className="p-4 text-right text-emerald-500">-${(Math.random() * 2).toFixed(2)}/mo</td>
                                                            </tr>
                                                        );
                                                    })}
                                                </tbody>
                                            </table>
                                        </div>
                                    </div>

                                    <div className="space-y-6">
                                        <div className="bg-slate-900 border border-white/5 p-6 rounded-lg backdrop-blur-sm">
                                            <h4 className="text-[10px] font-mono text-slate-500 uppercase mb-4 flex items-center gap-2">
                                                <TrendingDown size={14} className="text-emerald-500" />
                                                Economic Projection
                                            </h4>
                                            <div className="space-y-4">
                                                <div>
                                                    <div className="text-[9px] text-slate-600 uppercase mb-1">Estimated Savings</div>
                                                    <div className="text-2xl font-black text-white">$242.40 <span className="text-[10px] text-slate-500 font-normal">/ YEAR</span></div>
                                                </div>
                                                <div className="p-3 bg-emerald-500/5 border border-emerald-500/10 rounded text-[9px] text-emerald-500 leading-relaxed italic">
                                                    "Hive analyzed 4.2M access patterns. Moving cold objects to L3 reduces storage TCO by 62%."
                                                </div>
                                            </div>
                                        </div>
                                        <div className="bg-slate-900 border border-white/5 p-6 rounded-lg backdrop-blur-sm">
                                            <h4 className="text-[10px] font-mono text-slate-500 uppercase mb-4 flex items-center gap-2">
                                                <DollarSign size={14} className="text-hive-cyan" />
                                                Cost Structure
                                            </h4>
                                            <div className="space-y-3">
                                                <div className="flex justify-between text-[10px] font-mono">
                                                    <span className="text-slate-500">L1 NVMe (Fast)</span>
                                                    <span className="text-white">$0.20/GB</span>
                                                </div>
                                                <div className="flex justify-between text-[10px] font-mono">
                                                    <span className="text-slate-500">L2 SSD (Warm)</span>
                                                    <span className="text-white">$0.08/GB</span>
                                                </div>
                                                <div className="flex justify-between text-[10px] font-mono">
                                                    <span className="text-slate-500">L3 Glacier (Cold)</span>
                                                    <span className="text-white">$0.004/GB</span>
                                                </div>
                                            </div>
                                        </div>
                                    </div>
                                </div>
                            </div>
                        )}
                    </div>
                </div>
            </div>
        </div>
    );
};

export default Storage;

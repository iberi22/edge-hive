
import React, { useState, useEffect } from 'react';
import { 
  FileText, ShieldCheck, Clock, Hash, Database, Search, 
  RefreshCw, CheckCircle2, ChevronRight, Lock, Binary, ArrowRight
} from 'lucide-react';
import { LedgerBlock } from '../types';

const Ledger: React.FC = () => {
    const [blocks, setBlocks] = useState<LedgerBlock[]>([
        { id: 'B-4021', timestamp: '2024-05-20T14:42:01', operation: 'UPDATE', record_id: 'person:tobie', hash: 'e3b0c44298fc1c14', prev_hash: 'c89324a...', verified_by: ['HN-01', 'HN-02', 'HN-05'] },
        { id: 'B-4020', timestamp: '2024-05-20T14:41:55', operation: 'CREATE', record_id: 'post:new_launch', hash: 'f8212e3a1102b1c', prev_hash: 'a2b112c...', verified_by: ['HN-01', 'HN-04'] },
        { id: 'B-4019', timestamp: '2024-05-20T14:40:12', operation: 'RELATE', record_id: 'wrote:0x921', hash: 'd92k112la8172c', prev_hash: 'z12281m...', verified_by: ['HN-02', 'HN-03'] },
    ]);

    return (
        <div className="space-y-8 animate-in fade-in duration-500 pb-12">
            <div className="flex flex-col md:flex-row items-start md:items-end justify-between gap-4 border-b border-white/5 pb-6">
                <div>
                    <div className="flex items-center gap-2 mb-2">
                        <FileText className="text-emerald-500" size={14} />
                        <span className="text-[10px] font-mono text-emerald-400 font-bold uppercase tracking-widest">SurrealDB Immutable Timeline</span>
                    </div>
                    <h1 className="text-4xl font-black text-white tracking-tighter uppercase">Crypto Ledger</h1>
                </div>
                <div className="flex gap-4">
                    <button className="px-4 py-2 bg-slate-900 border border-white/10 rounded text-[10px] font-bold text-white uppercase hover:border-emerald-500 transition-all flex items-center gap-2">
                        <ShieldCheck size={14} /> Verify Full Chain
                    </button>
                </div>
            </div>

            <div className="grid grid-cols-1 xl:grid-cols-4 gap-8">
                <div className="xl:col-span-3 space-y-6">
                    {blocks.map((block, i) => (
                        <div key={block.id} className="relative group">
                            {i < blocks.length - 1 && (
                                <div className="absolute left-6 top-full h-6 w-0.5 bg-gradient-to-b from-emerald-500/30 to-transparent"></div>
                            )}
                            <div className="bg-slate-900/40 border border-white/5 rounded-xl p-6 group-hover:border-emerald-500/30 transition-all backdrop-blur-sm relative overflow-hidden">
                                <div className="absolute top-0 right-0 p-4 opacity-5 text-emerald-500">
                                    <Lock size={60} />
                                </div>
                                <div className="flex flex-col md:flex-row gap-6">
                                    <div className="flex-shrink-0 flex flex-col items-center">
                                        <div className="w-12 h-12 rounded-2xl bg-slate-950 border border-white/10 flex items-center justify-center text-emerald-500 shadow-neon-cyan">
                                            <Binary size={20} />
                                        </div>
                                        <span className="text-[9px] font-mono text-slate-600 mt-2 uppercase">{block.id}</span>
                                    </div>
                                    
                                    <div className="flex-1 grid grid-cols-1 md:grid-cols-3 gap-6">
                                        <div className="space-y-2">
                                            <div className="flex items-center gap-2">
                                                <span className={`px-2 py-0.5 rounded text-[8px] font-bold uppercase 
                                                    ${block.operation === 'UPDATE' ? 'bg-blue-500/10 text-blue-400' : 'bg-emerald-500/10 text-emerald-400'}`}>
                                                    {block.operation}
                                                </span>
                                                <span className="text-xs font-bold text-white font-mono">{block.record_id}</span>
                                            </div>
                                            <div className="text-[9px] text-slate-500 flex items-center gap-2 font-mono">
                                                <Clock size={10} /> {new Date(block.timestamp).toLocaleString()}
                                            </div>
                                        </div>

                                        <div className="space-y-2 font-mono">
                                            <div>
                                                <span className="text-[8px] text-slate-600 uppercase block">Block Hash</span>
                                                <span className="text-[10px] text-slate-400 truncate block">{block.hash}...</span>
                                            </div>
                                            <div>
                                                <span className="text-[8px] text-slate-600 uppercase block">Quorum Consens</span>
                                                <div className="flex gap-1 mt-1">
                                                    {block.verified_by.map(v => (
                                                        <div key={v} className="w-1.5 h-1.5 rounded-full bg-emerald-500 shadow-neon-cyan" title={v}></div>
                                                    ))}
                                                </div>
                                            </div>
                                        </div>

                                        <div className="flex items-center justify-end">
                                            <button className="flex items-center gap-2 text-[10px] font-bold text-slate-500 hover:text-white transition-colors uppercase">
                                                Inspect State <ArrowRight size={12} />
                                            </button>
                                        </div>
                                    </div>
                                </div>
                            </div>
                        </div>
                    ))}
                    
                    <button className="w-full py-4 border-2 border-dashed border-white/5 rounded-xl text-slate-600 hover:text-emerald-500 hover:border-emerald-500/30 hover:bg-white/5 transition-all text-xs font-bold uppercase tracking-widest">
                        Load Older Blocks
                    </button>
                </div>

                <div className="space-y-6">
                    <div className="bg-slate-950 border border-white/10 p-6 rounded-xl backdrop-blur-md">
                        <h4 className="text-[10px] font-bold text-white uppercase tracking-widest mb-6 flex items-center gap-2">
                            <ShieldCheck size={14} className="text-emerald-500" />
                            Veracity Engine
                        </h4>
                        <div className="space-y-6">
                            <div>
                                <div className="text-[9px] text-slate-500 uppercase mb-2">Global State Hash</div>
                                <div className="text-[10px] font-mono text-emerald-400 bg-emerald-500/5 p-2 rounded border border-emerald-500/20 break-all">
                                    0x92f...a2e811c02b339...
                                </div>
                            </div>
                            <div className="space-y-2">
                                <div className="flex items-center justify-between text-[10px] font-mono">
                                    <span className="text-slate-500">Atomic Depth</span>
                                    <span className="text-white">14,204</span>
                                </div>
                                <div className="flex items-center justify-between text-[10px] font-mono">
                                    <span className="text-slate-500">Merkle Root Status</span>
                                    <span className="text-emerald-500 font-bold">VERIFIED</span>
                                </div>
                            </div>
                        </div>
                    </div>

                    <div className="bg-slate-900 border border-white/5 p-6 rounded-xl relative group">
                        <h4 className="text-xs font-bold text-white uppercase tracking-widest mb-4 flex items-center gap-2">
                           <Database size={16} className="text-hive-cyan" />
                           Node Consensus
                        </h4>
                        <div className="grid grid-cols-4 gap-2">
                            {Array.from({ length: 12 }).map((_, i) => (
                                <div key={i} className="w-full pt-[100%] rounded-sm bg-emerald-500/40 shadow-neon-cyan animate-pulse"></div>
                            ))}
                        </div>
                        <p className="mt-4 text-[9px] text-slate-500 font-mono italic uppercase">
                            "Raft leader HN-01 confirmed 100% agreement on block 0x4f22ae"
                        </p>
                    </div>
                </div>
            </div>
        </div>
    );
};

export default Ledger;

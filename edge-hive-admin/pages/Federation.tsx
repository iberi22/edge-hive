
import React, { useState } from 'react';
import { 
  Building2, Globe, ShieldCheck, Users, Briefcase, Plus, Search, 
  ExternalLink, CheckCircle2, MoreVertical, LayoutGrid, Network, 
  ArrowRight, Key, ShieldAlert, Cpu, Box, RefreshCw
} from 'lucide-react';
import { useToast } from '../context/ToastContext';

const ProjectCard = ({ project }: any) => (
    <div className="bg-slate-900/40 border border-white/5 rounded-xl p-6 group hover:bg-slate-900/60 transition-all hover:border-hive-cyan/30">
        <div className="flex justify-between items-start mb-6">
            <div className="p-2.5 bg-slate-950 border border-white/10 rounded-lg text-hive-cyan group-hover:scale-110 transition-transform">
                <Box size={20} />
            </div>
            <div className={`px-2 py-0.5 rounded-full text-[8px] font-bold uppercase tracking-widest border 
                ${project.status === 'active' ? 'bg-emerald-500/10 text-emerald-500 border-emerald-500/20' : 'bg-slate-800 text-slate-500 border-white/5'}`}>
                {project.status}
            </div>
        </div>
        <h4 className="text-sm font-bold text-white uppercase tracking-tighter mb-1">{project.name}</h4>
        <div className="text-[10px] font-mono text-slate-500 uppercase mb-4">{project.region}</div>
        
        <div className="flex items-center justify-between pt-4 border-t border-white/5">
             <div className="flex -space-x-2">
                {[1, 2, 3].map(i => (
                    <div key={i} className="w-5 h-5 rounded-full bg-slate-800 border border-slate-950 flex items-center justify-center text-[8px] font-bold text-slate-500">U{i}</div>
                ))}
             </div>
             <button className="text-slate-500 hover:text-white transition-colors"><MoreVertical size={14}/></button>
        </div>
    </div>
);

const Federation: React.FC = () => {
    // Fix: useToast returns the toast object directly
    const toast = useToast();
    const [orgName] = useState("CYBERMESH_LABS_INC");
    const [projects] = useState([
        { id: '1', name: 'Core_Prod', region: 'AWS us-east-1', status: 'active' },
        { id: '2', name: 'Sentinel_Ingress', region: 'Cloudflare Edge', status: 'active' },
        { id: '3', name: 'Archive_Store', region: 'GCP europe-west3', status: 'hibernating' },
    ]);

    const handleSync = () => {
        toast.info("Federating Organization Identity with Global Mesh...", "Identity Sync");
        setTimeout(() => {
            toast.success("Identity synchronization complete. 42 users imported from Azure AD.");
        }, 2000);
    };

    return (
        <div className="space-y-8 animate-in fade-in duration-500 pb-12">
            <div className="flex flex-col md:flex-row items-start md:items-end justify-between gap-4 border-b border-white/5 pb-6">
                <div>
                    <div className="flex items-center gap-2 mb-2">
                        <Building2 className="text-hive-cyan" size={14} />
                        <span className="text-[10px] font-mono text-slate-500 font-bold uppercase tracking-widest">Enterprise Federation Hub</span>
                    </div>
                    <h1 className="text-4xl font-black text-white tracking-tighter uppercase">{orgName}</h1>
                </div>
                <div className="flex gap-4">
                    <button 
                        onClick={handleSync}
                        className="px-4 py-2 bg-slate-900 border border-white/10 rounded text-[10px] font-bold text-white uppercase hover:border-hive-cyan transition-all flex items-center gap-2"
                    >
                        <RefreshCw size={14} /> Sync Identity
                    </button>
                    <button className="px-4 py-2 bg-hive-orange text-black font-bold text-[10px] rounded shadow-neon-orange uppercase flex items-center gap-2">
                        <Plus size={14} /> New Project
                    </button>
                </div>
            </div>

            <div className="grid grid-cols-1 lg:grid-cols-3 gap-8">
                <div className="lg:col-span-2 space-y-8">
                    <div className="grid grid-cols-1 md:grid-cols-2 xl:grid-cols-3 gap-6">
                        {projects.map(p => <ProjectCard key={p.id} project={p} />)}
                        <button className="border-2 border-dashed border-white/5 rounded-xl p-6 flex flex-col items-center justify-center text-slate-600 hover:text-hive-cyan hover:border-hive-cyan/50 hover:bg-white/5 transition-all group">
                             <Plus size={24} className="mb-2 group-hover:scale-125 transition-transform" />
                             <span className="text-[10px] font-bold uppercase tracking-widest">Add Shard Project</span>
                        </button>
                    </div>

                    <div className="bg-slate-900/40 border border-white/5 rounded-xl p-8 backdrop-blur-md relative overflow-hidden group">
                        <div className="absolute top-0 right-0 p-12 text-hive-cyan opacity-5 pointer-events-none group-hover:rotate-12 transition-transform">
                             <Network size={200} />
                        </div>
                        <h3 className="text-xs font-bold text-white uppercase tracking-widest flex items-center gap-2 mb-8 border-b border-white/5 pb-4">
                            <Users size={16} className="text-hive-cyan" />
                            Directory Federation
                        </h3>
                        <div className="space-y-6">
                             <div className="flex items-center justify-between p-4 bg-slate-950/80 border border-white/5 rounded-xl hover:border-white/10 transition-colors">
                                <div className="flex items-center gap-4">
                                    <div className="p-2 bg-blue-500/10 rounded text-blue-500 border border-blue-500/20">
                                        <Globe size={24} />
                                    </div>
                                    <div>
                                        <div className="text-sm font-bold text-white">Microsoft Entra ID (Azure AD)</div>
                                        <div className="text-[10px] font-mono text-slate-500 uppercase">Status: Connected • 14.2k Users</div>
                                    </div>
                                </div>
                                <div className="flex items-center gap-2">
                                    <CheckCircle2 size={16} className="text-emerald-500" />
                                    <span className="text-[9px] font-bold text-emerald-500 uppercase">Synchronized</span>
                                </div>
                             </div>

                             <div className="flex items-center justify-between p-4 bg-slate-950/80 border border-white/5 rounded-xl hover:border-white/10 transition-colors">
                                <div className="flex items-center gap-4">
                                    <div className="p-2 bg-white/5 rounded text-slate-400 border border-white/10">
                                        <Building2 size={24} />
                                    </div>
                                    <div>
                                        <div className="text-sm font-bold text-white">Okta Federated Mesh</div>
                                        <div className="text-[10px] font-mono text-slate-500 uppercase">Status: Not Configured</div>
                                    </div>
                                </div>
                                <button className="text-[10px] font-bold text-hive-orange hover:underline uppercase">Link Domain</button>
                             </div>
                        </div>
                    </div>
                </div>

                <div className="space-y-6">
                    <div className="bg-slate-900/40 border border-white/5 p-6 rounded-xl backdrop-blur-sm">
                        <h4 className="text-[10px] font-bold text-white uppercase tracking-widest mb-6 flex items-center gap-2 border-b border-white/5 pb-4">
                            <ShieldAlert size={14} className="text-red-500" />
                            Security Posture
                        </h4>
                        <div className="space-y-4">
                             <div className="p-3 bg-slate-950/50 border border-white/5 rounded text-[10px]">
                                <div className="flex justify-between text-slate-500 uppercase mb-2">
                                    <span>Global Threat Level</span>
                                    <span className="text-emerald-500">LOW</span>
                                </div>
                                <div className="h-1 bg-slate-800 rounded-full overflow-hidden">
                                    <div className="h-full bg-emerald-500 w-[15%]"></div>
                                </div>
                             </div>
                             <div className="p-3 bg-slate-950/50 border border-white/5 rounded text-[10px]">
                                <div className="flex justify-between text-slate-500 uppercase mb-2">
                                    <span>Identity Drift</span>
                                    <span className="text-hive-orange">NOMINAL</span>
                                </div>
                                <div className="h-1 bg-slate-800 rounded-full overflow-hidden">
                                    <div className="h-full bg-hive-orange w-[5%]"></div>
                                </div>
                             </div>
                        </div>
                    </div>

                    <div className="bg-slate-900/60 border border-purple-500/20 p-8 rounded-xl relative overflow-hidden group shadow-2xl">
                        <div className="absolute top-0 right-0 p-4 opacity-5 text-purple-500 pointer-events-none">
                            <Cpu size={80} />
                        </div>
                        <h4 className="text-xs font-bold text-white uppercase tracking-widest mb-4 flex items-center gap-2">
                           <ShieldCheck size={16} className="text-purple-400" />
                           Compliance Matrix
                        </h4>
                        <div className="space-y-3">
                            {['SOC2_TYPE_2', 'GDPR_EU_NODE', 'HIPAA_COMPLIANT'].map(c => (
                                <div key={c} className="flex items-center justify-between text-[10px] font-mono border-b border-white/5 pb-2 last:border-0">
                                    <span className="text-slate-400">{c}</span>
                                    <span className="text-emerald-500 font-bold">VERIFIED</span>
                                </div>
                            ))}
                        </div>
                        <button className="w-full mt-6 py-2 bg-slate-950 border border-white/10 rounded text-[10px] font-bold text-slate-400 hover:text-white uppercase transition-all">
                            Export Org Report
                        </button>
                    </div>
                </div>
            </div>
        </div>
    );
};

export default Federation;

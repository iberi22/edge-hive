
import React, { useState, useEffect } from 'react';
import { AreaChart, Area, XAxis, YAxis, CartesianGrid, Tooltip, ResponsiveContainer, BarChart, Bar, Cell } from 'recharts';
import { 
  Wallet, TrendingUp, AlertCircle, Calendar, Cloud, Globe, Cpu, 
  Zap, CreditCard, ArrowUpRight, DollarSign, PieChart, Info, 
  TrendingDown, Activity, Layers, Download
} from 'lucide-react';

const mockBillingData = [
  { date: '2024-01', cloudflare: 45, aws: 120, gcp: 80, azure: 30 },
  { date: '2024-02', cloudflare: 52, aws: 135, gcp: 75, azure: 35 },
  { date: '2024-03', cloudflare: 48, aws: 110, gcp: 95, azure: 40 },
  { date: '2024-04', cloudflare: 60, aws: 145, gcp: 110, azure: 45 },
  { date: '2024-05', cloudflare: 75, aws: 160, gcp: 125, azure: 55 },
];

const Billing: React.FC = () => {
  const [totalSpent, setTotalSpent] = useState(415.50);
  const [activeProvider, setActiveProvider] = useState<'all' | 'aws' | 'cloudflare' | 'gcp' | 'azure'>('all');

  const providerStats = [
    { name: 'Cloudflare', value: 75.20, color: '#f97316', icon: Globe },
    { name: 'AWS', value: 160.40, color: '#eab308', icon: Cloud },
    { name: 'GCP', value: 125.10, color: '#3b82f6', icon: Cpu },
    { name: 'Azure', value: 54.80, color: '#06b6d4', icon: Zap },
  ];

  return (
    <div className="space-y-8 animate-in fade-in duration-500 pb-12">
      <div className="flex flex-col md:flex-row items-start md:items-end justify-between gap-4 border-b border-white/5 pb-6">
          <div>
              <div className="flex items-center gap-2 mb-2">
                  <div className="w-2 h-2 rounded-full bg-emerald-500 shadow-neon-cyan animate-pulse"></div>
                  <span className="text-[10px] font-mono text-emerald-500 font-bold uppercase tracking-widest">Neural Billing Matrix Active</span>
              </div>
              <h1 className="text-4xl font-black text-white tracking-tighter uppercase">Billing Matrix</h1>
          </div>
          <div className="flex gap-4">
              <div className="text-right">
                  <div className="text-[9px] font-mono text-slate-500 uppercase">Aggregated MTD Spend</div>
                  <div className="text-2xl font-black text-white flex items-center gap-2">
                      <DollarSign size={20} className="text-hive-orange" /> {totalSpent.toFixed(2)}
                  </div>
              </div>
              <div className="pl-4 border-l border-white/5">
                 <button className="p-2 bg-slate-900 border border-white/10 rounded hover:bg-white/5 transition text-slate-400" title="Export Invoices"><Download size={20}/></button>
              </div>
          </div>
      </div>

      <div className="grid grid-cols-1 lg:grid-cols-3 gap-8">
        <div className="lg:col-span-2 space-y-8">
            <div className="bg-slate-900/40 border border-white/5 rounded-xl p-8 backdrop-blur-md relative overflow-hidden group">
                {/* Decoration */}
                <div className="absolute -bottom-10 -right-10 opacity-5 text-hive-orange rotate-12 group-hover:rotate-6 transition-transform">
                   <Wallet size={200} />
                </div>

                <div className="flex items-center justify-between mb-8 relative z-10">
                    <div>
                      <h3 className="text-xs font-bold text-white uppercase tracking-widest flex items-center gap-2">
                          <TrendingUp size={16} className="text-hive-cyan" />
                          Multi-Cloud Expenditure Trends
                      </h3>
                      <p className="text-[10px] font-mono text-slate-500 uppercase mt-1">Cross-Provider Consolidated View</p>
                    </div>
                    <div className="flex bg-slate-950 p-1 rounded-lg border border-white/10">
                         {['6M', '1Y', 'ALL'].map(t => (
                             <button key={t} className={`px-3 py-1 text-[10px] font-bold rounded transition-colors ${t === '6M' ? 'bg-white/10 text-white' : 'text-slate-500 hover:text-slate-300'}`}>{t}</button>
                         ))}
                    </div>
                </div>
                
                <div className="h-[300px] relative z-10">
                    <ResponsiveContainer width="100%" height="100%">
                        <AreaChart data={mockBillingData}>
                            <defs>
                                <linearGradient id="colorSpent" x1="0" y1="0" x2="0" y2="1">
                                    <stop offset="5%" stopColor="#f97316" stopOpacity={0.2}/>
                                    <stop offset="95%" stopColor="#f97316" stopOpacity={0}/>
                                </linearGradient>
                            </defs>
                            <CartesianGrid strokeDasharray="3 3" stroke="#1e293b" vertical={false} strokeOpacity={0.05} />
                            <XAxis dataKey="date" stroke="#475569" fontSize={10} fontFamily='monospace' axisLine={false} />
                            <YAxis stroke="#475569" fontSize={10} fontFamily='monospace' axisLine={false} tickLine={false} />
                            <Tooltip 
                                contentStyle={{ backgroundColor: '#020617', border: '1px solid rgba(255,255,255,0.1)', fontSize: '10px', fontFamily: 'monospace' }}
                                cursor={{ stroke: '#f97316', strokeWidth: 1 }}
                            />
                            <Area type="monotone" dataKey="aws" stackId="1" stroke="#eab308" fill="#eab308" fillOpacity={0.1} animationDuration={1000} />
                            <Area type="monotone" dataKey="cloudflare" stackId="1" stroke="#f97316" fill="#f97316" fillOpacity={0.1} animationDuration={1000} />
                            <Area type="monotone" dataKey="gcp" stackId="1" stroke="#3b82f6" fill="#3b82f6" fillOpacity={0.1} animationDuration={1000} />
                            <Area type="monotone" dataKey="azure" stackId="1" stroke="#06b6d4" fill="#06b6d4" fillOpacity={0.1} animationDuration={1000} />
                        </AreaChart>
                    </ResponsiveContainer>
                </div>
            </div>

            <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
                <div className="bg-slate-900/40 border border-white/5 rounded-xl p-6 backdrop-blur-md relative overflow-hidden">
                    <h3 className="text-[10px] font-bold text-slate-500 uppercase tracking-widest mb-6 flex justify-between">
                      Neural Forecast 
                      <Info size={12} className="opacity-40" />
                    </h3>
                    <div className="flex items-center gap-3 mb-4">
                        <div className="p-2 bg-purple-500/10 rounded-lg text-purple-500 border border-purple-500/20 shadow-[0_0_15px_rgba(168,85,247,0.2)]">
                            <PieChart size={24} />
                        </div>
                        <div>
                            <span className="text-2xl font-black text-white">~$580.40</span>
                            <span className="text-[10px] text-slate-500 font-mono ml-2 uppercase">Proj. End of Month</span>
                        </div>
                    </div>
                    <div className="space-y-2">
                        <div className="flex justify-between text-[9px] font-mono text-slate-500 uppercase">
                          <span>Usage Intensity</span>
                          <span>75% of Limit</span>
                        </div>
                        <div className="h-1.5 w-full bg-slate-850 rounded-full overflow-hidden">
                            <div className="h-full bg-purple-500 shadow-[0_0_10px_#a855f7] w-3/4"></div>
                        </div>
                    </div>
                </div>

                <div className="bg-slate-900/40 border border-white/5 rounded-xl p-6 backdrop-blur-md">
                    <h3 className="text-[10px] font-bold text-slate-500 uppercase tracking-widest mb-6">Autonomous Saving Plan</h3>
                    <div className="p-4 bg-emerald-500/5 border border-emerald-500/20 rounded-lg">
                       <div className="flex items-center gap-3 mb-2">
                          <TrendingDown size={16} className="text-emerald-500" />
                          <span className="text-[10px] font-bold text-emerald-500 uppercase tracking-widest">Optimized Delta</span>
                       </div>
                       <p className="text-[11px] text-slate-400 font-mono leading-relaxed italic mb-4">
                          "Moving 12 TB from Azure to R2 will reduce annual cost by $1,420.00."
                       </p>
                       <button className="text-[10px] font-bold text-emerald-400 hover:text-white uppercase tracking-widest underline decoration-dotted">Review Proposal</button>
                    </div>
                </div>
            </div>
        </div>

        <div className="space-y-6">
            <div className="bg-slate-900/40 border border-white/5 rounded-xl p-6 backdrop-blur-md">
                <h3 className="text-[10px] font-bold text-white uppercase tracking-widest mb-6 flex items-center gap-2">
                   <Layers size={14} className="text-hive-cyan" />
                   Cloud Distribution
                </h3>
                <div className="space-y-4">
                    {providerStats.map((p) => (
                        <div key={p.name} className="flex items-center justify-between p-4 bg-slate-950/50 border border-white/5 rounded-lg group hover:border-white/20 transition-all cursor-pointer">
                            <div className="flex items-center gap-4">
                                <div className="p-2 rounded bg-slate-900 border border-white/5" style={{ color: p.color }}>
                                  <p.icon size={20} />
                                </div>
                                <div>
                                  <span className="text-xs font-bold text-white uppercase tracking-tighter">{p.name}</span>
                                  <div className="text-[8px] text-slate-600 font-mono uppercase">API Key Linked</div>
                                </div>
                            </div>
                            <div className="text-right">
                                <span className="text-sm font-bold text-white">${p.value.toFixed(2)}</span>
                                <div className="text-[9px] text-slate-600 font-mono uppercase">{Math.round((p.value / totalSpent) * 100)}%</div>
                            </div>
                        </div>
                    ))}
                </div>
            </div>

            <div className="bg-slate-900 border border-white/5 p-8 rounded-xl relative overflow-hidden group shadow-2xl">
                 <div className="absolute top-0 right-0 p-4 opacity-10 text-hive-orange pointer-events-none">
                    <Activity size={80} className="animate-pulse" />
                 </div>
                 <h4 className="text-xs font-bold text-hive-orange uppercase tracking-widest mb-4 flex items-center gap-2">
                    <AlertCircle size={16} />
                    Economic Safeguard
                 </h4>
                 <div className="space-y-4">
                   <p className="text-[11px] text-slate-400 font-mono leading-relaxed uppercase">
                      "GCP Compute Engine budget (US-EAST) is at 92%. Automatic node migration to AWS Spot instances is ready."
                   </p>
                   <div className="bg-slate-950 p-4 rounded border border-white/5 font-mono text-[10px]">
                      <div className="flex justify-between mb-2">
                        <span className="text-slate-500">GCP Hourly:</span>
                        <span className="text-red-400">$0.124/hr</span>
                      </div>
                      <div className="flex justify-between">
                        <span className="text-slate-500">AWS Spot:</span>
                        <span className="text-emerald-400">$0.038/hr</span>
                      </div>
                   </div>
                   <button className="w-full py-3 bg-hive-orange text-black font-bold text-[10px] rounded shadow-neon-orange uppercase tracking-widest hover:scale-[1.02] active:scale-95 transition-transform">
                      Trigger Cost Migration
                   </button>
                 </div>
            </div>

            <div className="bg-slate-950/50 border border-white/5 p-6 rounded-xl">
               <div className="flex items-center gap-3 mb-4">
                  <CreditCard size={16} className="text-slate-500" />
                  <span className="text-[10px] font-bold text-white uppercase tracking-widest">Active Payment Mesh</span>
               </div>
               <div className="flex items-center justify-between text-[11px] font-mono">
                  <span className="text-slate-400">Card ending in 4242</span>
                  <span className="text-emerald-500 uppercase font-bold">Primary</span>
               </div>
            </div>
        </div>
      </div>
    </div>
  );
};

export default Billing;

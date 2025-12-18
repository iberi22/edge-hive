
import React, { useState, useEffect } from 'react';
import { 
  LayoutDashboard, Database, TerminalSquare, Settings, Cpu, Box, 
  LogOut, ShieldCheck, Users, HardDrive, Menu, X, Search, ListTodo,
  ChevronDown, Zap, Activity, ChevronRight, GitBranch, Shield, Eye,
  PlugZap, Wallet, Scale, Globe, Building2, ChevronUp, Radio, Ghost,
  Lock, Flame, FileText
} from 'lucide-react';
import { ViewState } from '../types';
import { StatusBadge } from '../components/StatusBadge';
import CommandPalette from '../components/CommandPalette';

interface DashboardLayoutProps {
  children: React.ReactNode;
  currentView: ViewState;
  onNavigate: (view: ViewState) => void;
}

const GlobalTicker: React.FC = () => {
    const [events, setEvents] = useState([
        "CHAOS: AI predicted 12.5% failure chance in Node:SGP-03",
        "LEDGER: Verified block 0x4f22ae using NIST Kyber-1024",
        "FEDERATION: synced project 'Hive_Core' with Okta Hub",
        "ONION: .onion address rotation complete (Next in 24h)",
        "VPN_MESH: New peer 'HN-05' authenticated via WireGuard"
    ]);

    useEffect(() => {
        const interval = setInterval(() => {
            setEvents(prev => {
                const newEvent = prev[0]; 
                return [...prev.slice(1), newEvent];
            });
        }, 4000);
        return () => clearInterval(interval);
    }, []);

    return (
        <div className="h-8 bg-slate-900 border-t border-white/10 px-4 flex items-center justify-between text-[10px] font-mono overflow-hidden whitespace-nowrap z-50">
            <div className="flex items-center gap-4 animate-marquee">
                {events.map((ev, i) => (
                    <div key={i} className="flex items-center gap-2">
                        <span className="text-hive-cyan">●</span>
                        <span className="text-slate-500 uppercase">{ev}</span>
                        <span className="text-slate-800 mx-4">|</span>
                    </div>
                ))}
            </div>
            <div className="bg-slate-900 pl-4 flex items-center gap-4 text-emerald-500 font-bold border-l border-white/5">
                <span>SYSTEM_RESILIENT</span>
                <span className="text-slate-500">TPS: 54.8k</span>
            </div>
        </div>
    );
};

const DashboardLayout: React.FC<DashboardLayoutProps> = ({ children, currentView, onNavigate }) => {
  const [isMobileMenuOpen, setIsMobileMenuOpen] = useState(false);
  const [isPaletteOpen, setIsPaletteOpen] = useState(false);
  const [isProjectMenuOpen, setIsProjectMenuOpen] = useState(false);
  const [selectedProject, setSelectedProject] = useState("Cybermesh_Prod");

  useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      if ((e.metaKey || e.ctrlKey) && e.key === 'k') {
        e.preventDefault();
        setIsPaletteOpen(prev => !prev);
      }
    };
    window.addEventListener('keydown', handleKeyDown);
    return () => window.removeEventListener('keydown', handleKeyDown);
  }, []);

  const NavItem = ({ view, icon: Icon, label, color = 'text-slate-500' }: { view: ViewState, icon: React.ElementType, label: string, color?: string }) => {
    const isActive = currentView === view;
    return (
      <button
        onClick={() => {
          onNavigate(view);
          setIsMobileMenuOpen(false);
        }}
        className={`w-full flex items-center gap-3 px-4 py-3 text-sm font-medium transition-all border-l-2
          ${isActive 
            ? 'border-hive-orange bg-white/5 text-white shadow-[inset_10px_0_20px_-10px_rgba(249,115,22,0.1)]' 
            : 'border-transparent text-slate-400 hover:text-white hover:bg-white/5'
          }
        `}
      >
        <Icon size={18} className={isActive ? 'text-hive-orange' : color} />
        <span className={isActive ? 'text-hive-orange' : ''}>{label}</span>
      </button>
    );
  };

  return (
    <div className="flex flex-col h-screen bg-slate-950 overflow-hidden text-slate-200 font-sans selection:bg-hive-orange/30">
      <CommandPalette isOpen={isPaletteOpen} onClose={() => setIsPaletteOpen(false)} onNavigate={onNavigate} />

      <div className="flex flex-1 overflow-hidden">
        {isMobileMenuOpen && <div className="fixed inset-0 bg-slate-950/80 backdrop-blur-sm z-40 md:hidden" onClick={() => setIsMobileMenuOpen(false)} />}
        
        <aside className={`fixed inset-y-0 left-0 z-50 w-64 bg-slate-900/95 backdrop-blur-xl border-r border-white/5 flex flex-col transition-transform duration-300 md:relative md:translate-x-0 ${isMobileMenuOpen ? 'translate-x-0 shadow-2xl' : '-translate-x-full'}`}>
            <div className="p-6 flex flex-col gap-4 border-b border-white/5">
                <div className="flex items-center justify-between">
                    <div className="flex items-center gap-3">
                        <div className="w-8 h-8 rounded bg-gradient-to-br from-hive-orange to-red-600 flex items-center justify-center shadow-neon-orange">
                            <Box size={20} className="text-white" />
                        </div>
                        <h1 className="font-bold tracking-tight text-white leading-tight uppercase">Edge Hive</h1>
                    </div>
                    <button className="md:hidden text-slate-400" onClick={() => setIsMobileMenuOpen(false)}><X size={20} /></button>
                </div>

                {/* Project Switcher */}
                <div className="relative">
                    <button 
                        onClick={() => setIsProjectMenuOpen(!isProjectMenuOpen)}
                        className="w-full bg-slate-950 border border-white/10 rounded-lg px-3 py-2 flex items-center justify-between group hover:border-hive-cyan/50 transition-all"
                    >
                        <div className="flex items-center gap-2 overflow-hidden">
                            <Globe size={14} className="text-hive-cyan shrink-0" />
                            <span className="text-xs font-bold text-slate-300 truncate">{selectedProject}</span>
                        </div>
                        {isProjectMenuOpen ? <ChevronUp size={14} className="text-slate-500" /> : <ChevronDown size={14} className="text-slate-500" />}
                    </button>
                    
                    {isProjectMenuOpen && (
                        <div className="absolute top-full left-0 right-0 mt-2 bg-slate-900 border border-white/10 rounded-xl shadow-2xl z-[60] overflow-hidden animate-in fade-in slide-in-from-top-2">
                            <div className="p-2 space-y-1">
                                {["Cybermesh_Prod", "Sentinel_Stage", "Core_API_Dev"].map(p => (
                                    <button 
                                        key={p}
                                        onClick={() => { setSelectedProject(p); setIsProjectMenuOpen(false); }}
                                        className={`w-full text-left px-3 py-2 rounded text-[10px] font-bold uppercase transition-colors ${selectedProject === p ? 'bg-hive-orange/10 text-hive-orange' : 'text-slate-500 hover:bg-white/5 hover:text-white'}`}
                                    >
                                        {p}
                                    </button>
                                ))}
                                <div className="pt-2 border-t border-white/5">
                                    <button 
                                        onClick={() => { onNavigate('federation'); setIsProjectMenuOpen(false); }}
                                        className="w-full text-left px-3 py-2 text-[10px] font-bold text-hive-cyan hover:bg-white/5 rounded uppercase flex items-center gap-2"
                                    >
                                        <Building2 size={12} /> Manage Org
                                    </button>
                                </div>
                            </div>
                        </div>
                    )}
                </div>
            </div>

            <nav className="flex-1 py-6 space-y-1 overflow-y-auto custom-scrollbar">
                <div className="px-4 text-[10px] font-mono text-slate-600 uppercase tracking-widest mb-2">Resilience & IA</div>
                <NavItem view="chaos-lab" icon={Flame} label="Chaos Lab" color="text-red-500" />
                <NavItem view="ledger" icon={FileText} label="Crypto Ledger" color="text-emerald-500" />
                <NavItem view="governance" icon={Scale} label="Autonomous Gov" />

                <div className="px-4 text-[10px] font-mono text-slate-600 uppercase tracking-widest mb-2 mt-8">Dark Mesh</div>
                <NavItem view="onion" icon={Ghost} label="Onion Nodes" />
                <NavItem view="vpn" icon={Lock} label="VPN Mesh (WG)" />

                <div className="px-4 text-[10px] font-mono text-slate-600 uppercase tracking-widest mb-2 mt-8">Platform Core</div>
                <NavItem view="dashboard" icon={LayoutDashboard} label="Overview" />
                <NavItem view="data" icon={Database} label="Data Browser" />
                <NavItem view="cache" icon={Zap} label="Hive Mind (Cache)" />
                <NavItem view="auth" icon={Users} label="Auth & RBAC" />
                
                <div className="px-4 text-[10px] font-mono text-slate-600 uppercase tracking-widest mb-2 mt-8">Orchestration</div>
                <NavItem view="deep-edge" icon={Radio} label="Deep Edge HW" />
                <NavItem view="integrations" icon={PlugZap} label="Integrations" />
                <NavItem view="tasks" icon={ListTodo} label="Global Tasks" />
                <NavItem view="settings" icon={Settings} label="Configuration" />
            </nav>
        </aside>

        <main className="flex-1 flex flex-col min-w-0 overflow-hidden relative">
            <header className="h-16 border-b border-white/5 flex items-center justify-between px-4 md:px-8 bg-slate-900/30 backdrop-blur-sm z-10 shrink-0">
                <div className="flex items-center gap-4">
                    <button className="md:hidden text-slate-400 p-2 rounded-lg" onClick={() => setIsMobileMenuOpen(true)}><Menu size={20} /></button>
                    <div className="flex items-center gap-2">
                        <ChevronRight size={14} className="text-hive-orange" />
                        <h2 className="text-sm font-mono text-slate-300 uppercase tracking-wider hidden sm:block">
                            {currentView === 'chaos-lab' && 'Resilience // Neural Chaos Lab'}
                            {currentView === 'ledger' && 'Integrity // Cryptographic Ledger'}
                            {currentView === 'onion' && 'Dark Mesh // Onion Services'}
                        </h2>
                    </div>
                </div>
            </header>

            <div className="flex-1 overflow-x-hidden overflow-y-auto p-4 md:p-8 z-10 custom-scrollbar">
                <div className="max-w-7xl mx-auto h-full">{children}</div>
            </div>
        </main>
      </div>

      <GlobalTicker />
    </div>
  );
};

export default DashboardLayout;

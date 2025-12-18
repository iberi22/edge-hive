
import React, { useState, useEffect, useRef } from 'react';
import { GoogleGenAI } from "@google/genai";
import { 
  Cloud, Zap, Shield, Key, Terminal, Globe, Send, Cpu, BrainCircuit, 
  Info, CheckCircle2, RefreshCw, Layers, LayoutGrid, Box, Database, 
  Network, Server, BookOpen, Lock, Settings, ExternalLink, ShieldCheck,
  Play
} from 'lucide-react';
import { useToast } from '../context/ToastContext';

// Gemini Config - Using Gemini 3 Flash as requested
const ai = new GoogleGenAI({ apiKey: process.env.API_KEY });

const ProviderCard = ({ id, name, icon: Icon, color, status, onLink }: any) => {
  const [isLinking, setIsLinking] = useState(false);

  const handleLink = () => {
    setIsLinking(true);
    setTimeout(() => {
      onLink(id);
      setIsLinking(false);
    }, 1500);
  };

  return (
    <div className={`bg-slate-900/40 border rounded-xl p-6 transition-all group hover:bg-slate-900/60 relative overflow-hidden
        ${status === 'connected' ? 'border-hive-cyan/30 shadow-neon-cyan' : 'border-white/5'}`}>
        
        {status === 'connected' && (
          <div className="absolute top-0 right-0 p-1">
             <div className="w-1.5 h-1.5 rounded-full bg-emerald-500 shadow-neon-cyan animate-pulse"></div>
          </div>
        )}

        <div className="flex items-center justify-between mb-4">
            <div className={`p-3 rounded-lg bg-slate-950 border border-white/5 ${color} group-hover:scale-110 transition-transform`}>
                <Icon size={24} />
            </div>
            {status === 'connected' ? (
                <div className="flex items-center gap-1.5 px-2 py-0.5 rounded-full bg-emerald-500/10 border border-emerald-500/20 text-emerald-500 text-[8px] font-bold uppercase tracking-widest">
                    Verified
                </div>
            ) : (
                <button 
                  onClick={handleLink}
                  disabled={isLinking}
                  className="text-[10px] font-bold text-hive-orange hover:text-white uppercase tracking-widest flex items-center gap-2"
                >
                  {isLinking ? <RefreshCw size={10} className="animate-spin" /> : <Key size={10} />}
                  Link Account
                </button>
            )}
        </div>
        <h4 className="text-sm font-bold text-white uppercase tracking-tighter">{name}</h4>
        <div className="mt-4 space-y-2">
            <div className="flex justify-between text-[9px] font-mono text-slate-500 uppercase">
                <span>Active Nodes:</span>
                <span className="text-white font-bold">{status === 'connected' ? Math.floor(Math.random() * 10) + 2 : '0'}</span>
            </div>
            <div className="flex justify-between text-[9px] font-mono text-slate-500 uppercase">
                <span>Region Scope:</span>
                <span className="text-white font-bold">{status === 'connected' ? 'Global/All' : 'N/A'}</span>
            </div>
        </div>
        
        {status === 'connected' && (
          <div className="mt-4 pt-4 border-t border-white/5 flex gap-2">
             <button className="flex-1 py-1 bg-slate-950 border border-white/5 rounded text-[8px] font-bold text-slate-400 uppercase hover:text-white">API Keys</button>
             <button className="flex-1 py-1 bg-slate-950 border border-white/5 rounded text-[8px] font-bold text-slate-400 uppercase hover:text-white">OAuth</button>
          </div>
        )}
    </div>
  );
};

const NeuralArchitectAgent = () => {
    // Fix: useToast returns the toast object directly
    const toast = useToast();
    const [messages, setMessages] = useState<{ role: 'user' | 'ai', text: string }[]>([
        { role: 'ai', text: 'Neural Architect v3.0 Online. I have indexed the complete CLI manuals for AWS, GCP, Azure and Cloudflare. I can provision nodes, configure VPCs, and setup global load balancing. Shadow CLI is active: I can execute my own suggestions upon confirmation.' }
    ]);
    const [input, setInput] = useState("");
    const [isTyping, setIsTyping] = useState(false);
    const [showManuals, setShowManuals] = useState(false);
    const scrollRef = useRef<HTMLDivElement>(null);

    useEffect(() => {
        if (scrollRef.current) scrollRef.current.scrollTop = scrollRef.current.scrollHeight;
    }, [messages, isTyping]);

    const handleSend = async () => {
        if (!input.trim() || isTyping) return;
        const userText = input;
        setInput("");
        setMessages(prev => [...prev, { role: 'user', text: userText }]);
        setIsTyping(true);

        try {
            const result = await ai.models.generateContent({
                model: 'gemini-3-flash-preview',
                contents: userText,
                config: {
                    systemInstruction: `You are the High-Level Neural Architect for Edge Hive. 
                    Your mission: Orquestrate Multi-Cloud infrastructure.
                    Capability: You generate Terraform HCL and CLI commands. 
                    IMPORTANT: If you provide a command, format it clearly in a code block. 
                    Tone: Technical, Cyberpunk. 
                    Assume you can call internal 'Shadow CLI' functions to apply changes.`
                }
            });
            setMessages(prev => [...prev, { role: 'ai', text: result.text || "Connection timeout." }]);
        } catch (e) {
            setMessages(prev => [...prev, { role: 'ai', text: "Cognitive failure. Check API_KEY." }]);
        } finally {
            setIsTyping(false);
        }
    };

    const handleShadowExecute = () => {
        toast.info("Initializing Shadow CLI Session...", "Agentic Control");
        setTimeout(() => {
            toast.success("Command context synchronized and executed.", "Infrastructure Updated");
        }, 2000);
    };

    return (
        <div className="bg-slate-900/60 border border-purple-500/20 rounded-xl p-8 backdrop-blur-md flex flex-col h-[700px] relative overflow-hidden group shadow-2xl">
            <div className="absolute top-0 right-0 p-12 text-purple-500/5 pointer-events-none group-hover:text-purple-500/10 transition-all">
                <Network size={400} />
            </div>

            <div className="relative z-10 flex flex-col h-full">
                <div className="flex items-center justify-between mb-8 border-b border-white/5 pb-6">
                    <div className="flex items-center gap-3">
                        <div className="w-12 h-12 rounded-2xl bg-purple-500/20 border border-purple-500/40 flex items-center justify-center shadow-[0_0_20px_rgba(168,85,247,0.3)] group-hover:shadow-neon-orange transition-all">
                            <BrainCircuit size={24} className="text-purple-400" />
                        </div>
                        <div>
                            <h3 className="text-sm font-bold text-white uppercase tracking-widest flex items-center gap-2">
                                Neural Architect v3
                                <span className="bg-purple-600 text-white text-[8px] px-1.5 py-0.5 rounded-full uppercase">Flash-Core</span>
                            </h3>
                            <div className="flex items-center gap-2">
                                <div className="w-1.5 h-1.5 bg-emerald-500 rounded-full animate-pulse shadow-neon-cyan"></div>
                                <span className="text-[9px] font-mono text-slate-500 uppercase">Knowledge Base: 4 Manuals / 1.2M Tokens</span>
                            </div>
                        </div>
                    </div>
                    <div className="flex gap-2">
                        <button 
                          onClick={() => setShowManuals(!showManuals)}
                          className={`p-2 rounded border transition-all ${showManuals ? 'bg-purple-500 text-black border-purple-500' : 'bg-slate-950 border-white/10 text-slate-500 hover:text-white'}`}
                        >
                          <BookOpen size={16} />
                        </button>
                        <button className="p-2 bg-slate-950 border border-white/10 rounded text-slate-500 hover:text-white transition"><Settings size={16} /></button>
                    </div>
                </div>

                <div className="flex-1 flex overflow-hidden gap-6">
                    <div className={`flex-1 flex flex-col transition-all duration-300 ${showManuals ? 'w-2/3' : 'w-full'}`}>
                        <div ref={scrollRef} className="flex-1 overflow-y-auto space-y-6 mb-6 custom-scrollbar pr-4">
                            {messages.map((m, i) => (
                                <div key={i} className={`flex ${m.role === 'user' ? 'justify-end' : 'justify-start'}`}>
                                    <div className={`max-w-[90%] p-5 rounded-2xl text-xs leading-relaxed font-mono relative
                                        ${m.role === 'user' ? 'bg-hive-orange text-black font-bold shadow-neon-orange' : 'bg-slate-950/80 border border-white/10 text-slate-300'}
                                    `}>
                                        <div className="whitespace-pre-wrap">{m.text}</div>
                                        {m.role === 'ai' && m.text.includes('```') && (
                                            <button 
                                                onClick={handleShadowExecute}
                                                className="mt-4 flex items-center gap-2 px-3 py-1 bg-purple-600 text-white rounded text-[9px] font-bold uppercase tracking-widest shadow-lg hover:scale-105 transition-transform"
                                            >
                                                <Play size={10} fill="currentColor" /> Apply via Shadow CLI
                                            </button>
                                        )}
                                        {m.role === 'ai' && (
                                          <div className="absolute -left-2 top-4 w-4 h-4 bg-slate-950 border-l border-t border-white/10 rotate-[-45deg]"></div>
                                        )}
                                    </div>
                                </div>
                            ))}
                            {isTyping && (
                                <div className="flex justify-start">
                                    <div className="bg-slate-950/50 border border-purple-500/20 p-4 rounded-2xl text-purple-400 italic text-[10px] animate-pulse flex items-center gap-3">
                                        <RefreshCw size={12} className="animate-spin" />
                                        Architect is synthesizing CLI commands...
                                    </div>
                                </div>
                            )}
                        </div>

                        <div className="relative">
                            <div className="absolute -top-6 left-2 text-[8px] font-mono text-slate-600 uppercase tracking-widest flex items-center gap-2">
                               <ShieldCheck size={10} className="text-emerald-500" /> Secure Agentic Layer Active
                            </div>
                            <input 
                                type="text" 
                                value={input}
                                onChange={e => setInput(e.target.value)}
                                onKeyDown={e => e.key === 'Enter' && handleSend()}
                                placeholder="Command: 'Provision 3 nodes in AWS Ireland with gcloud auth'" 
                                className="w-full bg-slate-950 border border-white/10 rounded-xl pl-6 pr-14 py-4 text-xs text-white focus:outline-none focus:border-purple-500/50 shadow-2xl transition-all font-mono"
                            />
                            <button 
                              onClick={handleSend}
                              className="absolute right-3 top-1/2 -translate-y-1/2 p-2 bg-purple-600 text-white rounded-lg hover:scale-110 transition-transform shadow-neon-orange active:scale-95"
                            >
                                <Send size={18} />
                            </button>
                        </div>
                    </div>

                    {showManuals && (
                      <div className="w-1/3 bg-slate-950/80 border border-white/5 rounded-xl p-4 animate-in slide-in-from-right-4 duration-300 flex flex-col">
                         <h4 className="text-[10px] font-bold text-white uppercase tracking-widest mb-4 flex items-center gap-2">
                            <BookOpen size={14} className="text-purple-500" />
                            Knowledge Context
                         </h4>
                         <div className="space-y-3 overflow-y-auto flex-1 custom-scrollbar pr-2">
                            {[
                              { name: 'AWS CLI v2.0', status: 'Indexed' },
                              { name: 'Google Cloud SDK', status: 'Indexed' },
                              { name: 'Azure CLI (az)', status: 'Indexed' },
                              { name: 'Wrangler (Cloudflare)', status: 'Indexed' },
                              { name: 'Terraform v1.5', status: 'Indexed' },
                              { name: 'SurrealDB v2.1', status: 'Indexed' },
                            ].map(manual => (
                              <div key={manual.name} className="p-2 bg-slate-900 border border-white/5 rounded flex items-center justify-between group">
                                 <span className="text-[10px] text-slate-400 font-mono">{manual.name}</span>
                                 <div className="flex items-center gap-1">
                                    <CheckCircle2 size={10} className="text-emerald-500" />
                                    <span className="text-[8px] text-emerald-500/60 uppercase font-bold">{manual.status}</span>
                                 </div>
                              </div>
                            ))}
                         </div>
                      </div>
                    )}
                </div>
            </div>
        </div>
    );
};

const Integrations: React.FC = () => {
    // Fix: useToast returns the toast object directly
    const toast = useToast();
    const [connectedProviders, setConnectedProviders] = useState<string[]>(['cloudflare', 'aws']);

    const handleLink = (id: string) => {
      setConnectedProviders(prev => [...prev, id]);
      toast.success(`${id.toUpperCase()} Root account linked successfully.`, "Credential Ingress");
    };

    return (
        <div className="space-y-8 animate-in fade-in duration-500 pb-12">
            <div className="flex flex-col md:flex-row items-start md:items-end justify-between gap-4 border-b border-white/5 pb-6">
                <div>
                    <div className="flex items-center gap-2 mb-2">
                        <Layers className="text-hive-orange" size={14} />
                        <span className="text-[10px] font-mono text-slate-500 font-bold uppercase tracking-widest">Universal Multi-Cloud Control</span>
                    </div>
                    <h1 className="text-4xl font-black text-white tracking-tighter uppercase">Universal Hub</h1>
                </div>
            </div>

            <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6">
                <ProviderCard id="cloudflare" name="Cloudflare" icon={Globe} color="text-orange-500" status={connectedProviders.includes('cloudflare') ? 'connected' : 'offline'} onLink={handleLink} />
                <ProviderCard id="aws" name="Amazon AWS" icon={Cloud} color="text-yellow-500" status={connectedProviders.includes('aws') ? 'connected' : 'offline'} onLink={handleLink} />
                <ProviderCard id="gcp" name="Google Cloud" icon={Cpu} color="text-blue-500" status={connectedProviders.includes('gcp') ? 'connected' : 'offline'} onLink={handleLink} />
                <ProviderCard id="azure" name="Microsoft Azure" icon={Zap} color="text-cyan-400" status={connectedProviders.includes('azure') ? 'connected' : 'offline'} onLink={handleLink} />
            </div>

            <div className="grid grid-cols-1 xl:grid-cols-3 gap-8">
                <div className="xl:col-span-2">
                    <NeuralArchitectAgent />
                </div>
                <div className="space-y-6">
                    <div className="bg-slate-900/40 border border-white/5 p-6 rounded-xl backdrop-blur-sm shadow-xl">
                        <h4 className="text-[10px] font-bold text-white uppercase tracking-widest mb-6 flex items-center gap-2 border-b border-white/5 pb-4">
                            <Terminal size={14} className="text-hive-orange" />
                            Shadow CLI Audit Log
                        </h4>
                        <div className="space-y-4 font-mono text-[9px] h-[300px] overflow-y-auto custom-scrollbar">
                            {[
                                { t: '16:42:10', p: 'AWS', m: 'EC2: Running t4g.small in eu-central-1', s: 'done' },
                                { t: '16:42:12', p: 'CF', m: 'Worker: deploying global-auth-edge', s: 'done' },
                            ].map((log, i) => (
                                <div key={i} className="flex gap-3 text-slate-500 border-l border-white/10 pl-4 group hover:border-hive-orange transition-all">
                                    <span className="text-slate-700">[{log.t}]</span>
                                    <span className={`font-bold ${log.p === 'AWS' ? 'text-yellow-500' : 'text-blue-500'}`}>{log.p}</span>
                                    <span className="flex-1 text-slate-300">{log.m}</span>
                                    {log.s === 'done' ? <CheckCircle2 size={10} className="text-emerald-500" /> : <RefreshCw size={10} className="animate-spin text-hive-orange" />}
                                </div>
                            ))}
                        </div>
                    </div>
                </div>
            </div>
        </div>
    );
};

export default Integrations;

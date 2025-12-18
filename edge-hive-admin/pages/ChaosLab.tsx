
import React, { useState, useEffect } from 'react';
import {
    Flame, Zap, Activity, ShieldAlert, Cpu, Network, Radio,
    BrainCircuit, Send, RefreshCw, AlertTriangle, Eye, ShieldCheck
} from 'lucide-react';
import { GoogleGenAI } from "@google/genai";
import { useToast } from '../context/ToastContext';
import { ChaosExperiment } from '../types';

// Initialize the Google GenAI client
const ai = new GoogleGenAI({ apiKey: process.env.API_KEY || "stub_key" });

const ChaosLab: React.FC = () => {
    const toast = useToast();
    const [experiments, setExperiments] = useState<ChaosExperiment[]>([]);
    const [aiAnalysis, setAiAnalysis] = useState("");
    const [isAnalyzing, setIsAnalyzing] = useState(false);
    const [glitchIntensity, setGlitchIntensity] = useState(0);

    useEffect(() => {
        // Initial fetch
        const loadExperiments = async () => {
            const { tauriApi } = await import('../api/tauriClient');
            const data = await tauriApi.getExperiments();
            if (data.length > 0) setExperiments(data);
            // If empty/fail, keep default for demo or show empty
        };
        loadExperiments();

        // Listen for updates (requires listen to be exposed or used directly)
        // For now we simulate or rely on manual refresh/polling or add subscription later
        // As per plan, we just trigger run.
    }, []);

    const runAiAnalysis = async (exp: ChaosExperiment) => {
        setIsAnalyzing(true);
        setAiAnalysis("Calculating blast radius via Gemini Flash...");
        try {
            const response = await ai.models.generateContent({
                model: 'gemini-1.5-flash',
                contents: [{ parts: [{ text: `Analyze blast radius: ${exp.type} on ${exp.target} at ${exp.intensity}%` }] }]
            });
            setAiAnalysis((response as any).response.text() || "Cognitive failure.");
        } catch (e) {
            setAiAnalysis("Analysis node offline.");
        } finally {
            setIsAnalyzing(false);
        }
    };

    const runExperiment = async (id: string) => {
        const { tauriApi } = await import('../api/tauriClient');
        await tauriApi.runExperiment(id);

        setExperiments(prev => prev.map(e => e.id === id ? { ...e, status: 'running' } : e));
        setGlitchIntensity(40);
        toast.warning("Injecting systemic chaos into production grid...", "INCIDENT_START");

        // Optimistic UI update, backend emits events but we'll mock the completion flow visually until subscription is active
        setTimeout(() => {
            setExperiments(prev => prev.map(e => e.id === id ? { ...e, status: 'healing', impact_score: Math.floor(Math.random() * 40) + 60 } : e));
            setGlitchIntensity(10);
            toast.info("Autonomous Governance detected anomaly. Initiating auto-healing...", "SELF_HEAL");

            setTimeout(() => {
                setExperiments(prev => prev.map(e => e.id === id ? { ...e, status: 'completed' } : e));
                setGlitchIntensity(0);
                toast.success("System stability restored. Shards re-balanced.", "STABLE");
            }, 3000);
        }, 3000);
    };

    return (
        <div className={`space-y-8 animate-in fade-in duration-700 pb-12 transition-all ${glitchIntensity > 0 ? 'animate-glitch' : ''}`}>
            <div className="flex flex-col md:flex-row items-start md:items-end justify-between gap-4 border-b border-white/5 pb-6">
                <div>
                    <div className="flex items-center gap-2 mb-2">
                        <Flame className="text-red-500 animate-pulse" size={14} />
                        <span className="text-[10px] font-mono text-red-400 font-bold uppercase tracking-widest">Disaster Simulation Unit</span>
                    </div>
                    <h1 className="text-4xl font-black text-white tracking-tighter uppercase">Chaos Lab</h1>
                </div>
                <div className="flex gap-4">
                    <div className="text-right">
                        <div className="text-[9px] font-mono text-slate-500 uppercase">System Fragility</div>
                        <div className="text-sm font-bold text-red-500 font-mono">1.24% Entropy</div>
                    </div>
                </div>
            </div>

            <div className="grid grid-cols-1 lg:grid-cols-3 gap-8">
                <div className="lg:col-span-2 space-y-8">
                    <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
                        {experiments.map(exp => (
                            <div key={exp.id} className={`bg-slate-900/40 border rounded-xl p-6 transition-all group hover:bg-slate-900/60
                                ${exp.status === 'running' ? 'border-red-500/50 shadow-[0_0_20px_rgba(239,68,68,0.2)]' : 'border-white/5'}`}>
                                <div className="flex justify-between items-start mb-6">
                                    <div className={`p-3 rounded-lg bg-slate-950 border border-white/10 ${exp.status === 'running' ? 'text-red-500 animate-pulse' : 'text-slate-500'}`}>
                                        {exp.type === 'node_failure' ? <Cpu size={24} /> : <Network size={24} />}
                                    </div>
                                    <div className="flex flex-col items-end">
                                        <div className={`px-2 py-0.5 rounded text-[8px] font-bold uppercase border
                                            ${exp.status === 'running' ? 'bg-red-500/10 text-red-500 border-red-500/20' :
                                                exp.status === 'healing' ? 'bg-hive-orange/10 text-hive-orange border-hive-orange/20' :
                                                    'bg-slate-800 text-slate-500 border-white/5'}`}>
                                            {exp.status}
                                        </div>
                                        {exp.impact_score > 0 && <span className="text-[10px] font-mono text-red-400 mt-2">Impact: {exp.impact_score}%</span>}
                                    </div>
                                </div>
                                <h4 className="text-sm font-bold text-white uppercase tracking-tighter mb-1">{exp.target}</h4>
                                <p className="text-[10px] font-mono text-slate-500 uppercase mb-6">{exp.type.replace('_', ' ')}</p>

                                <div className="space-y-4">
                                    <div className="flex justify-between text-[8px] font-mono text-slate-600 uppercase">
                                        <span>Intensity</span>
                                        <span>{exp.intensity}%</span>
                                    </div>
                                    <div className="h-1 bg-slate-950 rounded-full overflow-hidden">
                                        <div className="h-full bg-red-600" style={{ width: `${exp.intensity}%` }}></div>
                                    </div>
                                </div>

                                <div className="mt-6 grid grid-cols-2 gap-2">
                                    <button
                                        onClick={() => runAiAnalysis(exp)}
                                        className="py-1.5 bg-slate-950 border border-white/10 rounded text-[9px] font-bold text-purple-400 hover:text-white uppercase transition-all flex items-center justify-center gap-2"
                                    >
                                        <BrainCircuit size={12} /> AI Analyze
                                    </button>
                                    <button
                                        onClick={() => runExperiment(exp.id)}
                                        disabled={exp.status !== 'idle' && exp.status !== 'completed'}
                                        className="py-1.5 bg-red-600 text-white rounded text-[9px] font-bold uppercase shadow-lg hover:bg-red-500 transition-all disabled:opacity-30"
                                    >
                                        Explode
                                    </button>
                                </div>
                            </div>
                        ))}
                    </div>

                    <div className="bg-slate-900/60 border border-purple-500/20 rounded-xl p-8 backdrop-blur-md relative overflow-hidden group min-h-[250px]">
                        <div className="absolute top-0 right-0 p-12 text-purple-500 opacity-5 pointer-events-none group-hover:rotate-6 transition-transform">
                            <BrainCircuit size={200} />
                        </div>
                        <h3 className="text-xs font-bold text-white uppercase tracking-widest flex items-center gap-2 mb-6">
                            <ShieldCheck size={16} className="text-purple-500" />
                            Architect Predictive Analysis
                        </h3>
                        <div className="font-mono text-xs leading-relaxed text-slate-300 whitespace-pre-wrap">
                            {isAnalyzing ? (
                                <div className="flex items-center gap-3 animate-pulse text-purple-400">
                                    <RefreshCw size={14} className="animate-spin" />
                                    Synthesizing systemic vulnerabilities...
                                </div>
                            ) : aiAnalysis || "Select an experiment to run predictive blast radius via Gemini Flash."}
                        </div>
                    </div>
                </div>

                <div className="space-y-6">
                    <div className="bg-slate-900 border border-white/5 p-6 rounded-xl backdrop-blur-sm">
                        <h4 className="text-[10px] font-bold text-white uppercase tracking-widest mb-6 flex items-center gap-2">
                            <Activity size={14} className="text-red-500" />
                            Incident Chronology
                        </h4>
                        <div className="space-y-4 max-h-[300px] overflow-y-auto custom-scrollbar font-mono text-[9px]">
                            {[
                                { t: '14:02:10', m: 'HEALING: Resharded person:posts to HN-05', s: 'success' },
                                { t: '14:01:45', m: 'CHAOS: Injected 500ms jitter to Ingress', s: 'warn' },
                                { t: '13:58:22', m: 'MONITOR: Latency deviation +40ms detected', s: 'info' },
                            ].map((l, i) => (
                                <div key={i} className="flex gap-3 text-slate-500 border-l border-white/5 pl-3 group hover:border-red-500 transition-all">
                                    <span className="text-slate-700">[{l.t}]</span>
                                    <span className="flex-1 text-slate-400 uppercase tracking-tighter">{l.m}</span>
                                </div>
                            ))}
                        </div>
                    </div>

                    <div className="bg-slate-950 border border-white/10 p-6 rounded-xl relative overflow-hidden group shadow-2xl">
                        <div className="absolute top-0 right-0 p-4 opacity-5 text-red-500">
                            <AlertTriangle size={80} />
                        </div>
                        <h4 className="text-xs font-bold text-white uppercase tracking-widest mb-4 flex items-center gap-2">
                            Safety Protocols
                        </h4>
                        <div className="space-y-3">
                            <div className="flex items-center justify-between text-[10px] font-mono border-b border-white/5 pb-2">
                                <span className="text-slate-500">Auto-Heal Engine</span>
                                <span className="text-emerald-500 font-bold uppercase">Active</span>
                            </div>
                            <div className="flex items-center justify-between text-[10px] font-mono border-b border-white/5 pb-2">
                                <span className="text-slate-500">Chaos Blocker (Prod)</span>
                                <span className="text-red-500 font-bold uppercase">Disabled</span>
                            </div>
                        </div>
                        <button className="w-full mt-6 py-2 bg-red-500/10 border border-red-500/20 text-red-500 rounded text-[9px] font-bold uppercase hover:bg-red-500 hover:text-white transition-all">
                            Emergency Full Stop
                        </button>
                    </div>
                </div>
            </div>
        </div>
    );
};

export default ChaosLab;

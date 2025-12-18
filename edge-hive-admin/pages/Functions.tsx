
import React, { useEffect, useState } from 'react';
import { Play, Pause, Terminal as TerminalIcon, Clock, Code, Settings, Save, Lock, Plus, Zap, Send, History, RotateCcw, GitCommit, Cpu, Database, Activity, BarChart3, ChevronRight, CheckCircle2, AlertTriangle, Loader2, GitPullRequest, Trash2, ArrowLeftRight } from 'lucide-react';
import { mockApi } from '../api';
import { EdgeFunction, LogEntry, FunctionInvocationResult, EdgeFunctionVersion } from '../types';
import Terminal from '../components/Terminal';
import { StatusBadge } from '../components/StatusBadge';
import { useToast } from '../context/ToastContext';
import Modal from '../components/Modal';

const DeploymentPipeline: React.FC<{ isOpen: boolean; onClose: () => void; onComplete: () => void }> = ({ isOpen, onClose, onComplete }) => {
    const [step, setStep] = useState(0);
    const [logs, setLogs] = useState<string[]>([]);

    const steps = [
        "FETCHING_TOOLCHAIN: rustc 1.75.0 (nightly)",
        "COMPILING_DEPENDENCIES: surrealdb-core, wasm-bindgen",
        "OPTIMIZING_LLVM: --target wasm32-unknown-unknown",
        "STRIPPING_SYMBOLS: wasm-opt -Oz -all",
        "HIVE_INGRESS: broadcasting to 12 edge nodes"
    ];

    useEffect(() => {
        if (isOpen) {
            setStep(0);
            setLogs(["[SYSTEM] Initializing WASM build pipeline..."]);
            let currentStep = 0;
            const interval = setInterval(() => {
                if (currentStep < steps.length) {
                    setLogs(prev => [...prev, `[BUILD] ${steps[currentStep]} ... OK`]);
                    setStep(prev => prev + 1);
                    currentStep++;
                } else {
                    clearInterval(interval);
                    setLogs(prev => [...prev, "[SUCCESS] Artifact deployed. SHA256: e3b0c442..."]);
                    setTimeout(() => onComplete(), 1000);
                }
            }, 800);
            return () => clearInterval(interval);
        }
    }, [isOpen]);

    return (
        <Modal isOpen={isOpen} onClose={onClose} title="WASM Build Pipeline" icon={Cpu}>
            <div className="space-y-6">
                <div className="flex justify-between items-center mb-2">
                    <span className="text-[10px] font-mono text-slate-500 uppercase">Progress: {Math.round((step / steps.length) * 100)}%</span>
                    <span className="text-[10px] font-mono text-hive-cyan">Target: hive_node_cluster</span>
                </div>
                <div className="h-2 w-full bg-slate-800 rounded-full overflow-hidden">
                    <div className="h-full bg-hive-orange transition-all duration-500 shadow-neon-orange" style={{ width: `${(step / steps.length) * 100}%` }}></div>
                </div>
                <div className="bg-black/80 rounded border border-white/5 p-4 font-mono text-[10px] text-emerald-500/80 h-48 overflow-y-auto custom-scrollbar">
                    {logs.map((log, i) => (
                        <div key={i} className="mb-1 flex gap-2">
                            <span className="text-slate-600">[{new Date().toLocaleTimeString()}]</span>
                            <span>{log}</span>
                        </div>
                    ))}
                    {step < steps.length && <div className="animate-pulse">_</div>}
                </div>
                <div className="flex justify-end pt-4">
                    <button onClick={onClose} className="text-[11px] font-bold text-slate-500 hover:text-white uppercase">Cancel Build</button>
                </div>
            </div>
        </Modal>
    );
};

const FunctionHistoryView: React.FC<{ versions: EdgeFunctionVersion[] }> = ({ versions }) => {
    // Fix: useToast returns the toast object directly
    const toast = useToast();
    const handleRollback = (id: string) => {
        toast.success(`Atomic Rollback Triggered`, `Restoring to version ${id.substring(0, 8)}`);
    };

    return (
        <div className="flex-1 p-6 space-y-4 overflow-auto custom-scrollbar bg-[#090c10]">
            <div className="flex items-center justify-between mb-4 border-b border-white/5 pb-4">
                <h3 className="text-xs font-bold text-white uppercase tracking-widest flex items-center gap-2">
                    <History size={14} className="text-hive-cyan" />
                    Immutable Version Control
                </h3>
                <span className="text-[9px] font-mono text-slate-600">Max History: 50 Snapshots</span>
            </div>
            {versions.length === 0 ? (
                <div className="text-center py-20 text-slate-700 text-[10px] uppercase font-mono italic">No previous versions found</div>
            ) : (
                <div className="space-y-3">
                    {versions.map((v, i) => (
                        <div key={v.id} className={`p-4 rounded-lg border flex items-center justify-between transition-all group
                            ${i === 0 ? 'bg-hive-cyan/5 border-hive-cyan/30' : 'bg-slate-900/40 border-white/5 hover:border-white/20'}
                        `}>
                            <div className="flex items-center gap-4">
                                <div className={`p-2 rounded-full ${i === 0 ? 'bg-hive-cyan/20 text-hive-cyan' : 'bg-slate-800 text-slate-600'}`}>
                                    <GitCommit size={16} />
                                </div>
                                <div>
                                    <div className="flex items-center gap-3">
                                        <span className="text-sm font-bold text-white font-mono">{v.id.substring(0, 8)}</span>
                                        {i === 0 && <span className="text-[9px] px-2 py-0.5 bg-hive-cyan text-black font-bold rounded-full uppercase">Current</span>}
                                    </div>
                                    <p className="text-[11px] text-slate-400 mt-0.5">{v.commit_message || "Automatic system snapshot"}</p>
                                    <div className="flex gap-3 mt-1">
                                        <span className="text-[9px] font-mono text-slate-600 uppercase">Author: {v.author}</span>
                                        <span className="text-[9px] font-mono text-slate-600 uppercase">{new Date(v.created_at).toLocaleString()}</span>
                                    </div>
                                </div>
                            </div>
                            {i !== 0 && (
                                <button
                                    onClick={() => handleRollback(v.id)}
                                    className="px-4 py-1.5 bg-slate-800 border border-white/10 rounded text-[10px] font-bold text-white uppercase hover:bg-hive-orange hover:text-black hover:border-hive-orange transition-all opacity-0 group-hover:opacity-100"
                                >
                                    Atomic Rollback
                                </button>
                            )}
                        </div>
                    ))}
                </div>
            )}
        </div>
    );
};

const WasmProfiler = () => {
    return (
        <div className="bg-slate-950/50 rounded border border-white/5 p-4 space-y-4">
            <div className="flex items-center justify-between">
                <div className="text-[10px] font-mono text-slate-500 uppercase">WASM Linear Memory</div>
                <div className="text-xs font-bold text-hive-cyan">64.2 MB / 256 MB</div>
            </div>
            <div className="h-1.5 w-full bg-slate-800 rounded-full overflow-hidden">
                <div className="h-full bg-hive-cyan w-[25%] shadow-neon-cyan"></div>
            </div>

            <div className="space-y-2 mt-4">
                <div className="text-[9px] font-mono text-slate-600 uppercase mb-2">Instruction Tracer (Waterfall)</div>
                <div className="flex h-4 rounded-sm overflow-hidden text-[8px] font-mono font-bold">
                    <div className="bg-orange-500 w-[15%] flex items-center justify-center text-black" title="JIT Warmup">JIT</div>
                    <div className="bg-hive-cyan w-[60%] flex items-center justify-center text-black" title="CPU Execution">COMPUTE</div>
                    <div className="bg-purple-600 w-[25%] flex items-center justify-center text-white" title="I/O Blocking">I/O</div>
                </div>
            </div>

            <div className="grid grid-cols-2 gap-4 pt-2">
                <div>
                    <div className="text-[9px] font-mono text-slate-600 uppercase mb-1">Cold Start</div>
                    <div className="text-sm font-bold text-emerald-500">12ms</div>
                </div>
                <div>
                    <div className="text-[9px] font-mono text-slate-600 uppercase mb-1">P99 Latency</div>
                    <div className="text-sm font-bold text-white">42ms</div>
                </div>
            </div>
        </div>
    );
};

const Functions: React.FC = () => {
    // Fix: useToast returns the toast object directly
    const toast = useToast();
    const [functions, setFunctions] = useState<EdgeFunction[]>([]);
    const [logs, setLogs] = useState<LogEntry[]>([]);
    const [selectedFnId, setSelectedFnId] = useState<string | null>(null);
    const [activeTab, setActiveTab] = useState<'logs' | 'source' | 'config' | 'test' | 'history'>('logs');
    const [testPayload, setTestPayload] = useState('{\n  "record": "person:tobie",\n  "action": "notify"\n}');
    const [invocationResult, setInvocationResult] = useState<FunctionInvocationResult | null>(null);
    const [isInvoking, setIsInvoking] = useState(false);
    const [isDeploying, setIsDeploying] = useState(false);
    const [versions, setVersions] = useState<EdgeFunctionVersion[]>([]);

    useEffect(() => {
        mockApi.getFunctions().then(data => {
            setFunctions(data);
            if (data.length > 0) setSelectedFnId(data[0].id);
        });
        mockApi.getLogs().then(setLogs);
    }, []);

    useEffect(() => {
        if (selectedFnId && activeTab === 'history') {
            // Simulated versions
            setVersions([
                { id: 'v128_e92', function_id: selectedFnId, version: '1.2.0', created_at: new Date().toISOString(), status: 'active', author: 'root', commit_message: 'Optimize SurrealQL fetch logic' },
                { id: 'v127_b41', function_id: selectedFnId, version: '1.1.9', created_at: new Date(Date.now() - 86400000).toISOString(), status: 'outdated', author: 'jaime_dev', commit_message: 'Add security headers to WASM' },
                { id: 'v126_a02', function_id: selectedFnId, version: '1.1.8', created_at: new Date(Date.now() - 172800000).toISOString(), status: 'outdated', author: 'root', commit_message: 'Initial edge implementation' },
            ]);
        }
    }, [selectedFnId, activeTab]);

    const selectedFn = functions.find(f => f.id === selectedFnId);

    const handleTestRun = async () => {
        if (!selectedFnId) return;
        setIsInvoking(true);
        try {
            const payload = JSON.parse(testPayload);
            const result = await mockApi.invokeFunction(selectedFnId, payload);
            setInvocationResult(result);
            toast.success('Execution success', `${result.time_ms}ms`);
        } catch (e) {
            toast.error("Invalid input JSON");
        } finally {
            setIsInvoking(false);
        }
    };

    const handleDeploy = () => {
        setIsDeploying(true);
    };

    const handleDeployComplete = () => {
        setIsDeploying(false);
        toast.success("Deployment successful", "WASM module is live");
        setActiveTab('logs');
    };

    return (
        <div className="grid grid-cols-1 lg:grid-cols-4 gap-6 h-auto md:h-[calc(100vh-8rem)]">

            <DeploymentPipeline
                isOpen={isDeploying}
                onClose={() => setIsDeploying(false)}
                onComplete={handleDeployComplete}
            />

            {/* List Sidebar */}
            <div className="lg:col-span-1 flex flex-col gap-4">
                <div className="flex items-center justify-between mb-2">
                    <h3 className="text-xs font-bold text-slate-500 uppercase tracking-widest flex items-center gap-2">
                        <TerminalIcon size={14} /> Edge Runtime
                    </h3>
                    <button className="p-1 hover:bg-white/10 rounded text-hive-orange transition">
                        <Plus size={16} />
                    </button>
                </div>

                <div className="space-y-2 overflow-y-auto pr-2 custom-scrollbar flex-1">
                    {functions.map(fn => (
                        <div
                            key={fn.id}
                            onClick={() => setSelectedFnId(fn.id)}
                            className={`p-4 rounded border cursor-pointer transition-all relative
                        ${selectedFnId === fn.id ? 'bg-slate-900 border-hive-orange/40 shadow-neon-orange' : 'bg-slate-900/40 border-white/5 hover:border-white/10'}
                    `}
                        >
                            <div className="flex justify-between items-start mb-2">
                                <span className="font-mono text-xs font-bold text-white tracking-tight">{fn.name}</span>
                                <StatusBadge status={fn.status === 'running' ? 'healthy' : 'warning'} pulse={fn.status === 'running'} />
                            </div>
                            <div className="flex justify-between items-center text-[9px] font-mono text-slate-500 uppercase">
                                <span>{fn.invocations.toLocaleString()} CALLS</span>
                                <span>{fn.lastRun}</span>
                            </div>
                        </div>
                    ))}
                </div>

                {selectedFn && (
                    <div className="mt-4 animate-in fade-in">
                        <h4 className="text-[10px] font-mono text-slate-600 uppercase mb-3 flex items-center gap-2">
                            <BarChart3 size={12} /> Runtime Health
                        </h4>
                        <WasmProfiler />
                    </div>
                )}
            </div>

            {/* Code & Editor Area */}
            <div className="lg:col-span-3 flex flex-col gap-4 min-h-[500px]">
                <div className="bg-slate-900/40 border border-white/5 rounded-lg p-4 flex items-center justify-between backdrop-blur-sm shadow-xl">
                    <div className="flex items-center gap-3">
                        <div className="w-8 h-8 rounded bg-slate-950 flex items-center justify-center border border-white/10">
                            <Code size={16} className="text-hive-cyan" />
                        </div>
                        <div>
                            <span className="text-[10px] font-mono text-slate-500 uppercase leading-none">Runtime Environment</span>
                            <h2 className="text-sm font-bold text-white tracking-wider uppercase">{selectedFn?.name || '---'}</h2>
                        </div>
                    </div>
                    <div className="flex gap-2">
                        <button
                            onClick={handleDeploy}
                            className="px-6 py-2 bg-hive-orange hover:bg-orange-600 text-black font-bold text-[10px] rounded transition shadow-neon-orange uppercase flex items-center gap-2"
                        >
                            <Save size={14} /> Commit & Deploy
                        </button>
                    </div>
                </div>

                <div className="flex-1 bg-slate-900/30 border border-white/5 rounded-lg overflow-hidden flex flex-col backdrop-blur-md">
                    <div className="flex bg-slate-950/80 border-b border-white/5 px-2">
                        {[
                            { id: 'logs', label: 'Console', icon: TerminalIcon },
                            { id: 'source', label: 'Source', icon: Code },
                            { id: 'test', label: 'Sandbox', icon: Zap },
                            { id: 'history', label: 'Commits', icon: History }
                        ].map(tab => (
                            <button
                                key={tab.id}
                                onClick={() => setActiveTab(tab.id as any)}
                                className={`px-6 py-3 text-[10px] font-bold uppercase tracking-widest border-b-2 transition-all flex items-center gap-2 whitespace-nowrap
                            ${activeTab === tab.id ? 'border-hive-orange text-white bg-white/5' : 'border-transparent text-slate-500 hover:text-slate-300'}`}
                            >
                                <tab.icon size={12} /> {tab.label}
                            </button>
                        ))}
                    </div>

                    <div className="flex-1 overflow-hidden flex flex-col relative">
                        {activeTab === 'logs' && (
                            <Terminal logs={logs} className="flex-1 rounded-none border-0 h-full" title={`runtime_audit >> ${selectedFn?.id}.wasm`} />
                        )}

                        {activeTab === 'history' && (
                            <FunctionHistoryView versions={versions} />
                        )}

                        {activeTab === 'source' && selectedFn && (
                            <div className="flex-1 relative bg-[#090c10] overflow-auto">
                                <div className="absolute top-0 left-0 bottom-0 w-12 bg-slate-950 border-r border-white/5 flex flex-col items-center pt-6 text-[10px] font-mono text-slate-700 select-none">
                                    {Array.from({ length: 30 }).map((_, i) => <div key={i} className="h-5">{i + 1}</div>)}
                                </div>
                                <textarea
                                    className="w-full h-full bg-transparent text-emerald-400/90 font-mono text-sm p-6 pl-16 focus:outline-none resize-none leading-5 selection:bg-white/10"
                                    value={selectedFn.source_code}
                                    spellCheck={false}
                                />
                            </div>
                        )}

                        {activeTab === 'test' && (
                            <div className="flex-1 p-8 grid grid-cols-1 lg:grid-cols-2 gap-8 bg-[#090c10] overflow-auto custom-scrollbar">
                                <div className="space-y-4">
                                    <h4 className="text-[10px] font-mono text-slate-500 uppercase">Input Payload (JSON)</h4>
                                    <textarea value={testPayload} onChange={e => setTestPayload(e.target.value)} className="w-full h-64 bg-slate-950 border border-white/10 rounded p-4 text-xs font-mono text-slate-400 focus:outline-none focus:border-hive-cyan/50 resize-none shadow-inner" />
                                    <button onClick={handleTestRun} disabled={isInvoking} className="w-full py-3 bg-hive-cyan hover:bg-cyan-600 text-black font-bold text-xs rounded transition flex items-center justify-center gap-3 shadow-neon-cyan">
                                        {isInvoking ? <RotateCcw size={16} className="animate-spin" /> : <Send size={16} />}
                                        Trigger WASM Runtime
                                    </button>
                                </div>

                                <div className="space-y-4">
                                    <h4 className="text-[10px] font-mono text-slate-500 uppercase">Output Stream</h4>
                                    <div className="w-full h-64 bg-slate-950 border border-white/10 rounded p-4 font-mono text-xs overflow-auto relative">
                                        {invocationResult ? (
                                            <div className="animate-in fade-in">
                                                <div className="flex items-center gap-4 mb-4 border-b border-white/5 pb-2 uppercase text-[10px]">
                                                    <span className="text-emerald-500 font-bold">Success</span>
                                                    <span className="text-slate-600">Runtime: {invocationResult.time_ms}ms</span>
                                                </div>
                                                <pre className="text-hive-cyan/80 whitespace-pre-wrap">{JSON.stringify(invocationResult.body, null, 2)}</pre>
                                            </div>
                                        ) : (
                                            <div className="absolute inset-0 flex items-center justify-center text-slate-800 text-[10px] uppercase tracking-widest font-mono">
                                                No active execution
                                            </div>
                                        )}
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

export default Functions;

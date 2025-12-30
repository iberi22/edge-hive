
import React, { useState, useEffect, useRef } from 'react';
import { listen } from '@tauri-apps/api/event';
import { invoke } from '@tauri-apps/api/tauri';
import { useToast } from '../context/ToastContext';

const LogViewer: React.FC = () => {
    const [logs, setLogs] = useState<string[]>([]);
    const logsEndRef = useRef<HTMLDivElement>(null);

    const scrollToBottom = () => {
        logsEndRef.current?.scrollIntoView({ behavior: "smooth" });
    };

    const toast = useToast();

    useEffect(() => {
        invoke('stream_logs');

        const unlistenLogs = listen<string>('log-message', (event) => {
            setLogs((prevLogs) => [...prevLogs, event.payload]);
        });

        const unlistenError = listen<string>('log-stream-error', (event) => {
            toast.error(`Log stream error: ${event.payload}`);
        });

        return () => {
            unlistenLogs.then(f => f());
            unlistenError.then(f => f());
        };
    }, []);

    useEffect(scrollToBottom, [logs]);

    return (
        <div className="bg-slate-900/40 border border-white/5 rounded-xl p-6 backdrop-blur-md">
            <h3 className="text-xs font-bold text-white uppercase tracking-widest flex items-center gap-2 mb-6">
                Real-Time Log Stream
            </h3>
            <div className="space-y-3 max-h-80 overflow-y-auto custom-scrollbar font-mono text-[10px] bg-slate-950 border-white/5 p-3 rounded">
                {logs.map((log, i) => (
                    <div key={i} className="text-slate-400">
                        {log}
                    </div>
                ))}
                <div ref={logsEndRef} />
            </div>
        </div>
    );
};

export default LogViewer;

import React, { useEffect, useRef } from 'react';
import { LogEntry } from '../types';

interface TerminalProps {
  logs: LogEntry[];
  title?: string;
  className?: string;
}

const Terminal: React.FC<TerminalProps> = ({ logs, title = "STD_OUT >> EDGE_HIVE_RUNTIME", className = "" }) => {
  const bottomRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    bottomRef.current?.scrollIntoView({ behavior: 'smooth' });
  }, [logs]);

  const getLevelColor = (level: LogEntry['level']) => {
    switch (level) {
      case 'ERROR': return 'text-red-500';
      case 'WARN': return 'text-hive-orange';
      case 'DEBUG': return 'text-purple-400';
      case 'INFO': return 'text-hive-cyan';
      default: return 'text-slate-300';
    }
  };

  return (
    <div className={`flex flex-col bg-hive-void border border-white/10 rounded-md overflow-hidden shadow-2xl ${className}`}>
      {/* Terminal Header */}
      <div className="flex items-center justify-between px-4 py-2 bg-slate-900 border-b border-white/5">
        <div className="flex items-center gap-2">
          <div className="w-3 h-3 rounded-full bg-red-500/20 border border-red-500/50"></div>
          <div className="w-3 h-3 rounded-full bg-yellow-500/20 border border-yellow-500/50"></div>
          <div className="w-3 h-3 rounded-full bg-green-500/20 border border-green-500/50"></div>
          <span className="ml-2 text-xs font-mono text-slate-500 uppercase tracking-widest">{title}</span>
        </div>
        <div className="text-[10px] text-slate-600 font-mono">BASH - 80x24</div>
      </div>

      {/* Terminal Body */}
      <div className="flex-1 p-4 overflow-y-auto font-mono text-xs md:text-sm space-y-1 h-64 md:h-96 scrollbar-thin scrollbar-thumb-slate-700 scrollbar-track-transparent">
        {logs.map((log) => (
          <div key={log.id} className="flex gap-3 hover:bg-white/5 p-0.5 rounded px-2 transition-colors">
            <span className="text-slate-600 shrink-0 w-24">[{log.timestamp.split('T')[1].split('.')[0]}]</span>
            <span className={`${getLevelColor(log.level)} font-bold shrink-0 w-16`}>{log.level}</span>
            <span className="text-slate-500 shrink-0 w-24">[{log.service}]</span>
            <span className="text-slate-300 break-all">{log.message}</span>
          </div>
        ))}
        <div ref={bottomRef} />
        
        {/* Blinking Cursor */}
        <div className="flex items-center gap-2 mt-2 px-2">
          <span className="text-hive-orange">root@edge-hive:~$</span>
          <span className="w-2 h-4 bg-hive-cyan animate-pulse"></span>
        </div>
      </div>
    </div>
  );
};

export default Terminal;
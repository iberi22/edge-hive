import React from 'react';

interface StatusBadgeProps {
  status: 'healthy' | 'warning' | 'error' | 'offline';
  pulse?: boolean;
}

export const StatusBadge: React.FC<StatusBadgeProps> = ({ status, pulse = true }) => {
  const colors = {
    healthy: 'bg-emerald-500 shadow-neon-cyan',
    warning: 'bg-hive-orange shadow-neon-orange',
    error: 'bg-red-500 shadow-[0_0_10px_rgba(239,68,68,0.5)]',
    offline: 'bg-slate-600',
  };

  const ringColors = {
    healthy: 'bg-emerald-500/20',
    warning: 'bg-orange-500/20',
    error: 'bg-red-500/20',
    offline: 'bg-slate-600/20',
  };

  return (
    <div className="relative flex items-center justify-center w-3 h-3">
      {pulse && (
        <span className={`absolute inline-flex h-full w-full rounded-full opacity-75 animate-ping ${ringColors[status]}`}></span>
      )}
      <span className={`relative inline-flex rounded-full h-2 w-2 ${colors[status]}`}></span>
    </div>
  );
};
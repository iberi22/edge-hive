import React from 'react';
import { RefreshCw } from 'lucide-react';

interface LoadingStateProps {
   message?: string;
}

export const LoadingState: React.FC<LoadingStateProps> = ({ message = "Loading Neural Matrix..." }) => {
   return (
      <div className="flex flex-col items-center justify-center p-12 h-64 w-full">
         <RefreshCw size={32} className="text-hive-cyan animate-spin mb-4" />
         <p className="text-[10px] font-mono text-slate-500 uppercase tracking-widest animate-pulse">
            {message}
         </p>
      </div>
   );
};

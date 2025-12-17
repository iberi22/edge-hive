import React, { createContext, useContext, useState, useCallback } from 'react';
import { X, CheckCircle, AlertTriangle, Info, Loader2, AlertOctagon } from 'lucide-react';

type ToastType = 'success' | 'error' | 'info' | 'loading' | 'warning';

interface Toast {
  id: string;
  type: ToastType;
  title?: string;
  message: string;
  duration?: number;
}

interface ToastContextProps {
  toast: {
    success: (message: string, title?: string) => void;
    error: (message: string, title?: string) => void;
    info: (message: string, title?: string) => void;
    warning: (message: string, title?: string) => void;
    loading: (message: string, title?: string) => string; // Returns ID to close later
    dismiss: (id: string) => void;
  };
}

const ToastContext = createContext<ToastContextProps | undefined>(undefined);

export const useToast = () => {
  const context = useContext(ToastContext);
  if (!context) {
    throw new Error('useToast must be used within a ToastProvider');
  }
  return context.toast;
};

const ToastItem: React.FC<{ toast: Toast; onDismiss: (id: string) => void }> = ({ toast, onDismiss }) => {
  const styles = {
    success: {
      border: 'border-emerald-500/50',
      bg: 'bg-slate-900/95',
      icon: <CheckCircle className="text-emerald-500" size={20} />,
      shadow: 'shadow-[0_0_15px_-3px_rgba(16,185,129,0.3)]',
      progress: 'bg-emerald-500',
    },
    error: {
      border: 'border-red-500/50',
      bg: 'bg-slate-900/95',
      icon: <AlertOctagon className="text-red-500" size={20} />,
      shadow: 'shadow-[0_0_15px_-3px_rgba(239,68,68,0.3)]',
      progress: 'bg-red-500',
    },
    warning: {
      border: 'border-hive-orange/50',
      bg: 'bg-slate-900/95',
      icon: <AlertTriangle className="text-hive-orange" size={20} />,
      shadow: 'shadow-[0_0_15px_-3px_rgba(249,115,22,0.3)]',
      progress: 'bg-hive-orange',
    },
    info: {
      border: 'border-blue-500/50',
      bg: 'bg-slate-900/95',
      icon: <Info className="text-blue-500" size={20} />,
      shadow: 'shadow-[0_0_15px_-3px_rgba(59,130,246,0.3)]',
      progress: 'bg-blue-500',
    },
    loading: {
      border: 'border-hive-cyan/50',
      bg: 'bg-slate-900/95',
      icon: <Loader2 className="text-hive-cyan animate-spin" size={20} />,
      shadow: 'shadow-[0_0_15px_-3px_rgba(6,182,212,0.3)]',
      progress: 'bg-hive-cyan',
    },
  };

  const style = styles[toast.type];

  // Auto dismiss logic for non-loading toasts
  React.useEffect(() => {
    if (toast.type !== 'loading') {
      const timer = setTimeout(() => {
        onDismiss(toast.id);
      }, toast.duration || 5000);
      return () => clearTimeout(timer);
    }
  }, [toast, onDismiss]);

  return (
    <div 
      className={`
        relative w-full md:w-96 overflow-hidden rounded-lg border ${style.border} ${style.bg} ${style.shadow}
        backdrop-blur-xl p-4 mb-3 transition-all duration-300 animate-in slide-in-from-right-full fade-in
        group cursor-pointer
      `}
      onClick={() => onDismiss(toast.id)}
    >
      <div className="flex items-start gap-3">
        <div className="shrink-0 mt-0.5">{style.icon}</div>
        <div className="flex-1 min-w-0">
          {toast.title && <h4 className="text-sm font-bold text-white mb-0.5">{toast.title}</h4>}
          <p className="text-xs text-slate-300 font-mono break-words">{toast.message}</p>
        </div>
        <button 
          onClick={(e) => { e.stopPropagation(); onDismiss(toast.id); }}
          className="shrink-0 text-slate-500 hover:text-white transition-colors"
        >
          <X size={14} />
        </button>
      </div>
      
      {/* Decorative scanline */}
      <div className="absolute top-0 left-0 w-full h-[1px] bg-gradient-to-r from-transparent via-white/20 to-transparent opacity-50"></div>
      
      {/* Progress bar for auto-dismiss items */}
      {toast.type !== 'loading' && (
        <div className="absolute bottom-0 left-0 h-[2px] w-full bg-slate-800">
          <div 
            className={`h-full ${style.progress} animate-toast-progress origin-left`} 
            style={{ animationDuration: `${toast.duration || 5000}ms` }}
          />
        </div>
      )}
    </div>
  );
};

export const ToastProvider: React.FC<{ children: React.ReactNode }> = ({ children }) => {
  const [toasts, setToasts] = useState<Toast[]>([]);

  const addToast = useCallback((type: ToastType, message: string, title?: string, duration?: number) => {
    const id = Math.random().toString(36).substring(2, 9);
    setToasts((prev) => [...prev, { id, type, message, title, duration }]);
    return id;
  }, []);

  const dismiss = useCallback((id: string) => {
    setToasts((prev) => prev.filter((t) => t.id !== id));
  }, []);

  const toastFuncs = {
    success: (msg: string, title?: string) => addToast('success', msg, title),
    error: (msg: string, title?: string) => addToast('error', msg, title),
    info: (msg: string, title?: string) => addToast('info', msg, title),
    warning: (msg: string, title?: string) => addToast('warning', msg, title),
    loading: (msg: string, title?: string) => addToast('loading', msg, title),
    dismiss,
  };

  return (
    <ToastContext.Provider value={{ toast: toastFuncs }}>
      {children}
      
      {/* Toast Container - Fixed Position */}
      <div className="fixed bottom-0 right-0 z-[100] p-4 md:p-6 w-full md:w-auto flex flex-col items-end pointer-events-none">
        <div className="pointer-events-auto w-full md:w-auto">
          {toasts.map((t) => (
            <ToastItem key={t.id} toast={t} onDismiss={dismiss} />
          ))}
        </div>
      </div>
      
      {/* Custom Keyframe for progress bar */}
      <style>{`
        @keyframes toast-progress {
          from { transform: scaleX(1); }
          to { transform: scaleX(0); }
        }
        .animate-toast-progress {
          animation-name: toast-progress;
          animation-timing-function: linear;
          animation-fill-mode: forwards;
        }
      `}</style>
    </ToastContext.Provider>
  );
};
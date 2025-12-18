import React, { useState, useEffect, useRef } from 'react';
import { Search, ArrowRight, LayoutDashboard, Database, Users, HardDrive, TerminalSquare, Settings, Zap, ListTodo } from 'lucide-react';
import { ViewState } from '../types';

interface CommandPaletteProps {
  isOpen: boolean;
  onClose: () => void;
  onNavigate: (view: ViewState) => void;
}

type PaletteItem = 
  | { id: string; label: string; icon: React.ElementType; type: 'navigation'; view: ViewState }
  | { id: string; label: string; icon: React.ElementType; type: 'action'; action: () => void };

const CommandPalette: React.FC<CommandPaletteProps> = ({ isOpen, onClose, onNavigate }) => {
  const [query, setQuery] = useState('');
  const [selectedIndex, setSelectedIndex] = useState(0);
  const inputRef = useRef<HTMLInputElement>(null);

  const navigationItems: PaletteItem[] = [
    { id: 'dashboard', label: 'Go to Dashboard', icon: LayoutDashboard, type: 'navigation', view: 'dashboard' },
    { id: 'data', label: 'Go to Data Browser', icon: Database, type: 'navigation', view: 'data' },
    { id: 'auth', label: 'Go to Authentication', icon: Users, type: 'navigation', view: 'auth' },
    { id: 'storage', label: 'Go to Storage', icon: HardDrive, type: 'navigation', view: 'storage' },
    { id: 'functions', label: 'Go to Edge Functions', icon: TerminalSquare, type: 'navigation', view: 'functions' },
    { id: 'tasks', label: 'Go to System Tasks', icon: ListTodo, type: 'navigation', view: 'tasks' },
    { id: 'settings', label: 'Go to Settings', icon: Settings, type: 'navigation', view: 'settings' },
  ];

  const actionItems: PaletteItem[] = [
    { id: 'new_query', label: 'Run New SQL Query', icon: Zap, type: 'action', action: () => { onNavigate('data'); } },
    { id: 'deploy', label: 'Deploy Active Function', icon: Zap, type: 'action', action: () => { onNavigate('functions'); } },
    { id: 'backup', label: 'Trigger Manual Backup', icon: Database, type: 'action', action: () => { onNavigate('settings'); } },
  ];

  const allItems = [...navigationItems, ...actionItems];
  
  const filteredItems = allItems.filter(item => 
    item.label.toLowerCase().includes(query.toLowerCase())
  );

  useEffect(() => {
    if (isOpen) {
      setTimeout(() => inputRef.current?.focus(), 50);
      setSelectedIndex(0);
      setQuery('');
    }
  }, [isOpen]);

  useEffect(() => {
    setSelectedIndex(0);
  }, [query]);

  const handleKeyDown = (e: React.KeyboardEvent) => {
    if (e.key === 'ArrowDown') {
      e.preventDefault();
      setSelectedIndex(prev => Math.min(prev + 1, filteredItems.length - 1));
    } else if (e.key === 'ArrowUp') {
      e.preventDefault();
      setSelectedIndex(prev => Math.max(prev - 1, 0));
    } else if (e.key === 'Enter') {
      e.preventDefault();
      const selected = filteredItems[selectedIndex];
      if (selected) {
        if (selected.type === 'navigation') {
            onNavigate(selected.view);
        } else if (selected.type === 'action') {
            selected.action();
        }
        onClose();
      }
    } else if (e.key === 'Escape') {
      onClose();
    }
  };

  if (!isOpen) return null;

  return (
    <div className="fixed inset-0 z-[100] flex items-start justify-center pt-[20vh] px-4">
      {/* Backdrop */}
      <div 
        className="fixed inset-0 bg-slate-950/80 backdrop-blur-sm transition-opacity" 
        onClick={onClose}
      />

      {/* Modal */}
      <div className="w-full max-w-xl bg-slate-900 border border-white/10 rounded-xl shadow-2xl overflow-hidden relative animate-in zoom-in-95 duration-200 ring-1 ring-white/10">
        <div className="flex items-center px-4 py-3 border-b border-white/5 bg-slate-900">
          <Search className="w-5 h-5 text-slate-500 mr-3" />
          <input
            ref={inputRef}
            type="text"
            className="flex-1 bg-transparent text-white placeholder-slate-500 focus:outline-none text-sm font-medium h-6"
            placeholder="Type a command or search..."
            value={query}
            onChange={(e) => setQuery(e.target.value)}
            onKeyDown={handleKeyDown}
          />
          <div className="text-[10px] font-mono text-slate-500 bg-white/5 px-1.5 py-0.5 rounded border border-white/5">ESC</div>
        </div>

        <div className="max-h-[300px] overflow-y-auto py-2">
          {filteredItems.length > 0 ? (
            <>
                <div className="px-3 py-1.5 text-[10px] font-mono text-slate-500 uppercase tracking-wider">Results</div>
                {filteredItems.map((item, index) => {
                    const Icon = item.icon;
                    const isActive = index === selectedIndex;
                    return (
                    <button
                        key={item.id}
                        onClick={() => {
                            if (item.type === 'navigation') onNavigate(item.view);
                            else if (item.type === 'action') item.action();
                            onClose();
                        }}
                        onMouseEnter={() => setSelectedIndex(index)}
                        className={`w-full flex items-center px-4 py-3 text-sm transition-colors cursor-pointer
                        ${isActive ? 'bg-hive-orange/10 text-white border-l-2 border-hive-orange' : 'text-slate-400 border-l-2 border-transparent hover:bg-white/5'}
                        `}
                    >
                        <Icon size={16} className={`mr-3 ${isActive ? 'text-hive-orange' : 'text-slate-500'}`} />
                        <span className="flex-1 text-left">{item.label}</span>
                        {isActive && <ArrowRight size={14} className="text-slate-500" />}
                    </button>
                    );
                })}
            </>
          ) : (
             <div className="px-4 py-8 text-center text-slate-500 text-sm">
                No results found for "{query}"
             </div>
          )}
        </div>
        
        <div className="bg-slate-950/50 px-4 py-2 border-t border-white/5 flex justify-between items-center text-[10px] text-slate-500 font-mono">
            <div>
                <span className="mr-2">↑↓ to navigate</span>
                <span>↵ to select</span>
            </div>
            <div>Edge Hive Command</div>
        </div>
      </div>
    </div>
  );
};

export default CommandPalette;

import React, { useState, useEffect } from 'react';
import {
  Filter, Calendar, AlertCircle, CheckCircle, Clock, XCircle, Search,
  ChevronDown, ArrowUp, ArrowDown, RefreshCw, MoreVertical, Briefcase,
  Ghost, Lock, Zap
} from 'lucide-react';
import { mockApi } from '../api';
import { SystemTask } from '../types';
import { useToast } from '../context/ToastContext';

const Tasks: React.FC = () => {
  // Fix: useToast returns the toast object directly
  const toast = useToast();
  const [tasks, setTasks] = useState<SystemTask[]>([]);
  const [filteredTasks, setFilteredTasks] = useState<SystemTask[]>([]);
  const [loading, setLoading] = useState(true);

  const [statusFilter, setStatusFilter] = useState<string>('all');
  const [priorityFilter, setPriorityFilter] = useState<string>('all');
  const [searchTerm, setSearchTerm] = useState('');
  const [sortBy, setSortBy] = useState<'due_date' | 'created_at' | 'priority'>('due_date');
  const [sortOrder, setSortOrder] = useState<'asc' | 'desc'>('asc');

  const fetchTasks = async () => {
    setLoading(true);
    try {
      // Extended Mock Tasks for the new modules
      const baseTasks = await mockApi.getTasks();
      const extendedTasks: SystemTask[] = [
        ...baseTasks,
        { id: 'TSK-942', title: 'Provision Hidden Onion Service (v3)', description: 'Generating ED25519 identity keys and configuring Tor circuits.', status: 'processing', priority: 'high', due_date: new Date().toISOString(), created_at: new Date().toISOString(), assignee: 'neural_agent' },
        { id: 'TSK-119', title: 'Update VPN Mesh Peer Lattice', description: 'Broadcasting new public keys to all peers in the WireGuard mesh.', status: 'completed', priority: 'critical', due_date: new Date().toISOString(), created_at: new Date().toISOString(), assignee: 'wg_daemon' },
        { id: 'TSK-032', title: 'Self-Heal Shard HN-02', description: 'Relocating data shards post-entropy scan.', status: 'pending', priority: 'medium', due_date: new Date().toISOString(), created_at: new Date().toISOString(), assignee: 'shard_balancer' },
      ];
      setTasks(extendedTasks);
    } catch (e) {
      toast.error("Failed to load tasks");
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    fetchTasks();
  }, []);

  useEffect(() => {
    let result = [...tasks];
    if (searchTerm) {
      result = result.filter(t => t.title.toLowerCase().includes(searchTerm.toLowerCase()) || t.id.toLowerCase().includes(searchTerm.toLowerCase()));
    }
    if (statusFilter !== 'all') result = result.filter(t => t.status === statusFilter);
    if (priorityFilter !== 'all') result = result.filter(t => t.priority === priorityFilter);

    result.sort((a, b) => {
      let valA, valB;
      if (sortBy === 'priority') {
        const weights: any = { critical: 4, high: 3, medium: 2, low: 1 };
        valA = weights[a.priority];
        valB = weights[b.priority];
      } else {
        valA = new Date(a[sortBy]).getTime();
        valB = new Date(b[sortBy]).getTime();
      }
      return sortOrder === 'asc' ? valA - valB : valB - valA;
    });
    setFilteredTasks(result);
  }, [tasks, searchTerm, statusFilter, priorityFilter, sortBy, sortOrder]);

  const getStatusBadge = (status: string) => {
    switch (status) {
      case 'completed': return <span className="flex items-center gap-1.5 px-2 py-0.5 rounded text-[10px] uppercase font-bold bg-emerald-500/10 text-emerald-500 border border-emerald-500/20"><CheckCircle size={10} /> Completed</span>;
      case 'processing': return <span className="flex items-center gap-1.5 px-2 py-0.5 rounded text-[10px] uppercase font-bold bg-blue-500/10 text-blue-500 border border-blue-500/20"><RefreshCw size={10} className="animate-spin" /> Processing</span>;
      case 'failed': return <span className="flex items-center gap-1.5 px-2 py-0.5 rounded text-[10px] uppercase font-bold bg-red-500/10 text-red-500 border border-red-500/20"><XCircle size={10} /> Failed</span>;
      default: return <span className="flex items-center gap-1.5 px-2 py-0.5 rounded text-[10px] uppercase font-bold bg-slate-800 text-slate-400 border border-slate-700"><Clock size={10} /> Pending</span>;
    }
  };

  const getPriorityBadge = (priority: string) => {
    switch (priority) {
      case 'critical': return <span className="text-red-500 font-bold uppercase text-[10px] tracking-wider animate-pulse">CRITICAL</span>;
      case 'high': return <span className="text-orange-500 font-bold uppercase text-[10px] tracking-wider">HIGH</span>;
      case 'medium': return <span className="text-yellow-500 font-bold uppercase text-[10px] tracking-wider">MED</span>;
      default: return <span className="text-slate-500 font-bold uppercase text-[10px] tracking-wider">LOW</span>;
    }
  };

  return (
    <div className="space-y-8 animate-in fade-in duration-500 pb-12">
      <div className="flex flex-col md:flex-row md:items-center justify-between gap-4">
        <div>
          <h2 className="text-4xl font-black text-white tracking-tighter uppercase mb-1 flex items-center gap-4">
            <Briefcase className="text-hive-orange" size={32} />
            Global Tasks
          </h2>
          <p className="text-slate-400 text-sm font-mono">Real-time orchestration of Hive-managed autonomous processes.</p>
        </div>
        <div className="flex gap-2">
          <button onClick={fetchTasks} className="p-2 bg-slate-900 border border-white/10 rounded hover:bg-white/5 text-slate-400 hover:text-white transition"><RefreshCw size={18} /></button>
          <button className="px-4 py-2 bg-hive-cyan hover:bg-cyan-600 text-black font-bold text-sm rounded transition shadow-neon-cyan flex items-center gap-2">+ New Deployment</button>
        </div>
      </div>

      <div className="bg-slate-900/40 border border-white/5 rounded-lg p-4 flex flex-col md:flex-row gap-4 items-center">
        <div className="relative flex-1 w-full">
          <Search className="absolute left-3 top-1/2 -translate-y-1/2 text-slate-500" size={16} />
          <input type="text" placeholder="Search tasks..." value={searchTerm} onChange={(e) => setSearchTerm(e.target.value)} className="w-full bg-slate-950 border border-white/10 rounded pl-10 pr-4 py-2 text-sm text-white focus:outline-none focus:border-hive-orange/50 font-mono" />
        </div>
        <div className="flex gap-3">
          <select value={statusFilter} onChange={(e) => setStatusFilter(e.target.value)} className="bg-slate-950 border border-white/10 rounded px-4 py-2 text-sm text-slate-300 focus:outline-none">
            <option value="all">All Status</option>
            <option value="pending">Pending</option>
            <option value="processing">Processing</option>
            <option value="completed">Completed</option>
          </select>
          <select value={sortBy} onChange={(e) => setSortBy(e.target.value as any)} className="bg-slate-950 border border-white/10 rounded px-4 py-2 text-sm text-slate-300 focus:outline-none">
            <option value="due_date">Due Date</option>
            <option value="created_at">Created</option>
            <option value="priority">Priority</option>
          </select>
          <button onClick={() => setSortOrder(prev => prev === 'asc' ? 'desc' : 'asc')} className="p-2 bg-slate-950 border border-white/10 rounded text-slate-400">
            {sortOrder === 'asc' ? <ArrowUp size={16} /> : <ArrowDown size={16} />}
          </button>
        </div>
      </div>

      <div className="bg-slate-900/40 border border-white/5 rounded-lg overflow-hidden relative">
        <table className="w-full text-left text-sm border-collapse">
          <thead className="bg-slate-950 text-slate-500 font-mono text-[10px] uppercase tracking-wider sticky top-0">
            <tr>
              <th className="p-4 border-b border-white/10">Orchestration Process</th>
              <th className="p-4 border-b border-white/10">Status</th>
              <th className="p-4 border-b border-white/10">Urgency</th>
              <th className="p-4 border-b border-white/10">Executer</th>
              <th className="p-4 border-b border-white/10 text-right">Control</th>
            </tr>
          </thead>
          <tbody className="divide-y divide-white/5 text-slate-300">
            {loading ? (
              <tr><td colSpan={5} className="p-8 text-center text-slate-500 font-mono animate-pulse">Syncing task lattice...</td></tr>
            ) : (
              filteredTasks.map(task => (
                <tr key={task.id} className="hover:bg-white/5 transition-colors group">
                  <td className="p-4">
                    <div className="flex items-center gap-3">
                      <div className="p-2 bg-slate-950 rounded text-slate-500">
                        {task.title.includes('Onion') ? <Ghost size={16} className="text-purple-500" /> :
                          task.title.includes('VPN') ? <Lock size={16} className="text-hive-cyan" /> :
                            <Zap size={16} />}
                      </div>
                      <div>
                        <div className="font-bold text-white mb-0.5">{task.title}</div>
                        <div className="text-[9px] font-mono text-slate-600 uppercase truncate max-w-md">{task.description}</div>
                      </div>
                    </div>
                  </td>
                  <td className="p-4">{getStatusBadge(task.status)}</td>
                  <td className="p-4">{getPriorityBadge(task.priority)}</td>
                  <td className="p-4 font-mono text-[10px] text-slate-500">{task.assignee}</td>
                  <td className="p-4 text-right"><button className="text-slate-600 hover:text-white transition"><MoreVertical size={16} /></button></td>
                </tr>
              ))
            )}
          </tbody>
        </table>
      </div>
    </div>
  );
};

export default Tasks;


import React, { useState, useEffect } from 'react';
import {
  Filter, Calendar, AlertCircle, CheckCircle, Clock, XCircle, Search,
  ChevronDown, ArrowUp, ArrowDown, RefreshCw, MoreVertical, Briefcase,
  Ghost, Lock, Zap
} from 'lucide-react';
import { api as tauriApi } from '../api';
import { SystemTask } from '../types';
import { useToast } from '../context/ToastContext';

const Tasks: React.FC = () => {
  const toast = useToast();
  const [tasks, setTasks] = useState<SystemTask[]>([]);
  const [filteredTasks, setFilteredTasks] = useState<SystemTask[]>([]);
  const [loading, setLoading] = useState(true);

  const [statusFilter, setStatusFilter] = useState<string>('all');
  const [priorityFilter, setPriorityFilter] = useState<string>('all');
  const [searchTerm, setSearchTerm] = useState('');
  const [sortBy, setSortBy] = useState<'due_date' | 'created_at' | 'priority'>('due_date');
  const [sortOrder, setSortOrder] = useState<'asc' | 'desc'>('asc');

  const [isModalOpen, setIsModalOpen] = useState(false);
  const [selectedTask, setSelectedTask] = useState<SystemTask | null>(null);

  const fetchTasks = async () => {
    setLoading(true);
    try {
      const data = await tauriApi.getTasks();
      setTasks(data);
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

  const handleCreate = () => {
    setSelectedTask(null);
    setIsModalOpen(true);
  };

  const handleEdit = (task: SystemTask) => {
    setSelectedTask(task);
    setIsModalOpen(true);
  };

  const handleDelete = async (id: string) => {
    if (window.confirm('Are you sure you want to delete this task?')) {
      try {
        await tauriApi.deleteTask(id);
        toast.success('Task deleted successfully');
        fetchTasks();
      } catch (e) {
        toast.error('Failed to delete task');
      }
    }
  };

  const handleSave = async (task: SystemTask) => {
    try {
      if (selectedTask) {
        await tauriApi.updateTask(task);
        toast.success('Task updated successfully');
      } else {
        await tauriApi.createTask(task);
        toast.success('Task created successfully');
      }
      fetchTasks();
      setIsModalOpen(false);
    } catch (e) {
      toast.error('Failed to save task');
    }
  };

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
          <button onClick={handleCreate} className="px-4 py-2 bg-hive-cyan hover:bg-cyan-600 text-black font-bold text-sm rounded transition shadow-neon-cyan flex items-center gap-2">+ New Task</button>
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
                  <td className="p-4 text-right">
                    <div className="flex items-center justify-end gap-2">
                      <button onClick={() => handleEdit(task)} className="text-slate-600 hover:text-white transition">Edit</button>
                      <button onClick={() => handleDelete(task.id)} className="text-slate-600 hover:text-red-500 transition">Delete</button>
                    </div>
                  </td>
                </tr>
              ))
            )}
          </tbody>
        </table>
      </div>
      {isModalOpen && (
        <TaskModal
          task={selectedTask}
          onClose={() => setIsModalOpen(false)}
          onSave={handleSave}
        />
      )}
    </div>
  );
};

export default Tasks;

interface TaskModalProps {
  task: SystemTask | null;
  onClose: () => void;
  onSave: (task: SystemTask) => void;
}

const TaskModal: React.FC<TaskModalProps> = ({ task, onClose, onSave }) => {
  const [formData, setFormData] = useState<SystemTask>(
    task || {
      id: '',
      title: '',
      description: '',
      status: 'pending',
      priority: 'medium',
      due_date: new Date().toISOString(),
      created_at: new Date().toISOString(),
      assignee: '',
    }
  );

  const handleChange = (e: React.ChangeEvent<HTMLInputElement | HTMLTextAreaElement | HTMLSelectElement>) => {
    const { name, value } = e.target;
    setFormData((prev) => ({ ...prev, [name]: value }));
  };

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    onSave(formData);
  };

  return (
    <div className="fixed inset-0 bg-black/50 flex items-center justify-center z-50">
      <div className="bg-slate-900 border border-white/10 rounded-lg p-8 w-full max-w-lg">
        <h2 className="text-2xl font-bold text-white mb-4">{task ? 'Edit Task' : 'Create Task'}</h2>
        <form onSubmit={handleSubmit} className="space-y-4">
          <div>
            <label className="block text-sm font-medium text-slate-400">Title</label>
            <input
              type="text"
              name="title"
              value={formData.title}
              onChange={handleChange}
              className="w-full bg-slate-950 border border-white/10 rounded px-4 py-2 text-sm text-white focus:outline-none focus:border-hive-orange/50"
              required
            />
          </div>
          <div>
            <label className="block text-sm font-medium text-slate-400">Description</label>
            <textarea
              name="description"
              value={formData.description}
              onChange={handleChange}
              className="w-full bg-slate-950 border border-white/10 rounded px-4 py-2 text-sm text-white focus:outline-none focus:border-hive-orange/50"
              rows={3}
              required
            />
          </div>
          <div className="grid grid-cols-2 gap-4">
            <div>
              <label className="block text-sm font-medium text-slate-400">Status</label>
              <select
                name="status"
                value={formData.status}
                onChange={handleChange}
                className="w-full bg-slate-950 border border-white/10 rounded px-4 py-2 text-sm text-white focus:outline-none focus:border-hive-orange/50"
              >
                <option value="pending">Pending</option>
                <option value="processing">Processing</option>
                <option value="completed">Completed</option>
                <option value="failed">Failed</option>
              </select>
            </div>
            <div>
              <label className="block text-sm font-medium text-slate-400">Priority</label>
              <select
                name="priority"
                value={formData.priority}
                onChange={handleChange}
                className="w-full bg-slate-950 border border-white/10 rounded px-4 py-2 text-sm text-white focus:outline-none focus:border-hive-orange/50"
              >
                <option value="low">Low</option>
                <option value="medium">Medium</option>
                <option value="high">High</option>
                <option value="critical">Critical</option>
              </select>
            </div>
          </div>
          <div>
            <label className="block text-sm font-medium text-slate-400">Due Date</label>
            <input
              type="datetime-local"
              name="due_date"
              value={formData.due_date}
              onChange={handleChange}
              className="w-full bg-slate-950 border border-white/10 rounded px-4 py-2 text-sm text-white focus:outline-none focus:border-hive-orange/50"
              required
            />
          </div>
          <div className="flex justify-end gap-4">
            <button
              type="button"
              onClick={onClose}
              className="px-4 py-2 bg-slate-800 text-white font-bold text-sm rounded hover:bg-slate-700 transition"
            >
              Cancel
            </button>
            <button
              type="submit"
              className="px-4 py-2 bg-hive-cyan text-black font-bold text-sm rounded hover:bg-cyan-600 transition"
            >
              Save
            </button>
          </div>
        </form>
      </div>
    </div>
  );
};


import React, { useEffect, useState } from 'react';
import { Key, Eye, EyeOff, Copy, ShieldAlert, Server, Mail, Save, CheckCircle, FileText, Activity, Database, Archive, Download, RefreshCw, ShieldCheck, Lock, Binary, Cpu } from 'lucide-react';
import { mockApi } from '../api';
import { ApiKey, AccessLogEntry, Backup } from '../types';
import { StatusBadge } from '../components/StatusBadge';
import { useToast } from '../context/ToastContext';
import ConfigEditor from '../components/ConfigEditor';

const ApiKeyRow: React.FC<{ apiKey: ApiKey }> = ({ apiKey }) => {
  const [visible, setVisible] = useState(false);
  // Fix: useToast returns the toast object directly
  const toast = useToast();

  const handleCopy = () => {
    // In a real app, use navigator.clipboard.writeText
    toast.success('API Key copied to clipboard', 'Copied');
  };

  return (
    <div className="flex items-center justify-between p-4 bg-slate-900 border border-white/5 rounded-lg group hover:border-white/10 transition-colors">
      <div className="flex items-start gap-4">
        <div className={`mt-1 p-2 rounded-lg border ${apiKey.role === 'service_role' ? 'bg-red-500/10 border-red-500/20 text-red-500' : 'bg-emerald-500/10 border-emerald-500/20 text-emerald-500'}`}>
          <Key size={20} />
        </div>
        <div>
          <h4 className="text-sm font-bold text-white mb-1 flex items-center gap-2">
            {apiKey.name}
            <span className={`text-[10px] px-2 py-0.5 rounded-full border ${apiKey.role === 'service_role' ? 'border-red-500/30 text-red-400' : 'border-emerald-500/30 text-emerald-400'}`}>
              {apiKey.role}
            </span>
          </h4>
          <div className="flex items-center gap-2 font-mono text-xs text-slate-500">
            <span>Prefix: {apiKey.prefix.substring(0, 15)}...</span>
            <span>•</span>
            <span>Created: {apiKey.created_at}</span>
          </div>
        </div>
      </div>

      <div className="flex items-center gap-2">
        <div className="relative mr-2">
          <input
            type="text"
            readOnly
            value={visible ? `${apiKey.prefix}${Math.random().toString(36).substring(7)}` : '••••••••••••••••••••••••••••••••'}
            className="bg-slate-950 border border-white/10 rounded px-3 py-1.5 text-xs font-mono text-slate-300 w-64 focus:outline-none"
          />
          <button
            onClick={() => setVisible(!visible)}
            className="absolute right-2 top-1/2 -translate-y-1/2 text-slate-500 hover:text-white"
          >
            {visible ? <EyeOff size={12} /> : <Eye size={12} />}
          </button>
        </div>
        <button
          onClick={handleCopy}
          className="p-2 hover:bg-white/10 rounded text-slate-400 hover:text-white transition"
          title="Copy Key"
        >
          <Copy size={16} />
        </button>
      </div>
    </div>
  );
};

const Settings: React.FC = () => {
  // Fix: useToast returns the toast object directly
  const toast = useToast();
  const [apiKeys, setApiKeys] = useState<ApiKey[]>([]);
  const [accessLogs, setAccessLogs] = useState<AccessLogEntry[]>([]);
  const [backups, setBackups] = useState<Backup[]>([]);
  const [activeTab, setActiveTab] = useState<'general' | 'api' | 'smtp' | 'logs' | 'backups'>('api');
  const [isQuantumSecure, setIsQuantumSecure] = useState(true);

  // SMTP State
  const [smtpConfig, setSmtpConfig] = useState({
    host: 'smtp.sendgrid.net',
    port: '587',
    user: 'apikey',
    pass: 'SG.xxxxxxxxxxxxxxxxxx',
    sender: 'noreply@edgehive.io'
  });

  useEffect(() => {
    mockApi.getApiKeys().then(setApiKeys);
  }, []);

  useEffect(() => {
    if (activeTab === 'logs') {
      mockApi.getAccessLogs().then(setAccessLogs);
    } else if (activeTab === 'backups') {
      mockApi.getBackups().then(setBackups);
    }
  }, [activeTab]);

  const handleSaveSmtp = () => {
    const loadingId = toast.loading('Verifying SMTP credentials...', 'Connecting');
    setTimeout(() => {
      toast.dismiss(loadingId);
      toast.success('SMTP Configuration saved successfully.', 'System Updated');
    }, 1500);
  };

  const handleSendTestEmail = () => {
    const loadingId = toast.loading('Sending test email...', 'Sending');
    setTimeout(() => {
      toast.dismiss(loadingId);
      toast.info('Test email sent to ' + smtpConfig.sender, 'Email Sent');
    }, 2000);
  };

  const handleCreateSnapshot = () => {
    const loadingId = toast.loading(isQuantumSecure ? 'Generating Lattice-based encryption keys...' : 'Taking standard database snapshot...', 'Quantum Secure Ingress');
    setTimeout(() => {
      toast.dismiss(loadingId);
      toast.success(`Backup "manual_snapshot_${Date.now().toString().slice(-4)}" created.`, isQuantumSecure ? 'Quantum-Secure Hash: CRYSTALS-Kyber' : 'Backup Complete');

      // Mock add to list
      const newBackup: Backup = {
        id: `bk_${Date.now()}`,
        name: `manual_snapshot_${Date.now().toString().slice(-4)}`,
        size: '45 MB',
        created_at: new Date().toISOString(),
        status: 'completed',
        type: 'manual'
      };
      setBackups(prev => [newBackup, ...prev]);
    }, 2500);
  };

  const handleRestore = (name: string) => {
    if (confirm(`Are you sure you want to restore "${name}"? This will overwrite current data.`)) {
      const loadingId = toast.loading('Decrypting and restoring database...', 'Restoring');
      setTimeout(() => {
        toast.dismiss(loadingId);
        toast.success(`System restored to ${name} state.`, 'Restoration Complete');
      }, 3000);
    }
  };

  return (
    <div className="max-w-4xl mx-auto">
      <div className="mb-8">
        <h2 className="text-2xl font-bold text-white mb-2">Project Settings</h2>
        <p className="text-slate-400 text-sm">Manage access keys, system configuration and integrations.</p>
      </div>

      {/* Tabs */}
      <div className="flex border-b border-white/10 mb-6 overflow-x-auto">
        <button
          onClick={() => setActiveTab('general')}
          className={`px-4 py-2 text-sm font-medium border-b-2 transition-colors whitespace-nowrap ${activeTab === 'general' ? 'border-hive-orange text-hive-orange' : 'border-transparent text-slate-500 hover:text-slate-300'}`}
        >
          General
        </button>
        <button
          onClick={() => setActiveTab('api')}
          className={`px-4 py-2 text-sm font-medium border-b-2 transition-colors whitespace-nowrap ${activeTab === 'api' ? 'border-hive-orange text-hive-orange' : 'border-transparent text-slate-500 hover:text-slate-300'}`}
        >
          API & Keys
        </button>
        <button
          onClick={() => setActiveTab('smtp')}
          className={`px-4 py-2 text-sm font-medium border-b-2 transition-colors whitespace-nowrap ${activeTab === 'smtp' ? 'border-hive-orange text-hive-orange' : 'border-transparent text-slate-500 hover:text-slate-300'}`}
        >
          SMTP Settings
        </button>
        <button
          onClick={() => setActiveTab('logs')}
          className={`px-4 py-2 text-sm font-medium border-b-2 transition-colors whitespace-nowrap ${activeTab === 'logs' ? 'border-hive-orange text-hive-orange' : 'border-transparent text-slate-500 hover:text-slate-300'}`}
        >
          Access Logs
        </button>
        <button
          onClick={() => setActiveTab('backups')}
          className={`px-4 py-2 text-sm font-medium border-b-2 transition-colors whitespace-nowrap ${activeTab === 'backups' ? 'border-hive-orange text-hive-orange' : 'border-transparent text-slate-500 hover:text-slate-300'}`}
        >
          Backups
        </button>
      </div>

      {/* API Keys Content */}
      {activeTab === 'api' && (
        <div className="space-y-6">
          <div className="bg-slate-900/40 border border-white/5 rounded-lg p-6 backdrop-blur-sm">
            <div className="flex justify-between items-center mb-4">
              <h3 className="text-lg font-medium text-white">Project API Keys</h3>
              <button
                onClick={() => toast.info('Key rolling logic would open here.', 'Not Implemented')}
                className="px-3 py-1.5 bg-white/5 border border-white/10 hover:bg-white/10 rounded text-xs font-mono text-white transition"
              >
                + Roll Key
              </button>
            </div>
            <div className="p-4 mb-6 bg-blue-500/10 border border-blue-500/20 rounded-lg flex gap-3">
              <ShieldAlert className="text-blue-400 shrink-0" size={20} />
              <div>
                <h4 className="text-sm font-bold text-blue-400 mb-1">Security Note</h4>
                <p className="text-xs text-blue-300/80 leading-relaxed">
                  The <span className="font-mono bg-blue-500/20 px-1 rounded">service_role</span> key has full admin access and bypasses Row Level Security.
                  Never expose this key on the client-side (browser) or in public repositories.
                </p>
              </div>
            </div>

            <div className="space-y-3">
              {apiKeys.map(key => <ApiKeyRow key={key.id} apiKey={key} />)}
            </div>
          </div>

          <div className="bg-slate-900/40 border border-white/5 rounded-lg p-6 backdrop-blur-sm">
            <h3 className="text-lg font-medium text-white mb-4">JWT Settings</h3>
            <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
              <div>
                <label className="block text-xs font-mono text-slate-500 mb-2">JWT SECRET</label>
                <div className="flex gap-2">
                  <input type="password" value="super-secret-jwt-token-placeholder-do-not-reveal" disabled className="flex-1 bg-slate-950 border border-white/10 rounded px-3 py-2 text-sm text-slate-500" />
                  <button className="p-2 border border-white/10 rounded hover:bg-white/5 text-slate-400"><Eye size={16} /></button>
                </div>
              </div>
              <div>
                <label className="block text-xs font-mono text-slate-500 mb-2">TOKEN EXPIRY (SEC)</label>
                <input type="number" value="3600" className="w-full bg-slate-950 border border-white/10 rounded px-3 py-2 text-sm text-white focus:border-hive-orange/50 outline-none" />
              </div>
            </div>
          </div>
        </div>
      )}

      {/* SMTP Content */}
      {activeTab === 'smtp' && (
        <div className="max-w-2xl">
          <div className="bg-slate-900/40 border border-white/5 rounded-lg p-6 backdrop-blur-sm">
            <h3 className="text-lg font-medium text-white mb-6 flex items-center gap-2">
              <Mail size={20} className="text-hive-orange" />
              Mailer Configuration
            </h3>

            <div className="grid grid-cols-12 gap-4 mb-6">
              <div className="col-span-8">
                <label className="block text-xs font-mono text-slate-500 mb-2">SMTP HOST</label>
                <input type="text" value={smtpConfig.host} className="w-full bg-slate-950 border border-white/10 rounded px-3 py-2 text-sm text-white focus:border-hive-orange/50 outline-none" />
              </div>
              <div className="col-span-4">
                <label className="block text-xs font-mono text-slate-500 mb-2">PORT</label>
                <input type="text" value={smtpConfig.port} className="w-full bg-slate-950 border border-white/10 rounded px-3 py-2 text-sm text-white focus:border-hive-orange/50 outline-none" />
              </div>

              <div className="col-span-6">
                <label className="block text-xs font-mono text-slate-500 mb-2">USERNAME</label>
                <input type="text" value={smtpConfig.user} className="w-full bg-slate-950 border border-white/10 rounded px-3 py-2 text-sm text-white focus:border-hive-orange/50 outline-none" />
              </div>
              <div className="col-span-6">
                <label className="block text-xs font-mono text-slate-500 mb-2">PASSWORD</label>
                <input type="password" value={smtpConfig.pass} className="w-full bg-slate-950 border border-white/10 rounded px-3 py-2 text-sm text-white focus:border-hive-orange/50 outline-none" />
              </div>

              <div className="col-span-12">
                <label className="block text-xs font-mono text-slate-500 mb-2">SENDER EMAIL</label>
                <input type="text" value={smtpConfig.sender} className="w-full bg-slate-950 border border-white/10 rounded px-3 py-2 text-sm text-white focus:border-hive-orange/50 outline-none" />
              </div>
            </div>

            <div className="flex justify-between items-center pt-6 border-t border-white/5">
              <button
                onClick={handleSendTestEmail}
                className="text-xs text-slate-500 hover:text-white underline"
              >
                Send Test Email
              </button>
              <button
                onClick={handleSaveSmtp}
                className="px-4 py-2 bg-hive-orange hover:bg-orange-600 text-black font-bold text-sm rounded transition shadow-neon-orange flex items-center gap-2"
              >
                <Save size={16} /> Save Settings
              </button>
            </div>
          </div>
        </div>
      )}

      {/* Access Logs Content */}
      {activeTab === 'logs' && (
        <div className="space-y-4">
          <div className="flex justify-between items-center mb-2">
            <h3 className="text-lg font-medium text-white flex items-center gap-2">
              <FileText size={20} className="text-hive-cyan" />
              Request History
            </h3>
            <div className="flex gap-2">
              <button className="p-2 border border-white/10 rounded hover:bg-white/5 text-slate-400"><Activity size={16} /></button>
            </div>
          </div>
          <div className="bg-slate-900/40 border border-white/5 rounded-lg overflow-hidden backdrop-blur-sm">
            <table className="w-full text-left text-xs font-mono">
              <thead className="bg-slate-950 text-slate-500 uppercase tracking-wider">
                <tr>
                  <th className="p-3 border-b border-white/10 w-20">Status</th>
                  <th className="p-3 border-b border-white/10 w-20">Method</th>
                  <th className="p-3 border-b border-white/10">Path</th>
                  <th className="p-3 border-b border-white/10 text-right">Latency</th>
                  <th className="p-3 border-b border-white/10">IP Address</th>
                  <th className="p-3 border-b border-white/10">Time</th>
                </tr>
              </thead>
              <tbody className="divide-y divide-white/5 text-slate-300">
                {accessLogs.map(log => (
                  <tr key={log.id} className="hover:bg-white/5 transition-colors">
                    <td className="p-3">
                      <span className={`px-1.5 py-0.5 rounded font-bold
                                         ${log.status_code >= 500 ? 'bg-red-500/20 text-red-500' :
                          log.status_code >= 400 ? 'bg-orange-500/20 text-orange-500' :
                            'bg-emerald-500/20 text-emerald-500'
                        }
                                     `}>
                        {log.status_code}
                      </span>
                    </td>
                    <td className="p-3 font-bold text-slate-500">{log.method}</td>
                    <td className="p-3 text-white">{log.path}</td>
                    <td className="p-3 text-right text-slate-500">{log.duration_ms}ms</td>
                    <td className="p-3 text-slate-500">{log.ip_address}</td>
                    <td className="p-3 text-slate-600">{new Date(log.timestamp).toLocaleTimeString()}</td>
                  </tr>
                ))}
              </tbody>
            </table>
          </div>
        </div>
      )}

      {/* Backups Content */}
      {activeTab === 'backups' && (
        <div className="space-y-6">
          {/* Quantum Card */}
          <div className="bg-slate-900/60 border border-hive-cyan/20 rounded-lg p-6 backdrop-blur-sm relative overflow-hidden group">
            <div className="absolute top-0 right-0 p-4 text-hive-cyan opacity-10 group-hover:opacity-20 transition-opacity">
              <ShieldCheck size={120} />
            </div>
            <div className="relative z-10 flex flex-col md:flex-row md:items-center justify-between gap-6">
              <div className="flex-1">
                <div className="flex items-center gap-3 mb-2">
                  <h3 className="text-xl font-bold text-white flex items-center gap-2 tracking-tighter uppercase">
                    <Lock size={20} className="text-hive-cyan" />
                    Quantum-Secure Vault
                  </h3>
                  <span className="px-2 py-0.5 bg-hive-cyan text-black font-bold text-[9px] rounded uppercase">NIST_STD_2024</span>
                </div>
                <p className="text-xs text-slate-400 max-w-lg leading-relaxed">
                  Protect your snapshots against Shor's algorithm and future quantum decryptors. We utilize
                  <span className="text-hive-cyan font-mono mx-1">CRYSTALS-Kyber</span> and
                  <span className="text-hive-cyan font-mono mx-1">Dilithium</span> lattice-based signatures for atomic backup verification.
                </p>
              </div>
              <div className="flex items-center gap-4 bg-slate-950/50 p-4 rounded-xl border border-white/5">
                <div className="text-right">
                  <div className="text-[9px] font-mono text-slate-500 uppercase">Vault Armor</div>
                  <div className={`text-xs font-bold font-mono ${isQuantumSecure ? 'text-hive-cyan' : 'text-slate-600'}`}>
                    {isQuantumSecure ? 'PQE_ACTIVE' : 'PQE_DISABLED'}
                  </div>
                </div>
                <button
                  onClick={() => setIsQuantumSecure(!isQuantumSecure)}
                  className={`w-12 h-6 rounded-full p-1 transition-colors duration-300 ${isQuantumSecure ? 'bg-hive-cyan shadow-neon-cyan' : 'bg-slate-800'}`}
                >
                  <div className={`w-4 h-4 bg-white rounded-full transition-transform duration-300 ${isQuantumSecure ? 'translate-x-6' : 'translate-x-0'}`}></div>
                </button>
              </div>
            </div>
          </div>

          {/* Header Card */}
          <div className="bg-slate-900/40 border border-white/5 rounded-lg p-6 backdrop-blur-sm flex items-center justify-between">
            <div>
              <h3 className="text-lg font-medium text-white flex items-center gap-2">
                <Archive size={20} className="text-hive-orange" />
                Snapshot Registry
              </h3>
              <p className="text-xs text-slate-400 mt-1">
                Point-in-time recovery for your Edge Hive cluster.
              </p>
            </div>
            <button
              onClick={handleCreateSnapshot}
              className="px-4 py-2 bg-hive-orange text-black font-bold text-sm rounded transition shadow-neon-orange flex items-center gap-2 hover:bg-orange-600"
            >
              <Database size={16} /> New Snapshot
            </button>
          </div>

          {/* Backups List */}
          <div className="bg-slate-900/40 border border-white/5 rounded-lg overflow-hidden backdrop-blur-sm">
            <table className="w-full text-left text-xs font-mono">
              <thead className="bg-slate-950 text-slate-500 uppercase tracking-wider">
                <tr>
                  <th className="p-4 border-b border-white/10 w-1/3">Snapshot Name</th>
                  <th className="p-4 border-b border-white/10">Engine</th>
                  <th className="p-4 border-b border-white/10">Size</th>
                  <th className="p-4 border-b border-white/10">Status</th>
                  <th className="p-4 border-b border-white/10 text-right">Actions</th>
                </tr>
              </thead>
              <tbody className="divide-y divide-white/5 text-slate-300">
                {backups.map(bk => (
                  <tr key={bk.id} className="hover:bg-white/5 transition-colors group">
                    <td className="p-4 flex flex-col">
                      <span className="text-white font-bold">{bk.name}</span>
                      <span className="text-[9px] text-slate-600 mt-1">ID: {bk.id} • {new Date(bk.created_at).toLocaleString()}</span>
                    </td>
                    <td className="p-4">
                      <div className="flex items-center gap-2">
                        <Cpu size={12} className="text-hive-cyan" />
                        <span className="text-[10px] uppercase font-bold text-slate-400">Atomic_FS</span>
                      </div>
                    </td>
                    <td className="p-4 text-slate-400">{bk.size}</td>
                    <td className="p-4">
                      {bk.status === 'completed' ? (
                        <div className="flex items-center gap-2 text-emerald-500 font-bold uppercase text-[9px]">
                          <div className="w-1.5 h-1.5 bg-emerald-500 rounded-full shadow-neon-cyan"></div> Synced
                        </div>
                      ) : (
                        <span className="text-red-500">Failed</span>
                      )}
                    </td>
                    <td className="p-4 text-right">
                      <div className="flex items-center justify-end gap-2 opacity-0 group-hover:opacity-100 transition-opacity">
                        <button className="p-2 hover:bg-white/10 rounded text-slate-400 hover:text-white transition" title="Download SQL Dump">
                          <Download size={16} />
                        </button>
                        <button
                          onClick={() => handleRestore(bk.name)}
                          className="px-3 py-1 bg-hive-cyan text-black rounded text-[10px] font-bold shadow-neon-cyan"
                        >
                          RESTORE
                        </button>
                      </div>
                    </td>
                  </tr>
                ))}
              </tbody>
            </table>
          </div>
        </div>
      )}

      {/* General Content */}
      {activeTab === 'general' && (
        <ConfigEditor />
      )}

    </div>
  );
};

export default Settings;

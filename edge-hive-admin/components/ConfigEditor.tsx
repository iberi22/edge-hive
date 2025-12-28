
import React, { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/tauri';
import { useToast } from '../context/ToastContext';

const ConfigEditor: React.FC = () => {
    const [config, setConfig] = useState('');
    const [isLoading, setIsLoading] = useState(true);
    const toast = useToast();

    useEffect(() => {
        invoke<string>('get_config')
            .then(setConfig)
            .catch(err => {
                toast.error(`Failed to load config: ${err}`);
                setConfig(`# Failed to load config.toml.\n\n[server]\nport = 8080`);
            })
            .finally(() => setIsLoading(false));
    }, []);

    const handleSave = () => {
        const loadingId = toast.loading('Saving configuration...');
        invoke('save_config', { contents: config })
            .then(() => toast.success('Configuration saved successfully.'))
            .catch(err => toast.error(`Failed to save config: ${err}`))
            .finally(() => toast.dismiss(loadingId));
    };

    if (isLoading) {
        return <div>Loading...</div>;
    }

    return (
        <div className="bg-slate-900/40 border border-white/5 rounded-lg p-6 backdrop-blur-sm">
            <h3 className="text-lg font-medium text-white mb-4">Node Configuration</h3>
            <textarea
                className="w-full h-64 bg-slate-950 border border-white/10 rounded px-3 py-2 text-sm text-white font-mono focus:border-hive-orange/50 outline-none"
                value={config}
                onChange={e => setConfig(e.target.value)}
            />
            <div className="flex justify-end mt-4">
                <button
                    onClick={handleSave}
                    className="px-4 py-2 bg-hive-orange hover:bg-orange-600 text-black font-bold text-sm rounded transition shadow-neon-orange"
                >
                    Save Configuration
                </button>
            </div>
        </div>
    );
};

export default ConfigEditor;

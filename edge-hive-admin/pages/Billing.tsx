
import React from 'react';
import { ShieldOff } from 'lucide-react';

const Billing: React.FC = () => {
    return (
        <div className="flex flex-col items-center justify-center h-full text-center text-slate-500">
            <ShieldOff size={48} className="mb-4 text-slate-600" />
            <h2 className="text-xl font-bold text-slate-300">Billing Deactivated</h2>
            <p className="max-w-md mt-2 text-sm">
                This project is now free to use. All billing and payment features have been removed.
            </p>
        </div>
    );
};

export default Billing;

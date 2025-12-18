
import React, { useState } from 'react';
import DashboardLayout from './layouts/DashboardLayout';
import Overview from './pages/Overview';
import DataBrowser from './pages/DataBrowser';
import Functions from './pages/Functions';
import Auth from './pages/Auth';
import Storage from './pages/Storage';
import Settings from './pages/Settings';
import Tasks from './pages/Tasks';
import Cache from './pages/Cache';
import Sharding from './pages/Sharding';
import QuantumConnect from './pages/QuantumConnect';
import Observability from './pages/Observability';
import Integrations from './pages/Integrations';
import Billing from './pages/Billing';
import Governance from './pages/Governance';
import Federation from './pages/Federation';
import DeepEdge from './pages/DeepEdge';
import OnionNode from './pages/OnionNode';
import VPNMesh from './pages/VPNMesh';
import ChaosLab from './pages/ChaosLab';
import Ledger from './pages/Ledger';
import { ViewState } from './types';
import { ToastProvider } from './context/ToastContext';

const AppContent: React.FC = () => {
  const [currentView, setCurrentView] = useState<ViewState>('dashboard');

  const renderView = () => {
    switch (currentView) {
      case 'dashboard': return <Overview />;
      case 'data': return <DataBrowser />;
      case 'functions': return <Functions />;
      case 'auth': return <Auth />;
      case 'storage': return <Storage />;
      case 'settings': return <Settings />;
      case 'tasks': return <Tasks />;
      case 'cache': return <Cache />;
      case 'sharding': return <Sharding />;
      case 'quantum': return <QuantumConnect />;
      case 'observability': return <Observability />;
      case 'integrations': return <Integrations />;
      case 'billing': return <Billing />;
      case 'governance': return <Governance />;
      case 'federation': return <Federation />;
      case 'deep-edge': return <DeepEdge />;
      case 'onion': return <OnionNode />;
      case 'vpn': return <VPNMesh />;
      case 'chaos-lab': return <ChaosLab />;
      case 'ledger': return <Ledger />;
      default: return <Overview />;
    }
  };

  return (
    <DashboardLayout currentView={currentView} onNavigate={setCurrentView}>
      {renderView()}
    </DashboardLayout>
  );
};

const App: React.FC = () => {
  return (
    <ToastProvider>
      <AppContent />
    </ToastProvider>
  );
};

export default App;

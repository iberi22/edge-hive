<script>
  import { onMount } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';
  import NodeCard from './NodeCard.svelte';
  import CloudSection from './CloudSection.svelte';
  import QrScanner from './QrScanner.svelte';

  let status = $state(null);
  let peers = $state([]);
  let loading = $state(true);
  let error = $state(null);
  let activeTab = $state('home');
  let showScanner = $state(false);

  onMount(async () => {
    try {
      status = await invoke('get_node_status');
      peers = await invoke('get_peers');
    } catch (e) {
      error = e?.message || 'Offline Mode';
      // Mock data for UI dev if backend fails
      if (error) {
        status = {
           name: 'termux-node-alpha',
           peer_id: '12D3Koo...',
           status: 'stopped',
           peers_count: 0,
           tunnel_url: null
        };
      }
    } finally {
      loading = false;
    }
  });

  async function toggleServer() {
    if (status.status === 'running') {
        await invoke('stop_server');
        status.status = 'stopped';
    } else {
        await invoke('start_server', { port: 8080 });
        status.status = 'running';
    }
  }

  function handleScan(result) {
    showScanner = false;
    // Simulate successful pairing
    alert(`Node Paired! Payload: ${result.substring(0, 20)}...`);
  }
</script>

<div class="min-h-screen bg-background text-white pb-24 md:pb-0 flex flex-col md:flex-row">

  <!-- Desktop Sidebar (Hidden on Mobile) -->
  <aside class="hidden md:flex flex-col w-64 border-r border-white/5 bg-surface/30 backdrop-blur-xl p-6 fixed inset-y-0 left-0 z-50">
    <div class="flex items-center gap-3 mb-10">
      <div class="w-8 h-8 bg-gradient-to-br from-primary to-primaryDim rounded-lg flex items-center justify-center shadow-lg shadow-primary/20">
        <span class="text-sm">🐝</span>
      </div>
      <div>
        <h1 class="font-bold text-lg tracking-wide">Edge Hive</h1>
        <p class="text-[10px] text-gray-500 font-mono">v0.1.0 • Desktop</p>
      </div>
    </div>

    <nav class="space-y-2 flex-1">
        {#each ['home', 'cloud', 'plugin', 'settings'] as tab}
            <button
                class="w-full text-left px-4 py-3 rounded-xl transition-all flex items-center gap-3 {activeTab === tab ? 'bg-white/10 text-primary border border-white/5' : 'text-gray-400 hover:text-white hover:bg-white/5'}"
                onclick={() => activeTab = tab}
            >
                <!-- Icons (Simple mock) -->
                <span class="text-lg opacity-70">
                    {#if tab === 'home'}🏠
                    {:else if tab === 'cloud'}☁️
                    {:else if tab === 'plugin'}🔌
                    {:else}⚙️{/if}
                </span>
                <span class="capitalize text-sm font-medium">{tab}</span>
            </button>
        {/each}
    </nav>

    <!-- Connection Status Footer -->
    <div class="pt-6 border-t border-white/5">
        <div class="flex items-center gap-2">
            <div class="w-2 h-2 rounded-full {status?.status === 'running' ? 'bg-success animate-pulse' : 'bg-red-500'}"></div>
            <span class="text-xs text-gray-500 font-mono">{status?.status === 'running' ? 'Connected' : 'Offline'}</span>
        </div>
    </div>
  </aside>

  <!-- Main Content Area -->
  <main class="flex-1 md:ml-64 p-4 md:p-8 max-w-7xl mx-auto w-full relative">

  <!-- Mobile Header (Hidden on Desktop) -->
  <header class="flex justify-between items-center mb-8 md:hidden">
    <div class="flex items-center gap-3">
      <div class="w-10 h-10 bg-gradient-to-br from-primary to-primaryDim rounded-xl flex items-center justify-center shadow-lg shadow-primary/20">
        <span class="text-xl">🐝</span>
      </div>
      <div>
        <h1 class="font-bold text-xl tracking-wide bg-clip-text text-transparent bg-gradient-to-r from-white to-gray-400">Edge Hive</h1>
        <p class="text-xs text-gray-500 font-mono">v0.1.0 • BSL-1.1</p>
      </div>
    </div>
    <button class="p-2 rounded-full hover:bg-white/10 transition-colors backdrop-blur-sm">
      <div class="w-6 h-6 border-2 border-gray-600 rounded-lg flex items-center justify-center">
        <div class="w-1 h-1 bg-white rounded-full"></div>
      </div>
    </button>
  </header>

  <!-- Desktop Header (Title Only) -->
  <header class="hidden md:flex justify-between items-center mb-10">
    <h2 class="text-2xl font-bold capitalize">{activeTab} Dashboard</h2>
    <div class="flex gap-2">
         <button class="px-4 py-2 rounded-lg bg-surface border border-white/5 hover:border-white/10 text-xs font-mono text-gray-400 flex items-center gap-2">
            <span>ID:</span>
            <span class="text-white">{status?.peer_id?.slice(0,8) || '...'}</span>
         </button>
    </div>
  </header>

  {#if loading}
    <div class="flex flex-col items-center justify-center h-64 gap-4 animate-pulse">
      <div class="w-12 h-12 border-4 border-primary border-t-transparent rounded-full animate-spin"></div>
      <p class="text-gray-400 text-sm">Synchronizing Hive...</p>
    </div>
  {:else}
    <!-- Status Card -->
    <NodeCard
      {...status}
      onToggle={toggleServer}
    />

    <!-- Quick Actions (RAID / QR) -->
    <div class="grid grid-cols-2 gap-4 mb-8">
        <button class="bg-surface/50 backdrop-blur-md border border-white/5 p-4 rounded-2xl flex flex-col items-center gap-2 hover:bg-surface/80 transition-all active:scale-95 group">
            <div class="w-10 h-10 rounded-full bg-accent/20 flex items-center justify-center group-hover:bg-accent/30 transition-colors">
                <span class="text-xl">🕸️</span>
            </div>
            <span class="text-sm font-medium text-gray-300">Hive RAID</span>
        </button>
        <button class="bg-surface/50 backdrop-blur-md border border-white/5 p-4 rounded-2xl flex flex-col items-center gap-2 hover:bg-surface/80 transition-all active:scale-95 group"
            onclick={() => showScanner = true}
        >
            <div class="w-10 h-10 rounded-full bg-blue-500/20 flex items-center justify-center group-hover:bg-blue-500/30 transition-colors">
                <span class="text-xl">📱</span>
            </div>
            <span class="text-sm font-medium text-gray-300">Link Device</span>
        </button>
    </div>

    <!-- Active Peers -->
    <section class="mb-8">
      <div class="flex justify-between items-center mb-4">
        <h2 class="text-sm font-bold text-gray-400 uppercase tracking-wider">Swarm Peers</h2>
        <span class="text-xs bg-white/10 px-2 py-1 rounded-full text-gray-300">{peers.length} active</span>
      </div>

      {#if peers.length === 0}
        <div class="border border-dashed border-gray-700 rounded-2xl p-6 text-center">
            <p class="text-gray-500 text-sm">No peers connected.</p>
            <p class="text-xs text-gray-600 mt-1">Scan a QR code to link a node.</p>
        </div>
      {:else}
        <div class="space-y-2">
            {#each peers as peer}
                <div class="bg-surface/30 px-4 py-3 rounded-xl flex items-center justify-between border border-white/5">
                    <div class="flex items-center gap-3">
                        <div class="w-2 h-2 rounded-full bg-success shadow-[0_0_8px_rgba(34,197,94,0.5)]"></div>
                        <span class="font-mono text-sm">{peer.peer_id.slice(0, 8)}...</span>
                    </div>
                    <span class="text-xs text-gray-500">{peer.last_seen}</span>
                </div>
            {/each}
        </div>
      {/if}
    </section>

    <!-- Cloud Integration -->
    <CloudSection />

    {#if showScanner}
        <QrScanner
            onScan={handleScan}
            onCancel={() => showScanner = false}
        />
    {/if}

  {/if}

  <!-- Bottom Nav (Mobile Only) -->
  <nav class="md:hidden fixed bottom-6 left-4 right-4 bg-surface/80 backdrop-blur-xl border border-white/10 rounded-2xl p-2 shadow-2xl flex justify-around z-50">
    {#each ['home', 'cloud', 'plugin', 'settings'] as tab}
        <button
            class="p-3 rounded-xl transition-all relative {activeTab === tab ? 'text-primary bg-white/5' : 'text-gray-500 hover:text-gray-300'}"
            onclick={() => activeTab = tab}
        >
            <span class="capitalize text-xs font-medium block">{tab}</span>
            {#if activeTab === tab}
                <div class="absolute -bottom-1 left-1/2 -translate-x-1/2 w-1 h-1 bg-primary rounded-full"></div>
            {/if}
        </button>
    {/each}
  </nav>

  </main>
</div>

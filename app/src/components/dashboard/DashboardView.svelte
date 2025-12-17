<script>
  import { onMount, onDestroy } from 'svelte';
  import { fly, fade } from 'svelte/transition';
  import StatsCard from './StatsCard.svelte';
  import NodeList from './NodeList.svelte';
  import { IconCpu, IconDeviceDesktopAnalytics, IconDatabase, IconWifi } from '@tabler/icons-svelte';
  import { nodes, stats, dashboardActions } from '../../stores/dashboard';

  let cleanup;
  let ready = false;

  onMount(() => {
    // Start auto-refreshing data from backend every 2 seconds
    cleanup = dashboardActions.startAutoRefresh(2000);
    ready = true;
  });

  onDestroy(() => {
    if (cleanup) cleanup();
  });
</script>

{#if ready}
<div class="space-y-6" in:fade={{ duration: 300 }}>
  <!-- Header Section -->
  <div in:fly={{ y: -20, duration: 500, delay: 100 }}>
    <h1 class="text-2xl font-bold text-textHeading font-sans tracking-tight">Dashboard Overview</h1>
    <p class="text-muted mt-1 font-sans">Real-time monitoring of your Edge Hive cluster.</p>
  </div>

  <!-- Stats Grid -->
  <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
    <div in:fly={{ y: 20, duration: 500, delay: 200 }}>
      <StatsCard
        title="CPU Usage"
        value={$stats.cpu}
        unit="%"
        icon={IconCpu}
        color="primary"
        trend={5}
      />
    </div>
    <div in:fly={{ y: 20, duration: 500, delay: 300 }}>
      <StatsCard
        title="Memory"
        value={$stats.ram}
        unit="GB"
        icon={IconDeviceDesktopAnalytics}
        color="secondary"
        trend={-2}
      />
    </div>
    <div in:fly={{ y: 20, duration: 500, delay: 400 }}>
      <StatsCard
        title="Storage"
        value={$stats.storage}
        unit="GB"
        icon={IconDatabase}
        color="primaryDim"
        trend={0}
      />
    </div>
    <div in:fly={{ y: 20, duration: 500, delay: 500 }}>
      <StatsCard
        title="Network"
        value={$stats.network}
        unit="MB/s"
        icon={IconWifi}
        color="success"
        trend={12}
      />
    </div>
  </div>

  <!-- Main Content Grid -->
  <div class="grid grid-cols-1 lg:grid-cols-3 gap-6" in:fly={{ y: 20, duration: 500, delay: 600 }}>
    <!-- Node List (Takes up 2 columns) -->
    <div class="lg:col-span-2">
      <NodeList nodes={$nodes} />
    </div>

    <!-- Activity Feed / Logs (Placeholder for now) -->
    <div class="bg-surface/40 backdrop-blur-md border border-glass-border rounded-xl p-5 shadow-lg h-fit relative overflow-hidden">
      <!-- HUD Corner -->
      <div class="absolute top-0 right-0 w-4 h-4 border-t-2 border-r-2 border-primary/50 rounded-tr-sm"></div>

      <h3 class="font-bold text-lg text-textHeading mb-4 font-sans">Recent Activity</h3>
      <div class="space-y-4">
        <div class="flex gap-3 group">
          <div class="w-2 h-2 mt-2 rounded-full bg-secondary shadow-[0_0_8px_rgba(14,165,233,0.5)] group-hover:scale-125 transition-transform"></div>
          <div>
            <p class="text-sm text-textBody font-sans">Node <span class="font-mono text-secondary">Edge-01</span> synced successfully.</p>
            <span class="text-xs text-muted font-mono">2 mins ago</span>
          </div>
        </div>
        <div class="flex gap-3 group">
          <div class="w-2 h-2 mt-2 rounded-full bg-success shadow-[0_0_8px_rgba(16,185,129,0.5)] group-hover:scale-125 transition-transform"></div>
          <div>
            <p class="text-sm text-textBody font-sans">New device connected via Tor.</p>
            <span class="text-xs text-muted font-mono">15 mins ago</span>
          </div>
        </div>
        <div class="flex gap-3 group">
          <div class="w-2 h-2 mt-2 rounded-full bg-yellow-500 shadow-[0_0_8px_rgba(234,179,8,0.5)] group-hover:scale-125 transition-transform"></div>
          <div>
            <p class="text-sm text-gray-300">High memory usage detected on <span class="font-mono text-yellow-400">Edge-02</span>.</p>
            <span class="text-xs text-gray-500">1 hour ago</span>
          </div>
        </div>
      </div>
    </div>
  </div>
</div>
{/if}

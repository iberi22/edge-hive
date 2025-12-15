<script>
  import { onMount, onDestroy } from 'svelte';
  import StatsCard from './StatsCard.svelte';
  import NodeList from './NodeList.svelte';
  import { IconCpu, IconDeviceDesktopAnalytics, IconDatabase, IconWifi } from '@tabler/icons-svelte';
  import { nodes, stats, dashboardActions } from '../../stores/dashboard';

  let cleanup;

  onMount(() => {
    // Start auto-refreshing data from backend every 2 seconds
    cleanup = dashboardActions.startAutoRefresh(2000);
  });

  onDestroy(() => {
    if (cleanup) cleanup();
  });
</script>

<div class="space-y-6">
  <!-- Header Section -->
  <div>
    <h1 class="text-2xl font-bold text-white">Dashboard Overview</h1>
    <p class="text-gray-400 mt-1">Real-time monitoring of your Edge Hive cluster.</p>
  </div>

  <!-- Stats Grid -->
  <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
    <StatsCard
      title="CPU Usage"
      value={$stats.cpu}
      unit="%"
      icon={IconCpu}
      color="blue"
      trend={5}
    />
    <StatsCard
      title="Memory"
      value={$stats.ram}
      unit="GB"
      icon={IconDeviceDesktopAnalytics}
      color="purple"
      trend={-2}
    />
    <StatsCard
      title="Storage"
      value={$stats.storage}
      unit="GB"
      icon={IconDatabase}
      color="yellow"
      trend={0}
    />
    <StatsCard
      title="Network"
      value={$stats.network}
      unit="MB/s"
      icon={IconWifi}
      color="green"
      trend={12}
    />
  </div>

  <!-- Main Content Grid -->
  <div class="grid grid-cols-1 lg:grid-cols-3 gap-6">
    <!-- Node List (Takes up 2 columns) -->
    <div class="lg:col-span-2">
      <NodeList nodes={$nodes} />
    </div>

    <!-- Activity Feed / Logs (Placeholder for now) -->
    <div class="bg-gray-900/60 backdrop-blur-md border border-glass-border rounded-xl p-5 shadow-lg">
      <h3 class="font-bold text-lg text-white mb-4">Recent Activity</h3>
      <div class="space-y-4">
        <div class="flex gap-3">
          <div class="w-2 h-2 mt-2 rounded-full bg-blue-500 shadow-[0_0_8px_rgba(59,130,246,0.5)]"></div>
          <div>
            <p class="text-sm text-gray-300">Node <span class="font-mono text-blue-400">Edge-01</span> synced successfully.</p>
            <span class="text-xs text-gray-500">2 mins ago</span>
          </div>
        </div>
        <div class="flex gap-3">
          <div class="w-2 h-2 mt-2 rounded-full bg-green-500 shadow-[0_0_8px_rgba(34,197,94,0.5)]"></div>
          <div>
            <p class="text-sm text-gray-300">New device connected via Tor.</p>
            <span class="text-xs text-gray-500">15 mins ago</span>
          </div>
        </div>
        <div class="flex gap-3">
          <div class="w-2 h-2 mt-2 rounded-full bg-yellow-500 shadow-[0_0_8px_rgba(234,179,8,0.5)]"></div>
          <div>
            <p class="text-sm text-gray-300">High memory usage detected on <span class="font-mono text-yellow-400">Edge-02</span>.</p>
            <span class="text-xs text-gray-500">1 hour ago</span>
          </div>
        </div>
      </div>
    </div>
  </div>
</div>

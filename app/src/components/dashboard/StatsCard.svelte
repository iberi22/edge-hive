<script>
  import { onMount } from 'svelte';

  export let title = '';
  export let value = '';
  export let unit = '';
  export let icon = null;
  export let trend = 0; // Percentage change
  export let color = 'blue'; // blue, green, red, yellow

  // Map colors to Tailwind classes
  const colorMap = {
    blue: 'text-blue-500 bg-blue-500/10',
    green: 'text-green-500 bg-green-500/10',
    red: 'text-red-500 bg-red-500/10',
    yellow: 'text-yellow-500 bg-yellow-500/10',
    purple: 'text-purple-500 bg-purple-500/10',
  };

  $: iconClass = colorMap[color] || colorMap.blue;
</script>

<div class="bg-gray-900/60 backdrop-blur-md border border-glass-border rounded-xl p-5 hover:border-gray-600 transition-all duration-300 shadow-lg hover:shadow-xl hover:bg-gray-900/80 group">
  <div class="flex justify-between items-start mb-4">
    <div>
      <p class="text-gray-400 text-sm font-medium mb-1 group-hover:text-gray-300 transition-colors">{title}</p>
      <h3 class="text-2xl font-bold text-white tracking-tight">
        {value} <span class="text-sm text-gray-500 font-normal ml-1">{unit}</span>
      </h3>
    </div>
    {#if icon}
      <div class={`p-2 rounded-lg ${iconClass} backdrop-blur-sm shadow-inner`}>
        <svelte:component this={icon} size={22} stroke={1.5} />
      </div>
    {/if}
  </div>

  <!-- Trend / Sparkline Placeholder -->
  <div class="flex items-center gap-2">
    {#if trend !== 0}
      <span class={`text-xs font-medium px-1.5 py-0.5 rounded ${trend > 0 ? 'text-green-400 bg-green-400/10 border border-green-400/20' : 'text-red-400 bg-red-400/10 border border-red-400/20'}`}>
        {trend > 0 ? '+' : ''}{trend}%
      </span>
      <span class="text-xs text-gray-500">vs last hour</span>
    {:else}
      <span class="text-xs text-gray-500">Stable</span>
    {/if}
  </div>
</div>

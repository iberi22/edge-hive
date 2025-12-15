<script>
  let { name, peerId, status, peersCount, tunnelUrl, onToggle } = $props();

  let isRunning = $derived(status === 'running');
</script>

<div class="bg-surface border border-white/5 rounded-3xl p-6 mb-8 relative overflow-hidden group transition-all hover:border-white/10 hover:shadow-2xl hover:shadow-primary/5">
  <!-- Glow Effect -->
  <div class="absolute -right-10 -top-10 w-40 h-40 bg-primary/5 rounded-full blur-3xl group-hover:bg-primary/15 transition-all duration-700"></div>

  <div class="relative z-10 flex flex-col md:flex-row md:items-center justify-between gap-6">
    <div class="flex-1">
        <div class="flex items-center gap-3 mb-2">
             <div class="w-3 h-3 rounded-full {isRunning ? 'bg-success shadow-[0_0_10px_#22c55e]' : 'bg-danger'}"></div>
             <span class="text-xs font-bold uppercase tracking-widest text-gray-500">{isRunning ? 'Online' : 'Services Stopped'}</span>
        </div>
        <h2 class="text-3xl md:text-4xl font-bold bg-clip-text text-transparent bg-gradient-to-br from-white via-white to-gray-500 mb-2">{name}</h2>
        <code class="text-xs md:text-sm font-mono text-gray-500 bg-black/20 px-2 py-1 rounded select-all hover:text-gray-300 transition-colors cursor-copy">{peerId}</code>
    </div>

    <div class="flex items-center gap-6">
        <!-- Stats Grid -->
        <div class="grid grid-cols-2 gap-4 mr-4 border-r border-white/5 pr-8 hidden md:grid">
            <div class="text-right">
                <span class="block text-xs text-gray-500 uppercase">Peers</span>
                <span class="block text-xl font-mono">{peersCount}</span>
            </div>
            <div class="text-right">
                <span class="block text-xs text-gray-500 uppercase">Uptime</span>
                <span class="block text-xl font-mono">24h</span>
            </div>
        </div>

        <button
            onclick={onToggle}
            class="w-16 h-16 rounded-2xl flex items-center justify-center transition-all duration-300 shadow-lg {isRunning ? 'bg-surfaceHighlight border border-red-500/20 text-red-400 hover:bg-red-500/10 hover:border-red-500/50' : 'bg-white text-black hover:scale-105 hover:shadow-xl hover:shadow-white/20'}"
        >
            <span class="text-2xl">{isRunning ? '⏹' : '▶'}</span>
        </button>
    </div>
  </div>

  {#if tunnelUrl}
    <div class="mt-6 pt-6 border-t border-white/5 flex items-center justify-between">
        <div class="flex items-center gap-2">
            <div class="w-4 h-4 rounded bg-primary/20 flex items-center justify-center text-[10px]">🌐</div>
            <span class="text-xs text-gray-400">Public Tunnel Active</span>
        </div>
         <a href={tunnelUrl} target="_blank" class="text-xs text-primary hover:text-white transition-colors border-b border-primary/30 hover:border-white pb-0.5">
            {tunnelUrl} ↗
         </a>
    </div>
  {/if}
</div>

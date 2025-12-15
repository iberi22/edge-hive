<script>
  import { invoke } from '@tauri-apps/api/core';

  let cloudNodes = $state([]);
  let loading = $state(false);

  async function provisionNode() {
    // Invoke Rust command
    alert("Starting checkout flow...");
  }
</script>

<section class="mb-24">
  <div class="flex justify-between items-end mb-4">
    <div>
        <h2 class="text-sm font-bold text-gray-400 uppercase tracking-wider">Cloud Swarm</h2>
        <p class="text-xs text-gray-600 mt-0.5">Managed high-availability nodes</p>
    </div>
    <button onclick={provisionNode} class="text-xs bg-primary/10 text-primary px-3 py-1.5 rounded-lg border border-primary/20 hover:bg-primary/20 transition-colors font-medium">
        + Provision
    </button>
  </div>

  {#if cloudNodes.length === 0}
    <div class="bg-gradient-to-br from-surfaceHighlight to-surface border border-white/5 rounded-2xl p-6 relative overflow-hidden">
        <!-- Background decoration -->
        <div class="absolute top-0 right-0 p-8 opacity-5">
            <span class="text-8xl">☁️</span>
        </div>

        <div class="relative z-10">
            <h3 class="text-lg font-bold text-white mb-2">Extended Capability</h3>
            <p class="text-sm text-gray-400 mb-6 leading-relaxed">
                Add 24/7 cloud nodes (AWS/GCP) to create a **Hive RAID**. Ensures your plugins run even when your phone is offline.
            </p>

            <button onclick={provisionNode} class="w-full bg-white text-black font-bold py-3 rounded-xl hover:scale-[1.02] active:scale-95 transition-all shadow-lg shadow-white/10 flex items-center justify-center gap-2">
                <span>Deploy Cloud Node</span>
                <span class="bg-black/10 px-1.5 py-0.5 rounded text-xs">$25/mo</span>
            </button>
        </div>
    </div>
  {:else}
    <!-- List of nodes would go here -->
  {/if}
</section>

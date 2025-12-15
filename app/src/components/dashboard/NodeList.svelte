<script>
  import { IconServer, IconDotsVertical, IconCircleCheck, IconCircleX, IconRefresh } from '@tabler/icons-svelte';

  // Mock data - in real app this would come from a store or API
  export let nodes = [
    { id: 'n1', name: 'Edge-Node-01', ip: '192.168.1.10', status: 'online', cpu: '12%', ram: '2.4GB' },
    { id: 'n2', name: 'Edge-Node-02', ip: '192.168.1.11', status: 'syncing', cpu: '45%', ram: '4.1GB' },
    { id: 'n3', name: 'Edge-Node-03', ip: '192.168.1.12', status: 'offline', cpu: '-', ram: '-' },
    { id: 'n4', name: 'Termux-Mobile', ip: '10.0.0.5', status: 'online', cpu: '5%', ram: '1.2GB' },
  ];

  const statusColors = {
    online: 'text-green-400 bg-green-400/10 border-green-400/20',
    offline: 'text-red-400 bg-red-400/10 border-red-400/20',
    syncing: 'text-yellow-400 bg-yellow-400/10 border-yellow-400/20',
  };

  const statusIcons = {
    online: IconCircleCheck,
    offline: IconCircleX,
    syncing: IconRefresh,
  };
</script>

<div class="bg-gray-900/60 backdrop-blur-md border border-glass-border rounded-xl overflow-hidden shadow-xl">
  <div class="p-5 border-b border-glass-border flex justify-between items-center bg-gray-900/40">
    <h3 class="font-bold text-lg text-white">Active Nodes</h3>
    <button class="text-sm text-blue-400 hover:text-blue-300 font-medium transition-colors">View All</button>
  </div>

  <div class="overflow-x-auto">
    <table class="w-full text-left text-sm text-gray-400">
      <thead class="bg-gray-800/40 text-gray-300 uppercase text-xs font-semibold backdrop-blur-sm">
        <tr>
          <th class="px-6 py-3">Node Name</th>
          <th class="px-6 py-3">Status</th>
          <th class="px-6 py-3">IP Address</th>
          <th class="px-6 py-3">Resources</th>
          <th class="px-6 py-3 text-right">Actions</th>
        </tr>
      </thead>
      <tbody class="divide-y divide-glass-border">
        {#each nodes as node}
          <tr class="hover:bg-glass-100 transition-colors group">
            <td class="px-6 py-4 font-medium text-white flex items-center gap-3">
              <div class="p-2 bg-gray-800/50 rounded-lg text-gray-400 border border-glass-border group-hover:border-gray-600 transition-colors">
                <IconServer size={18} />
              </div>
              {node.name}
            </td>
            <td class="px-6 py-4">
              <span class={`inline-flex items-center gap-1.5 px-2.5 py-1 rounded-full text-xs font-medium border shadow-sm ${statusColors[node.status]}`}>
                <svelte:component this={statusIcons[node.status]} size={12} />
                {node.status.charAt(0).toUpperCase() + node.status.slice(1)}
              </span>
            </td>
            <td class="px-6 py-4 font-mono text-xs text-gray-500 group-hover:text-gray-400">{node.ip}</td>
            <td class="px-6 py-4">
              <div class="flex items-center gap-4">
                <div class="flex flex-col gap-1">
                  <span class="text-xs text-gray-500">CPU</span>
                  <span class="text-white font-mono">{node.cpu}</span>
                </div>
                <div class="flex flex-col gap-1">
                  <span class="text-xs text-gray-500">RAM</span>
                  <span class="text-white font-mono">{node.ram}</span>
                </div>
              </div>
            </td>
            <td class="px-6 py-4 text-right">
              <button class="text-gray-500 hover:text-white p-1.5 rounded-lg hover:bg-gray-700/50 transition-colors">
                <IconDotsVertical size={18} />
              </button>
            </td>
          </tr>
        {/each}
      </tbody>
    </table>
  </div>
</div>

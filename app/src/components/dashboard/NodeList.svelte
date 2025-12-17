<script>
  import { IconServer, IconDotsVertical, IconCircleCheck, IconCircleX, IconRefresh, IconPower, IconTerminal } from '@tabler/icons-svelte';
  import { slide } from 'svelte/transition';
  import Card from '../ui/Card.svelte';
  import Badge from '../ui/Badge.svelte';

  // Mock data - in real app this would come from a store or API
  export let nodes = [];

  const statusIcons = {
    online: IconCircleCheck,
    offline: IconCircleX,
    syncing: IconRefresh,
  };

  function getBadgeVariant(status) {
    switch(status) {
      case 'online': return 'success';
      case 'offline': return 'danger';
      case 'syncing': return 'warning';
      default: return 'default';
    }
  }
</script>

<Card title="Active Nodes" hud={true} noPadding={true} class="h-full shadow-xl">
  <div slot="actions">
    <button class="text-sm text-secondary hover:text-secondary/80 font-medium transition-colors px-3 py-1 rounded-lg hover:bg-secondary/10 font-sans">View All</button>
  </div>

  <div class="overflow-x-auto">
    <table class="w-full text-left text-sm text-muted">
      <thead class="bg-surface/60 text-textBody uppercase text-xs font-semibold backdrop-blur-sm font-mono tracking-wider">
        <tr>
          <th class="px-6 py-3">Node Name</th>
          <th class="px-6 py-3">Status</th>
          <th class="px-6 py-3">IP Address</th>
          <th class="px-6 py-3">Resources</th>
          <th class="px-6 py-3 text-right">Actions</th>
        </tr>
      </thead>
      <tbody class="divide-y divide-glass-border">
        {#each nodes as node (node.id)}
          <tr class="hover:bg-glass-100 transition-colors group" transition:slide|local>
            <td class="px-6 py-4 font-medium text-textHeading flex items-center gap-3 font-sans">
              <div class="p-2 bg-surface/50 rounded-lg text-muted border border-glass-border group-hover:border-primary/50 group-hover:text-primary transition-all duration-300 shadow-sm">
                <IconServer size={18} />
              </div>
              {node.name}
            </td>
            <td class="px-6 py-4">
              <Badge variant={getBadgeVariant(node.status)} class="gap-1.5">
                <svelte:component this={statusIcons[node.status]} size={12} />
                {node.status.charAt(0).toUpperCase() + node.status.slice(1)}
              </Badge>
            </td>
            <td class="px-6 py-4 font-mono text-xs text-muted group-hover:text-textBody transition-colors">{node.ip}</td>
            <td class="px-6 py-4">
              <div class="flex items-center gap-4">
                <div class="flex flex-col gap-1">
                  <span class="text-xs text-muted font-sans">CPU</span>
                  <span class="text-textHeading font-mono">{node.cpu}</span>
                </div>
                <div class="flex flex-col gap-1">
                  <span class="text-xs text-muted font-sans">RAM</span>
                  <span class="text-textHeading font-mono">{node.ram}</span>
                </div>
              </div>
            </td>
            <td class="px-6 py-4 text-right">
              <div class="flex justify-end gap-2 opacity-0 group-hover:opacity-100 transition-opacity">
                <button class="text-muted hover:text-secondary p-1.5 rounded-lg hover:bg-secondary/10 transition-colors" title="Terminal">
                  <IconTerminal size={18} />
                </button>
                <button class="text-muted hover:text-danger p-1.5 rounded-lg hover:bg-danger/10 transition-colors" title="Restart">
                  <IconPower size={18} />
                </button>
                <button class="text-muted hover:text-textHeading p-1.5 rounded-lg hover:bg-surface/50 transition-colors">
                  <IconDotsVertical size={18} />
                </button>
              </div>
            </td>
          </tr>
        {/each}
      </tbody>
    </table>
  </div>
</Card>

---
title: "[APP] Cloud Dashboard UI"
labels:
  - frontend
  - svelte
  - dashboard
assignees: []
---

## User Story

**As a** user
**I want** a dashboard to manage all my nodes
**So that** I can see local and cloud nodes in one place

## Technical Specs

### Pages

```
app/src/pages/
├── index.astro          # Landing / Login
├── dashboard.astro      # Main dashboard
├── nodes/
│   ├── index.astro      # All nodes list
│   ├── local.astro      # Local node details
│   └── cloud.astro      # Cloud nodes management
├── billing.astro        # Subscription & invoices
└── settings.astro       # Account settings
```

### Dashboard Components

```svelte
<!-- NodeCard.svelte -->
<script>
  export let node;
</script>

<div class="node-card {node.type}">
  <div class="header">
    <span class="icon">{node.type === 'cloud' ? '☁️' : '📱'}</span>
    <h3>{node.name}</h3>
    <span class="status {node.status}">{node.status}</span>
  </div>

  <div class="stats">
    <div class="stat">
      <span class="label">Peers</span>
      <span class="value">{node.peersCount}</span>
    </div>
    <div class="stat">
      <span class="label">Storage</span>
      <span class="value">{formatBytes(node.storageUsed)}</span>
    </div>
    <div class="stat">
      <span class="label">Uptime</span>
      <span class="value">{formatUptime(node.uptime)}</span>
    </div>
  </div>

  <div class="actions">
    <a href={node.tunnelUrl} target="_blank">🌐 Open</a>
    <button onclick={() => restart(node.id)}>🔄</button>
    <button onclick={() => settings(node.id)}>⚙️</button>
  </div>
</div>
```

### UI Mockup

```
┌─────────────────────────────────────────────────────┐
│  🐝 Edge Hive                    [Pro] [Settings]   │
├─────────────────────────────────────────────────────┤
│                                                      │
│  📊 Dashboard                                        │
│  ───────────────                                     │
│                                                      │
│  Local Nodes (2)                  Cloud Nodes (1)   │
│  ┌─────────────┐ ┌─────────────┐ ┌─────────────┐   │
│  │📱 Android-1 │ │💻 Laptop    │ │☁️ Pro Node  │   │
│  │ 🟢 Online   │ │ 🟢 Online   │ │ 🟢 Online   │   │
│  │ 3 peers     │ │ 5 peers     │ │ 12 peers    │   │
│  │ 2.1 GB      │ │ 45 GB       │ │ 8.2 GB      │   │
│  └─────────────┘ └─────────────┘ └─────────────┘   │
│                                                      │
│  [+ Add Cloud Node - $25/mo]                        │
│                                                      │
│  📈 Usage This Month                                │
│  Storage: 55.3 GB / 100 GB                          │
│  Egress: 12.4 GB / 50 GB                            │
│                                                      │
└─────────────────────────────────────────────────────┘
```

### Responsive Design

- Mobile-first (works in Tauri Android)
- Desktop web dashboard
- Dark mode by default
- Glassmorphism cards

## Acceptance Criteria

- [ ] Dashboard shows all nodes (local + cloud)
- [ ] Real-time status updates (WebSocket)
- [ ] One-click cloud node provisioning
- [ ] Usage metrics visible
- [ ] Billing link works
- [ ] Responsive on mobile and desktop

## Branch

`feat/cloud-dashboard`

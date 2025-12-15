---
title: "[APP] Tauri 2.0 Mobile App with Astro + Svelte"
labels:
  - mobile
  - frontend
  - tauri
  - priority-high
assignees: []
---

## User Story

**As a** mobile user
**I want** a native Android app
**So that** I can manage my Edge Hive node without terminal

## Technical Specs

### Project Structure

```
app/
├── src-tauri/
│   ├── Cargo.toml
│   ├── tauri.conf.json
│   ├── capabilities/
│   │   └── default.json
│   └── src/
│       ├── main.rs
│       ├── lib.rs
│       └── commands.rs
├── src/
│   ├── layouts/
│   │   └── Layout.astro
│   ├── pages/
│   │   ├── index.astro
│   │   ├── peers.astro
│   │   └── settings.astro
│   └── components/
│       ├── Dashboard.svelte
│       ├── PeerList.svelte
│       ├── NodeStatus.svelte
│       └── Settings.svelte
├── astro.config.mjs
├── svelte.config.js
├── tailwind.config.js
└── package.json
```

### Tauri Commands

```rust
// src-tauri/src/commands.rs

#[tauri::command]
async fn get_node_status() -> Result<NodeStatus, String> {
    Ok(NodeStatus {
        peer_id: identity.peer_id(),
        name: identity.name(),
        uptime: service.uptime(),
        peers_count: discovery.peers().len(),
    })
}

#[tauri::command]
async fn get_peers() -> Result<Vec<PeerInfo>, String> {
    Ok(discovery.peers())
}

#[tauri::command]
async fn start_server(port: u16) -> Result<String, String> {
    server.start(port).await
}

#[tauri::command]
async fn stop_server() -> Result<(), String> {
    server.stop().await
}
```

### Svelte Components

```svelte
<!-- Dashboard.svelte -->
<script>
  import { invoke } from '@tauri-apps/api/core';
  import { onMount } from 'svelte';

  let status = $state(null);
  let error = $state(null);

  onMount(async () => {
    try {
      status = await invoke('get_node_status');
    } catch (e) {
      error = e;
    }
  });
</script>

<div class="dashboard">
  {#if status}
    <h2>{status.name}</h2>
    <p>Peers: {status.peers_count}</p>
    <p>Uptime: {formatUptime(status.uptime)}</p>
  {:else if error}
    <p class="error">{error}</p>
  {:else}
    <p>Loading...</p>
  {/if}
</div>
```

### Build Commands

```bash
# Development
npm run dev        # Astro dev server
npm run tauri dev  # Full Tauri dev

# Android
npm run tauri android init
npm run tauri android dev
npm run tauri android build  # Produces APK
```

## Acceptance Criteria

- [ ] Tauri project initializes correctly
- [ ] Astro + Svelte integration works
- [ ] IPC commands communicate with Rust
- [ ] Dashboard shows node status
- [ ] Peer list updates in real-time
- [ ] APK builds successfully
- [ ] APK installs and runs on Android

## Branch

`feat/tauri-mobile-app`

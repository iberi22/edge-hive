---
title: "Edge Hive on Termux (Android)"
type: GUIDE
id: "guide-termux"
created: 2024-07-29
updated: 2024-07-29
agent: jules
model: gemini-1.5-pro
requested_by: user
summary: |
  A comprehensive guide to installing, configuring, and using Edge Hive on
  Termux for Android devices.
keywords: [termux, android, mobile, installation, guide]
tags: ["#termux", "#android", "#guide"]
project: Edge Hive
module: deployment
language: bash
priority: high
status: draft
confidence: 0.95
token_estimate: 1200
complexity: moderate
---

# Edge Hive on Termux (Android)

## Requirements
- Android 7.0+ (ARM64)
- Termux (F-Droid version recommended)
- 500 MB space free
- Internet connection

## Quick Installation
```bash
bash <(curl -fsSL https://edgehive.dev/install-termux.sh)
```

## Basic Usage
To start the server:
```bash
edge-hive start
```

To view logs:
```bash
edge-hive logs
```

To stop the server:
```bash
edge-hive stop
```

## Configuration
The main configuration file is located at `~/.config/edge-hive/config.toml`.
```toml
[network]
tor_enabled = true
libp2p_port = 4001

[storage]
db_path = "~/.local/share/edge-hive/edge-hive.db"
```

## Troubleshooting
**Error: Failed to bootstrap Tor**
- **Cause:** Network is blocking Tor.
- **Solution:** Use Tor bridges.

**Error: Permission denied (onion service)**
- **Cause:** Filesystem permissions are incorrect.
- **Solution:** `chmod 700 ~/.tor/`

**Error: No route to host**
- **Cause:** IPv6 issues.
- **Solution:** Disable IPv6 in the `arti` config.

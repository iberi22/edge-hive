---
title: "[DEPLOY] Windows Desktop App Build & Release"
labels:
  - deployment
  - windows
  - jules
  - MVP
assignees: []
---

## Description

Build and package the Edge Hive Admin as a Windows desktop application.

## Tasks

- [ ] Configure Tauri for Windows build
- [ ] Create Windows installer (MSI or NSIS)
- [ ] Add app icon and metadata
- [ ] Test on Windows 10/11
- [ ] Create GitHub Release with artifacts
- [ ] Add auto-update functionality (optional)

## Build Commands

```powershell
# Prerequisites
cargo install tauri-cli

# Build for Windows
cd edge-hive-admin
npm install
npm run tauri build

# Output: src-tauri/target/release/bundle/
# - edge-hive-admin.exe
# - edge-hive-admin_x.x.x_x64_en-US.msi
```

## Release Checklist

- [ ] Version bumped in `Cargo.toml` and `package.json`
- [ ] Build passes on Windows
- [ ] Installer works correctly
- [ ] App starts and all features work
- [ ] Create GitHub release with:
  - `edge-hive-admin-windows-x64.msi`
  - `edge-hive-admin-windows-x64.exe` (portable)
  - Release notes

## Acceptance Criteria

- [ ] One-click installation on Windows
- [ ] App runs without errors
- [ ] All backend commands work

## Estimated Effort
2-3 hours

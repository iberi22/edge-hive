---
title: "[FEAT] Implement Real Web Terminal with xterm.js and Tauri PTY"
labels:
  - enhancement
  - frontend
  - rust
  - copilot
assignees: []
---

## Description

Connect the frontend `xterm.js` component to a real backend shell using Tauri.

## Tasks

- [x] Add `portable-pty` crate to `src-tauri` ✅
- [x] Implement a Tauri command to spawn a shell (bash/zsh/powershell) ✅
- [x] Create Event channel to stream stdin/stdout between frontend and backend ✅
- [x] Handle terminal resizing events ✅
- [x] Create Terminal.svelte component with xterm.js ✅
- [x] Add glassmorphism styling to match dashboard ✅
- [x] Add terminal navigation to sidebar ✅
- [ ] Ensure security (restrict access to authenticated users) - Future
- [ ] Test on Android/Termux environment - Future

## ✅ Completion Status

**COMPLETED** (2025-12-15)

### What was implemented

**Frontend:**

- Terminal.svelte component with xterm.js, FitAddon, WebLinksAddon
- Custom dark theme matching dashboard aesthetic
- Glassmorphism styling with window controls
- Auto-resize with viewport observer
- /terminal page with full-height layout
- Terminal icon added to sidebar navigation

**Backend:**

- portable-pty integration for cross-platform PTY
- terminal_spawn: Spawns PowerShell (Windows) or sh (Linux/macOS)
- terminal_write: Sends input to PTY
- terminal_resize: Handles terminal dimension changes
- Background thread streaming PTY output via Tauri events
- TerminalState management with Arc<Mutex<T>>

**Features:**

- ✅ Real-time bidirectional communication
- ✅ Interactive shell access from web UI
- ✅ Cross-platform (Windows/Linux/macOS)
- ✅ Responsive design
- ✅ Glassmorphism aesthetic

### Commits

- `90507a7` - feat(terminal): implement web terminal with xterm.js + PTY

### Next Phase

- Authentication/authorization for terminal access
- Android/Termux testing and optimization

## Goal

Allow users to manage the VPS/Termux environment directly from the web UI.

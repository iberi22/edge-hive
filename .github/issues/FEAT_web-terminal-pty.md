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
- [ ] Add `portable-pty` crate to `src-tauri`.
- [ ] Implement a Tauri command to spawn a shell (bash/zsh/powershell).
- [ ] Create a WebSocket or Event channel to stream stdin/stdout between frontend and backend.
- [ ] Handle terminal resizing events.
- [ ] Ensure security (restrict access to authenticated users).

## Goal
Allow users to manage the VPS/Termux environment directly from the web UI.

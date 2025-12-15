---
title: "App Design: Modernize UI/UX (Glassmorphism + Dark Mode)"
labels:
  - enhancement
  - ui/ux
  - mobile
assignees: []
---

## Description
Update the mobile app UI to follow 2024/2025 design trends:
- **Dark Mode First**: Optimized for OLED screens.
- **Glassmorphism**: Use translucent surfaces with backdrop blur for depth.
- **Minimalism**: Clean typography, reduced noise.
- **Animations**: Subtle glow effects and interactions.

## Technical Stats
- **Framework**: Tailwind CSS + Astro + Svelte
- **Icons**: Emoji/SVG
- **Fonts**: System sans-serif

## Tasks
- [x] Create `tailwind.config.mjs` with custom theme
- [x] Update `Layout.astro` with deep dark background
- [x] Refactor `Dashboard.svelte` with new navigation
- [x] Refactor `NodeCard.svelte` with glow effects
- [x] Refactor `CloudSection.svelte` for "Hive RAID"

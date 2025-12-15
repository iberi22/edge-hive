---
title: "UI/UX Analysis & Improvement Plan"
type: ANALYSIS
id: "analysis-ui-ux-admin"
created: 2025-12-15
updated: 2025-12-15
agent: copilot
model: gemini-3-pro
requested_by: user
summary: |
  Analysis of the current Admin UI/UX compared to Dokploy standards.
  Proposes specific improvements for Sidebar, Dashboard, and Components.
keywords: [ui, ux, admin, dashboard, dokploy, astro, svelte, tailwind]
tags: ["#ui", "#ux", "#admin", "#analysis"]
project: edge-hive-app
---

# UI/UX Analysis & Improvement Plan

## 1. Current State Assessment

The current application (`app/`) is in a **skeleton state**.

- **Framework:** Astro + Svelte + Tailwind CSS.
- **Layout:** Basic Sidebar + Header + Main Content.
- **Styling:** Minimal Tailwind utility classes.
- **Components:** Functional but unstyled (no icons, no active states).
- **Content:** Placeholders.

## 2. Comparison with Dokploy (Reference)

[Dokploy](https://github.com/dokploy/dokploy) sets a high standard for VPS management UIs. Key features missing in Edge Hive:

| Feature | Dokploy Standard | Current Edge Hive |
|---------|------------------|-------------------|
| **Navigation** | Icons, Active States, Grouping, Collapsible | Text links list |
| **Dashboard** | Real-time Metrics (CPU/RAM), Status Indicators | Static text |
| **Visuals** | Cards, Modals, Drawers, Consistent Palette | Raw HTML elements |
| **Interactivity** | Real-time logs, Terminal, Action Buttons | None |
| **Feedback** | Toasts, Loading States, Confirmations | None |

## 3. Improvement Plan

### Phase 1: Foundation & Navigation (The "Shell")

**Goal:** Create a professional-looking admin shell.

1. **Enhanced Sidebar (`Sidebar.svelte`):**
    - **Icons:** Integrate `@tabler/icons-svelte` for every menu item.
    - **Active State:** Highlight the current page based on URL.
    - **Logo:** Add a proper branding area at the top.
    - **User Profile:** Add a user menu at the bottom.
    - **Responsive:** Mobile drawer support.

2. **Header (`Header.svelte`):**
    - **Breadcrumbs:** Show current location path.
    - **System Status:** Small indicator (Green/Red) for backend connection.
    - **Actions:** Quick actions (e.g., "Restart Node").

### Phase 2: Dashboard & Visualization

**Goal:** Provide immediate value upon login.

1. **Stats Cards Component:**
    - Create a reusable `StatsCard.svelte`.
    - Display: CPU Usage, RAM Usage, Disk Space, Network Traffic.
    - Use Sparklines (via D3 or simple SVG) for trends.

2. **Node List View:**
    - Table/Grid view for Nodes.
    - Status badges (Online, Offline, Syncing).
    - Action menu (three dots) for operations.

### Phase 3: Advanced Components (Glassmorphism & Polish)

**Goal:** Modernize the aesthetic.

1. **Glassmorphism:**
    - Use `bg-gray-800/50 backdrop-blur-md` for panels.
    - Add subtle borders `border-white/10`.

2. **Terminal Integration:**
    - Style the `xterm.js` container to look like a native terminal window.

## 4. Proposed Component Structure

```
src/
  components/
    ui/
      Card.svelte
      Button.svelte
      Badge.svelte
      Input.svelte
      Modal.svelte
    layout/
      Sidebar.svelte
      Header.svelte
      PageHeader.svelte
    dashboard/
      StatsCard.svelte
      ResourceChart.svelte
```

## 5. Action Items (Immediate)

1. [ ] Install `@tabler/icons-svelte` (Already in package.json).
2. [ ] Refactor `Sidebar.svelte` to use an array of route objects with icons.
3. [ ] Create `Card.svelte` base component.
4. [ ] Implement "Glassmorphism" utility classes in `global.css` or Tailwind config.

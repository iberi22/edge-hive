---
title: "Refactor: Optimize Metrics Event Loop"
labels:
  - refactor
assignees: []
---

## Description
The current system_metrics event loop in lib.rs is a simple span. Consider moving this to a dedicated service or actor for better performance and control.

---
title: "Cloud: Implement Multi-Cloud RAID & QR Pairing"
labels:
  - enhancement
  - architecture
  - cloud
assignees: []
---

## Description
Implement functionality for "Hive RAID" and QR code device linking.

### Hive RAID
- **Concept**: Cluster multiple nodes (AWS + GCP + Android) into a single logical "Hive".
- **Benefits**: High Availability (HA), Speed.
- **Implementation**:
    - [ ] Update `edge-hive-discovery` to support RAID groups
    - [ ] Update `edge-hive-cloud` to support GCP (in addition to AWS)

### QR Pairing
- **Concept**: Easy onboarding for new Android nodes.
- **Flow**:
    1. Primary node displays QR (Private Key / Peering Info).
    2. New Android node scans QR.
    3. Auto-configures `edge-hive-identity`.

## Tasks
- [ ] Design QR payload schema
- [ ] Implement QR scan/gen in Tauri app
- [ ] Implement RAID logic in core

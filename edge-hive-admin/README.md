# Edge Hive Admin

<div align="center">
<img width="1200" height="475" alt="Edge Hive Dashboard" src="https://github.com/user-attachments/assets/0aa67016-6eaf-458a-adb2-6e31a0763ed6" />
</div>

**Edge Hive Admin** is the centralized control plane for the Edge Hive node ecosystem. It provides a cyberpunk-themed, real-time dashboard for managing Compute, Networking, and Storage resources across the distributed grid.

## Architecture

This is a hybrid **Tauri** application:
- **Frontend**: React (Vite) + TypeScript + TailwindCSS.
- **Backend**: Rust (Tauri Core) integrating `edge-hive-*` crates.
- **Communication**: Tauri Commands (RPC) & Events (Real-time).

### Key Modules
| Module | Crate | Description |
|--------|-------|-------------|
| **Auth** | `edge-hive-auth` | RBAC, Identity, JWT, P2P Ops |
| **Billing** | `edge-hive-billing` | Usage metering, Invoicing, Plans |
| **Cache** | `edge-hive-cache` | L1 (Moka) / L2 (Redis) caching layer |
| **Tunnel** | `edge-hive-tunnel` | Onion routing, Hidden Services, Federation |
| **Chaos** | `local` | Disaster simulation and resilience testing |

## Development

### Prerequisites
- **Node.js** (v18+)
- **Rust** (Stable)
- **Visual Studio Build Tools** (Windows) or `build-essential` (Linux)

### Quick Start
1. **Install Dependencies**
   ```bash
   npm install
   ```
2. **Run Development Server** (Frontend + Tauri)
   ```bash
   npm run tauri dev
   ```
3. **Backend-Only Check**
   ```bash
   cd src-tauri
   cargo check
   ```

## Testing
Run End-to-End tests with Playwright:
```bash
npx playwright test
```

## License
MIT

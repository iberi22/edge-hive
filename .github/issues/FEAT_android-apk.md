---
title: "Build Android APK with Tauri"
labels:
  - enhancement
  - android
  - mobile
  - tauri
  - priority-medium
assignees: []
---

## 🎯 Objective

Package Edge Hive as a native Android APK using Tauri 2.0, with:

- Astro + Svelte UI (already implemented in `app/`)
- Rust backend (edge-hive-core)
- No root required (pure Tauri, not Termux)

## 📋 Context

Current state:

- ✅ Termux deployment works (PR #22)
- ✅ Admin Dashboard (Astro + Svelte) exists (PR #23)
- 🔵 Android APK pending

ARCHITECTURE.md specifies Tauri 2.0 for cross-platform apps.

## 🛠️ Prerequisites (Blockers)

**⚠️ DO NOT START until E2E testing is complete!**

Blocked by:

- [ ] Tor integration working (`.github/issues/FEAT_tor-core-integration.md`)
- [ ] libp2p discovery working (`.github/issues/FEAT_libp2p-discovery.md`)
- [ ] E2E tests passing (`.github/issues/TASK_e2e-testing.md`)

## 🛠️ Tasks

### Phase 1: Tauri Android Setup

- [ ] Install Android SDK + NDK
- [ ] Configure `app/src-tauri/tauri.conf.json` for Android
- [ ] Add Android-specific permissions (INTERNET, FOREGROUND_SERVICE)
- [ ] Setup Rust target: `aarch64-linux-android`

### Phase 2: Build Configuration

- [ ] Create `app/src-tauri/gen/android/` structure
- [ ] Configure Gradle build scripts
- [ ] Add signing key for APK (debug + release)
- [ ] Test local build: `npm run tauri android build`

### Phase 3: UI Adaptation

- [ ] Test Astro dashboard on mobile WebView
- [ ] Adjust responsive layout for small screens
- [ ] Add mobile-specific UI (bottom nav, drawer)
- [ ] Implement system tray for background service

### Phase 4: Permissions & Services

- [ ] Request INTERNET permission
- [ ] Implement foreground service for Tor/libp2p
- [ ] Add battery optimization exclusion prompt
- [ ] Persist state across app restarts

### Phase 5: Testing

- [ ] Test on Android 12+ (API 31+)
- [ ] Test on Android 10 (API 29) for compatibility
- [ ] Validate onion service starts on app launch
- [ ] Validate P2P discovery works on mobile network

### Phase 6: Distribution

- [ ] Generate signed APK for GitHub Releases
- [ ] Create F-Droid metadata (future)
- [ ] Document installation: "Enable Unknown Sources"
- [ ] Add QR code for APK download

## 📦 Dependencies

```json
{
  "devDependencies": {
    "@tauri-apps/cli": "^2.0.0"
  },
  "dependencies": {
    "@tauri-apps/api": "^2.0.0"
  }
}
```

## ✅ Success Criteria

1. APK builds successfully: `edge-hive-v0.1.0-android-arm64.apk`
2. APK size < 30MB
3. App starts and shows dashboard on Android 12+
4. Tor onion service runs in background
5. Can discover peers on mobile WiFi
6. Survives app backgrounding (Android battery optimization)

## 🔗 Related

- Tauri config: `app/src-tauri/tauri.conf.json`
- Dashboard: `app/src/` (Astro + Svelte)
- Backend: `src/main.rs` (will be invoked by Tauri)
- E2E Testing: `.github/issues/TASK_e2e-testing.md` (prerequisite)

## 📚 References

- [Tauri Android Guide](https://v2.tauri.app/develop/android/)
- [Android Permissions](https://developer.android.com/guide/topics/permissions/overview)
- [Foreground Services](https://developer.android.com/develop/background-work/services/foreground-services)

## 🚧 Post-Validation Only

This issue should remain `blocked` until E2E tests pass.

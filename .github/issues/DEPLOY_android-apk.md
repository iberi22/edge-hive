---
title: "[DEPLOY] Android APK Build"
labels:
  - deployment
  - android
  - jules
  - MVP
assignees: []
---

## Description

Build Android APK for Edge Hive Admin mobile app.

## Prerequisites

- Android SDK
- Android NDK
- Rust targets: `aarch64-linux-android`, `armv7-linux-androideabi`

## Tasks

- [ ] Configure Tauri for Android build
- [ ] Set up Android signing keys
- [ ] Build debug APK
- [ ] Build release APK
- [ ] Test on physical device
- [ ] Create GitHub Release with APK

## Build Commands

```bash
# Install Android targets
rustup target add aarch64-linux-android armv7-linux-androideabi x86_64-linux-android i686-linux-android

# Set up Android SDK
export ANDROID_HOME=$HOME/Android/Sdk
export NDK_HOME=$ANDROID_HOME/ndk/25.2.9519653

# Initialize Android project
cd edge-hive-admin
npm run tauri android init

# Build debug APK
npm run tauri android build -- --target aarch64

# Build release APK
npm run tauri android build -- --release

# Output: src-tauri/gen/android/app/build/outputs/apk/
```

## Signing Configuration

```properties
# android/key.properties
storePassword=<password>
keyPassword=<password>
keyAlias=edge-hive
storeFile=<path-to-keystore>
```

## Release APK Locations

- Debug: `src-tauri/gen/android/app/build/outputs/apk/universal/debug/app-universal-debug.apk`
- Release: `src-tauri/gen/android/app/build/outputs/apk/universal/release/app-universal-release.apk`

## Acceptance Criteria

- [ ] APK installs on Android 8+
- [ ] All UI screens work
- [ ] Backend commands work
- [ ] App doesn't crash

## Estimated Effort
4-6 hours

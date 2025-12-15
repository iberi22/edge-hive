---
title: "[INFRA] Termux Installation Script"
labels:
  - infrastructure
  - android
  - termux
assignees: []
---

## User Story

**As an** Android user
**I want** a simple installation script for Termux
**So that** I can run Edge Hive on my old phone

## Technical Specs

### Script: `scripts/termux-install.sh`

```bash
#!/data/data/com.termux/files/usr/bin/bash
# Edge Hive Termux Installer

set -e

echo "🐝 Edge Hive Termux Installer"
echo "=============================="

# Detect architecture
ARCH=$(uname -m)
case $ARCH in
    aarch64) TARGET="aarch64-linux-android" ;;
    armv7l)  TARGET="armv7-linux-androideabi" ;;
    x86_64)  TARGET="x86_64-linux-android" ;;
    *)       echo "❌ Unsupported architecture: $ARCH"; exit 1 ;;
esac

echo "📱 Detected architecture: $ARCH"

# Update packages
pkg update -y
pkg upgrade -y

# Install dependencies
pkg install -y rust openssl libsodium

# Install from crates.io or GitHub
if [ "$1" == "--dev" ]; then
    echo "🔧 Building from source..."
    pkg install -y git
    git clone https://github.com/your-org/edge-hive.git
    cd edge-hive
    cargo build --release
    cp target/release/edge-hive $PREFIX/bin/
else
    echo "📦 Installing from release..."
    RELEASE_URL="https://github.com/your-org/edge-hive/releases/latest/download/edge-hive-$TARGET"
    curl -sSL "$RELEASE_URL" -o $PREFIX/bin/edge-hive
    chmod +x $PREFIX/bin/edge-hive
fi

# Initialize
echo "🔑 Initializing node identity..."
edge-hive init

# Setup service (optional)
echo "⚙️ Setting up background service..."
mkdir -p ~/.termux/boot
cat > ~/.termux/boot/edge-hive.sh << 'EOF'
#!/data/data/com.termux/files/usr/bin/bash
termux-wake-lock
edge-hive serve >> ~/.edge-hive/server.log 2>&1 &
EOF
chmod +x ~/.termux/boot/edge-hive.sh

echo ""
echo "✅ Edge Hive installed successfully!"
echo ""
echo "Start server:  edge-hive serve"
echo "Check status:  edge-hive status"
echo "View logs:     tail -f ~/.edge-hive/server.log"
```

### Requirements

- Termux app from F-Droid (not Play Store)
- Termux:Boot addon for auto-start
- `termux-wake-lock` for preventing Android from killing process

## Acceptance Criteria

- [ ] Script detects architecture correctly
- [ ] Dependencies install successfully
- [ ] Binary downloads and runs
- [ ] Node initializes identity
- [ ] Boot script works for auto-start
- [ ] Wake lock prevents process kill
- [ ] Works on Android 8+ with Termux

## Branch

`feat/termux-installer`

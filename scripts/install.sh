#!/bin/bash
#
# Edge Hive - Universal Installation Script
# https://edge-hive.io
#
# Usage:
#   curl -sSL https://edge-hive.io/install.sh | bash
#

set -e

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

echo -e "${BLUE}"
echo "  ╔══════════════════════════════════════╗"
echo "  ║     🐝 Edge Hive Installer           ║"
echo "  ╚══════════════════════════════════════╝"
echo -e "${NC}"

# Detect OS and architecture
OS=$(uname -s | tr '[:upper:]' '[:lower:]')
ARCH=$(uname -m)

# Map architecture names
case $ARCH in
    x86_64|amd64)
        ARCH="x86_64"
        ;;
    aarch64|arm64)
        ARCH="aarch64"
        ;;
    armv7l|armv8l)
        ARCH="armv7"
        ;;
esac

# Detect platform
case $OS in
    linux)
        if [ -d "/data/data/com.termux" ]; then
            PLATFORM="android"
            TARGET="${ARCH}-linux-android"
        else
            PLATFORM="linux"
            TARGET="${ARCH}-unknown-linux-gnu"
        fi
        ;;
    darwin)
        PLATFORM="macos"
        TARGET="${ARCH}-apple-darwin"
        ;;
    mingw*|msys*|cygwin*)
        PLATFORM="windows"
        TARGET="${ARCH}-pc-windows-msvc"
        ;;
    *)
        echo -e "${RED}✗${NC} Unsupported OS: $OS"
        exit 1
        ;;
esac

echo -e "${GREEN}✓${NC} Detected: $PLATFORM ($TARGET)"

# Termux-specific installation
if [ "$PLATFORM" == "android" ]; then
    echo -e "${BLUE}📱 Detected Termux environment${NC}"

    # Download and run Termux-specific script
    if [ -f "./scripts/install-termux.sh" ]; then
        bash ./scripts/install-termux.sh "$@"
    else
        curl -sSL https://raw.githubusercontent.com/your-org/edge-hive/main/scripts/install-termux.sh | bash -s -- "$@"
    fi
    exit 0
fi

# Determine install location
if [ -w "/usr/local/bin" ]; then
    INSTALL_DIR="/usr/local/bin"
elif [ -w "$HOME/.local/bin" ]; then
    INSTALL_DIR="$HOME/.local/bin"
else
    INSTALL_DIR="$HOME/.edge-hive/bin"
    mkdir -p "$INSTALL_DIR"
fi

echo -e "${BLUE}📥 Downloading Edge Hive for $TARGET...${NC}"

RELEASE_URL="https://github.com/your-org/edge-hive/releases/latest/download/edge-hive-$TARGET"

# Download binary
if command -v curl &> /dev/null; then
    curl -L --progress-bar "$RELEASE_URL" -o "$INSTALL_DIR/edge-hive"
elif command -v wget &> /dev/null; then
    wget -q --show-progress "$RELEASE_URL" -O "$INSTALL_DIR/edge-hive"
else
    echo -e "${RED}✗${NC} Neither curl nor wget found"
    exit 1
fi

chmod +x "$INSTALL_DIR/edge-hive"

echo -e "${GREEN}✓${NC} Installed to: $INSTALL_DIR/edge-hive"

# Add to PATH if needed
if [[ ":$PATH:" != *":$INSTALL_DIR:"* ]]; then
    echo ""
    echo -e "${YELLOW}⚠${NC} Add to your PATH:"
    echo "   export PATH=\"\$PATH:$INSTALL_DIR\""

    # Add to shell config
    if [ -f "$HOME/.bashrc" ]; then
        echo "export PATH=\"\$PATH:$INSTALL_DIR\"" >> "$HOME/.bashrc"
        echo -e "${GREEN}✓${NC} Added to ~/.bashrc"
    elif [ -f "$HOME/.zshrc" ]; then
        echo "export PATH=\"\$PATH:$INSTALL_DIR\"" >> "$HOME/.zshrc"
        echo -e "${GREEN}✓${NC} Added to ~/.zshrc"
    fi
fi

# Initialize
echo ""
echo -e "${BLUE}🔑 Initializing node identity...${NC}"
"$INSTALL_DIR/edge-hive" init

# Print summary
echo ""
echo -e "${GREEN}════════════════════════════════════════${NC}"
echo -e "${GREEN}   ✅ Edge Hive installed successfully!${NC}"
echo -e "${GREEN}════════════════════════════════════════${NC}"
echo ""
echo "Quick Start:"
echo "  edge-hive serve           # Start the server"
echo "  edge-hive serve --tunnel  # Start with Cloudflare tunnel"
echo "  edge-hive status          # Check node status"
echo ""
echo "📚 Documentation: https://edge-hive.io/docs"

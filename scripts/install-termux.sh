#!/data/data/com.termux/files/usr/bin/bash
#
# Edge Hive - Termux Installation Script
# https://edge-hive.io
#
# Usage:
#   curl -sSL https://edge-hive.io/install.sh | bash
#   or
#   bash install-termux.sh [--dev]
#

set -e

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}"
echo "  ╔══════════════════════════════════════╗"
echo "  ║     🐝 Edge Hive Installer           ║"
echo "  ║     Termux Edition                   ║"
echo "  ╚══════════════════════════════════════╝"
echo -e "${NC}"

# Detect architecture
ARCH=$(uname -m)
case $ARCH in
    aarch64)
        TARGET="aarch64-linux-android"
        echo -e "${GREEN}✓${NC} Architecture: ARM64 (aarch64)"
        ;;
    armv7l|armv8l)
        TARGET="armv7-linux-androideabi"
        echo -e "${GREEN}✓${NC} Architecture: ARM32 (armv7)"
        ;;
    x86_64)
        TARGET="x86_64-linux-android"
        echo -e "${GREEN}✓${NC} Architecture: x86_64"
        ;;
    *)
        echo -e "${RED}✗${NC} Unsupported architecture: $ARCH"
        exit 1
        ;;
esac

# Check if Termux
if [ ! -d "/data/data/com.termux" ]; then
    echo -e "${YELLOW}⚠${NC} Not running in Termux. This script is designed for Termux."
    echo "   For other platforms, see: https://edge-hive.io/install"
    exit 1
fi

echo ""
echo -e "${BLUE}📦 Updating packages...${NC}"
pkg update -y
pkg upgrade -y

echo ""
echo -e "${BLUE}📦 Installing dependencies...${NC}"
pkg install -y rust clang openssl libsodium libevent zstd cloudflared termux-services

# Check for --dev flag (build from source)
if [ "$1" == "--dev" ]; then
    echo ""
    echo -e "${BLUE}🔧 Development mode: Building from source...${NC}"

    # Install Rust
    if ! command -v rustc &> /dev/null; then
        echo -e "${BLUE}📦 Installing Rust...${NC}"
        pkg install -y rust
    fi

    echo -e "${GREEN}✓${NC} Rust: $(rustc --version)"

    # Install git
    pkg install -y git

    # Clone repository
    if [ -d "edge-hive" ]; then
        echo "   Updating existing repository..."
        cd edge-hive
        git pull
    else
        echo "   Cloning repository..."
        git clone https://github.com/your-org/edge-hive.git
        cd edge-hive
    fi

    # Build
    echo ""
    echo -e "${BLUE}🔨 Building Edge Hive (this may take a while)...${NC}"
    cargo build --release -p edge-hive-core

    # Install binary
    cp target/release/edge-hive $PREFIX/bin/
    chmod +x $PREFIX/bin/edge-hive

else
    echo ""
    echo -e "${BLUE}📥 Downloading Edge Hive binary...${NC}"

    RELEASE_URL="https://github.com/your-org/edge-hive/releases/latest/download/edge-hive-$TARGET"

    # Download with progress
    if command -v curl &> /dev/null; then
        curl -L --progress-bar "$RELEASE_URL" -o $PREFIX/bin/edge-hive
    elif command -v wget &> /dev/null; then
        wget -q --show-progress "$RELEASE_URL" -O $PREFIX/bin/edge-hive
    else
        echo -e "${RED}✗${NC} Neither curl nor wget found"
        exit 1
    fi

    chmod +x $PREFIX/bin/edge-hive
fi

echo ""
echo -e "${GREEN}✓${NC} Edge Hive installed to: $PREFIX/bin/edge-hive"

# Initialize identity
echo ""
echo -e "${BLUE}🔑 Initializing node identity...${NC}"
edge-hive init

# Setup auto-start (optional)
echo ""
echo -e "${BLUE}⚙️  Setting up auto-start on boot...${NC}"

# 1. Instalar Termux:Boot (F-Droid)
pkg install termux-services

# 2. Crear servicio
mkdir -p ~/.termux/boot
cat > ~/.termux/boot/edge-hive.sh << 'EOF'
#!/data/data/com.termux/files/usr/bin/bash
termux-wake-lock
edge-hive start --daemon
EOF

chmod +x ~/.termux/boot/edge-hive.sh

# 3. Activar servicio
sv-enable edge-hive

echo -e "${GREEN}✓${NC} Auto-start configured"
echo ""
echo -e "${YELLOW}📱 Note:${NC} For auto-start to work, install Termux:Boot from F-Droid"
echo "   https://f-droid.org/packages/com.termux.boot/"

# Print summary
echo ""
echo -e "${GREEN}════════════════════════════════════════${NC}"
echo -e "${GREEN}   ✅ Edge Hive installed successfully!${NC}"
echo -e "${GREEN}════════════════════════════════════════${NC}"
echo ""
echo "Quick Start:"
echo "  ${BLUE}edge-hive serve${NC}           Start the server"
echo "  ${BLUE}edge-hive serve --tunnel${NC}  Start with Cloudflare tunnel"
echo "  ${BLUE}edge-hive status${NC}          Check node status"
echo "  ${BLUE}edge-hive peers${NC}           List discovered peers"
echo ""
echo "Server logs: ~/.edge-hive/server.log"
echo ""
echo "📚 Documentation: https://edge-hive.io/docs"
echo "🐛 Issues: https://github.com/your-org/edge-hive/issues"
echo ""

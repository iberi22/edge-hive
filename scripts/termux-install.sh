#!/data/data/com.termux/files/usr/bin/bash
# Edge Hive Termux Installer
# One-command install: curl -fsSL https://raw.githubusercontent.com/iberi22/edge-hive/master/scripts/termux-install.sh | bash

set -e

echo ""
echo "🐝 ═══════════════════════════════════════════"
echo "   Edge Hive - Termux Installer"
echo "═══════════════════════════════════════════════"
echo ""

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Check if running in Termux
if [ ! -d "/data/data/com.termux" ]; then
    echo -e "${RED}Error: This script must be run in Termux${NC}"
    exit 1
fi

echo -e "${YELLOW}[1/6]${NC} Updating packages..."
pkg update -y && pkg upgrade -y

echo -e "${YELLOW}[2/6]${NC} Installing dependencies..."
pkg install -y rust openssl-tool pkg-config git clang make libsqlite

echo -e "${YELLOW}[3/6]${NC} Setting up installation directory..."
INSTALL_DIR="$HOME/edge-hive"
BIN_DIR="$HOME/.local/bin"
CONFIG_DIR="$HOME/.config/edge-hive"

mkdir -p "$BIN_DIR"
mkdir -p "$CONFIG_DIR"

if [ -d "$INSTALL_DIR" ]; then
    echo "Found existing installation, updating..."
    cd "$INSTALL_DIR"
    git fetch origin master
    git reset --hard origin/master
else
    echo "Cloning repository..."
    git clone --depth 1 https://github.com/iberi22/edge-hive.git "$INSTALL_DIR"
fi

echo -e "${YELLOW}[4/6]${NC} Building edge-hive-core (this may take 10-20 minutes)..."
cd "$INSTALL_DIR"

# Use release build for performance
CARGO_TERM_COLOR=always cargo build --release -p edge-hive-core 2>&1 | tail -20

if [ ! -f "target/release/edge-hive-core" ]; then
    echo -e "${RED}Build failed. Check errors above.${NC}"
    exit 1
fi

echo -e "${YELLOW}[5/6]${NC} Installing binaries..."
cp target/release/edge-hive-core "$BIN_DIR/"

# Create helper scripts
cat > "$BIN_DIR/edge-hive-start" << 'SCRIPT'
#!/data/data/com.termux/files/usr/bin/bash
PORT=${1:-8080}
echo "Starting Edge Hive on port $PORT..."
nohup edge-hive-core serve --port $PORT > ~/.config/edge-hive/server.log 2>&1 &
echo $! > ~/.config/edge-hive/server.pid
echo "Server started! PID: $(cat ~/.config/edge-hive/server.pid)"
echo "Log: ~/.config/edge-hive/server.log"
echo "URL: http://localhost:$PORT"
SCRIPT

cat > "$BIN_DIR/edge-hive-stop" << 'SCRIPT'
#!/data/data/com.termux/files/usr/bin/bash
if [ -f ~/.config/edge-hive/server.pid ]; then
    kill $(cat ~/.config/edge-hive/server.pid) 2>/dev/null && echo "Server stopped"
    rm ~/.config/edge-hive/server.pid
else
    pkill -f edge-hive-core && echo "Server stopped" || echo "Server not running"
fi
SCRIPT

cat > "$BIN_DIR/edge-hive-status" << 'SCRIPT'
#!/data/data/com.termux/files/usr/bin/bash
if [ -f ~/.config/edge-hive/server.pid ]; then
    PID=$(cat ~/.config/edge-hive/server.pid)
    if ps -p $PID > /dev/null 2>&1; then
        echo "✅ Server running (PID: $PID)"
        curl -s http://localhost:8080/health 2>/dev/null && echo "" || echo "⚠️  API not responding"
    else
        echo "❌ Server not running (stale PID file)"
        rm ~/.config/edge-hive/server.pid
    fi
else
    echo "❌ Server not running"
fi
SCRIPT

cat > "$BIN_DIR/edge-hive-update" << 'SCRIPT'
#!/data/data/com.termux/files/usr/bin/bash
echo "Updating Edge Hive..."
cd ~/edge-hive
git pull
cargo build --release -p edge-hive-core
cp target/release/edge-hive-core ~/.local/bin/
echo "✅ Update complete!"
SCRIPT

chmod +x "$BIN_DIR/edge-hive-start"
chmod +x "$BIN_DIR/edge-hive-stop"
chmod +x "$BIN_DIR/edge-hive-status"
chmod +x "$BIN_DIR/edge-hive-update"

echo -e "${YELLOW}[6/6]${NC} Configuring shell..."
if ! grep -q 'edge-hive' "$HOME/.bashrc" 2>/dev/null; then
    cat >> "$HOME/.bashrc" << 'BASHRC'

# Edge Hive
export PATH="$HOME/.local/bin:$PATH"
alias eh="edge-hive-core"
alias ehs="edge-hive-start"
alias ehx="edge-hive-stop"
BASHRC
fi

echo ""
echo -e "${GREEN}═══════════════════════════════════════════════${NC}"
echo -e "${GREEN}   ✅ Installation Complete!${NC}"
echo -e "${GREEN}═══════════════════════════════════════════════${NC}"
echo ""
echo "Commands:"
echo "  edge-hive-start    - Start server"
echo "  edge-hive-stop     - Stop server"
echo "  edge-hive-status   - Check status"
echo "  edge-hive-update   - Update to latest"
echo "  edge-hive-core     - CLI tool"
echo ""
echo "Aliases (after reload):"
echo "  eh   = edge-hive-core"
echo "  ehs  = edge-hive-start"
echo "  ehx  = edge-hive-stop"
echo ""
echo -e "${YELLOW}Run 'source ~/.bashrc' or restart Termux${NC}"
echo ""

#!/data/data/com.termux/files/usr/bin/bash
# scripts/uninstall-termux.sh

set -e

echo "🗑️ Uninstalling Edge Hive from Termux..."

# 1. Stop the service if it's running
if [ -f ~/.termux/boot/edge-hive.sh ]; then
    echo "🛑 Stopping Edge Hive service..."
    sv stop edge-hive || true
    sv-disable edge-hive || true
    rm -f ~/.termux/boot/edge-hive.sh
fi

# 2. Remove the binary
echo "🔥 Removing edge-hive binary..."
rm -f /data/data/com.termux/files/usr/bin/edge-hive

# 3. Remove the config directory
echo "🔥 Removing configuration files..."
rm -rf ~/.config/edge-hive

# 4. Remove the data directory
echo "🔥 Removing database and local data..."
rm -rf ~/.local/share/edge-hive

# 5. Remove old log directory if it exists
if [ -d ~/.edge-hive ]; then
    echo "🔥 Removing legacy log directory..."
    rm -rf ~/.edge-hive
fi


echo "✅ Edge Hive uninstalled successfully!"

#!/bin/bash

# Exit on error
set -e

# Cleanup function
cleanup() {
    echo "Cleaning up..."
    pkill -f edge-hive || true
    rm -rf /tmp/edge-hive-test
}

# Trap cleanup function on exit
trap cleanup EXIT

# 1. Build the project
echo "Building the project..."
cargo build

# 2. Setup test environment
echo "Setting up test environment..."
NODE1_DIR=/tmp/edge-hive-test/node1
NODE2_DIR=/tmp/edge-hive-test/node2
mkdir -p $NODE1_DIR
mkdir -p $NODE2_DIR

# 3. Scenario 1: Single Node Bootstrap
echo "--- Scenario 1: Single Node Bootstrap ---"
./target/debug/edge-hive --config-dir $NODE1_DIR identity new
./target/debug/edge-hive --config-dir $NODE1_DIR start --port 8081 --tor &
NODE1_PID=$!
sleep 10 # Give Tor some time to start
echo "Checking health..."
curl --silent --show-error --fail http://localhost:8081/health

# 4. Scenario 2: Two Nodes Discovery
echo "--- Scenario 2: Two Nodes Discovery ---"
./target/debug/edge-hive --config-dir $NODE2_DIR identity new
NODE1_PEER_ID=$(./target/debug/edge-hive --config-dir $NODE1_DIR status | grep "Peer ID" | awk '{print $3}')
NODE1_ADDR="/ip4/127.0.0.1/tcp/8081/p2p/$NODE1_PEER_ID"
./target/debug/edge-hive --config-dir $NODE2_DIR start --port 8082 --discovery --bootstrap-peer $NODE1_ADDR &
sleep 5
echo "Checking peers..."
./target/debug/edge-hive peers --api-server http://localhost:8082

# 5. Scenario 3: Message Exchange
echo "--- Scenario 3: Message Exchange ---"
NODE2_PEER_ID=$(./target/debug/edge-hive --config-dir $NODE2_DIR status | grep "Peer ID" | awk '{print $3}')

echo "Node 1 Peer ID: $NODE1_PEER_ID"
echo "Node 2 Peer ID: $NODE2_PEER_ID"

MESSAGE="Hello from Node 1"
echo "Sending message..."
curl --silent --show-error --fail -X POST -H "Content-Type: application/json" -d "{\"to\":\"$NODE2_PEER_ID\",\"from\":\"$NODE1_PEER_ID\",\"body\":\"$MESSAGE\", \"timestamp\": \"$(date -u +%Y-%m-%dT%H:%M:%SZ)\"}" http://localhost:8081/api/v1/messages
sleep 2
echo "Checking for message..."
curl --silent --show-error --fail http://localhost:8082/api/v1/messages/$NODE2_PEER_ID | grep "$MESSAGE"

# 6. Scenario 4: Database Persistence (Simulated by checking message after restart)
echo "--- Scenario 4: Database Persistence ---"
echo "Restarting nodes..."
pkill -f edge-hive
sleep 2
./target/debug/edge-hive --config-dir $NODE1_DIR start --port 8081 &
./target/debug/edge-hive --config-dir $NODE2_DIR start --port 8082 --discovery --bootstrap-peer $NODE1_ADDR &
sleep 5
echo "Checking for message after restart..."
curl --silent --show-error --fail http://localhost:8082/api/v1/messages/$NODE2_PEER_ID | grep "$MESSAGE"

echo "All scenarios passed!"
exit 0

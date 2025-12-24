# Tor Onion Services Integration

This document describes the Tor Onion Services implementation in Edge Hive.

## Overview

Edge Hive includes built-in support for exposing your node via Tor Onion Services (v3), providing anonymous access without revealing your IP address.

## Architecture

The Tor integration is implemented in the `edge-hive-tunnel` crate with the following components:

- **TorService**: Main service manager for starting/stopping Tor
- **TorConfig**: Configuration management with builder pattern
- **OnionService**: Handles onion address generation and key management
- **TorBootstrap**: Manages Tor network bootstrap

## Usage

### Starting a Node with Tor

```bash
# Start Edge Hive with Tor enabled
edge-hive start --tor --port 8080

# The node will generate a .onion address and print it to the logs:
# 🧅 Onion service available at: http://abcdef123456.onion
```

### Configuration

The Tor service can be configured programmatically:

```rust
use edge_hive_tunnel::{TorConfig, TorService};

// Create configuration
let config = TorConfig::default()?
    .with_local_port(8080)
    .with_nickname("my-edge-node")
    .with_enabled(true);

// Start service
let mut tor_service = TorService::new(config);
let onion_address = tor_service.start().await?;

println!("Your onion address: {}.onion", onion_address);
```

### Data Directory

Tor data (keys, state, cache) is stored in:
- Linux: `~/.local/share/Edge Hive/tor/`
- macOS: `~/Library/Application Support/Edge Hive/tor/`
- Windows: `C:\Users\<user>\AppData\Roaming\Edge Hive\tor\`

The onion service key is persistent across restarts, maintaining the same .onion address.

## API Reference

### TorService

```rust
impl TorService {
    /// Create a new Tor service
    pub fn new(config: TorConfig) -> Self;
    
    /// Start the Tor service
    pub async fn start(&mut self) -> Result<String>;
    
    /// Stop the Tor service
    pub async fn stop(&mut self) -> Result<()>;
    
    /// Get the onion address if running
    pub fn onion_address(&self) -> Option<&str>;
    
    /// Check if service is running
    pub fn is_running(&self) -> bool;
}
```

### TorConfig

```rust
impl TorConfig {
    /// Create default configuration
    pub fn default() -> Result<Self>;
    
    /// Set data directory
    pub fn with_data_dir<P: Into<PathBuf>>(self, path: P) -> Self;
    
    /// Set local port to forward to
    pub fn with_local_port(self, port: u16) -> Self;
    
    /// Set service nickname
    pub fn with_nickname<S: Into<String>>(self, name: S) -> Self;
    
    /// Enable/disable service
    pub fn with_enabled(self, enabled: bool) -> Self;
}
```

## Implementation Details

### Onion Address Generation

Edge Hive generates Tor v3 onion addresses using:
- Ed25519 cryptography for signing keys
- SHA3-256 for address checksum
- Base32 encoding (RFC 4648, no padding)

The resulting addresses are 56 characters long and end with `.onion`.

### Dependencies

The implementation uses the official Tor crates:
- `tor-hsservice` 0.37: Onion service v3 support
- `tor-rtcompat` 0.37: Tokio runtime compatibility
- `tor-hscrypto` 0.37: Hidden service cryptography

### Bootstrap Process

When started, the TorService:
1. Creates necessary directories
2. Loads or generates Ed25519 keypair
3. Derives .onion address from public key
4. Bootstraps connection to Tor network
5. Publishes onion service descriptor

## Testing

Run the test suite:

```bash
# Test the tunnel crate
cargo test -p edge-hive-tunnel

# Run specific tests
cargo test -p edge-hive-tunnel tor::tests
```

All tests use temporary directories and don't require network access.

## Platform Support

The Tor implementation is designed to work on:
- ✅ Linux (x86_64, aarch64)
- ✅ Windows (x86_64)
- ✅ macOS (x86_64, aarch64)
- ✅ Android (via Termux)

## Security Considerations

1. **Key Storage**: Onion service keys are stored unencrypted. Protect your data directory.
2. **Traffic Forwarding**: Currently, traffic forwarding is limited by tor-hsservice API.
3. **Network Bootstrap**: Initial connection to Tor network may take 30-60 seconds.

## Limitations

Current limitations (as of v0.1.0):
- Traffic forwarding not yet implemented (waiting for tor-hsservice improvements)
- No support for multiple onion services per node
- No support for onion service authentication

These will be addressed in future releases as the tor-hsservice API matures.

## Future Work

Planned improvements:
- Full traffic forwarding implementation
- Support for client authentication
- Bandwidth throttling options
- Integration with Tor control port
- Support for bridge relays

## References

- [Tor Project](https://www.torproject.org/)
- [Tor v3 Onion Services](https://community.torproject.org/onion-services/)
- [tor-hsservice crate](https://tpo.pages.torproject.org/core/arti/)

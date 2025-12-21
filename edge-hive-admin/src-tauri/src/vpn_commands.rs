use std::process::Command;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VpnPeer {
    pub public_key: String,
    pub endpoint: Option<String>,
    pub allowed_ips: Vec<String>,
    pub latest_handshake: Option<String>,
    pub transfer_rx: u64,
    pub transfer_tx: u64,
}

/// Check if WireGuard is installed
fn check_wg_installed() -> Result<(), String> {
    if which::which("wg").is_err() {
        return Err("WireGuard (wg) is not installed or not in PATH. Please install WireGuard.".into());
    }
    Ok(())
}

/// Parse output from `wg show all dump`
#[tauri::command]
pub async fn get_vpn_peers() -> Result<Vec<VpnPeer>, String> {
    if let Err(e) = check_wg_installed() {
        // For development/demo without wg, return empty list instead of erroring loudly on dashboard
        eprintln!("WireGuard check failed: {}", e);
        return Ok(Vec::new());
    }

    let output = Command::new("wg")
        .args(["show", "all", "dump"])
        .output()
        .map_err(|e| format!("Failed to run wg: {}", e))?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut peers = Vec::new();

    for line in stdout.lines().skip(1) { // Skip interface line
        let parts: Vec<&str> = line.split('\t').collect();
        if parts.len() >= 8 {
            peers.push(VpnPeer {
                public_key: parts[1].to_string(),
                endpoint: if parts[3] != "(none)" { Some(parts[3].to_string()) } else { None },
                allowed_ips: parts[4].split(',').map(|s| s.to_string()).collect(),
                latest_handshake: Some(parts[5].to_string()),
                transfer_rx: parts[6].parse().unwrap_or(0),
                transfer_tx: parts[7].parse().unwrap_or(0),
            });
        }
    }

    Ok(peers)
}

/// Generate WireGuard keypair
#[tauri::command]
pub async fn generate_vpn_keypair() -> Result<(String, String), String> {
    check_wg_installed()?;

    let privkey = Command::new("wg")
        .arg("genkey")
        .output()
        .map_err(|e| format!("Failed to generate key: {}", e))?;

    let private_key = String::from_utf8_lossy(&privkey.stdout).trim().to_string();

    let pubkey = Command::new("wg")
        .arg("pubkey")
        .stdin(std::process::Stdio::piped())
        .output()
        .map_err(|e| format!("Failed to derive pubkey: {}", e))?;

    let public_key = String::from_utf8_lossy(&pubkey.stdout).trim().to_string();

    Ok((private_key, public_key))
}

/// Generate WireGuard config file content
#[tauri::command]
pub async fn generate_vpn_config(
    peer_name: String,
    peer_public_key: String,
    peer_endpoint: String,
    allowed_ips: String,
) -> Result<String, String> {
    check_wg_installed()?;
    let (private_key, _public_key) = generate_vpn_keypair().await?;

    // Note: peer_name is used for logging/UI context usually, but included in comment here.
    let config = format!(r#"# Config for {}
[Interface]
PrivateKey = {}
Address = 10.0.0.2/24
DNS = 1.1.1.1

[Peer]
PublicKey = {}
Endpoint = {}
AllowedIPs = {}
PersistentKeepalive = 25
"#, peer_name, private_key, peer_public_key, peer_endpoint, allowed_ips);

    Ok(config)
}

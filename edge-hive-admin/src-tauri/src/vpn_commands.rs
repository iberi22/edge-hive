use tauri::State;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VPNPeer {
    pub id: String,
    pub public_key: String,
    pub endpoint: String,
    pub allowed_ips: Vec<String>,
    pub last_handshake: String,
    pub transfer_rx: String,
    pub transfer_tx: String,
    pub status: String,
}

#[tauri::command]
pub async fn get_vpn_peers() -> Result<Vec<VPNPeer>, String> {
    Ok(vec![
        VPNPeer {
            id: "HN-01-MASTER".into(),
            public_key: "x62k...92j".into(),
            endpoint: "AWS_HUB:51820".into(),
            allowed_ips: vec!["10.0.0.1/32".into()],
            last_handshake: "12s ago".into(),
            transfer_rx: "1.2 GB".into(),
            transfer_tx: "4.5 GB".into(),
            status: "connected".into()
        },
        VPNPeer {
            id: "LAPTOP-ADMIN".into(),
            public_key: "z12a...02m".into(),
            endpoint: "dynamic_ip:3921".into(),
            allowed_ips: vec!["10.0.0.5/32".into()],
            last_handshake: "2m ago".into(),
            transfer_rx: "12 MB".into(),
            transfer_tx: "82 MB".into(),
            status: "idle".into()
        },
    ])
}

#[tauri::command]
pub async fn generate_vpn_config() -> Result<String, String> {
    Ok("[Interface]\nPrivateKey = ...\nAddress = 10.0.0.99/32\nDNS = 1.1.1.1".to_string())
}

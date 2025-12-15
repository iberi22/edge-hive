// Integration tests for `edge-hive serve` command
//
// These tests verify end-to-end functionality:
// - CLI argument parsing
// - HTTP server startup
// - Tor integration (when --tor flag is used)

use assert_cmd::Command;
use std::time::Duration;

#[test]
fn test_serve_basic_startup() {
    // Start server on random available port
    let mut cmd = Command::cargo_bin("edge-hive").unwrap();
    let mut child = cmd
        .arg("serve")
        .arg("--port")
        .arg("9999")
        .spawn()
        .expect("Failed to start edge-hive serve");

    // Wait for server initialization
    std::thread::sleep(Duration::from_secs(2));

    // Test health endpoint
    let client = reqwest::blocking::Client::new();
    let resp = client
        .get("http://127.0.0.1:9999/health")
        .send()
        .expect("Failed to reach health endpoint");

    assert_eq!(resp.status(), reqwest::StatusCode::OK);

    // Cleanup: kill server process
    child.kill().expect("Failed to kill server");
}

#[test]
#[ignore] // Requires Tor to be installed on test machine
fn test_serve_with_tor_flag() {
    let mut cmd = Command::cargo_bin("edge-hive").unwrap();
    let mut child = cmd
        .arg("serve")
        .arg("--port")
        .arg("9998")
        .arg("--tor")
        .spawn()
        .expect("Failed to start edge-hive serve with --tor");

    // Wait for Tor bootstrap (longer timeout)
    std::thread::sleep(Duration::from_secs(5));

    // TODO: Verify onion address is printed to stdout
    // TODO: Test connection via Tor SOCKS proxy

    child.kill().expect("Failed to kill server");
}

// TODO (Jules): Add more tests after #29 and #30 are merged:
// - test_p2p_discovery_mdns()
// - test_surrealdb_persistence()

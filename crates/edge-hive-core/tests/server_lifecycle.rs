
use std::process::{Command, Stdio};
use std::thread;
use std::time::Duration;
use assert_cmd::prelude::*;

#[test]
fn test_server_lifecycle() {
    let port = 8081u16;
    let server_binary = env!("CARGO_BIN_EXE_edge-hive-core");

    // 1. Start server
    let mut server_process = Command::new(server_binary)
        .arg("serve")
        .arg("--port")
        .arg(port.to_string())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .expect("Failed to start server");

    // Give the server time to start
    thread::sleep(Duration::from_secs(2));

    // 2. Ping server to check if it's running
    let mut cmd = Command::cargo_bin("edge-hive-core").unwrap();
    cmd.arg("ping")
        .arg("--port")
        .arg(port.to_string())
        .assert()
        .success();

    // 3. Stop server
    server_process.kill().expect("Failed to kill server process");
    server_process.wait().expect("Failed to wait for server process");

    // 4. Ping server again to ensure it's stopped
    let mut cmd = Command::cargo_bin("edge-hive-core").unwrap();
    cmd.arg("ping")
        .arg("--port")
        .arg(port.to_string())
        .assert()
        .failure();
}

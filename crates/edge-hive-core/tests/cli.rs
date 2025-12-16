use assert_cmd::Command;
use predicates::prelude::*;
use std::error::Error;
use tempfile::tempdir;

type TestResult = Result<(), Box<dyn Error>>;

#[test]
fn test_init_command() -> TestResult {
    let temp_dir = tempdir()?;
    let data_dir = temp_dir.path();

    let mut cmd = Command::cargo_bin("edge-hive")?;
    cmd.arg("init")
        .env("EDGE_HIVE_DATA_DIR", data_dir)
        .assert()
        .success()
        .stdout(predicate::str::contains("Node identity created!"));

    assert!(data_dir.join("identity.key").exists());

    Ok(())
}

#[test]
fn test_status_command_no_init() -> TestResult {
    let temp_dir = tempdir()?;
    let data_dir = temp_dir.path();

    let mut cmd = Command::cargo_bin("edge-hive")?;
    cmd.arg("status")
        .env("EDGE_HIVE_DATA_DIR", data_dir)
        .assert()
        .failure()
        .stderr(predicate::str::contains("Identity file not found"));

    Ok(())
}

#[test]
fn test_peers_command() -> TestResult {
    let temp_dir = tempdir()?;
    let data_dir = temp_dir.path();

    // Init first
    let mut init_cmd = Command::cargo_bin("edge-hive")?;
    init_cmd.arg("init")
        .env("EDGE_HIVE_DATA_DIR", data_dir)
        .assert()
        .success();

    let mut cmd = Command::cargo_bin("edge-hive")?;
    cmd.arg("peers")
        .arg("list")
        .env("EDGE_HIVE_DATA_DIR", data_dir)
        .assert()
        .success()
        .stdout(predicate::str::contains("Discovery finished"));

    Ok(())
}

#[test]
fn test_serve_command_help() -> TestResult {
    let mut cmd = Command::cargo_bin("edge-hive")?;
    cmd.arg("serve")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Start the Edge Hive server"));
    Ok(())
}

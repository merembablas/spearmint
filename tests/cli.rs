use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn apply() {
    let mut cmd = Command::cargo_bin("spearmint").unwrap();
    cmd.args([
        "apply",
        "-f",
        "D:\\projects\\Trading\\spearmint\\config.example.toml",
    ])
    .assert()
    .success()
    .stdout(predicate::str::contains("Margin"));
}

#[test]
fn status() {
    let mut cmd = Command::cargo_bin("spearmint").unwrap();
    cmd.args(["status", "ftm_btc"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Status"));
}

#[test]
fn list() {
    let mut cmd = Command::cargo_bin("spearmint").unwrap();
    cmd.args(["list"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Title"));
}

use std::fs::File;
use std::io::Write as _;

use assert_cmd::Command;
use mado::Config;
use tempfile::tempdir;

#[test]
fn check() {
    let mut cmd = Command::cargo_bin("mado").unwrap();
    let assert = cmd.args(["check", "."]).assert();
    assert.success().stdout("All checks passed!\n");
}

#[test]
fn check_quiet() {
    let mut cmd = Command::cargo_bin("mado").unwrap();
    let assert = cmd.args(["check", "--quiet", "."]).assert();
    assert.success().stdout("");
}

#[test]
fn check_quiet_with_config() {
    // Create new mado.toml
    let tmp_dir = tempdir().unwrap();
    let path = tmp_dir.path().join("mado.toml");
    let mut tmp_file = File::create(path.clone()).unwrap();
    let mut config = Config::default();
    config.lint.quiet = true;
    config.lint.md013.tables = false;
    config.lint.md013.code_blocks = false;
    let content = toml::to_string(&config).unwrap();
    write!(tmp_file, "{content}").unwrap();

    let mut cmd = Command::cargo_bin("mado").unwrap();
    let assert = cmd
        .args(["--config", path.to_str().unwrap(), "check", "."])
        .assert();
    assert.success().stdout("");

    tmp_dir.close().unwrap();
}

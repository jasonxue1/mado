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

#[test]
fn check_stdin() {
    let mut cmd = Command::cargo_bin("mado").unwrap();
    let assert = cmd.write_stdin("#Hello.").args(["check"]).assert();
    assert.failure().stdout(
        "\u{1b}[1m(stdin)\u{1b}[0m\u{1b}[34m:\u{1b}[0m1\u{1b}[34m:\u{1b}[0m1\u{1b}[34m:\u{1b}[0m \u{1b}[1;31mMD018\u{1b}[0m No space after hash on atx style header
\u{1b}[1m(stdin)\u{1b}[0m\u{1b}[34m:\u{1b}[0m1\u{1b}[34m:\u{1b}[0m1\u{1b}[34m:\u{1b}[0m \u{1b}[1;31mMD041\u{1b}[0m First line in file should be a top level header
\u{1b}[1m(stdin)\u{1b}[0m\u{1b}[34m:\u{1b}[0m1\u{1b}[34m:\u{1b}[0m1\u{1b}[34m:\u{1b}[0m \u{1b}[1;31mMD047\u{1b}[0m File should end with a single newline character

Found 3 errors.\n",
    );
}

#[test]
fn check_empty_stdin() {
    let mut cmd = Command::cargo_bin("mado").unwrap();
    let assert = cmd.write_stdin("").args(["check"]).assert();
    assert.success().stdout("All checks passed!\n");
}

#[test]
fn check_empty_stdin_with_file() {
    // Create test.md
    let tmp_dir = tempdir().unwrap();
    let path = tmp_dir.path().join("test.md");
    let mut tmp_file = File::create(path.clone()).unwrap();
    write!(tmp_file, "#Hello.").unwrap();

    let mut cmd = Command::cargo_bin("mado").unwrap();
    let path_str = path.to_str().unwrap();
    let assert = cmd.write_stdin("").args(["check", path_str]).assert();
    assert.failure().stdout(
        format!(
        "\u{1b}[1m{path_str}\u{1b}[0m\u{1b}[34m:\u{1b}[0m1\u{1b}[34m:\u{1b}[0m1\u{1b}[34m:\u{1b}[0m \u{1b}[1;31mMD018\u{1b}[0m No space after hash on atx style header
\u{1b}[1m{path_str}\u{1b}[0m\u{1b}[34m:\u{1b}[0m1\u{1b}[34m:\u{1b}[0m1\u{1b}[34m:\u{1b}[0m \u{1b}[1;31mMD041\u{1b}[0m First line in file should be a top level header
\u{1b}[1m{path_str}\u{1b}[0m\u{1b}[34m:\u{1b}[0m1\u{1b}[34m:\u{1b}[0m1\u{1b}[34m:\u{1b}[0m \u{1b}[1;31mMD047\u{1b}[0m File should end with a single newline character

Found 3 errors.\n",
        )
    );

    tmp_dir.close().unwrap();
}

#[test]
fn check_stdin_with_file() {
    // Create test.md
    let tmp_dir = tempdir().unwrap();
    let path = tmp_dir.path().join("test.md");
    let mut tmp_file = File::create(path.clone()).unwrap();
    write!(tmp_file, "#Hello.").unwrap();

    let mut cmd = Command::cargo_bin("mado").unwrap();
    let path_str = path.to_str().unwrap();
    let assert = cmd
        .write_stdin("#Hello.")
        .args(["check", path_str])
        .assert();
    assert.failure().stdout(
        "\u{1b}[1m(stdin)\u{1b}[0m\u{1b}[34m:\u{1b}[0m1\u{1b}[34m:\u{1b}[0m1\u{1b}[34m:\u{1b}[0m \u{1b}[1;31mMD018\u{1b}[0m No space after hash on atx style header
\u{1b}[1m(stdin)\u{1b}[0m\u{1b}[34m:\u{1b}[0m1\u{1b}[34m:\u{1b}[0m1\u{1b}[34m:\u{1b}[0m \u{1b}[1;31mMD041\u{1b}[0m First line in file should be a top level header
\u{1b}[1m(stdin)\u{1b}[0m\u{1b}[34m:\u{1b}[0m1\u{1b}[34m:\u{1b}[0m1\u{1b}[34m:\u{1b}[0m \u{1b}[1;31mMD047\u{1b}[0m File should end with a single newline character

Found 3 errors.\n",
    );

    tmp_dir.close().unwrap();
}

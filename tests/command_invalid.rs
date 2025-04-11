use assert_cmd::Command;
use indoc::formatdoc;
use miette::IntoDiagnostic as _;
use miette::Result;

#[test]
fn no_command() -> Result<()> {
    let mut cmd = Command::cargo_bin("mado").into_diagnostic()?;
    let assert = cmd.assert();
    assert.failure();
    Ok(())
}

#[test]
fn unknown_command() -> Result<()> {
    let mut cmd = Command::cargo_bin("mado").into_diagnostic()?;
    let assert = cmd.args(["foobar"]).assert();
    assert.failure().stderr(formatdoc! {"
        \u{1b}[1m\u{1b}[31merror:\u{1b}[0m unrecognized subcommand \'\u{1b}[33mfoobar\u{1b}[0m\'

        \u{1b}[1m\u{1b}[4mUsage:\u{1b}[0m \u{1b}[1mmado\u{1b}[0m [OPTIONS] <COMMAND>

        For more information, try \'\u{1b}[1m--help\u{1b}[0m\'.
    "});
    Ok(())
}

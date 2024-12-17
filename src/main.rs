#![warn(clippy::pedantic)]

use std::process::ExitCode;

use clap::Parser as _;
use miette::Result;

use downlint::command::check::Checker;
use downlint::Cli;
use downlint::Command;

fn main() -> Result<ExitCode> {
    let cli = Cli::parse();

    match &cli.command {
        Some(Command::Check {
            files,
            output_format,
        }) => {
            let checker = Checker::new(files, output_format.clone())?;
            checker.check()
        }
        _ => Ok(ExitCode::FAILURE),
    }
}

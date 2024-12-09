use std::process::ExitCode;

use clap::Parser;
use miette::Result;

use downlint::command::check::Checker;
use downlint::Cli;
use downlint::Command;

fn main() -> Result<ExitCode> {
    let cli = Cli::parse();

    match &cli.command {
        Some(Command::Check { files }) => {
            let checker = Checker::new(files)?;
            checker.check()
        }
        None => Ok(ExitCode::FAILURE),
    }
}

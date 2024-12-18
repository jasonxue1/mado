#[cfg(all(
    not(target_os = "windows"),
    not(target_os = "openbsd"),
    not(target_os = "aix"),
    any(
        target_arch = "x86_64",
        target_arch = "aarch64",
        target_arch = "powerpc64"
    )
))]
#[global_allocator]
static GLOBAL: tikv_jemallocator::Jemalloc = tikv_jemallocator::Jemalloc;

#[cfg(target_os = "windows")]
#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

use std::process::ExitCode;

use clap::Parser as _;
use downlint::command::check::Options;
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
            let options = Options {
                output_format: output_format.clone(),
                config_path: cli.config,
            };
            let config = options.to_config()?;
            let checker = Checker::new(files, config)?;
            checker.check()
        }
        _ => Ok(ExitCode::FAILURE),
    }
}

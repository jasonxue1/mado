use std::path::PathBuf;

use clap::Parser;

use crate::Command;

#[derive(Parser)]
#[command(
    name = "mado",
    bin_name = "mado",
    version,
    about,
    long_about = None,
    arg_required_else_help = true
)]
#[non_exhaustive]
pub struct Cli {
    /// A path to a TOML configuration file overriding a specific configuration option
    #[arg(long, value_name = "FILE")]
    pub config: Option<PathBuf>,

    #[command(subcommand)]
    pub command: Option<Command>,
}

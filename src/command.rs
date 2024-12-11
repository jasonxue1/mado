use std::path::PathBuf;

use clap::Subcommand;

pub mod check;

#[derive(Subcommand)]
#[non_exhaustive]
pub enum Command {
    /// Check markdown on the given files or directories
    Check {
        /// List of files or directories to check
        #[arg(required = true)]
        files: Vec<PathBuf>,
    },
}

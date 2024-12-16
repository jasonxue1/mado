use std::path::PathBuf;

use clap::Subcommand;

use crate::output::Format;

pub mod check;

#[derive(Subcommand)]
#[non_exhaustive]
pub enum Command {
    /// Check markdown on the given files or directories
    Check {
        /// List of files or directories to check
        #[arg(default_value = ".")]
        files: Vec<PathBuf>,

        #[arg(value_enum, long = "output-format", default_value_t = Format::Concise)]
        output_format: Format,
    },
}

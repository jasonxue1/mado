use clap::Parser;

use crate::Command;

#[derive(Parser)]
#[command(name = "markdownlint", bin_name = "markdownlint", version, about, long_about = None, arg_required_else_help = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Command>,
}

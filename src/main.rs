use clap::Parser;

use markdownlint::Cli;
use markdownlint::Command;
use markdownlint::Linter;

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Command::Check { files }) => {
            let linter = Linter::new();

            for file in files {
                println!("{:?}", linter.check(file));
            }
        }
        None => {}
    }
}

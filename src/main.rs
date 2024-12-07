use clap::Parser;
use glob::glob;

use markdownlint::Cli;
use markdownlint::Command;
use markdownlint::Linter;

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Command::Check { files }) => {
            let linter = Linter::new();

            for pattern in files {
                for entry in glob(pattern).unwrap() {
                    println!("{:?}", linter.check(&entry.unwrap()));
                }
            }
        }
        None => {}
    }
}

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
                    // TODO: Don't use unwrap
                    let path = entry.unwrap();
                    let violations = linter.check(&path);
                    if !violations.is_empty() {
                        println!("{}", path.to_str().unwrap());
                        for violation in violations {
                            println!(
                                "{}:{} {}",
                                violation.position().start.line,
                                violation.name(),
                                violation.description()
                            );
                        }
                    }
                }
            }
        }
        None => {}
    }
}

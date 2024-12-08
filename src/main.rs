use std::process;

use clap::Parser;
use miette::{IntoDiagnostic, Result};

use downlint::Cli;
use downlint::Command;
use downlint::Linter;
use downlint::MarkdownWalker;

fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Some(Command::Check { files }) => {
            let linter = Linter::new();
            let walker = MarkdownWalker::new(files);
            for maybe_entry in walker {
                let entry = maybe_entry.into_diagnostic()?;
                let path = entry.path();
                let violations = linter.check(path)?;
                if !violations.is_empty() {
                    let path_str = path.to_str().expect("path must convert to string");
                    for violation in violations {
                        println!(
                            "{}:{}:{} {}",
                            path_str,
                            violation.position().start.line,
                            violation.name(),
                            violation.description()
                        );
                    }

                    process::exit(1);
                }
            }
            Ok(())
        }
        None => Ok(()),
    }
}

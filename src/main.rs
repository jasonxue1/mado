use clap::Parser;

use downlint::Cli;
use downlint::Command;
use downlint::Linter;
use downlint::MarkdownWalker;

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Command::Check { files }) => {
            let linter = Linter::new();
            let walker = MarkdownWalker::new(files);
            for maybe_entry in walker {
                // TODO: Don't use unwrap
                let entry = maybe_entry.unwrap();
                let path = entry.path();
                let violations = linter.check(path);
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
        None => {}
    }
}

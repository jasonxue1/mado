use std::path::PathBuf;
use std::process;

use colored::Colorize;
use miette::IntoDiagnostic;
use miette::Result;

use crate::Linter;
use crate::MarkdownWalker;
use crate::Violation;

pub struct Checker {
    linter: Linter,
    walker: MarkdownWalker,
}

impl Checker {
    pub fn new(files: &[PathBuf]) -> Self {
        let linter = Linter::new();
        let walker = MarkdownWalker::new(files);

        Self { linter, walker }
    }

    pub fn check(self) -> Result<()> {
        let mut all_violations: Vec<(String, Violation)> = vec![];

        for maybe_entry in self.walker {
            let entry = maybe_entry.into_diagnostic()?;
            let path = entry.path();
            let path_str = path
                .to_str()
                .expect("path must convert to string")
                .to_string();
            let violations = self.linter.check(path)?;
            for violation in violations {
                all_violations.push((path_str.clone(), violation));
            }
        }

        if all_violations.is_empty() {
            println!("All checks passed!");
            return Ok(());
        }

        let num_violations = all_violations.len();
        for (file, violation) in all_violations {
            println!(
                "{}{}{}{}{}{} {} {}",
                file.bold(),
                ":".blue(),
                violation.position().start.line,
                ":".blue(),
                violation.position().start.column,
                ":".blue(),
                violation.name().red().bold(),
                violation.description()
            );
        }

        println!("");

        if num_violations == 1 {
            println!("Found 1 error.");
        } else {
            println!("Found {} errors.", num_violations);
        }

        process::exit(1);
    }
}

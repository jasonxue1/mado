use std::path::PathBuf;
use std::process;

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
    pub fn new(files: &[PathBuf]) -> Result<Self> {
        let linter = Linter::new();
        let walker = MarkdownWalker::new(files)?;

        Ok(Self { linter, walker })
    }

    pub fn check(self) -> Result<()> {
        let mut all_violations: Vec<Violation> = vec![];

        for maybe_entry in self.walker {
            let entry = maybe_entry.into_diagnostic()?;
            let path = entry.path();
            let violations = self.linter.check(path)?;
            for violation in violations {
                all_violations.push(violation);
            }
        }

        if all_violations.is_empty() {
            println!("All checks passed!");
            return Ok(());
        }

        let num_violations = all_violations.len();
        for violation in all_violations {
            println!("{violation}");
        }

        if num_violations == 1 {
            println!("\nFound 1 error.");
        } else {
            println!("\nFound {} errors.", num_violations);
        }

        process::exit(1);
    }
}

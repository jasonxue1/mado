use std::path::PathBuf;
use std::process::ExitCode;

use miette::Result;

use crate::runner::ParallelLintRunner;
use crate::MarkdownWalker;

pub struct Checker {
    walker: MarkdownWalker,
}

impl Checker {
    pub fn new(patterns: &[PathBuf]) -> Result<Self> {
        let walker = MarkdownWalker::new(patterns)?;

        Ok(Self { walker })
    }

    pub fn check(self) -> Result<ExitCode> {
        let walker = self.walker.walker;
        let runner = ParallelLintRunner::new(walker, 100);
        let violations = runner.run()?;

        if violations.is_empty() {
            println!("All checks passed!");
            return Ok(ExitCode::SUCCESS);
        }

        let num_violations = violations.len();
        for violation in violations {
            println!("{violation}");
        }

        if num_violations == 1 {
            println!("\nFound 1 error.");
        } else {
            println!("\nFound {} errors.", num_violations);
        }

        Ok(ExitCode::FAILURE)
    }
}

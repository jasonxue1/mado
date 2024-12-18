use std::fs;
use std::path::PathBuf;
use std::process::ExitCode;

use ignore::WalkParallel;
use miette::IntoDiagnostic as _;
use miette::Result;

use crate::config::Config;
use crate::output::{Concise, Format, Markdownlint, Mdl};
use crate::service::runner::ParallelLintRunner;
use crate::service::walker::WalkParallelBuilder;

pub struct Checker {
    walker: WalkParallel,
    output_format: Format,
    config: Config,
}

impl Checker {
    #[inline]
    pub fn new(patterns: &[PathBuf], output_format: Format) -> Result<Self> {
        let walker = WalkParallelBuilder::build(patterns)?;

        // TODO: Find config
        let config_text = fs::read_to_string("downlint.toml").into_diagnostic()?;
        let config = toml::from_str(&config_text).into_diagnostic()?;

        Ok(Self {
            walker,
            output_format,
            config,
        })
    }

    #[inline]
    pub fn check(self) -> Result<ExitCode> {
        let runner = ParallelLintRunner::new(self.walker, self.config, 100);
        let violations = runner.run()?;

        if violations.is_empty() {
            println!("All checks passed!");
            return Ok(ExitCode::SUCCESS);
        }

        let num_violations = violations.len();
        for violation in violations {
            match self.output_format {
                Format::Concise => println!("{}", Concise::new(violation)),
                Format::Mdl => println!("{}", Mdl::new(violation)),
                Format::Markdownlint => println!("{}", Markdownlint::new(violation)),
            }
        }

        if num_violations == 1 {
            println!("\nFound 1 error.");
        } else {
            println!("\nFound {num_violations} errors.");
        }

        Ok(ExitCode::FAILURE)
    }
}

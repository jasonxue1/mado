use std::path::PathBuf;
use std::process::ExitCode;

use ignore::WalkParallel;
use miette::Result;

use crate::config;
use crate::config::Config;
use crate::output::{Concise, Format, Markdownlint, Mdl};
use crate::service::runner::ParallelLintRunner;
use crate::service::walker::WalkParallelBuilder;

#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(clippy::exhaustive_structs)]
pub struct Options {
    pub config_path: Option<PathBuf>,
    pub output_format: Option<Format>,
}

impl Options {
    #[inline]
    pub fn to_config(self) -> Result<Config> {
        let mut config = match self.config_path {
            Some(config_path) => config::load(&config_path)?,
            None => config::resolve()?,
        };

        if let Some(format) = self.output_format {
            config.lint.output_format = format;
        }

        Ok(config)
    }
}

pub struct Checker {
    walker: WalkParallel,
    config: Config,
}

impl Checker {
    #[inline]
    pub fn new(patterns: &[PathBuf], config: Config) -> Result<Self> {
        let walker = WalkParallelBuilder::build(patterns)?;

        Ok(Self { walker, config })
    }

    #[inline]
    pub fn check(self) -> Result<ExitCode> {
        let runner = ParallelLintRunner::new(self.walker, self.config.clone(), 100);
        let violations = runner.run()?;

        if violations.is_empty() {
            println!("All checks passed!");
            return Ok(ExitCode::SUCCESS);
        }

        let num_violations = violations.len();
        for violation in violations {
            match self.config.lint.output_format {
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

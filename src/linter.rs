use std::fs;
use std::path::Path;

use markdown::ParseOptions;
use miette::miette;
use miette::IntoDiagnostic;
use miette::Result;

use crate::rule;
use crate::violation::Violation;
use crate::Rule;

#[derive(Default)]
pub struct Linter {
    rules: Vec<Box<dyn Rule>>,
}

impl Linter {
    pub fn new() -> Self {
        Self {
            rules: vec![Box::new(rule::MD001::new()), Box::new(rule::MD022::new())],
        }
    }

    pub fn check(&self, path: &Path) -> Result<Vec<Violation>> {
        if !path.is_file() || path.extension() != Some("md".as_ref()) {
            panic!("Unexpected file: {:?}", path);
        }

        let text = &fs::read_to_string(path).into_diagnostic()?;
        let doc = markdown::to_mdast(text, &ParseOptions::default()).map_err(|err| miette!(err))?;
        let violation_results: Vec<Result<Vec<Violation>>> =
            self.rules.iter().map(|rule| rule.check(&doc)).collect();
        let either_nested_violations: Result<Vec<Vec<Violation>>> =
            violation_results.into_iter().collect();
        let either_violations: Result<Vec<Violation>> = either_nested_violations
            .map(|nested_violations| nested_violations.iter().flatten().cloned().collect());

        match either_violations {
            Ok(mut violations) => {
                violations.sort();
                Ok(violations)
            }
            err => err,
        }
    }
}

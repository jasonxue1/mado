use std::path::Path;

use miette::miette;
use miette::Result;

use crate::rule;
use crate::violation::Violation;
use crate::Document;
use crate::Rule;

#[derive(Default)]
pub struct Linter {
    rules: Vec<Box<dyn Rule>>,
}

impl Linter {
    pub fn new() -> Self {
        Self {
            rules: vec![
                Box::new(rule::MD001::new()),
                Box::new(rule::MD002::default()),
                Box::new(rule::MD005::new()),
                Box::new(rule::MD009::new()),
                Box::new(rule::MD010::new()),
                Box::new(rule::MD022::new()),
            ],
        }
    }

    pub fn check(&self, path: &Path) -> Result<Vec<Violation>> {
        if !path.is_file() || path.extension() != Some("md".as_ref()) {
            return Err(miette!("Unexpected file: {:?}", path));
        }

        // Iterate rules while unrolling Vec<Result<Vec<..>>> to Result<Vec<..>>
        let doc = Document::open(path)?;
        let either_violations: Result<Vec<Violation>> =
            self.rules.iter().try_fold(vec![], |mut unrolled, rule| {
                let result = rule.check(&doc);
                unrolled.extend(result?);
                Ok(unrolled)
            });

        match either_violations {
            Ok(mut violations) => {
                violations.sort();
                Ok(violations)
            }
            err => err,
        }
    }
}

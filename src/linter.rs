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
                Box::new(rule::MD003::default()),
                Box::new(rule::MD004::default()),
                Box::new(rule::MD005::new()),
                Box::new(rule::MD006::new()),
                Box::new(rule::MD007::default()),
                Box::new(rule::MD009::new()),
                Box::new(rule::MD010::new()),
                Box::new(rule::MD012::new()),
                Box::new(rule::MD013::default()),
                Box::new(rule::MD022::new()),
            ],
        }
    }

    pub fn check(&self, doc: Document) -> Result<Vec<Violation>> {
        // Iterate rules while unrolling Vec<Result<Vec<..>>> to Result<Vec<..>>
        let either_violations: Result<Vec<Violation>> =
            self.rules.iter().try_fold(vec![], |mut unrolled, rule| {
                let result = rule.check(&doc);
                unrolled.extend(result?);
                Ok(unrolled)
            });

        either_violations.map(|mut violations| {
            violations.sort();
            violations
        })
    }
}

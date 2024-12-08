use std::fs;
use std::path::Path;

use markdown::ParseOptions;

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

    pub fn check(&self, path: &Path) -> Vec<Violation> {
        if !path.is_file() || path.extension() != Some("md".as_ref()) {
            panic!("Unexpected file: {:?}", path);
        }

        // TODO: Don't use unwrap
        let text = &fs::read_to_string(path).unwrap();
        let doc = markdown::to_mdast(text, &ParseOptions::default()).unwrap();
        let mut violations: Vec<Violation> = self
            .rules
            .iter()
            .flat_map(|rule| rule.check(&doc))
            .collect();

        violations.sort();
        violations
    }
}

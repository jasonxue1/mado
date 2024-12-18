use miette::Result;

use crate::config::Config;
use crate::rule;
use crate::rule::RuleLike;
use crate::violation::Violation;
use crate::Document;
use crate::Rule;

#[derive(Default)]
pub struct Linter {
    rules: Vec<Box<dyn RuleLike>>,
}

impl Linter {
    #[inline]
    #[must_use]
    pub fn new(config: &Config) -> Self {
        let rules: Vec<_> = config
            .rules
            .iter()
            .map(|rule| {
                let boxed: Box<dyn RuleLike> = match rule {
                    Rule::MD001 => Box::new(rule::MD001::new()),
                    Rule::MD002 => Box::new(rule::MD002::default()),
                    Rule::MD003 => Box::new(rule::MD003::default()),
                    Rule::MD004 => Box::new(rule::MD004::default()),
                    Rule::MD005 => Box::new(rule::MD005::new()),
                    Rule::MD006 => Box::new(rule::MD006::new()),
                    Rule::MD007 => Box::new(rule::MD007::default()),
                    Rule::MD009 => Box::new(rule::MD009::new()),
                    Rule::MD010 => Box::new(rule::MD010::new()),
                    Rule::MD012 => Box::new(rule::MD012::new()),
                    Rule::MD013 => Box::new(rule::MD013::default()),
                    Rule::MD014 => Box::new(rule::MD014::new()),
                    Rule::MD018 => Box::new(rule::MD018::new()),
                    Rule::MD019 => Box::new(rule::MD019::new()),
                    Rule::MD022 => Box::new(rule::MD022::new()),
                    Rule::MD023 => Box::new(rule::MD023::new()),
                    Rule::MD024 => Box::new(rule::MD024::new()),
                    Rule::MD025 => Box::new(rule::MD025::default()),
                    Rule::MD026 => Box::new(rule::MD026::default()),
                    Rule::MD027 => Box::new(rule::MD027::new()),
                    Rule::MD028 => Box::new(rule::MD028::new()),
                    Rule::MD029 => Box::new(rule::MD029::default()),
                };
                boxed
            })
            .collect();

        Self { rules }
    }

    #[inline]
    pub fn check(&self, doc: &Document) -> Result<Vec<Violation>> {
        // Iterate rules while unrolling Vec<Result<Vec<..>>> to Result<Vec<..>>
        let either_violations: Result<Vec<Violation>> =
            self.rules.iter().try_fold(vec![], |mut unrolled, rule| {
                let result = rule.check(doc);
                unrolled.extend(result?);
                Ok(unrolled)
            });

        either_violations.map(|mut violations| {
            violations.sort();
            violations
        })
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use comrak::{nodes::Sourcepos, parse_document, Arena, Options};
    use pretty_assertions::assert_eq;

    use super::*;
    use rule::MD026;

    #[test]
    fn check_with_front_matter() {
        let text = "---
comments: false
description: Some text
---

# This is a header."
            .to_owned();
        let path = Path::new("test.md").to_path_buf();
        let arena = Arena::new();
        let mut options = Options::default();
        options.extension.front_matter_delimiter = Some("---".to_owned());
        let ast = parse_document(&arena, &text, &options);
        let doc = Document {
            path: path.clone(),
            ast,
            text,
        };
        let md026 = MD026::default();
        let rules = vec![Rule::MD026];
        let config = Config { rules };
        let linter = Linter::new(&config);
        let actual = linter.check(&doc).unwrap();
        let expected = vec![md026.to_violation(path, Sourcepos::from((6, 1, 6, 19)))];
        assert_eq!(actual, expected);
    }
}

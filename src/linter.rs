use comrak::nodes::Sourcepos;
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
    #[inline]
    #[must_use]
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
                Box::new(rule::MD014::new()),
                Box::new(rule::MD018::new()),
                Box::new(rule::MD019::new()),
                Box::new(rule::MD022::new()),
                Box::new(rule::MD023::new()),
                Box::new(rule::MD024::new()),
                Box::new(rule::MD025::default()),
                Box::new(rule::MD026::default()),
            ],
        }
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
            // NOTE: Change Sourcepos to a value that takes front matter into account,
            //       as comrak's sourcepos does not include front matter.
            if let Some(front_matter) = doc.front_matter() {
                let len = front_matter.lines().count();
                violations = violations
                    .into_iter()
                    .map(|mut violation| {
                        let position = violation.position();
                        let new_position = Sourcepos::from((
                            position.start.line + len,
                            position.start.column,
                            position.end.line + len,
                            position.end.column,
                        ));
                        violation.update_position(new_position);
                        violation
                    })
                    .collect();
            }

            violations.sort();
            violations
        })
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use comrak::{parse_document, Arena, Options};
    use rule::MD026;

    use super::*;

    #[test]
    fn hoge() {
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
        let linter = Linter::new();
        let actual = linter.check(&doc).unwrap();
        let expected = vec![md026.to_violation(path, Sourcepos::from((6, 1, 6, 19)))];
        assert_eq!(actual, expected);
    }
}

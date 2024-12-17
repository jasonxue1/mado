use comrak::nodes::NodeValue;
use miette::Result;

use crate::{violation::Violation, Document};

use super::Rule;

#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct MD025 {
    level: u8,
}

impl MD025 {
    #[inline]
    #[must_use]
    pub fn new(level: u8) -> Self {
        Self { level }
    }
}

impl Default for MD025 {
    #[inline]
    fn default() -> Self {
        Self { level: 1 }
    }
}

impl Rule for MD025 {
    #[inline]
    fn name(&self) -> String {
        "MD025".to_owned()
    }

    #[inline]
    fn description(&self) -> String {
        "Multiple top level headers in the same document".to_owned()
    }

    #[inline]
    fn tags(&self) -> Vec<String> {
        vec!["headers".to_owned()]
    }

    #[inline]
    fn aliases(&self) -> Vec<String> {
        vec!["single-h1".to_owned()]
    }

    #[inline]
    fn check(&self, doc: &Document) -> Result<Vec<Violation>> {
        let mut violations = vec![];
        let mut seen_top_level_header = false;

        for node in doc.ast.children() {
            if let NodeValue::Heading(heading) = node.data.borrow().value {
                if heading.level == self.level {
                    if seen_top_level_header {
                        let position = node.data.borrow().sourcepos;
                        let violation = self.to_violation(doc.path.clone(), position);
                        violations.push(violation);
                    } else {
                        seen_top_level_header = true;
                    }
                }
            }
        }

        Ok(violations)
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use comrak::{nodes::Sourcepos, parse_document, Arena, Options};

    use super::*;

    #[test]
    fn check_errors() {
        let text = "# Top level header

# Another top level header"
            .to_owned();
        let path = Path::new("test.md").to_path_buf();
        let arena = Arena::new();
        let ast = parse_document(&arena, &text, &Options::default());
        let doc = Document {
            path: path.clone(),
            ast,
            text,
        };
        let rule = MD025::default();
        let actual = rule.check(&doc).unwrap();
        let expected = vec![rule.to_violation(path.clone(), Sourcepos::from((3, 1, 3, 26)))];
        assert_eq!(actual, expected);
    }

    #[test]
    fn check_no_errors() {
        let text = "# Title

## Header

## Another header"
            .to_owned();
        let path = Path::new("test.md").to_path_buf();
        let arena = Arena::new();
        let ast = parse_document(&arena, &text, &Options::default());
        let doc = Document { path, ast, text };
        let rule = MD025::default();
        let actual = rule.check(&doc).unwrap();
        let expected = vec![];
        assert_eq!(actual, expected);
    }
}

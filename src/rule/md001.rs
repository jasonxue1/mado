use comrak::nodes::NodeValue;
use miette::Result;

use crate::{violation::Violation, Document};

use super::Rule;

#[derive(Default, Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct MD001;

impl MD001 {
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        Self {}
    }
}

impl Rule for MD001 {
    #[inline]
    fn name(&self) -> String {
        "MD001".to_owned()
    }

    #[inline]
    fn description(&self) -> String {
        "Header levels should only increment by one level at a time".to_owned()
    }

    #[inline]
    fn tags(&self) -> Vec<String> {
        vec!["headers".to_owned()]
    }

    #[inline]
    fn aliases(&self) -> Vec<String> {
        vec!["header-increment".to_owned()]
    }

    #[inline]
    fn check(&self, doc: &Document) -> Result<Vec<Violation>> {
        let mut violations = vec![];
        let mut maybe_prev_level = None;

        for node in doc.ast.children() {
            if let NodeValue::Heading(heading) = node.data.borrow().value {
                if let Some(prev_level) = maybe_prev_level {
                    if heading.level > prev_level + 1 {
                        let position = node.data.clone().borrow().sourcepos;
                        let violation = self.to_violation(doc.path.clone(), position);
                        violations.push(violation);
                    }
                }

                maybe_prev_level = Some(heading.level);
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
        let text = "# Header 1

### Header 3

We skipped out a 2nd level header in this document"
            .to_owned();
        let path = Path::new("test.md").to_path_buf();
        let arena = Arena::new();
        let ast = parse_document(&arena, &text, &Options::default());
        let doc = Document {
            path: path.clone(),
            ast,
            text,
        };
        let rule = MD001::new();
        let actual = rule.check(&doc).unwrap();
        let expected = vec![rule.to_violation(path, Sourcepos::from((3, 1, 3, 12)))];
        assert_eq!(actual, expected);
    }

    #[test]
    fn check_no_errors() {
        let text = "# Header 1

## Header 2

### Header 3

#### Header 4

## Another Header 2

### Another Header 3"
            .to_owned();
        let path = Path::new("test.md").to_path_buf();
        let arena = Arena::new();
        let ast = parse_document(&arena, &text, &Options::default());
        let doc = Document { path, ast, text };
        let rule = MD001::new();
        let actual = rule.check(&doc).unwrap();
        let expected = vec![];
        assert_eq!(actual, expected);
    }

    #[test]
    fn check_no_errors_no_top_level() {
        let text = "## This isn't a H1 header".to_owned();
        let path = Path::new("test.md").to_path_buf();
        let arena = Arena::new();
        let ast = parse_document(&arena, &text, &Options::default());
        let doc = Document { path, ast, text };
        let rule = MD001::new();
        let actual = rule.check(&doc).unwrap();
        let expected = vec![];
        assert_eq!(actual, expected);
    }
}

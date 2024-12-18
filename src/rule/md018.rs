use comrak::nodes::NodeValue;
use miette::Result;

use crate::violation::Violation;
use crate::Document;

use super::RuleLike;

#[derive(Default, Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct MD018;

impl MD018 {
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        Self {}
    }
}

impl RuleLike for MD018 {
    #[inline]
    fn name(&self) -> String {
        "MD018".to_owned()
    }

    #[inline]
    fn description(&self) -> String {
        "No space after hash on atx style header".to_owned()
    }

    #[inline]
    fn tags(&self) -> Vec<String> {
        vec!["headers".to_owned(), "atx".to_owned(), "spaces".to_owned()]
    }

    #[inline]
    fn aliases(&self) -> Vec<String> {
        vec!["no-missing-space-atx".to_owned()]
    }

    #[inline]
    fn check(&self, doc: &Document) -> Result<Vec<Violation>> {
        let mut violations = vec![];

        for node in doc.ast.descendants() {
            if let NodeValue::Text(text) = node.data.borrow().value.clone() {
                let position = node.data.borrow().sourcepos;
                if position.start.column == 1 && text.starts_with('#') {
                    let violation = self.to_violation(doc.path.clone(), position);
                    violations.push(violation);
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
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn check_errors() {
        let text = "#Header 1

##Header 2"
            .to_owned();
        let path = Path::new("test.md").to_path_buf();
        let arena = Arena::new();
        let ast = parse_document(&arena, &text, &Options::default());
        let doc = Document {
            path: path.clone(),
            ast,
            text,
        };
        let rule = MD018::default();
        let actual = rule.check(&doc).unwrap();
        let expected = vec![
            rule.to_violation(path.clone(), Sourcepos::from((1, 1, 1, 9))),
            rule.to_violation(path, Sourcepos::from((3, 1, 3, 10))),
        ];
        assert_eq!(actual, expected);
    }

    #[test]
    fn check_no_errors() {
        let text = "# Header 1

## Header 2"
            .to_owned();
        let path = Path::new("test.md").to_path_buf();
        let arena = Arena::new();
        let ast = parse_document(&arena, &text, &Options::default());
        let doc = Document { path, ast, text };
        let rule = MD018::default();
        let actual = rule.check(&doc).unwrap();
        let expected = vec![];
        assert_eq!(actual, expected);
    }

    #[test]
    fn check_no_errors_with_issue_number() {
        let text = "# Header 1

See [#4649](https://example.com) for details."
            .to_owned();
        let path = Path::new("test.md").to_path_buf();
        let arena = Arena::new();
        let ast = parse_document(&arena, &text, &Options::default());
        let doc = Document { path, ast, text };
        let rule = MD018::default();
        let actual = rule.check(&doc).unwrap();
        let expected = vec![];
        assert_eq!(actual, expected);
    }
}

use comrak::nodes::NodeValue;
use miette::IntoDiagnostic as _;
use miette::Result;
use regex::Regex;

use crate::{violation::Violation, Document};

use super::RuleLike;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct MD037;

impl MD037 {
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        Self {}
    }
}

impl RuleLike for MD037 {
    #[inline]
    fn name(&self) -> String {
        "MD037".to_owned()
    }

    #[inline]
    fn description(&self) -> String {
        "Spaces inside emphasis markers".to_owned()
    }

    #[inline]
    fn tags(&self) -> Vec<String> {
        vec!["whitespace".to_owned(), "emphasis".to_owned()]
    }

    #[inline]
    fn aliases(&self) -> Vec<String> {
        vec!["no-space-in-emphasis".to_owned()]
    }

    // TODO: Use safe casting
    #[inline]
    #[allow(clippy::cast_possible_wrap)]
    fn check(&self, doc: &Document) -> Result<Vec<Violation>> {
        let mut violations = vec![];
        let re = Regex::new(r"(\*\*? *[^*]+ *\*?\*|\_\_? *[^_]+ *\_?\_)").into_diagnostic()?;

        for node in doc.ast.descendants() {
            if let NodeValue::Text(text) = &node.data.borrow().value {
                if let Some(m) = re.find(text) {
                    let mut position = node.data.borrow().sourcepos;
                    position.end = position.start.column_add(m.end() as isize - 1);
                    position.start = position.start.column_add(m.start() as isize);

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
        let text = "Here is some ** bold ** text.

Here is some * italic * text.

Here is some more __ bold __ text.

Here is some more _ italic _ text."
            .to_owned();
        let path = Path::new("test.md").to_path_buf();
        let arena = Arena::new();
        let ast = parse_document(&arena, &text, &Options::default());
        let doc = Document {
            path: path.clone(),
            ast,
            text,
        };
        let rule = MD037::new();
        let actual = rule.check(&doc).unwrap();
        let expected = vec![
            rule.to_violation(path.clone(), Sourcepos::from((1, 14, 1, 23))),
            rule.to_violation(path.clone(), Sourcepos::from((3, 14, 3, 23))),
            rule.to_violation(path.clone(), Sourcepos::from((5, 19, 5, 28))),
            rule.to_violation(path, Sourcepos::from((7, 19, 7, 28))),
        ];
        assert_eq!(actual, expected);
    }

    #[test]
    fn check_errors_2() {
        let text = "Here is some **bold ** text.

Here is some * italic* text.

Here is some more __bold __ text.

Here is some more _ italic_ text."
            .to_owned();
        let path = Path::new("test.md").to_path_buf();
        let arena = Arena::new();
        let ast = parse_document(&arena, &text, &Options::default());
        let doc = Document {
            path: path.clone(),
            ast,
            text,
        };
        let rule = MD037::new();
        let actual = rule.check(&doc).unwrap();
        let expected = vec![
            rule.to_violation(path.clone(), Sourcepos::from((1, 14, 1, 22))),
            rule.to_violation(path.clone(), Sourcepos::from((3, 14, 3, 22))),
            rule.to_violation(path.clone(), Sourcepos::from((5, 19, 5, 27))),
            rule.to_violation(path, Sourcepos::from((7, 19, 7, 27))),
        ];
        assert_eq!(actual, expected);
    }

    #[test]
    fn check_no_errors() {
        let text = "Here is some **bold** text.

Here is some *italic* text.

Here is some more __bold__ text.

Here is some more _italic_ text."
            .to_owned();
        let path = Path::new("test.md").to_path_buf();
        let arena = Arena::new();
        let ast = parse_document(&arena, &text, &Options::default());
        let doc = Document { path, ast, text };
        let rule = MD037::new();
        let actual = rule.check(&doc).unwrap();
        let expected = vec![];
        assert_eq!(actual, expected);
    }

    #[test]
    fn check_no_errors_nested() {
        let text = "Here is ** some **bold** text ** .

Here is * some *italic* text * .

Here is some __ more __bold__ text __ .

Here is some _ more _italic_ text _ ."
            .to_owned();
        let path = Path::new("test.md").to_path_buf();
        let arena = Arena::new();
        let ast = parse_document(&arena, &text, &Options::default());
        let doc = Document { path, ast, text };
        let rule = MD037::new();
        let actual = rule.check(&doc).unwrap();
        let expected = vec![];
        assert_eq!(actual, expected);
    }
}

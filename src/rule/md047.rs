use comrak::nodes::Sourcepos;
use miette::Result;

use crate::{violation::Violation, Document};

use super::RuleLike;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct MD047;

impl MD047 {
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        Self {}
    }
}

impl RuleLike for MD047 {
    #[inline]
    fn name(&self) -> &'static str {
        "MD047"
    }

    #[inline]
    fn description(&self) -> &'static str {
        "Code block style"
    }

    #[inline]
    fn tags(&self) -> Vec<&'static str> {
        vec!["code"]
    }

    #[inline]
    fn aliases(&self) -> Vec<&'static str> {
        vec!["code-block-style"]
    }

    #[inline]
    fn check(&self, doc: &Document) -> Result<Vec<Violation>> {
        if doc.text.is_empty() || doc.text.ends_with('\n') {
            return Ok(vec![]);
        }

        let lineno = doc.text.lines().count();
        let end_column = doc.text.lines().last().unwrap_or("").len() + 1;
        let position = Sourcepos::from((lineno, 1, lineno, end_column));
        let violation = self.to_violation(doc.path.clone(), position);

        Ok(vec![violation])
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
        let text = "Some text".to_owned();
        let path = Path::new("test.md").to_path_buf();
        let arena = Arena::new();
        let ast = parse_document(&arena, &text, &Options::default());
        let doc = Document {
            path: path.clone(),
            ast,
            text,
        };
        let rule = MD047::default();
        let actual = rule.check(&doc).unwrap();
        let expected = vec![rule.to_violation(path, Sourcepos::from((1, 1, 1, 10)))];
        assert_eq!(actual, expected);
    }

    #[test]
    fn check_no_errors() {
        let text = "Some text
"
        .to_owned();
        let path = Path::new("test.md").to_path_buf();
        let arena = Arena::new();
        let ast = parse_document(&arena, &text, &Options::default());
        let doc = Document { path, ast, text };
        let rule = MD047::default();
        let actual = rule.check(&doc).unwrap();
        let expected = vec![];
        assert_eq!(actual, expected);
    }

    #[test]
    fn check_errors_with_empty_string() {
        let text = String::new();
        let path = Path::new("test.md").to_path_buf();
        let arena = Arena::new();
        let ast = parse_document(&arena, &text, &Options::default());
        let doc = Document {
            path: path.clone(),
            ast,
            text,
        };
        let rule = MD047::default();
        let actual = rule.check(&doc).unwrap();
        let expected = vec![];
        assert_eq!(actual, expected);
    }
}

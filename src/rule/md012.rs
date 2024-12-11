use comrak::nodes::Sourcepos;
use miette::Result;

use crate::{violation::Violation, Document};

use super::Rule;

#[derive(Default, Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct MD012;

impl MD012 {
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        Self {}
    }
}

impl Rule for MD012 {
    #[inline]
    fn name(&self) -> String {
        "MD012".to_owned()
    }

    #[inline]
    fn description(&self) -> String {
        "Multiple consecutive blank lines".to_owned()
    }

    #[inline]
    fn tags(&self) -> Vec<String> {
        vec!["whitespace".to_owned(), "blank_lines".to_owned()]
    }

    #[inline]
    fn aliases(&self) -> Vec<String> {
        vec!["no-multiple-blanks".to_owned()]
    }

    #[inline]
    fn check(&self, doc: &Document) -> Result<Vec<Violation>> {
        let mut violations = vec![];
        let mut maybe_prev_line: Option<&str> = None;

        for (i, line) in doc.text.lines().enumerate() {
            let lineno = i + 1;

            if let Some(prev_line) = maybe_prev_line {
                if prev_line.is_empty() && line.is_empty() {
                    let position = Sourcepos::from((lineno, 0, lineno, 0));
                    let violation = self.to_violation(doc.path.clone(), position);
                    violations.push(violation);
                }
            }

            maybe_prev_line = Some(line);
        }

        Ok(violations)
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use comrak::{parse_document, Arena, Options};

    use super::*;

    #[test]
    fn check_errors() {
        let text = "Some text here


Some more text here";
        let path = Path::new("test.md").to_path_buf();
        let arena = Arena::new();
        let ast = parse_document(&arena, text, &Options::default());
        let doc = Document {
            path: path.clone(),
            ast,
            text: text.to_string(),
        };
        let rule = MD012::new();
        let actual = rule.check(&doc).unwrap();
        let expected = vec![rule.to_violation(path, Sourcepos::from((3, 0, 3, 0)))];
        assert_eq!(actual, expected);
    }

    #[test]
    fn check_no_errors() {
        let text = "Some text here

Some more text here";
        let path = Path::new("test.md").to_path_buf();
        let arena = Arena::new();
        let ast = parse_document(&arena, text, &Options::default());
        let doc = Document {
            path,
            ast,
            text: text.to_string(),
        };
        let rule = MD012::new();
        let actual = rule.check(&doc).unwrap();
        let expected = vec![];
        assert_eq!(actual, expected);
    }
}

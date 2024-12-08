use markdown::unist::Position;
use miette::Result;

use crate::{violation::Violation, Document};

use super::Rule;

#[derive(Default)]
pub struct MD012 {}

impl MD012 {
    pub fn new() -> Self {
        Self {}
    }
}

impl Rule for MD012 {
    #[inline]
    fn name(&self) -> String {
        "MD012".to_string()
    }

    #[inline]
    fn description(&self) -> String {
        "Multiple consecutive blank lines".to_string()
    }

    #[inline]
    fn tags(&self) -> Vec<String> {
        vec!["whitespace".to_string(), "blank_lines".to_string()]
    }

    #[inline]
    fn aliases(&self) -> Vec<String> {
        vec!["no-multiple-blanks".to_string()]
    }

    fn check(&self, doc: &Document) -> Result<Vec<Violation>> {
        let mut violations = vec![];
        let mut maybe_prev_line: Option<&str> = None;

        for (i, line) in doc.text.lines().enumerate() {
            let lineno = i + 1;

            if let Some(prev_line) = maybe_prev_line {
                if prev_line.is_empty() && line.is_empty() {
                    // TODO: Use correct offset
                    let position = Position::new(lineno, 0, 0, lineno, 1, 0);
                    let violation = self.to_violation(position);
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

    use markdown::{unist::Position, ParseOptions};

    use super::*;

    #[test]
    fn check_errors() {
        let text = "Some text here


Some more text here";
        let path = Path::new("test.md").to_path_buf();
        let ast = markdown::to_mdast(text, &ParseOptions::default()).unwrap();
        let doc = Document {
            path,
            ast,
            text: text.to_string(),
        };
        let rule = MD012::new();
        let actual = rule.check(&doc).unwrap();
        let expected = vec![rule.to_violation(Position::new(3, 0, 0, 3, 1, 0))];
        assert_eq!(actual, expected);
    }

    #[test]
    fn check_no_errors() {
        let text = "Some text here

Some more text here";
        let path = Path::new("test.md").to_path_buf();
        let ast = markdown::to_mdast(text, &ParseOptions::default()).unwrap();
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

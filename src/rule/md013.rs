use markdown::unist::Position;
use miette::Result;

use crate::{violation::Violation, Document};

use super::Rule;

pub struct MD013 {
    line_length: usize,
}

impl MD013 {
    pub fn new(line_length: usize) -> Self {
        Self { line_length }
    }
}

impl Default for MD013 {
    fn default() -> Self {
        Self { line_length: 80 }
    }
}

impl Rule for MD013 {
    #[inline]
    fn name(&self) -> String {
        "MD013".to_string()
    }

    #[inline]
    fn description(&self) -> String {
        "Line length".to_string()
    }

    #[inline]
    fn tags(&self) -> Vec<String> {
        vec!["line_length".to_string()]
    }

    #[inline]
    fn aliases(&self) -> Vec<String> {
        vec!["line-length".to_string()]
    }

    fn check(&self, doc: &Document) -> Result<Vec<Violation>> {
        let mut violations = vec![];

        for (i, line) in doc.text.lines().enumerate() {
            let lineno = i + 1;

            if line.len() > self.line_length {
                // TODO: Use correct offset
                let position =
                    Position::new(lineno, self.line_length + 1, 0, lineno, line.len(), 0);
                let violation = self.to_violation(position);
                violations.push(violation);
            }
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
        let text =
            "This is a very very very very very very very very very very very very very long line";
        let path = Path::new("test.md").to_path_buf();
        let ast = markdown::to_mdast(text, &ParseOptions::default()).unwrap();
        let doc = Document {
            path,
            ast,
            text: text.to_string(),
        };
        let rule = MD013::default();
        let actual = rule.check(&doc).unwrap();
        let expected = vec![rule.to_violation(Position::new(1, 81, 0, 1, 84, 0))];
        assert_eq!(actual, expected);
    }

    #[test]
    fn check_no_errors() {
        let text = "This is a short line";
        let path = Path::new("test.md").to_path_buf();
        let ast = markdown::to_mdast(text, &ParseOptions::default()).unwrap();
        let doc = Document {
            path,
            ast,
            text: text.to_string(),
        };
        let rule = MD013::default();
        let actual = rule.check(&doc).unwrap();
        let expected = vec![];
        assert_eq!(actual, expected);
    }
}

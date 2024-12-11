use comrak::nodes::Sourcepos;
use miette::Result;

use crate::{violation::Violation, Document};

use super::Rule;

pub struct MD013 {
    line_length: usize,
}

impl MD013 {
    #[inline]
    #[must_use]
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
                let position =
                    Sourcepos::from((lineno, self.line_length + 1, lineno, line.len() - 1));
                let violation = self.to_violation(doc.path.clone(), position);
                violations.push(violation);
            }
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
        let text =
            "This is a very very very very very very very very very very very very very long line";
        let path = Path::new("test.md").to_path_buf();
        let arena = Arena::new();
        let ast = parse_document(&arena, text, &Options::default());
        let doc = Document {
            path: path.clone(),
            ast,
            text: text.to_string(),
        };
        let rule = MD013::default();
        let actual = rule.check(&doc).unwrap();
        let expected = vec![rule.to_violation(path, Sourcepos::from((1, 81, 1, 83)))];
        assert_eq!(actual, expected);
    }

    #[test]
    fn check_no_errors() {
        let text = "This is a short line";
        let path = Path::new("test.md").to_path_buf();
        let arena = Arena::new();
        let ast = parse_document(&arena, text, &Options::default());
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

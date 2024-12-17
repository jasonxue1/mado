use comrak::nodes::NodeValue;
use miette::Result;

use crate::{violation::Violation, Document};

use super::{helper::inline_text_of, Rule};

#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct MD026 {
    punctuation: String,
}

impl MD026 {
    #[inline]
    #[must_use]
    pub fn new(punctuation: String) -> Self {
        Self { punctuation }
    }
}

impl Default for MD026 {
    #[inline]
    fn default() -> Self {
        Self {
            punctuation: ".,;:!?".to_owned(),
        }
    }
}

impl Rule for MD026 {
    #[inline]
    fn name(&self) -> String {
        "MD026".to_owned()
    }

    #[inline]
    fn description(&self) -> String {
        "Trailing punctuation in header".to_owned()
    }

    #[inline]
    fn tags(&self) -> Vec<String> {
        vec!["headers".to_owned()]
    }

    #[inline]
    fn aliases(&self) -> Vec<String> {
        vec!["no-trailing-punctuation".to_owned()]
    }

    #[inline]
    fn check(&self, doc: &Document) -> Result<Vec<Violation>> {
        let mut violations = vec![];

        for node in doc.ast.children() {
            if let NodeValue::Heading(_) = node.data.borrow().value {
                let text = inline_text_of(node);
                if let Some(last_char) = text.chars().last() {
                    if self.punctuation.contains(last_char) {
                        let position = node.data.borrow().sourcepos;
                        let violation = self.to_violation(doc.path.clone(), position);
                        violations.push(violation);
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
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn check_errors() {
        let text = "# This is a header.".to_owned();
        let path = Path::new("test.md").to_path_buf();
        let arena = Arena::new();
        let ast = parse_document(&arena, &text, &Options::default());
        let doc = Document {
            path: path.clone(),
            ast,
            text,
        };
        let rule = MD026::default();
        let actual = rule.check(&doc).unwrap();
        let expected = vec![rule.to_violation(path.clone(), Sourcepos::from((1, 1, 1, 19)))];
        assert_eq!(actual, expected);
    }

    #[test]
    fn check_no_errors() {
        let text = "# This is a header".to_owned();
        let path = Path::new("test.md").to_path_buf();
        let arena = Arena::new();
        let ast = parse_document(&arena, &text, &Options::default());
        let doc = Document { path, ast, text };
        let rule = MD026::default();
        let actual = rule.check(&doc).unwrap();
        let expected = vec![];
        assert_eq!(actual, expected);
    }
}

use comrak::nodes::NodeValue;
use miette::Result;

use crate::{violation::Violation, Document};

use super::Rule;

#[derive(Default)]
pub struct MD005 {}

impl MD005 {
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        Self {}
    }
}

impl Rule for MD005 {
    #[inline]
    fn name(&self) -> String {
        "MD005".to_string()
    }

    #[inline]
    fn description(&self) -> String {
        "Inconsistent indentation for list items at the same level".to_string()
    }

    #[inline]
    fn tags(&self) -> Vec<String> {
        vec![
            "bullet".to_string(),
            "ul".to_string(),
            "indentation".to_string(),
        ]
    }

    #[inline]
    fn aliases(&self) -> Vec<String> {
        vec!["list-indent".to_string()]
    }

    fn check(&self, doc: &Document) -> Result<Vec<Violation>> {
        let mut violations = vec![];

        for node in doc.ast.descendants() {
            if let NodeValue::List(_) = node.data.borrow().value {
                let mut maybe_first_item_offset = None;
                for item_node in node.children() {
                    if let NodeValue::Item(item) = item_node.data.borrow().value {
                        match maybe_first_item_offset {
                            Some(first_item_offset) => {
                                if first_item_offset != item.marker_offset {
                                    let position = item_node.data.borrow().sourcepos;
                                    let violation = self.to_violation(doc.path.clone(), position);
                                    violations.push(violation);
                                }
                            }
                            None => {
                                maybe_first_item_offset = Some(item.marker_offset);
                            }
                        }
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
        let text = "* Item 1
    * Nested Item 1
    * Nested Item 2
   * A misaligned item";
        let path = Path::new("test.md").to_path_buf();
        let arena = Arena::new();
        let ast = parse_document(&arena, text, &Options::default());
        let doc = Document {
            path: path.clone(),
            ast,
            text: text.to_string(),
        };
        let rule = MD005::new();
        let actual = rule.check(&doc).unwrap();
        let expected = vec![rule.to_violation(path, Sourcepos::from((4, 4, 4, 22)))];
        assert_eq!(actual, expected);
    }

    #[test]
    fn check_errors_for_empty_item_text() {
        let text = "*
    *
    *
   *";
        let path = Path::new("test.md").to_path_buf();
        let arena = Arena::new();
        let ast = parse_document(&arena, text, &Options::default());
        let doc = Document {
            path: path.clone(),
            ast,
            text: text.to_string(),
        };
        let rule = MD005::new();
        let actual = rule.check(&doc).unwrap();
        let expected = vec![rule.to_violation(path, Sourcepos::from((4, 4, 4, 4)))];
        assert_eq!(actual, expected);
    }

    #[test]
    fn check_no_errors() {
        let text = "* Item 1
    * Nested Item 1
    * Nested Item 2
    * Nested Item 3";
        let path = Path::new("test.md").to_path_buf();
        let arena = Arena::new();
        let ast = parse_document(&arena, text, &Options::default());
        let doc = Document {
            path,
            ast,
            text: text.to_string(),
        };
        let rule = MD005::new();
        let actual = rule.check(&doc).unwrap();
        let expected = vec![];
        assert_eq!(actual, expected);
    }
}

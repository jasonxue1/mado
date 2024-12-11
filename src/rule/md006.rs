use comrak::nodes::NodeValue;
use miette::Result;

use crate::{violation::Violation, Document};

use super::Rule;

#[derive(Default)]
pub struct MD006 {}

impl MD006 {
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        Self {}
    }
}

impl Rule for MD006 {
    #[inline]
    fn name(&self) -> String {
        "MD006".to_string()
    }

    #[inline]
    fn description(&self) -> String {
        "Consider starting bulleted lists at the beginning of the line".to_string()
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
        vec!["ul-start-left".to_string()]
    }

    fn check(&self, doc: &Document) -> Result<Vec<Violation>> {
        let mut violations = vec![];

        for node in doc.ast.children() {
            if let NodeValue::List(_) = node.data.borrow().value {
                for item_node in node.children() {
                    if let NodeValue::Item(item) = item_node.data.borrow().value {
                        if item.marker_offset > 0 {
                            let position = item_node.data.borrow().sourcepos;
                            let violation = self.to_violation(doc.path.clone(), position);
                            violations.push(violation);
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
        let text = "Some text

  * List item
  * List item";
        let path = Path::new("test.md").to_path_buf();
        let arena = Arena::new();
        let ast = parse_document(&arena, text, &Options::default());
        let doc = Document {
            path: path.clone(),
            ast,
            text: text.to_string(),
        };
        let rule = MD006::new();
        let actual = rule.check(&doc).unwrap();
        let expected = vec![
            rule.to_violation(path.clone(), Sourcepos::from((3, 3, 3, 13))),
            rule.to_violation(path, Sourcepos::from((4, 3, 4, 13))),
        ];
        assert_eq!(actual, expected);
    }

    #[test]
    fn check_no_errors() {
        let text = "Some test

* List item
* List item";
        let path = Path::new("test.md").to_path_buf();
        let arena = Arena::new();
        let ast = parse_document(&arena, text, &Options::default());
        let doc = Document {
            path: path.clone(),
            ast,
            text: text.to_string(),
        };
        let rule = MD006::new();
        let actual = rule.check(&doc).unwrap();
        let expected = vec![];
        assert_eq!(actual, expected);
    }
}

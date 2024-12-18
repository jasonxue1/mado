use comrak::nodes::{ListType, NodeValue};
use miette::Result;

use crate::{violation::Violation, Document};

use super::RuleLike;

#[derive(Default, Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct MD006;

impl MD006 {
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        Self {}
    }
}

impl RuleLike for MD006 {
    #[inline]
    fn name(&self) -> String {
        "MD006".to_owned()
    }

    #[inline]
    fn description(&self) -> String {
        "Consider starting bulleted lists at the beginning of the line".to_owned()
    }

    #[inline]
    fn tags(&self) -> Vec<String> {
        vec![
            "bullet".to_owned(),
            "ul".to_owned(),
            "indentation".to_owned(),
        ]
    }

    #[inline]
    fn aliases(&self) -> Vec<String> {
        vec!["ul-start-left".to_owned()]
    }

    #[inline]
    fn check(&self, doc: &Document) -> Result<Vec<Violation>> {
        let mut violations = vec![];

        for node in doc.ast.children() {
            if let NodeValue::List(list) = node.data.borrow().value {
                if list.list_type != ListType::Bullet {
                    continue;
                }

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
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn check_errors() {
        let text = "Some text

  * List item
  * List item"
            .to_owned();
        let path = Path::new("test.md").to_path_buf();
        let arena = Arena::new();
        let ast = parse_document(&arena, &text, &Options::default());
        let doc = Document {
            path: path.clone(),
            ast,
            text,
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
* List item"
            .to_owned();
        let path = Path::new("test.md").to_path_buf();
        let arena = Arena::new();
        let ast = parse_document(&arena, &text, &Options::default());
        let doc = Document { path, ast, text };
        let rule = MD006::new();
        let actual = rule.check(&doc).unwrap();
        let expected = vec![];
        assert_eq!(actual, expected);
    }

    #[test]
    fn check_no_errors_with_ordered_list() {
        let text = "Some test

 1. Ordered list item
 2. Ordered list item"
            .to_owned();
        let path = Path::new("test.md").to_path_buf();
        let arena = Arena::new();
        let ast = parse_document(&arena, &text, &Options::default());
        let doc = Document { path, ast, text };
        let rule = MD006::new();
        let actual = rule.check(&doc).unwrap();
        let expected = vec![];
        assert_eq!(actual, expected);
    }
}

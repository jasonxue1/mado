use std::path::PathBuf;

use comrak::nodes::{AstNode, ListType, NodeValue};
use miette::Result;
use serde::Deserialize;

use crate::{violation::Violation, Document};

use super::RuleLike;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "lowercase")]
#[non_exhaustive]
pub enum OrderedListStyle {
    One,
    Ordered,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct MD029 {
    style: OrderedListStyle,
}

impl MD029 {
    #[inline]
    #[must_use]
    pub fn new(style: OrderedListStyle) -> Self {
        Self { style }
    }

    fn check_recursive<'a>(
        &self,
        root: &'a AstNode<'a>,
        path: &PathBuf,
        violations: &mut Vec<Violation>,
    ) {
        for node in root.children() {
            if let NodeValue::List(list) = node.data.borrow().value {
                let mut maybe_prev_start = None;

                for item_node in node.children() {
                    if let NodeValue::Item(item) = item_node.data.borrow().value {
                        if list.list_type == ListType::Ordered {
                            let is_violated = match (&self.style, maybe_prev_start) {
                                (OrderedListStyle::One, _) => item.start != 1,
                                (OrderedListStyle::Ordered, Some(prev_start)) => {
                                    item.start != prev_start + 1
                                }
                                _ => false,
                            };

                            if is_violated {
                                let position = item_node.data.borrow().sourcepos;
                                let violation = self.to_violation(path.clone(), position);
                                violations.push(violation);
                            }

                            maybe_prev_start = Some(item.start);
                        }

                        self.check_recursive(item_node, path, violations);
                    }
                }
            }
        }
    }
}

impl Default for MD029 {
    #[inline]
    fn default() -> Self {
        Self {
            style: OrderedListStyle::One,
        }
    }
}

impl RuleLike for MD029 {
    #[inline]
    fn name(&self) -> String {
        "MD029".to_owned()
    }

    #[inline]
    fn description(&self) -> String {
        "Ordered list item prefix".to_owned()
    }

    #[inline]
    fn tags(&self) -> Vec<String> {
        vec!["ol".to_owned()]
    }

    #[inline]
    fn aliases(&self) -> Vec<String> {
        vec!["ol-prefix".to_owned()]
    }

    #[inline]
    fn check(&self, doc: &Document) -> Result<Vec<Violation>> {
        let mut violations = vec![];

        self.check_recursive(doc.ast, &doc.path, &mut violations);

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
    fn check_errors_one() {
        let text = "1. Do this.
2. Do that.
3. Done."
            .to_owned();
        let path = Path::new("test.md").to_path_buf();
        let arena = Arena::new();
        let ast = parse_document(&arena, &text, &Options::default());
        let doc = Document {
            path: path.clone(),
            ast,
            text,
        };
        let rule = MD029::default();
        let actual = rule.check(&doc).unwrap();
        let expected = vec![
            rule.to_violation(path.clone(), Sourcepos::from((2, 1, 2, 11))),
            rule.to_violation(path, Sourcepos::from((3, 1, 3, 8))),
        ];
        assert_eq!(actual, expected);
    }

    #[test]
    fn check_errors_recursive() {
        let text = "* Parent list
    1. Do this.
    2. Do that.
    3. Done."
            .to_owned();
        let path = Path::new("test.md").to_path_buf();
        let arena = Arena::new();
        let ast = parse_document(&arena, &text, &Options::default());
        let doc = Document {
            path: path.clone(),
            ast,
            text,
        };
        let rule = MD029::default();
        let actual = rule.check(&doc).unwrap();
        let expected = vec![
            rule.to_violation(path.clone(), Sourcepos::from((3, 5, 3, 15))),
            rule.to_violation(path, Sourcepos::from((4, 5, 4, 12))),
        ];
        assert_eq!(actual, expected);
    }

    #[test]
    fn check_errors_ordered() {
        let text = "1. Do this.
1. Do that.
1. Done."
            .to_owned();
        let path = Path::new("test.md").to_path_buf();
        let arena = Arena::new();
        let ast = parse_document(&arena, &text, &Options::default());
        let doc = Document {
            path: path.clone(),
            ast,
            text,
        };
        let rule = MD029::new(OrderedListStyle::Ordered);
        let actual = rule.check(&doc).unwrap();
        let expected = vec![
            rule.to_violation(path.clone(), Sourcepos::from((2, 1, 2, 11))),
            rule.to_violation(path, Sourcepos::from((3, 1, 3, 8))),
        ];
        assert_eq!(actual, expected);
    }

    #[test]
    fn check_no_errors_one() {
        let text = "1. Do this.
1. Do that.
1. Done."
            .to_owned();
        let path = Path::new("test.md").to_path_buf();
        let arena = Arena::new();
        let ast = parse_document(&arena, &text, &Options::default());
        let doc = Document { path, ast, text };
        let rule = MD029::default();
        let actual = rule.check(&doc).unwrap();
        let expected = vec![];
        assert_eq!(actual, expected);
    }

    #[test]
    fn check_no_errors_ordered() {
        let text = "1. Do this.
2. Do that.
3. Done."
            .to_owned();
        let path = Path::new("test.md").to_path_buf();
        let arena = Arena::new();
        let ast = parse_document(&arena, &text, &Options::default());
        let doc = Document { path, ast, text };
        let rule = MD029::new(OrderedListStyle::Ordered);
        let actual = rule.check(&doc).unwrap();
        let expected = vec![];
        assert_eq!(actual, expected);
    }

    #[test]
    fn check_no_errors_recursive() {
        let text = "* Parent list
    1. Do this.
    1. Do that.
    1. Done."
            .to_owned();
        let path = Path::new("test.md").to_path_buf();
        let arena = Arena::new();
        let ast = parse_document(&arena, &text, &Options::default());
        let doc = Document { path, ast, text };
        let rule = MD029::default();
        let actual = rule.check(&doc).unwrap();
        let expected = vec![];
        assert_eq!(actual, expected);
    }
}

use markdown::{mdast::Node, unist::Position};
use miette::Result;

use crate::{violation::Violation, Document};

use super::Rule;

#[derive(Default)]
pub struct MD005 {}

impl MD005 {
    pub fn new() -> Self {
        Self {}
    }

    fn check_recursive(&self, ast: &Node) -> Result<Vec<Violation>> {
        match ast.children() {
            Some(children) => {
                let all_violations = children.iter().fold(vec![], |mut acc, node| match node {
                    Node::List(list) => {
                        let (list_violations, _) = list.children.iter().fold(
                            (vec![], None::<Position>),
                            |(mut acc, maybe_indent), item_node| match item_node.clone() {
                                // NOTE: The position of Node::ListItem's is limited to be the same column even if it is not consistent,
                                //       so use the position of the inner node instead.
                                //       This prevents consistency checks if inner nodes are empty.
                                // TODO: Use Result instead of expect
                                Node::ListItem(item) if item.children.is_empty() => {
                                    (acc, maybe_indent)
                                }
                                Node::ListItem(item) => {
                                    let first_child = item
                                        .children
                                        .first()
                                        .expect("list item must have children");
                                    let position = first_child
                                        .position()
                                        .expect("child of list item must have position")
                                        .clone();

                                    if let Some(indent) = maybe_indent {
                                        if position.start.column != indent.start.column {
                                            acc.push(self.to_violation(position.clone()));
                                        }
                                    }

                                    // Check list recursively
                                    let item_violations = self
                                        .check_recursive(item_node)
                                        .expect("check should be successful");
                                    acc.extend(item_violations);

                                    (acc, Some(position))
                                }
                                _ => (acc, maybe_indent),
                            },
                        );

                        acc.extend(list_violations);
                        acc
                    }
                    _ => acc,
                });
                Ok(all_violations)
            }
            None => Ok(vec![]),
        }
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
        self.check_recursive(&doc.ast)
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use markdown::{unist::Position, ParseOptions};

    use super::*;

    #[test]
    fn check_errors() {
        let text = "* Item 1
    * Nested Item 1
    * Nested Item 2
   * A misaligned item";
        let path = Path::new("test.md").to_path_buf();
        let ast = markdown::to_mdast(text, &ParseOptions::default()).unwrap();
        let doc = Document {
            path,
            ast,
            text: text.to_string(),
        };
        let rule = MD005::new();
        let actual = rule.check(&doc).unwrap();
        let expected = vec![rule.to_violation(Position::new(4, 6, 54, 4, 23, 71))];
        assert_eq!(actual, expected);
    }

    #[test]
    fn check_no_errors() {
        let text = "* Item 1
    * Nested Item 1
    * Nested Item 2
    * Nested Item 3";
        let path = Path::new("test.md").to_path_buf();
        let ast = markdown::to_mdast(text, &ParseOptions::default()).unwrap();
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

    // NOTE: Due to limitations of markdown-rs, consistency cannot be checked correctly.
    #[test]
    fn check_no_errors_empty_children() {
        let text = "*
    *
    *
   *";
        let path = Path::new("test.md").to_path_buf();
        let ast = markdown::to_mdast(text, &ParseOptions::default()).unwrap();
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

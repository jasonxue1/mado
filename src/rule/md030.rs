use std::path::PathBuf;

use comrak::nodes::{AstNode, ListType, NodeValue};
use miette::Result;

use crate::{violation::Violation, Document};

use super::RuleLike;

#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct MD030 {
    ul_single: usize,
    ol_single: usize,
    ul_multi: usize,
    ol_multi: usize,
}

impl MD030 {
    #[inline]
    #[must_use]
    pub fn new(ul_single: usize, ol_single: usize, ul_multi: usize, ol_multi: usize) -> Self {
        Self {
            ul_single,
            ol_single,
            ul_multi,
            ol_multi,
        }
    }

    fn check_recursive<'a>(
        &self,
        root: &'a AstNode<'a>,
        path: &PathBuf,
        violations: &mut Vec<Violation>,
    ) {
        for node in root.children() {
            if let NodeValue::List(_) = node.data.borrow().value {
                let num_items = node.children().count();
                for item_node in node.children() {
                    if let NodeValue::Item(item) = item_node.data.borrow().value {
                        let is_violated = match item.list_type {
                            ListType::Bullet if num_items == 1 => item.padding > self.ul_single + 1,
                            ListType::Bullet => item.padding > self.ul_multi + 1,
                            ListType::Ordered if num_items == 1 => {
                                item.padding > self.ol_single + 2
                            }
                            ListType::Ordered => item.padding > self.ol_multi + 2,
                        };

                        if is_violated {
                            let position = item_node.data.borrow().sourcepos;
                            let violation = self.to_violation(path.clone(), position);
                            violations.push(violation);
                        }

                        self.check_recursive(item_node, path, violations);
                    }
                }
            }
        }
    }
}

impl Default for MD030 {
    #[inline]
    fn default() -> Self {
        Self {
            ul_single: 1,
            ol_single: 1,
            ul_multi: 1,
            ol_multi: 1,
        }
    }
}

impl RuleLike for MD030 {
    #[inline]
    fn name(&self) -> String {
        "MD030".to_owned()
    }

    #[inline]
    fn description(&self) -> String {
        "Spaces after list markers".to_owned()
    }

    #[inline]
    fn tags(&self) -> Vec<String> {
        vec!["ol".to_owned(), "ul".to_owned(), "whitespace".to_owned()]
    }

    #[inline]
    fn aliases(&self) -> Vec<String> {
        vec!["list-marker-space".to_owned()]
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
    fn check_errors() {
        let text = "*   Foo

    Second paragraph

*   Bar"
            .to_owned();
        let path = Path::new("test.md").to_path_buf();
        let arena = Arena::new();
        let ast = parse_document(&arena, &text, &Options::default());
        let doc = Document {
            path: path.clone(),
            ast,
            text,
        };
        let rule = MD030::default();
        let actual = rule.check(&doc).unwrap();
        let expected = vec![
            rule.to_violation(path.clone(), Sourcepos::from((1, 1, 4, 0))),
            rule.to_violation(path, Sourcepos::from((5, 1, 5, 7))),
        ];
        assert_eq!(actual, expected);
    }

    #[test]
    fn check_errors_nested() {
        let text = "* Parent list
    1.  Foo
    2.  Bar"
            .to_owned();
        let path = Path::new("test.md").to_path_buf();
        let arena = Arena::new();
        let ast = parse_document(&arena, &text, &Options::default());
        let doc = Document {
            path: path.clone(),
            ast,
            text,
        };
        let rule = MD030::default();
        let actual = rule.check(&doc).unwrap();
        let expected = vec![
            rule.to_violation(path.clone(), Sourcepos::from((2, 5, 2, 11))),
            rule.to_violation(path, Sourcepos::from((3, 5, 3, 11))),
        ];
        assert_eq!(actual, expected);
    }

    #[test]
    fn check_no_errors() {
        let text = "* Foo
* Bar
* Baz

1. Foo
1. Bar
1. Baz

1. Foo
   * Bar
1. Baz"
            .to_owned();
        let path = Path::new("test.md").to_path_buf();
        let arena = Arena::new();
        let ast = parse_document(&arena, &text, &Options::default());
        let doc = Document { path, ast, text };
        let rule = MD030::default();
        let actual = rule.check(&doc).unwrap();
        let expected = vec![];
        assert_eq!(actual, expected);
    }
}

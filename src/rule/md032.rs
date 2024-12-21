use comrak::nodes::{AstNode, NodeValue};
use miette::Result;

use crate::{violation::Violation, Document};

use super::RuleLike;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct MD032;

impl MD032 {
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        Self {}
    }
}

impl RuleLike for MD032 {
    #[inline]
    fn name(&self) -> &'static str {
        "MD032"
    }

    #[inline]
    fn description(&self) -> &'static str {
        "Lists should be surrounded by blank lines"
    }

    #[inline]
    fn tags(&self) -> Vec<&'static str> {
        vec!["bullet", "ul", "ol", "blank_lines"]
    }

    #[inline]
    fn aliases(&self) -> Vec<&'static str> {
        vec!["blanks-around-lists"]
    }

    #[inline]
    fn check(&self, doc: &Document) -> Result<Vec<Violation>> {
        let mut violations = vec![];
        let mut maybe_prev_node: Option<&'_ AstNode<'_>> = None;

        for node in doc.ast.children() {
            if let Some(prev_node) = maybe_prev_node {
                let position = node.data.borrow().sourcepos;
                let prev_position = prev_node.data.borrow().sourcepos;

                if let NodeValue::List(_) = prev_node.data.borrow().value {
                    if position.start.line == prev_position.end.line + 1
                        && prev_position.end.column != 0
                    {
                        let violation = self.to_violation(doc.path.clone(), position);
                        violations.push(violation);
                    }
                }

                if let NodeValue::List(_) = node.data.borrow().value {
                    if position.start.line == prev_position.end.line + 1
                        && prev_position.end.column != 0
                    {
                        let violation = self.to_violation(doc.path.clone(), position);
                        violations.push(violation);
                    }
                }
            }

            // Check Paragraph in Item
            if let (NodeValue::List(_), Some(item_node)) =
                (&node.data.borrow().value, node.last_child())
            {
                if let Some(child_node) = item_node.last_child() {
                    for grandchild_node in child_node.descendants() {
                        let position = grandchild_node.data.borrow().sourcepos;
                        if position.start.column == 1 {
                            let violation = self.to_violation(doc.path.clone(), position);
                            violations.push(violation);
                        }
                    }
                }
            }

            maybe_prev_node = Some(node);
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
* Some
* List

1. Some
2. List
Some text"
            .to_owned();
        let path = Path::new("test.md").to_path_buf();
        let arena = Arena::new();
        let ast = parse_document(&arena, &text, &Options::default());
        let doc = Document {
            path: path.clone(),
            ast,
            text,
        };
        let rule = MD032::new();
        let actual = rule.check(&doc).unwrap();
        let expected = vec![
            rule.to_violation(path.clone(), Sourcepos::from((2, 1, 4, 0))),
            rule.to_violation(path, Sourcepos::from((7, 1, 7, 9))),
        ];
        assert_eq!(actual, expected);
    }

    #[test]
    fn check_errors2() {
        let text = "1. Some
2. List
   Text

   Text
Some text"
            .to_owned();
        let path = Path::new("test.md").to_path_buf();
        let arena = Arena::new();
        let ast = parse_document(&arena, &text, &Options::default());
        let doc = Document {
            path: path.clone(),
            ast,
            text,
        };
        let rule = MD032::new();
        let actual = rule.check(&doc).unwrap();
        let expected = vec![rule.to_violation(path, Sourcepos::from((6, 1, 6, 9)))];
        assert_eq!(actual, expected);
    }

    #[test]
    fn check_errors_nested() {
        let text = "1. Some
2. List
    * Item
Some text"
            .to_owned();
        let path = Path::new("test.md").to_path_buf();
        let arena = Arena::new();
        let ast = parse_document(&arena, &text, &Options::default());
        let doc = Document {
            path: path.clone(),
            ast,
            text,
        };
        let rule = MD032::new();
        let actual = rule.check(&doc).unwrap();
        let expected = vec![rule.to_violation(path, Sourcepos::from((4, 1, 4, 9)))];
        assert_eq!(actual, expected);
    }

    #[test]
    fn check_errors_nested2() {
        let text = "1. Some
2. List
   Text

   * Text
Some text"
            .to_owned();
        let path = Path::new("test.md").to_path_buf();
        let arena = Arena::new();
        let ast = parse_document(&arena, &text, &Options::default());
        let doc = Document {
            path: path.clone(),
            ast,
            text,
        };
        let rule = MD032::new();
        let actual = rule.check(&doc).unwrap();
        let expected = vec![rule.to_violation(path, Sourcepos::from((6, 1, 6, 9)))];
        assert_eq!(actual, expected);
    }

    #[test]
    fn check_errors_with_code_block() {
        let text = "    Indented code block
* Some
* List

1. Some
2. List
```
Fenced code block
```"
        .to_owned();
        let path = Path::new("test.md").to_path_buf();
        let arena = Arena::new();
        let ast = parse_document(&arena, &text, &Options::default());
        let doc = Document {
            path: path.clone(),
            ast,
            text,
        };
        let rule = MD032::new();
        let actual = rule.check(&doc).unwrap();
        let expected = vec![
            rule.to_violation(path.clone(), Sourcepos::from((2, 1, 4, 0))),
            rule.to_violation(path, Sourcepos::from((7, 1, 9, 3))),
        ];
        assert_eq!(actual, expected);
    }

    #[test]
    fn check_errors_with_code_block2() {
        let text = "1. Some
2. List
   Text

   Text
```
Fenced code block
```"
        .to_owned();
        let path = Path::new("test.md").to_path_buf();
        let arena = Arena::new();
        let ast = parse_document(&arena, &text, &Options::default());
        let doc = Document {
            path: path.clone(),
            ast,
            text,
        };
        let rule = MD032::new();
        let actual = rule.check(&doc).unwrap();
        let expected = vec![rule.to_violation(path, Sourcepos::from((6, 1, 8, 3)))];
        assert_eq!(actual, expected);
    }

    #[test]
    fn check_no_errors() {
        let text = "Some text

* Some
* List

1. Some
2. List

Some text"
            .to_owned();
        let path = Path::new("test.md").to_path_buf();
        let arena = Arena::new();
        let ast = parse_document(&arena, &text, &Options::default());
        let doc = Document { path, ast, text };
        let rule = MD032::new();
        let actual = rule.check(&doc).unwrap();
        let expected = vec![];
        assert_eq!(actual, expected);
    }

    #[test]
    fn check_no_errors_with_indented_text() {
        let text = "1. Some
2. List
   Some text"
            .to_owned();
        let path = Path::new("test.md").to_path_buf();
        let arena = Arena::new();
        let ast = parse_document(&arena, &text, &Options::default());
        let doc = Document { path, ast, text };
        let rule = MD032::new();
        let actual = rule.check(&doc).unwrap();
        let expected = vec![];
        assert_eq!(actual, expected);
    }

    #[test]
    fn check_no_errors_nested() {
        let text = "1. Some
2. List
    * Item

Some text"
            .to_owned();
        let path = Path::new("test.md").to_path_buf();
        let arena = Arena::new();
        let ast = parse_document(&arena, &text, &Options::default());
        let doc = Document {
            path: path.clone(),
            ast,
            text,
        };
        let rule = MD032::new();
        let actual = rule.check(&doc).unwrap();
        let expected = vec![];
        assert_eq!(actual, expected);
    }

    #[test]
    fn check_no_errors_nested2() {
        let text = "1. Some
2. List
   Text

   * Text

Some text"
            .to_owned();
        let path = Path::new("test.md").to_path_buf();
        let arena = Arena::new();
        let ast = parse_document(&arena, &text, &Options::default());
        let doc = Document {
            path: path.clone(),
            ast,
            text,
        };
        let rule = MD032::new();
        let actual = rule.check(&doc).unwrap();
        let expected = vec![];
        assert_eq!(actual, expected);
    }
}

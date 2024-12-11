use comrak::nodes::{AstNode, NodeValue};
use miette::Result;

use crate::{violation::Violation, Document};

use super::Rule;

#[derive(Default, Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct MD022;

impl MD022 {
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        Self {}
    }
}

impl Rule for MD022 {
    #[inline]
    fn name(&self) -> String {
        "MD022".to_owned()
    }

    #[inline]
    fn description(&self) -> String {
        "Headers should be surrounded by blank lines".to_owned()
    }

    #[inline]
    fn tags(&self) -> Vec<String> {
        vec!["headers".to_owned(), "blank_lines".to_owned()]
    }

    #[inline]
    fn aliases(&self) -> Vec<String> {
        vec!["blanks-around-headers".to_owned()]
    }

    #[inline]
    fn check(&self, doc: &Document) -> Result<Vec<Violation>> {
        let mut violations = vec![];
        let mut maybe_prev_node: Option<&AstNode> = None;

        for node in doc.ast.children() {
            if let Some(prev_node) = maybe_prev_node {
                let prev_position = prev_node.data.borrow().sourcepos;
                let position = node.data.borrow().sourcepos;

                match (
                    prev_node.data.borrow().value.clone(),
                    node.data.borrow().value.clone(),
                ) {
                    (NodeValue::Heading(_), _) => {
                        if position.start.line == prev_position.end.line + 1 {
                            let violation = self.to_violation(doc.path.clone(), prev_position);
                            violations.push(violation);
                        }
                    }
                    (_, NodeValue::Heading(_)) => {
                        // NOTE: Ignore column 0, as the List may end on the next line
                        if position.start.line == prev_position.end.line + 1
                            && prev_position.end.column != 0
                        {
                            let violation = self.to_violation(doc.path.clone(), position);
                            violations.push(violation);
                        }
                    }
                    _ => {}
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

    use super::*;

    #[test]
    fn check_errors_for_atx() {
        let text = "# Header 1
Some text

Some more text
## Header 2";
        let path = Path::new("test.md").to_path_buf();
        let arena = Arena::new();
        let ast = parse_document(&arena, text, &Options::default());
        let doc = Document {
            path: path.clone(),
            ast,
            text: text.to_string(),
        };
        let rule = MD022::new();
        let actual = rule.check(&doc).unwrap();
        let expected = vec![
            rule.to_violation(path.clone(), Sourcepos::from((1, 1, 1, 10))),
            rule.to_violation(path, Sourcepos::from((5, 1, 5, 11))),
        ];
        assert_eq!(actual, expected);
    }

    #[test]
    fn check_errors_for_setext() {
        let text = "Setext style H1
===============
Some text

```
Some code block
```
Setext style H2
---------------";
        let path = Path::new("test.md").to_path_buf();
        let arena = Arena::new();
        let ast = parse_document(&arena, text, &Options::default());
        let doc = Document {
            path: path.clone(),
            ast,
            text: text.to_string(),
        };
        let rule = MD022::new();
        let actual = rule.check(&doc).unwrap();
        let expected = vec![
            rule.to_violation(path.clone(), Sourcepos::from((1, 1, 2, 15))),
            rule.to_violation(path, Sourcepos::from((8, 1, 9, 15))),
        ];
        assert_eq!(actual, expected);
    }

    #[test]
    fn check_no_errors() {
        let text = "# Header 1

Some text

Some more text

## Header 2";
        let path = Path::new("test.md").to_path_buf();
        let arena = Arena::new();
        let ast = parse_document(&arena, text, &Options::default());
        let doc = Document {
            path,
            ast,
            text: text.to_string(),
        };
        let rule = MD022::new();
        let actual = rule.check(&doc).unwrap();
        let expected = vec![];
        assert_eq!(actual, expected);
    }

    #[test]
    fn check_no_errors_for_setext() {
        let text = "Setext style H1
===============

Some text

```
Some code block
```

Setext style H2
---------------";
        let path = Path::new("test.md").to_path_buf();
        let arena = Arena::new();
        let ast = parse_document(&arena, text, &Options::default());
        let doc = Document {
            path,
            ast,
            text: text.to_string(),
        };
        let rule = MD022::new();
        let actual = rule.check(&doc).unwrap();
        let expected = vec![];
        assert_eq!(actual, expected);
    }

    #[test]
    fn check_no_errors_for_list() {
        let text = "# Header 1

- Some list item
- Some more list item

## Header 2";
        let path = Path::new("test.md").to_path_buf();
        let arena = Arena::new();
        let ast = parse_document(&arena, text, &Options::default());
        let doc = Document {
            path,
            ast,
            text: text.to_string(),
        };
        let rule = MD022::new();
        let actual = rule.check(&doc).unwrap();
        let expected = vec![];
        assert_eq!(actual, expected);
    }
}

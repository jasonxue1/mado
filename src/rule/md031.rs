use comrak::nodes::{AstNode, NodeValue};
use miette::Result;

use crate::{violation::Violation, Document};

use super::RuleLike;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct MD031;

impl MD031 {
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        Self {}
    }
}

impl RuleLike for MD031 {
    #[inline]
    fn name(&self) -> String {
        "MD031".to_owned()
    }

    #[inline]
    fn description(&self) -> String {
        "Fenced code blocks should be surrounded by blank lines".to_owned()
    }

    #[inline]
    fn tags(&self) -> Vec<String> {
        vec!["code".to_owned(), "blank_lines".to_owned()]
    }

    #[inline]
    fn aliases(&self) -> Vec<String> {
        vec!["blanks-around-fences".to_owned()]
    }

    #[inline]
    fn check(&self, doc: &Document) -> Result<Vec<Violation>> {
        let mut violations = vec![];
        let mut maybe_prev_node: Option<&'_ AstNode<'_>> = None;

        for node in doc.ast.children() {
            if let Some(prev_node) = maybe_prev_node {
                let prev_position = prev_node.data.borrow().sourcepos;
                let position = node.data.borrow().sourcepos;

                if let NodeValue::CodeBlock(code) = &prev_node.data.borrow().value {
                    if code.fenced
                        && position.start.line == prev_position.end.line + 1
                        && prev_position.end.column != 0
                    {
                        let violation = self.to_violation(doc.path.clone(), position);
                        violations.push(violation);
                    }
                }

                if let NodeValue::CodeBlock(code) = &node.data.borrow().value {
                    if code.fenced
                        && position.start.line == prev_position.end.line + 1
                        && prev_position.end.column != 0
                    {
                        let violation = self.to_violation(doc.path.clone(), position);
                        violations.push(violation);
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
```
Code block
```

```
Another code block
```
Some more text"
            .to_owned();
        let path = Path::new("test.md").to_path_buf();
        let arena = Arena::new();
        let ast = parse_document(&arena, &text, &Options::default());
        let doc = Document {
            path: path.clone(),
            ast,
            text,
        };
        let rule = MD031::new();
        let actual = rule.check(&doc).unwrap();
        let expected = vec![
            rule.to_violation(path.clone(), Sourcepos::from((2, 1, 4, 3))),
            rule.to_violation(path, Sourcepos::from((9, 1, 9, 14))),
        ];
        assert_eq!(actual, expected);
    }

    #[test]
    fn check_no_errors() {
        let text = "Some text

```
Code block
```

```
Another code block
```

Some more text"
            .to_owned();
        let path = Path::new("test.md").to_path_buf();
        let arena = Arena::new();
        let ast = parse_document(&arena, &text, &Options::default());
        let doc = Document { path, ast, text };
        let rule = MD031::new();
        let actual = rule.check(&doc).unwrap();
        let expected = vec![];
        assert_eq!(actual, expected);
    }

    #[test]
    fn check_no_errors_indented() {
        let text = "Some text
    Code block

    Another code block
Some more text"
            .to_owned();
        let path = Path::new("test.md").to_path_buf();
        let arena = Arena::new();
        let ast = parse_document(&arena, &text, &Options::default());
        let doc = Document { path, ast, text };
        let rule = MD031::new();
        let actual = rule.check(&doc).unwrap();
        let expected = vec![];
        assert_eq!(actual, expected);
    }

    #[test]
    fn check_no_errors_with_list() {
        let text = "* List

```
Code block
```

```
Another code block
```

Some more text"
            .to_owned();
        let path = Path::new("test.md").to_path_buf();
        let arena = Arena::new();
        let ast = parse_document(&arena, &text, &Options::default());
        let doc = Document { path, ast, text };
        let rule = MD031::new();
        let actual = rule.check(&doc).unwrap();
        let expected = vec![];
        assert_eq!(actual, expected);
    }
}

use comrak::nodes::{AstNode, NodeValue};
use miette::Result;

use crate::{violation::Violation, Document};

use super::RuleLike;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct MD028;

impl MD028 {
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        Self {}
    }
}

impl RuleLike for MD028 {
    #[inline]
    fn name(&self) -> String {
        "MD028".to_owned()
    }

    #[inline]
    fn description(&self) -> String {
        "Blank line inside blockquote".to_owned()
    }

    #[inline]
    fn tags(&self) -> Vec<String> {
        vec!["blockquote".to_owned(), "whitespace".to_owned()]
    }

    #[inline]
    fn aliases(&self) -> Vec<String> {
        vec!["no-blanks-blockquote".to_owned()]
    }

    #[inline]
    fn check(&self, doc: &Document) -> Result<Vec<Violation>> {
        let mut violations = vec![];
        let mut maybe_prev_node: Option<&'_ AstNode<'_>> = None;

        for node in doc.ast.children() {
            if let Some(prev_node) = maybe_prev_node {
                if let (NodeValue::BlockQuote, NodeValue::BlockQuote) =
                    (&prev_node.data.borrow().value, &node.data.borrow().value)
                {
                    let position = node.data.borrow().sourcepos;
                    let violation = self.to_violation(doc.path.clone(), position);
                    violations.push(violation);
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
        let text = "> This is a blockquote
> which is immediately followed by

> this blockquote. Unfortunately
> In some parsers, these are treated as the same blockquote."
            .to_owned();
        let path = Path::new("test.md").to_path_buf();
        let arena = Arena::new();
        let ast = parse_document(&arena, &text, &Options::default());
        let doc = Document {
            path: path.clone(),
            ast,
            text,
        };
        let rule = MD028::new();
        let actual = rule.check(&doc).unwrap();
        let expected = vec![rule.to_violation(path, Sourcepos::from((4, 1, 5, 60)))];
        assert_eq!(actual, expected);
    }

    #[test]
    fn check_no_errors() {
        let text = "> This is a blockquote.
>
> This is the same blockquote."
            .to_owned();
        let path = Path::new("test.md").to_path_buf();
        let arena = Arena::new();
        let ast = parse_document(&arena, &text, &Options::default());
        let doc = Document { path, ast, text };
        let rule = MD028::new();
        let actual = rule.check(&doc).unwrap();
        let expected = vec![];
        assert_eq!(actual, expected);
    }

    #[test]
    fn check_no_errors_with_some_text_in_between() {
        let text = "> This is a blockquote.

And Jimmy also said:

> This too is a blockquote."
            .to_owned();
        let path = Path::new("test.md").to_path_buf();
        let arena = Arena::new();
        let ast = parse_document(&arena, &text, &Options::default());
        let doc = Document { path, ast, text };
        let rule = MD028::new();
        let actual = rule.check(&doc).unwrap();
        let expected = vec![];
        assert_eq!(actual, expected);
    }
}

use std::path::PathBuf;

use comrak::nodes::{AstNode, NodeValue, Sourcepos};
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

    fn check_recursive<'a>(
        &self,
        root: &'a AstNode<'a>,
        path: &PathBuf,
        violations: &mut Vec<Violation>,
    ) {
        let mut maybe_prev_node: Option<&'_ AstNode<'_>> = None;

        for node in root.children() {
            if let Some(prev_node) = maybe_prev_node {
                if let (NodeValue::BlockQuote, NodeValue::BlockQuote) =
                    (&prev_node.data.borrow().value, &node.data.borrow().value)
                {
                    let prev_position = prev_node.data.borrow().sourcepos;
                    let position = node.data.borrow().sourcepos;
                    let blank_line_position = Sourcepos::from((
                        prev_position.end.line + 1,
                        1,
                        position.start.line - 1,
                        1,
                    ));
                    let violation = self.to_violation(path.clone(), blank_line_position);
                    violations.push(violation);
                }
            }

            if let NodeValue::List(_) = node.data.borrow().value {
                for item_node in node.children() {
                    self.check_recursive(item_node, path, violations);
                }
            }

            maybe_prev_node = Some(node);
        }
    }
}

impl RuleLike for MD028 {
    #[inline]
    fn name(&self) -> &'static str {
        "MD028"
    }

    #[inline]
    fn description(&self) -> &'static str {
        "Blank line inside blockquote"
    }

    #[inline]
    fn tags(&self) -> Vec<&'static str> {
        vec!["blockquote", "whitespace"]
    }

    #[inline]
    fn aliases(&self) -> Vec<&'static str> {
        vec!["no-blanks-blockquote"]
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
        let text = "Some text

> a quote
> same quote

> blank line above this


> two blank lines above this
 
> space above this

* List with embedded blockquote

  > Test
  > Test

  > Test

* Item 2

  > Test. The blank line below should _not_ trigger MD028 as one blockquote is
  > inside the list, and the other is outside it.

> Test"
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
        let expected = vec![
            rule.to_violation(path.clone(), Sourcepos::from((5, 1, 5, 1))),
            // NOTE: This ranged result may differ from mdl
            rule.to_violation(path.clone(), Sourcepos::from((7, 1, 8, 1))),
            rule.to_violation(path.clone(), Sourcepos::from((10, 1, 10, 1))),
            rule.to_violation(path, Sourcepos::from((17, 1, 17, 1))),
        ];
        assert_eq!(actual, expected);
    }

    #[test]
    fn check_no_errors() {
        let text = "Some text

> a quote
> same quote
>
> blank line above this
>
>
> two blank lines above this
> 
> space above this

* List with embedded blockquote

  > Test
  > Test
  >
  > Test

* Item 2

  > Test. The blank line below should _not_ trigger MD028 as one blockquote is
  > inside the list, and the other is outside it.

> Test"
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

    // NOTE: This case may differ from mdl
    // #[test]
    // fn check_no_errors_nested() {
    //     let text = "* List
    // > This is a blockquote
    // > which is immediately followed by
    //
    // > this blockquote. Unfortunately
    // > In some parsers, these are treated as the same blockquote."
    //         .to_owned();
    //     let path = Path::new("test.md").to_path_buf();
    //     let arena = Arena::new();
    //     let ast = parse_document(&arena, &text, &Options::default());
    //     let doc = Document {
    //         path: path.clone(),
    //         ast,
    //         text,
    //     };
    //     let rule = MD028::new();
    //     let actual = rule.check(&doc).unwrap();
    //     let expected = vec![];
    //     assert_eq!(actual, expected);
    // }
}

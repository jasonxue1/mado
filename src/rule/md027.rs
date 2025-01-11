use std::collections::HashSet;

use comrak::nodes::{AstNode, NodeValue};
use miette::Result;

use crate::{violation::Violation, Document};

use super::{
    node::{NodeContext, NodeValueMatcher},
    NewRuleLike, Rule, RuleLike, RuleMetadata,
};

#[derive(Debug, Default, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct MD027;

impl MD027 {
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        Self {}
    }
}

impl RuleLike for MD027 {
    #[inline]
    fn name(&self) -> &'static str {
        "MD027"
    }

    #[inline]
    fn description(&self) -> &'static str {
        "Multiple spaces after blockquote symbol"
    }

    #[inline]
    fn tags(&self) -> Vec<&'static str> {
        vec!["blockquote", "whitespace", "indentation"]
    }

    #[inline]
    fn aliases(&self) -> Vec<&'static str> {
        vec!["no-multiple-space-blockquote"]
    }

    #[inline]
    fn check(&self, doc: &Document) -> Result<Vec<Violation>> {
        let mut violations = vec![];

        for node in doc.ast.descendants() {
            if let NodeValue::BlockQuote = node.data.borrow().value {
                if let Some(child_node) = node.first_child() {
                    match &child_node.data.borrow().value {
                        NodeValue::Paragraph => {
                            let mut lines = HashSet::new();
                            for inline_node in child_node.children() {
                                let block_quote_position = node.data.borrow().sourcepos;
                                let inline_position = inline_node.data.borrow().sourcepos;
                                let lineno = inline_position.start.line;

                                // NOTE: `code` pos is wrong: https://github.com/kivikakk/comrak/issues/503
                                let is_code =
                                    matches!(inline_node.data.borrow().value, NodeValue::Code(_));
                                let expected_column = if is_code {
                                    block_quote_position.start.column + 3
                                } else {
                                    block_quote_position.start.column + 2
                                };

                                if inline_position.start.column > expected_column
                                    && !lines.contains(&lineno)
                                {
                                    let violation =
                                        self.to_violation(doc.path.clone(), inline_position);
                                    violations.push(violation);
                                }

                                lines.insert(lineno);
                            }
                        }
                        NodeValue::List(_) => {
                            for item_node in child_node.children() {
                                let block_quote_position = node.data.borrow().sourcepos;
                                let item_position = item_node.data.borrow().sourcepos;
                                let expected_column = block_quote_position.start.column + 2;

                                if item_position.start.column > expected_column {
                                    let violation =
                                        self.to_violation(doc.path.clone(), item_position);
                                    violations.push(violation);
                                }
                            }
                        }
                        _ => {
                            // TODO: Support multi-line errors
                            let parent_position = node.data.borrow().sourcepos;
                            let child_position = child_node.data.borrow().sourcepos;
                            if child_position.start.column > parent_position.start.column + 2 {
                                let violation = self.to_violation(doc.path.clone(), child_position);
                                violations.push(violation);
                            }
                        }
                    }
                }
            }
        }

        Ok(violations)
    }
}

impl NewRuleLike for MD027 {
    #[inline]
    fn metadata(&self) -> RuleMetadata {
        RuleMetadata {
            name: "MD027",
            description: "Multiple spaces after blockquote symbol",
            tags: vec!["blockquote", "whitespace", "indentation"],
            aliases: vec!["no-multiple-space-blockquote"],
        }
    }

    #[inline]
    fn reset(&mut self) {}
}

impl<'a> Rule<&NodeContext, &'a AstNode<'a>, NodeValueMatcher> for MD027 {
    #[inline]
    fn matcher(&self) -> NodeValueMatcher {
        NodeValueMatcher::new(|node| matches!(node, NodeValue::BlockQuote))
    }

    #[inline]
    fn run(&mut self, ctx: &NodeContext, node: &'a AstNode<'a>) -> Result<Vec<Violation>> {
        let mut violations = vec![];

        if let Some(child_node) = node.first_child() {
            let block_quote_position = node.data.borrow().sourcepos;
            let child_position = child_node.data.borrow().sourcepos;

            if child_position.start.column > block_quote_position.start.column + 2 {
                let violation = self.to_violation(ctx.path.clone(), child_position);
                violations.push(violation);
            }
        }

        Ok(violations)
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use comrak::{nodes::Sourcepos, Arena};
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn check_errors_paragraph() {
        let text = ">  Indented text
>  More indented
> Not indented
>  *Emph* and text
>  **Strong** and text
>  `code` and text
>  [link](https://example.com) and text"
            .to_owned();
        let path = Path::new("test.md").to_path_buf();
        let arena = Arena::new();
        let doc = Document::new(&arena, path.clone(), text).unwrap();
        let rule = MD027::new();
        let actual = rule.check(&doc).unwrap();
        let expected = vec![
            rule.to_violation(path.clone(), Sourcepos::from((1, 4, 1, 16))),
            rule.to_violation(path.clone(), Sourcepos::from((2, 4, 2, 16))),
            rule.to_violation(path.clone(), Sourcepos::from((4, 4, 4, 9))),
            rule.to_violation(path.clone(), Sourcepos::from((5, 4, 5, 13))),
            rule.to_violation(path.clone(), Sourcepos::from((6, 5, 6, 8))),
            rule.to_violation(path.clone(), Sourcepos::from((7, 4, 7, 30))),
        ];
        assert_eq!(actual, expected);
    }

    #[test]
    fn check_errors_list() {
        let text = ">  * foo
> * bar
>   * baz
>  * quz"
            .to_owned();
        let path = Path::new("test.md").to_path_buf();
        let arena = Arena::new();
        let doc = Document::new(&arena, path.clone(), text).unwrap();
        let rule = MD027::new();
        let actual = rule.check(&doc).unwrap();
        let expected = vec![
            rule.to_violation(path.clone(), Sourcepos::from((1, 4, 1, 8))),
            rule.to_violation(path.clone(), Sourcepos::from((4, 4, 4, 8))),
        ];
        assert_eq!(actual, expected);
    }

    #[test]
    fn check_errors_code_block() {
        let text = ">  ```
>  foo
> bar
>  baz
>  ```"
            .to_owned();
        let path = Path::new("test.md").to_path_buf();
        let arena = Arena::new();
        let doc = Document::new(&arena, path.clone(), text).unwrap();
        let rule = MD027::new();
        let actual = rule.check(&doc).unwrap();
        let expected = vec![
            rule.to_violation(path.clone(), Sourcepos::from((1, 4, 5, 6))),
            // TODO: This results are expected
            // rule.to_violation(path.clone(), Sourcepos::from((1, 4, 1, 6))),
            // rule.to_violation(path.clone(), Sourcepos::from((2, 4, 1, 6))),
            // rule.to_violation(path.clone(), Sourcepos::from((4, 4, 4, 6))),
            // rule.to_violation(path, Sourcepos::from((5, 4, 5, 6))),
        ];
        assert_eq!(actual, expected);
    }

    #[test]
    fn check_errors_html_block() {
        let text = ">  <div>
> <p>some text</p>
>   </div>"
            .to_owned();
        let path = Path::new("test.md").to_path_buf();
        let arena = Arena::new();
        let doc = Document::new(&arena, path.clone(), text).unwrap();
        let rule = MD027::new();
        let actual = rule.check(&doc).unwrap();
        let expected = vec![
            rule.to_violation(path.clone(), Sourcepos::from((1, 4, 3, 10))),
            // TODO: This results are expected
            // rule.to_violation(path.clone(), Sourcepos::from((1, 4, 1, 8))),
            // rule.to_violation(path, Sourcepos::from((3, 4, 3, 9))),
        ];
        assert_eq!(actual, expected);
    }

    // NOTE: This case is not an error in markdownlint
    #[test]
    fn check_errors_with_nested_block_quotes() {
        let text = ">>>  This is multiple blockquotes with bad indentation.
>>> More multiple blockquotes with good indentation.
>>>  More multiple blockquotes with bad indentation."
            .to_owned();
        let path = Path::new("test.md").to_path_buf();
        let arena = Arena::new();
        let doc = Document::new(&arena, path.clone(), text).unwrap();
        let rule = MD027::new();
        let actual = rule.check(&doc).unwrap();
        let expected = vec![
            rule.to_violation(path.clone(), Sourcepos::from((1, 6, 1, 55))),
            rule.to_violation(path.clone(), Sourcepos::from((3, 6, 3, 52))),
        ];
        assert_eq!(actual, expected);
    }

    #[test]
    fn check_errors_with_nested_block_quotes2() {
        let text = ">  >  >  This is multiple blockquote with bad indentation.
> > > More multiple blockquote with good indentation.
>  >  >  More multiple blockquote with bad indentation."
            .to_owned();
        let path = Path::new("test.md").to_path_buf();
        let arena = Arena::new();
        let doc = Document::new(&arena, path.clone(), text).unwrap();
        let rule = MD027::new();
        let actual = rule.check(&doc).unwrap();
        let expected = vec![
            rule.to_violation(path.clone(), Sourcepos::from((1, 4, 3, 55))),
            rule.to_violation(path.clone(), Sourcepos::from((1, 7, 3, 55))),
            rule.to_violation(path.clone(), Sourcepos::from((1, 10, 1, 58))),
            rule.to_violation(path, Sourcepos::from((3, 10, 3, 55))),
            // TODO: This results are expected
            // rule.to_violation(path.clone(), Sourcepos::from((1, 4, 1, 58))),
            // rule.to_violation(path.clone(), Sourcepos::from((1, 7, 1, 58))),
            // rule.to_violation(path.clone(), Sourcepos::from((1, 10, 1, 58))),
            // rule.to_violation(path.clone(), Sourcepos::from((3, 4, 3, 55))),
            // rule.to_violation(path.clone(), Sourcepos::from((3, 7, 3, 55))),
            // rule.to_violation(path, Sourcepos::from((3, 10, 3, 55))),
        ];
        assert_eq!(actual, expected);
    }

    #[test]
    fn check_no_errors_paragraph() {
        let text = "> Text
> More text
> *Emph* and text
> **Strong** and text
> `code` and text
> [link](https://example.com) and text"
            .to_owned();
        let path = Path::new("test.md").to_path_buf();
        let arena = Arena::new();
        let doc = Document::new(&arena, path, text).unwrap();
        let rule = MD027::new();
        let actual = rule.check(&doc).unwrap();
        let expected = vec![];
        assert_eq!(actual, expected);
    }

    #[test]
    fn check_no_errors_list() {
        let text = "> * foo
> * bar
>   * baz
> * quz"
            .to_owned();
        let path = Path::new("test.md").to_path_buf();
        let arena = Arena::new();
        let doc = Document::new(&arena, path, text).unwrap();
        let rule = MD027::new();
        let actual = rule.check(&doc).unwrap();
        let expected = vec![];
        assert_eq!(actual, expected);
    }

    #[test]
    fn check_no_errors_code_block() {
        let text = "> ```
> foo
> bar
> baz
> ```"
            .to_owned();
        let path = Path::new("test.md").to_path_buf();
        let arena = Arena::new();
        let doc = Document::new(&arena, path, text).unwrap();
        let rule = MD027::new();
        let actual = rule.check(&doc).unwrap();
        let expected = vec![];
        assert_eq!(actual, expected);
    }

    #[test]
    fn check_no_errors_html_block() {
        let text = "> <div>
> <p>some text</p>
> </div>"
            .to_owned();
        let path = Path::new("test.md").to_path_buf();
        let arena = Arena::new();
        let doc = Document::new(&arena, path, text).unwrap();
        let rule = MD027::new();
        let actual = rule.check(&doc).unwrap();
        let expected = vec![];
        assert_eq!(actual, expected);
    }

    #[test]
    fn check_no_errors_with_nested_block_quotes() {
        let text = ">>> This is multiple blockquotes with good indentation.
>>> More multiple blockquotes with good indentation.
>>> More multiple blockquotes with good indentation."
            .to_owned();
        let path = Path::new("test.md").to_path_buf();
        let arena = Arena::new();
        let doc = Document::new(&arena, path, text).unwrap();
        let rule = MD027::new();
        let actual = rule.check(&doc).unwrap();
        let expected = vec![];
        assert_eq!(actual, expected);
    }

    #[test]
    fn check_no_errors_with_nested_block_quotes2() {
        let text = "> > > This is multiple blockquote with good indentation.
> > > More multiple blockquote with good indentation.
> > > More multiple blockquote with good indentation."
            .to_owned();
        let path = Path::new("test.md").to_path_buf();
        let arena = Arena::new();
        let doc = Document::new(&arena, path, text).unwrap();
        let rule = MD027::new();
        let actual = rule.check(&doc).unwrap();
        let expected = vec![];
        assert_eq!(actual, expected);
    }

    #[test]
    fn check_no_errors_with_nested_block_quotes3() {
        let text = ">>> This is multiple blockquote with good indentation.
    More multiple blockquote with good indentation.
    More multiple blockquote with good indentation."
            .to_owned();
        let path = Path::new("test.md").to_path_buf();
        let arena = Arena::new();
        let doc = Document::new(&arena, path, text).unwrap();
        let rule = MD027::new();
        let actual = rule.check(&doc).unwrap();
        let expected = vec![];
        assert_eq!(actual, expected);
    }
}

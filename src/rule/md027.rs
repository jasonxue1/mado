use comrak::nodes::NodeValue;
use miette::Result;

use crate::{violation::Violation, Document};

use super::RuleLike;

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
    fn name(&self) -> String {
        "MD027".to_owned()
    }

    #[inline]
    fn description(&self) -> String {
        "Multiple spaces after blockquote symbol".to_owned()
    }

    #[inline]
    fn tags(&self) -> Vec<String> {
        vec![
            "blockquote".to_owned(),
            "whitespace".to_owned(),
            "indentation".to_owned(),
        ]
    }

    #[inline]
    fn aliases(&self) -> Vec<String> {
        vec!["no-multiple-space-blockquote".to_owned()]
    }

    #[inline]
    fn check(&self, doc: &Document) -> Result<Vec<Violation>> {
        let mut violations = vec![];

        for node in doc.ast.children() {
            if let NodeValue::BlockQuote = node.data.borrow().value {
                let block_quote_position = node.data.borrow().sourcepos;

                for child in node.children() {
                    let child_position = child.data.borrow().sourcepos;

                    if child_position.start.column > block_quote_position.start.column + 2 {
                        let violation = self.to_violation(doc.path.clone(), child_position);
                        violations.push(violation);
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
        let text = ">  This is a block quote with bad indentation
>  there should only be one."
            .to_owned();
        let path = Path::new("test.md").to_path_buf();
        let arena = Arena::new();
        let ast = parse_document(&arena, &text, &Options::default());
        let doc = Document {
            path: path.clone(),
            ast,
            text,
        };
        let rule = MD027::new();
        let actual = rule.check(&doc).unwrap();
        let expected = vec![
            // TODO: Support multiple erros
            rule.to_violation(path.clone(), Sourcepos::from((1, 4, 2, 28))),
            // rule.to_violation(path.clone(), Sourcepos::from((1, 4, 1, 45))),
            // rule.to_violation(path, Sourcepos::from((2, 4, 2, 28))),
        ];
        assert_eq!(actual, expected);
    }

    // NOTE: This case is not an error in markdownlint
    // #[test]
    // fn check_errors_with_nested_block_quotes() {
    //     let text = ">>>  This is multiple blockquote with bad indentation.".to_owned();
    //     let path = Path::new("test.md").to_path_buf();
    //     let arena = Arena::new();
    //     let ast = parse_document(&arena, &text, &Options::default());
    //     let doc = Document {
    //         path: path.clone(),
    //         ast,
    //         text,
    //     };
    //     let rule = MD027::new();
    //     let actual = rule.check(&doc).unwrap();
    //     let expected = vec![rule.to_violation(path.clone(), Sourcepos::from((1, 6, 1, 49)))];
    //     assert_eq!(actual, expected);
    // }

    // NOTE: This case is not an error in markdownlint
    // #[test]
    // fn check_errors_with_nested_block_quotes2() {
    //     let text = ">>> This is multiple blockquote with bad
    // indentation.".to_owned();
    //     let path = Path::new("test.md").to_path_buf();
    //     let arena = Arena::new();
    //     let ast = parse_document(&arena, &text, &Options::default());
    //     let doc = Document {
    //         path: path.clone(),
    //         ast,
    //         text,
    //     };
    //     let rule = MD027::new();
    //     let actual = rule.check(&doc).unwrap();
    //     let expected = vec![rule.to_violation(path.clone(), Sourcepos::from((2, 5, 2, 17)))];
    //     assert_eq!(actual, expected);
    // }

    // TODO: Support this case
    // #[test]
    // fn check_errors_with_nested_block_quotes3() {
    //     let text = ">  >  >  This is multiple blockquote with bad indentation.".to_owned();
    //     let path = Path::new("test.md").to_path_buf();
    //     let arena = Arena::new();
    //     let ast = parse_document(&arena, &text, &Options::default());
    //     let doc = Document {
    //         path: path.clone(),
    //         ast,
    //         text,
    //     };
    //     let rule = MD027::new();
    //     let actual = rule.check(&doc).unwrap();
    //     let expected = vec![
    //         rule.to_violation(path.clone(), Sourcepos::from((1, 4, 1, 58))),
    //         rule.to_violation(path.clone(), Sourcepos::from((1, 7, 1, 58))),
    //         rule.to_violation(path, Sourcepos::from((1, 10, 1, 58))),
    //     ];
    //     assert_eq!(actual, expected);
    // }

    #[test]
    fn check_no_errors() {
        let text = "> This is a blockquote with correct
> indentation."
            .to_owned();
        let path = Path::new("test.md").to_path_buf();
        let arena = Arena::new();
        let ast = parse_document(&arena, &text, &Options::default());
        let doc = Document { path, ast, text };
        let rule = MD027::new();
        let actual = rule.check(&doc).unwrap();
        let expected = vec![];
        assert_eq!(actual, expected);
    }

    #[test]
    fn check_no_errors_with_nested_block_quotes() {
        let text = ">>> This is multiple blockquote with correct indentation.".to_owned();
        let path = Path::new("test.md").to_path_buf();
        let arena = Arena::new();
        let ast = parse_document(&arena, &text, &Options::default());
        let doc = Document { path, ast, text };
        let rule = MD027::new();
        let actual = rule.check(&doc).unwrap();
        let expected = vec![];
        assert_eq!(actual, expected);
    }

    #[test]
    fn check_no_errors_with_nested_block_quotes2() {
        let text = "> > > This is multiple blockquote with correct indentation.".to_owned();
        let path = Path::new("test.md").to_path_buf();
        let arena = Arena::new();
        let ast = parse_document(&arena, &text, &Options::default());
        let doc = Document { path, ast, text };
        let rule = MD027::new();
        let actual = rule.check(&doc).unwrap();
        let expected = vec![];
        assert_eq!(actual, expected);
    }
}

use comrak::nodes::{ListType, NodeList, NodeValue};
use miette::Result;
use serde::Deserialize;

use crate::violation::Violation;
use crate::Document;

use super::RuleLike;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "lowercase")]
#[non_exhaustive]
pub enum ListStyle {
    Consistent,
    Asterisk,
    Plus,
    Dash,
    // Sublist, // TODO
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct MD004 {
    style: ListStyle,
}

impl MD004 {
    #[inline]
    #[must_use]
    pub fn new(style: ListStyle) -> Self {
        Self { style }
    }
}

impl Default for MD004 {
    #[inline]
    fn default() -> Self {
        Self {
            style: ListStyle::Consistent,
        }
    }
}

impl RuleLike for MD004 {
    #[inline]
    fn name(&self) -> String {
        "MD004".to_owned()
    }

    #[inline]
    fn description(&self) -> String {
        "Unordered list style".to_owned()
    }

    #[inline]
    fn tags(&self) -> Vec<String> {
        vec!["bullet".to_owned(), "ul".to_owned()]
    }

    #[inline]
    fn aliases(&self) -> Vec<String> {
        vec!["ul-style".to_owned()]
    }

    #[inline]
    fn check(&self, doc: &Document) -> Result<Vec<Violation>> {
        let mut violations = vec![];
        let mut maybe_first_list_style = None;

        for node in doc.ast.descendants() {
            if let NodeValue::Item(
                item @ NodeList {
                    list_type: ListType::Bullet,
                    ..
                },
            ) = node.data.borrow().value
            {
                match self.style {
                    ListStyle::Consistent => match maybe_first_list_style {
                        Some(first_list_style) => {
                            if item.bullet_char != first_list_style {
                                let position = node.data.borrow().sourcepos;
                                let violation = self.to_violation(doc.path.clone(), position);
                                violations.push(violation);
                            }
                        }
                        None => maybe_first_list_style = Some(item.bullet_char),
                    },
                    ListStyle::Asterisk if item.bullet_char != b'*' => {
                        let position = node.data.borrow().sourcepos;
                        let violation = self.to_violation(doc.path.clone(), position);
                        violations.push(violation);
                    }
                    ListStyle::Plus if item.bullet_char != b'+' => {
                        let position = node.data.borrow().sourcepos;
                        let violation = self.to_violation(doc.path.clone(), position);
                        violations.push(violation);
                    }
                    ListStyle::Dash if item.bullet_char != b'-' => {
                        let position = node.data.borrow().sourcepos;
                        let violation = self.to_violation(doc.path.clone(), position);
                        violations.push(violation);
                    }
                    _ => {}
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
    fn check_errors_for_consistent() {
        let text = "* Item 1
+ Item 2
- Item 3"
            .to_owned();
        let path = Path::new("test.md").to_path_buf();
        let arena = Arena::new();
        let ast = parse_document(&arena, &text, &Options::default());
        let doc = Document {
            path: path.clone(),
            ast,
            text,
        };
        let rule = MD004::default();
        let actual = rule.check(&doc).unwrap();
        let expected = vec![
            rule.to_violation(path.clone(), Sourcepos::from((2, 1, 2, 8))),
            rule.to_violation(path, Sourcepos::from((3, 1, 3, 8))),
        ];
        assert_eq!(actual, expected);
    }

    #[test]
    fn check_no_errors_for_consistent() {
        let text = "* Item 1
* Item 2
* Item 3"
            .to_owned();
        let path = Path::new("test.md").to_path_buf();
        let arena = Arena::new();
        let ast = parse_document(&arena, &text, &Options::default());
        let doc = Document { path, ast, text };
        let rule = MD004::default();
        let actual = rule.check(&doc).unwrap();
        let expected = vec![];
        assert_eq!(actual, expected);
    }

    #[test]
    fn check_no_errors_for_asterisk() {
        let text = "* Item 1
* Item 2
* Item 3"
            .to_owned();
        let path = Path::new("test.md").to_path_buf();
        let arena = Arena::new();
        let ast = parse_document(&arena, &text, &Options::default());
        let doc = Document { path, ast, text };
        let rule = MD004::new(ListStyle::Asterisk);
        let actual = rule.check(&doc).unwrap();
        let expected = vec![];
        assert_eq!(actual, expected);
    }

    #[test]
    fn check_no_errors_for_plus() {
        let text = "+ Item 1
+ Item 2
+ Item 3"
            .to_owned();
        let path = Path::new("test.md").to_path_buf();
        let arena = Arena::new();
        let ast = parse_document(&arena, &text, &Options::default());
        let doc = Document { path, ast, text };
        let rule = MD004::new(ListStyle::Plus);
        let actual = rule.check(&doc).unwrap();
        let expected = vec![];
        assert_eq!(actual, expected);
    }

    #[test]
    fn check_no_errors_for_dash() {
        let text = "- Item 1
- Item 2
- Item 3"
            .to_owned();
        let path = Path::new("test.md").to_path_buf();
        let arena = Arena::new();
        let ast = parse_document(&arena, &text, &Options::default());
        let doc = Document { path, ast, text };
        let rule = MD004::new(ListStyle::Dash);
        let actual = rule.check(&doc).unwrap();
        let expected = vec![];
        assert_eq!(actual, expected);
    }

    // NOTE: This test case is marked as a violation in markdownlint
    #[test]
    fn check_errors_with_blockquote() {
        let text = ">- Item 1
>- Item 2
>- Item 3"
            .to_owned();
        let path = Path::new("test.md").to_path_buf();
        let arena = Arena::new();
        let ast = parse_document(&arena, &text, &Options::default());
        let doc = Document {
            path: path.clone(),
            ast,
            text,
        };
        let rule = MD004::default();
        let actual = rule.check(&doc).unwrap();
        let expected = vec![];
        assert_eq!(actual, expected);
    }
}
